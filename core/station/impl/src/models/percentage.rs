use orbit_essentials::storable;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Percentage(pub u16);

impl TryFrom<u16> for Percentage {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > 100 {
            return Err(anyhow::anyhow!(
                "invalid percentage value, must be between >= 0 and <= 100"
            ));
        }

        Ok(Percentage(value))
    }
}

impl From<Percentage> for u16 {
    fn from(percentage: Percentage) -> u16 {
        percentage.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_try_from() {
        let percentage: Percentage = 50.try_into().unwrap();
        assert_eq!(percentage.0, 50);

        let percentage: Percentage = 100.try_into().unwrap();
        assert_eq!(percentage.0, 100);

        let percentage: Percentage = 0.try_into().unwrap();
        assert_eq!(percentage.0, 0);

        let result = Percentage::try_from(101);
        assert!(result.is_err());
    }

    #[test]
    fn test_into() {
        let percentage = Percentage(50);
        let value: u16 = percentage.into();
        assert_eq!(value, 50);
    }
}
