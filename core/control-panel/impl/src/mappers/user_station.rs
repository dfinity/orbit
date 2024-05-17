use crate::models::UserStation;

impl From<UserStation> for control_panel_api::UserStationDTO {
    fn from(user_station: UserStation) -> Self {
        control_panel_api::UserStationDTO {
            canister_id: user_station.canister_id,
            name: user_station.name,
            labels: Vec::new(),
        }
    }
}

impl From<control_panel_api::UserStationDTO> for UserStation {
    fn from(dto: control_panel_api::UserStationDTO) -> Self {
        UserStation {
            canister_id: dto.canister_id,
            name: dto.name,
            labels: Vec::new(),
        }
    }
}

pub trait UpdateUserStationInputInto {
    fn into_user_station(self) -> (Option<u64>, UserStation);
}

impl UpdateUserStationInputInto for control_panel_api::UpdateUserStationInput {
    fn into_user_station(self) -> (Option<u64>, UserStation) {
        (self.index, UserStation::from(self.station))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    #[test]
    fn correct_dto_to_model_mapping() {
        let dto = control_panel_api::UserStationDTO {
            canister_id: Principal::from_text("wkt3w-3iaaa-aaaaa-774ba-cai").unwrap(),
            name: "Station".to_string(),
            labels: Vec::new(),
        };

        let model = UserStation::from(dto.clone());

        assert_eq!(model.canister_id, dto.canister_id);
        assert_eq!(model.name, dto.name);
    }

    #[test]
    fn correct_model_to_dto_mapping() {
        let model = UserStation {
            canister_id: Principal::from_text("wkt3w-3iaaa-aaaaa-774ba-cai").unwrap(),
            name: "Station".to_string(),
            labels: Vec::new(),
        };

        let dto = control_panel_api::UserStationDTO::from(model.clone());

        assert_eq!(dto.canister_id, model.canister_id);
        assert_eq!(dto.name, model.name);
    }

    #[test]
    fn correct_update_user_station_input_into() {
        let index = Some(2);
        let station = control_panel_api::UserStationDTO {
            canister_id: Principal::from_slice(&[2; 29]),
            name: "Station".to_string(),
            labels: Vec::new(),
        };
        let input = control_panel_api::UpdateUserStationInput {
            index,
            station: station.clone(),
        };

        let (index, user_station) = input.into_user_station();

        assert_eq!(index, Some(2));
        assert_eq!(user_station.canister_id, station.canister_id);
        assert_eq!(user_station.name, station.name);
    }

    #[test]
    fn correct_update_user_station_input_into_with_no_index() {
        let index = None;
        let station = control_panel_api::UserStationDTO {
            canister_id: Principal::from_slice(&[2; 29]),
            name: "Station".to_string(),
            labels: Vec::new(),
        };
        let input = control_panel_api::UpdateUserStationInput {
            index,
            station: station.clone(),
        };

        let (index, user_station) = input.into_user_station();

        assert_eq!(index, None);
        assert_eq!(user_station.canister_id, station.canister_id);
        assert_eq!(user_station.name, station.name);
    }
}
