use crate::models::criteria::Percentage;

pub fn calculate_minimum_threshold(percentage: &Percentage, total_value: &usize) -> usize {
    let Percentage(percentage) = percentage;
    let percentage = *percentage as usize;
    let scaled_total = percentage * total_value;

    match scaled_total % 100 {
        0 => scaled_total / 100,
        _ => (scaled_total / 100) + 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_calculates_min_threshold() {
        assert_eq!(calculate_minimum_threshold(&Percentage(1), &1), 1);
        assert_eq!(calculate_minimum_threshold(&Percentage(100), &1), 1);
        assert_eq!(calculate_minimum_threshold(&Percentage(0), &10), 0);
        assert_eq!(calculate_minimum_threshold(&Percentage(1), &10), 1);
        assert_eq!(calculate_minimum_threshold(&Percentage(5), &10), 1);
        assert_eq!(calculate_minimum_threshold(&Percentage(50), &10), 5);
        assert_eq!(calculate_minimum_threshold(&Percentage(51), &10), 6);
        assert_eq!(calculate_minimum_threshold(&Percentage(100), &10), 10);
        assert_eq!(calculate_minimum_threshold(&Percentage(0), &0), 0);
        assert_eq!(calculate_minimum_threshold(&Percentage(50), &0), 0);
        assert_eq!(calculate_minimum_threshold(&Percentage(100), &0), 0);
    }
}
