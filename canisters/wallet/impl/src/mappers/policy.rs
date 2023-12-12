use super::HelperMapper;
use crate::models::{
    criteria::{Criteria, Ratio},
    specifier::UserSpecifier,
    EvaluationStatus,
};
use uuid::Uuid;
use wallet_api::{CriteriaDTO, EvaluationStatusDTO, UserSpecifierDTO};

impl From<Criteria> for CriteriaDTO {
    fn from(criteria: Criteria) -> Self {
        match criteria {
            Criteria::Auto(status) => match status {
                EvaluationStatus::Adopted => CriteriaDTO::Auto(EvaluationStatusDTO::Adopted),
                EvaluationStatus::Pending => CriteriaDTO::Auto(EvaluationStatusDTO::Pending),
                EvaluationStatus::Rejected => CriteriaDTO::Auto(EvaluationStatusDTO::Rejected),
            },
            Criteria::ApprovalThreshold(specifier, threshold) => {
                CriteriaDTO::ApprovalThreshold(specifier.into(), threshold.0)
            }
            Criteria::MinimumVotes(specifier, votes) => {
                CriteriaDTO::MinimumVotes(specifier.into(), votes)
            }
            Criteria::IsAddressKYC => CriteriaDTO::IsAddressKYC,
            Criteria::Or(criterias) => {
                CriteriaDTO::Or(criterias.into_iter().map(Into::into).collect())
            }
            Criteria::And(criterias) => {
                CriteriaDTO::And(criterias.into_iter().map(Into::into).collect())
            }
            Criteria::Not(criteria) => CriteriaDTO::Not(Box::new(Into::into(*criteria))),
        }
    }
}

impl From<CriteriaDTO> for Criteria {
    fn from(dto: CriteriaDTO) -> Self {
        match dto {
            CriteriaDTO::Auto(status) => match status {
                EvaluationStatusDTO::Adopted => Criteria::Auto(EvaluationStatus::Adopted),
                EvaluationStatusDTO::Pending => Criteria::Auto(EvaluationStatus::Pending),
                EvaluationStatusDTO::Rejected => Criteria::Auto(EvaluationStatus::Rejected),
            },
            CriteriaDTO::ApprovalThreshold(specifier, threshold) => {
                Criteria::ApprovalThreshold(specifier.into(), Ratio(threshold))
            }
            CriteriaDTO::MinimumVotes(specifier, votes) => {
                Criteria::MinimumVotes(specifier.into(), votes)
            }
            CriteriaDTO::IsAddressKYC => Criteria::IsAddressKYC,
            CriteriaDTO::Or(criterias) => {
                Criteria::Or(criterias.into_iter().map(Into::into).collect())
            }
            CriteriaDTO::And(criterias) => {
                Criteria::And(criterias.into_iter().map(Into::into).collect())
            }
            CriteriaDTO::Not(criteria) => Criteria::Not(Box::new(Into::into(*criteria))),
        }
    }
}

impl From<UserSpecifierDTO> for UserSpecifier {
    fn from(dto: UserSpecifierDTO) -> Self {
        match dto {
            UserSpecifierDTO::Any => UserSpecifier::Any,
            UserSpecifierDTO::Group(ids) => UserSpecifier::Group(
                ids.into_iter()
                    .map(|id| *HelperMapper::to_uuid(id).expect("invalid uuid").as_bytes())
                    .collect(),
            ),
            UserSpecifierDTO::Id(ids) => UserSpecifier::Id(
                ids.into_iter()
                    .map(|id| *HelperMapper::to_uuid(id).expect("invalid uuid").as_bytes())
                    .collect(),
            ),
            UserSpecifierDTO::Owner => UserSpecifier::Owner,
            UserSpecifierDTO::Proposer => UserSpecifier::Proposer,
        }
    }
}

impl From<UserSpecifier> for UserSpecifierDTO {
    fn from(specifier: UserSpecifier) -> Self {
        match specifier {
            UserSpecifier::Any => UserSpecifierDTO::Any,
            UserSpecifier::Group(ids) => UserSpecifierDTO::Group(
                ids.into_iter()
                    .map(|id| Uuid::from_bytes(id).hyphenated().to_string())
                    .collect::<Vec<_>>(),
            ),
            UserSpecifier::Id(ids) => UserSpecifierDTO::Id(
                ids.into_iter()
                    .map(|id| Uuid::from_bytes(id).hyphenated().to_string())
                    .collect::<Vec<_>>(),
            ),
            UserSpecifier::Owner => UserSpecifierDTO::Owner,
            UserSpecifier::Proposer => UserSpecifierDTO::Proposer,
        }
    }
}
