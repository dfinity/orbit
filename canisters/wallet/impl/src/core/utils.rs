use super::CallContext;
use crate::errors::AccessControlError;
use crate::{errors::PaginationError, models::criteria::Percentage};
use futures::StreamExt;
use futures::{stream, Future};
use std::pin::Pin;

pub const DEFAULT_PAGINATION_LIMIT: u16 = 10;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PaginatedData<T> {
    pub items: Vec<T>,
    pub next_offset: Option<u64>,
    pub total: u64,
}

pub struct PaginatedItemsArgs<'a, T> {
    pub offset: Option<u64>,
    pub limit: Option<u16>,
    pub default_limit: Option<u16>,
    pub max_limit: Option<u16>,
    pub items: &'a [T],
}

/// Paginates a list of items based on limit and offset.
pub fn paginated_items<T>(
    args: PaginatedItemsArgs<'_, T>,
) -> Result<PaginatedData<T>, PaginationError>
where
    T: Clone,
{
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

    let total = args.items.len();

    let next_offset = match (offset + limit) < total {
        true => Some((offset + limit) as u64),
        false => None,
    };

    let items = args
        .items
        .get(offset..std::cmp::min(offset + limit, total))
        .unwrap_or(&[])
        .to_vec();

    Ok(PaginatedData {
        items,
        next_offset,
        total: total as u64,
    })
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

/// Matches a date against a date range.
///
/// If the provided range is `None`, then the date is considered to be within the range.
pub(crate) fn match_date_range(date: &u64, start_dt: &Option<u64>, to_dt: &Option<u64>) -> bool {
    match (start_dt, to_dt) {
        (Some(start_dt), Some(to_dt)) => date >= start_dt && date <= to_dt,
        (Some(start_dt), None) => date >= start_dt,
        (None, Some(to_dt)) => date <= to_dt,
        (None, None) => true,
    }
}

pub(crate) async fn filter_accessible_resources<T, F>(
    ctx: &CallContext,
    items: &[T],
    evaluate_access: F,
) -> Vec<T>
where
    T: Clone + Send + 'static,
    F: Fn(&CallContext, &T) -> Pin<Box<dyn Future<Output = Result<(), AccessControlError>> + Send>>
        + Copy,
{
    stream::iter(items.iter())
        .filter_map(move |item| {
            let access_future = evaluate_access(ctx, item);
            async move {
                match access_future.await {
                    Ok(_) => Some(item.clone()),
                    Err(_) => None,
                }
            }
        })
        .collect()
        .await
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
            items: &[1; 10],
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
            items: &[1; 10],
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
            items: &[1; 10],
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
            items: &[1; 10],
        })
        .unwrap();

        assert_eq!(result.items.len(), 4);
        assert_eq!(result.next_offset, None);
    }
}
