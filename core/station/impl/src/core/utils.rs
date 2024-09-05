use candid::Principal;
use orbit_essentials::types::UUID;

use super::authorization::Authorization;
use super::CallContext;
use crate::errors::PaginationError;
use crate::models::resource::Resource;
use crate::models::Percentage;

pub const DEFAULT_PAGINATION_LIMIT: u16 = 10;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SortDirection {
    Asc,
    Desc,
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

/// Retains items based on the result of a authorization check evaluation.
///
/// This function will evaluate the access for each item in the list and retain only the
/// items for which the authorization check is successful.
pub(crate) fn retain_accessible_resources<T, F>(
    ctx: &CallContext,
    items: &mut Vec<T>,
    to_resource: F,
) where
    T: Clone,
    F: Fn(&T) -> Resource,
{
    let mut i = 0;
    while i < items.len() {
        let item = &items[i];
        let resource = to_resource(item);
        let is_allowed = Authorization::is_allowed(ctx, &resource);

        match is_allowed {
            true => i += 1,
            false => {
                items.remove(i);
            }
        }
    }
}

pub(crate) fn format_unique_string(text: &str) -> String {
    deunicode::deunicode(text).to_lowercase().replace(' ', "")
}

/// The minimum principal value that can be used.
pub const MIN_PRINCIPAL: Principal = Principal::from_slice(&[0; 29]);

/// The maximum principal value that can be used.
pub const MAX_PRINCIPAL: Principal = Principal::from_slice(&[255; 29]);

/// The minimum UUID value that can be used.
pub const MIN_UUID: UUID = [u8::MIN; 16];

/// The maximum UUID value that can be used.
pub const MAX_UUID: UUID = [u8::MAX; 16];

/// The last ordered string of the ASCII table that can be used, with the given size.
pub fn max_string_of_size(size: &usize) -> String {
    let mut max_string = String::new();
    for _ in 0..*size {
        max_string.push(char::MAX);
    }

    max_string
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

    #[test]
    fn should_format_unique_string() {
        assert_eq!(format_unique_string("áéíóúñç"), "aeiounc");
        assert_eq!(format_unique_string("àèìòù"), "aeiou");
        assert_eq!(format_unique_string("âêîôû"), "aeiou");
        assert_eq!(format_unique_string("äëïöü"), "aeiou");
        assert_eq!(format_unique_string("ÁÉÍÓÚÑÇ"), "aeiounc");
        assert_eq!(format_unique_string("ÀÈÌÒÙ"), "aeiou");
        assert_eq!(format_unique_string("ÂÊÎÔÛ"), "aeiou");
        assert_eq!(format_unique_string("ÄËÏÖÜ"), "aeiou");
        assert_eq!(format_unique_string("Hello, World!"), "hello,world!");
        assert_eq!(format_unique_string("Hello,  World!"), "hello,world!");
        assert_eq!(format_unique_string("Hello, World! "), "hello,world!");
    }

    #[test]
    fn should_return_max_string_of_size() {
        assert!(*max_string_of_size(&1) > *"");
        assert!(*max_string_of_size(&1) > *"a");
        assert!(*max_string_of_size(&1) > *"z");
        assert!(*max_string_of_size(&1) > *"A");
        assert!(*max_string_of_size(&1) > *"Z");
    }
}
