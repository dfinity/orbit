use crate::api::DetailableError;
use std::collections::HashMap;

pub const DEFAULT_PAGINATION_LIMIT: u16 = 100;

#[derive(Debug, thiserror::Error)]
pub enum PaginationError {
    /// Invalid max limit.
    #[error(r#"Invalid list limit, it cannot be more than {max}."#)]
    MaxLimitExceeded { max: u16 },
}

impl DetailableError for PaginationError {
    fn details(&self) -> Option<HashMap<String, String>> {
        let mut details = HashMap::new();
        match self {
            PaginationError::MaxLimitExceeded { max } => {
                details.insert("max".to_string(), max.to_string());
                Some(details)
            }
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

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
