use crate::models::UserStation;
use control_panel_api::UserStationDTO;

impl From<UserStation> for UserStationDTO {
    fn from(user_station: UserStation) -> Self {
        UserStationDTO {
            canister_id: user_station.canister_id,
            name: user_station.name,
        }
    }
}

impl From<UserStationDTO> for UserStation {
    fn from(dto: UserStationDTO) -> Self {
        UserStation {
            canister_id: dto.canister_id,
            name: dto.name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    #[test]
    fn correct_dto_to_model_mapping() {
        let dto = UserStationDTO {
            canister_id: Principal::from_text("wkt3w-3iaaa-aaaaa-774ba-cai").unwrap(),
            name: "Station".to_string(),
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
        };

        let dto = UserStationDTO::from(model.clone());

        assert_eq!(dto.canister_id, model.canister_id);
        assert_eq!(dto.name, model.name);
    }
}
