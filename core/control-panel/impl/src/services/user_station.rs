use crate::{
    core::{ic_cdk::api::print, CallContext},
    models::{UserId, UserStation},
    repositories::{UserRepository, USER_REPOSITORY},
    services::{UserService, USER_SERVICE},
};
use candid::Principal;
use lazy_static::lazy_static;
use orbit_essentials::repository::Repository;
use orbit_essentials::{api::ServiceResult, model::ModelValidator};
use std::{collections::HashSet, sync::Arc};

lazy_static! {
    pub static ref USER_STATION_SERVICE: Arc<UserStationService> = Arc::new(
        UserStationService::new(Arc::clone(&USER_REPOSITORY), Arc::clone(&USER_SERVICE))
    );
}

#[derive(Default, Debug)]
pub struct UserStationService {
    user_repository: Arc<UserRepository>,
    user_service: Arc<UserService>,
}

impl UserStationService {
    pub fn new(user_repository: Arc<UserRepository>, user_service: Arc<UserService>) -> Self {
        Self {
            user_repository,
            user_service,
        }
    }

    /// Finds all stations associated with the user.
    ///
    /// If labels are provided, only stations with some of those labels are returned.
    pub fn list_stations(
        &self,
        user_id: &UserId,
        labels: &[String],
        ctx: &CallContext,
    ) -> ServiceResult<Vec<UserStation>> {
        let user = self.user_service.get_user(user_id, ctx)?;

        if labels.is_empty() {
            return Ok(user.stations);
        }

        let filter_by_labels = labels
            .iter()
            .map(|label| label.trim().to_ascii_lowercase())
            .collect::<HashSet<String>>();

        let stations = user
            .stations
            .iter()
            .filter(|station| {
                station
                    .labels
                    .iter()
                    .any(|label| filter_by_labels.contains(&label.to_ascii_lowercase()))
            })
            .cloned()
            .collect::<Vec<_>>();

        Ok(stations)
    }

    /// Adds the provided stations to the user.
    ///
    /// If a station with the same ID already exists, it is updated.
    pub fn add_stations(
        &self,
        user_id: &UserId,
        stations: Vec<UserStation>,
        ctx: &CallContext,
    ) -> ServiceResult<()> {
        let mut user = self.user_service.get_user(user_id, ctx)?;

        for new_station in stations {
            let station_index = user
                .stations
                .iter()
                .position(|user_station| user_station.canister_id == new_station.canister_id);

            match station_index {
                Some(index) => {
                    user.stations[index] = new_station;
                }
                None => {
                    user.stations.push(new_station);
                }
            }
        }

        user.validate()?;

        self.user_repository.insert(user.to_key(), user.clone());

        Ok(())
    }

    /// Removes stations with the given IDs from the user.
    pub fn remove_stations(
        &self,
        user_id: &UserId,
        station_ids: Vec<Principal>,
        ctx: &CallContext,
    ) -> ServiceResult<()> {
        let mut user = self.user_service.get_user(user_id, ctx)?;

        user.stations
            .retain(|station| !station_ids.contains(&station.canister_id));

        user.validate()?;

        self.user_repository.insert(user.to_key(), user.clone());

        Ok(())
    }

    /// Updates the user's stations with the provided list of stations.
    ///
    /// If a provided station is not found in the user's stations, it is ignored.
    pub fn update_stations(
        &self,
        user_id: &UserId,
        stations: Vec<(Option<u64>, UserStation)>,
        ctx: &CallContext,
    ) -> ServiceResult<()> {
        let mut user = self.user_service.get_user(user_id, ctx)?;

        for (maybe_new_index, new_station) in stations {
            let station_index = user
                .stations
                .iter()
                .position(|user_station| user_station.canister_id == new_station.canister_id);

            match station_index {
                Some(index) => {
                    user.stations[index] = new_station;

                    if let Some(mut new_index) = maybe_new_index {
                        new_index = new_index.min(user.stations.len() as u64 - 1);

                        user.stations.swap(index, new_index as usize);
                    }
                }
                None => print(format!("Station not found: {:?}", new_station.canister_id)),
            }
        }

        user.validate()?;

        self.user_repository.insert(user.to_key(), user.clone());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{user_model_utils::mock_user, user_station_model_utils::mock_user_station};

    #[test]
    fn test_add_stations() {
        let mut user = mock_user();
        user.stations = Vec::new();

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let ctx = CallContext::new(user.identity);
        let new_station = mock_user_station();

        USER_STATION_SERVICE
            .add_stations(&user.id, vec![new_station.clone()], &ctx)
            .unwrap();

        let updated_user = USER_REPOSITORY.get(&user.to_key()).unwrap();

        assert_eq!(updated_user.stations.len(), 1);
        assert_eq!(
            updated_user.stations[0].canister_id,
            new_station.canister_id
        );
        assert_eq!(updated_user.stations[0].name, new_station.name);
        assert_eq!(updated_user.stations[0].labels, new_station.labels);
    }

    #[test]
    fn test_update_stations() {
        let new_station = mock_user_station();
        let mut user = mock_user();
        user.stations = vec![new_station.clone()];

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let ctx = CallContext::new(user.identity);

        USER_STATION_SERVICE
            .add_stations(&user.id, vec![new_station.clone()], &ctx)
            .unwrap();

        let updated_station = UserStation {
            canister_id: new_station.canister_id,
            name: "Updated Station".to_string(),
            labels: Vec::new(),
        };

        USER_STATION_SERVICE
            .update_stations(&user.id, vec![(None, updated_station.clone())], &ctx)
            .unwrap();

        let updated_user = USER_REPOSITORY.get(&user.to_key()).unwrap();

        assert_eq!(updated_user.stations.len(), 1);
        assert_eq!(
            updated_user.stations[0].canister_id,
            updated_station.canister_id
        );
        assert_eq!(updated_user.stations[0].name, updated_station.name);
        assert_eq!(updated_user.stations[0].labels, updated_station.labels);
    }

    #[test]
    fn test_update_stations_with_reordering() {
        let first_station = mock_user_station();
        let last_station = mock_user_station();
        let mut stations = Vec::new();

        stations.push(first_station.clone());
        for _ in 0..3 {
            stations.push(mock_user_station());
        }
        stations.push(last_station.clone());

        let mut user = mock_user();
        user.stations = stations;

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let ctx = CallContext::new(user.identity);

        let initial_ordered_user_stations = USER_STATION_SERVICE
            .list_stations(&user.id, &[], &ctx)
            .unwrap();

        for (i, station) in initial_ordered_user_stations.iter().enumerate() {
            assert_eq!(station.canister_id, user.stations[i].canister_id);
        }

        USER_STATION_SERVICE
            .update_stations(&user.id, vec![(Some(0), last_station.clone())], &ctx)
            .unwrap();

        let updated_station_list = USER_STATION_SERVICE
            .list_stations(&user.id, &[], &ctx)
            .unwrap();

        assert_eq!(
            updated_station_list[0].canister_id,
            last_station.canister_id
        );

        assert_eq!(
            updated_station_list[updated_station_list.len() - 1].canister_id,
            first_station.canister_id
        );
    }

    #[test]
    fn test_remove_stations() {
        let new_station = mock_user_station();

        let mut user = mock_user();
        user.stations = vec![new_station.clone()];

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let ctx = CallContext::new(user.identity);

        USER_STATION_SERVICE
            .add_stations(&user.id, vec![new_station.clone()], &ctx)
            .unwrap();

        USER_STATION_SERVICE
            .remove_stations(&user.id, vec![new_station.canister_id], &ctx)
            .unwrap();

        let updated_user = USER_REPOSITORY.get(&user.to_key()).unwrap();

        assert_eq!(updated_user.stations.len(), 0);
    }

    #[test]
    fn test_update_checks_permissions() {
        let mut user = mock_user();
        user.identity = Principal::from_slice(&[1; 29]);

        let ctx = CallContext::new(Principal::from_slice(&[2; 29]));

        let result =
            USER_STATION_SERVICE.update_stations(&user.id, vec![(None, mock_user_station())], &ctx);

        assert!(result.is_err());
    }

    #[test]
    fn test_add_checks_permissions() {
        let mut user = mock_user();
        user.identity = Principal::from_slice(&[1; 29]);

        let ctx = CallContext::new(Principal::from_slice(&[2; 29]));

        let result = USER_STATION_SERVICE.add_stations(&user.id, vec![mock_user_station()], &ctx);

        assert!(result.is_err());
    }

    #[test]
    fn test_remove_checks_permissions() {
        let mut user = mock_user();
        user.identity = Principal::from_slice(&[1; 29]);

        let ctx = CallContext::new(Principal::from_slice(&[2; 29]));

        let result =
            USER_STATION_SERVICE.remove_stations(&user.id, vec![Principal::anonymous()], &ctx);

        assert!(result.is_err());
    }

    #[test]
    fn test_list_checks_permissions() {
        let mut user = mock_user();
        user.identity = Principal::from_slice(&[1; 29]);

        let ctx = CallContext::new(Principal::from_slice(&[2; 29]));

        let result = USER_STATION_SERVICE.list_stations(&user.id, &[], &ctx);

        assert!(result.is_err());
    }

    #[test]
    fn test_list_with_labels() {
        let mut new_station = mock_user_station();
        new_station.labels = vec!["label1".to_string()];

        let mut station_to_keep = mock_user_station();
        station_to_keep.labels = vec!["label2".to_string()];

        let mut user = mock_user();
        user.stations = vec![new_station.clone(), station_to_keep.clone()];

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let ctx = CallContext::new(user.identity);

        let stations = USER_STATION_SERVICE
            .list_stations(&user.id, &["Label1".to_string()], &ctx)
            .unwrap();

        assert_eq!(stations.len(), 1);
    }
}
