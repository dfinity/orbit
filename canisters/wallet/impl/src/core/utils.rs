use crate::{errors::PaginationError, models::criteria::Percentage};

pub const DEFAULT_PAGINATION_LIMIT: u16 = 10;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PaginatedData<T> {
    pub items: Vec<T>,
    pub next_offset: Option<u64>,
}

pub struct PaginatedItemsArgs<'a, T> {
    pub offset: Option<u64>,
    pub limit: Option<u16>,
    pub default_limit: Option<u16>,
    pub max_limit: Option<u16>,
    pub items: Box<dyn Fn() -> Vec<T> + 'a>,
}

/// Paginates a list of items based on limit and offset.
pub fn paginated_items<T>(
    args: PaginatedItemsArgs<'_, T>,
) -> Result<PaginatedData<T>, PaginationError> {
    let offset = args.offset.unwrap_or(0) as usize;

    if let (Some(max_limit), Some(limit)) = (args.max_limit, args.limit) {
        if limit > max_limit {
            Err(PaginationError::MaxLimitExceeded { max: max_limit })?;
        }
    }

    let default_limit = args.default_limit.unwrap_or(match args.max_limit {
        Some(max_limit) => max_limit,
        None => DEFAULT_PAGINATION_LIMIT,
    });
    let limit = args.limit.unwrap_or(default_limit) as usize;

    let items = (args.items)();
    let total = items.len();

    let next_offset = match (offset + limit) < total {
        true => Some((offset + limit) as u64),
        false => None,
    };

    let items = items.into_iter().skip(offset).take(limit).collect();

    Ok(PaginatedData { items, next_offset })
}

/// Calculates the minimum threshold for a given percentage and total value.
///
/// This only works for percentages between 0 and 100 (inclusive).
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

    #[test]
    fn paginated_items_should_fail_when_limit_is_greater_than_max() {
        let result = paginated_items(PaginatedItemsArgs {
            offset: None,
            limit: Some(10),
            default_limit: None,
            max_limit: Some(1),
            items: Box::new(|| vec![1; 10]),
        });

        assert!(result.is_err());
    }

    #[test]
    fn paginated_items_should_return_max_limit_by_default() {
        let result = paginated_items(PaginatedItemsArgs {
            offset: None,
            limit: None,
            default_limit: None,
            max_limit: Some(5),
            items: Box::new(|| vec![1; 10]),
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap().items.len(), 5);
    }

    #[test]
    fn paginated_items_should_return_next_offset_when_there_are_more_items() {
        let result = paginated_items(PaginatedItemsArgs {
            offset: None,
            limit: Some(5),
            default_limit: None,
            max_limit: None,
            items: Box::new(|| vec![1; 10]),
        })
        .unwrap();

        assert_eq!(result.items.len(), 5);
        assert_eq!(result.next_offset, Some(5));
    }

    #[test]
    fn paginated_items_should_filter_by_offset() {
        let result = paginated_items(PaginatedItemsArgs {
            offset: Some(6),
            limit: Some(5),
            default_limit: None,
            max_limit: None,
            items: Box::new(|| vec![1; 10]),
        })
        .unwrap();

        assert_eq!(result.items.len(), 4);
        assert_eq!(result.next_offset, None);
    }
}
