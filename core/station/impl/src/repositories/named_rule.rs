use crate::core::ic_cdk::api::print;
use crate::core::utils::format_unique_string;
use crate::models::indexes::unique_index::UniqueIndexKey;
use crate::models::NamedRuleId;
use crate::{
    core::{cache::Cache, with_memory_manager, Memory, NAMED_RULE_MEMORY_ID},
    models::{NamedRule, NamedRuleKey},
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::repository::{IndexedRepository, Repository, StableDb};
use std::{cell::RefCell, sync::Arc};

use super::indexes::unique_index::UniqueIndexRepository;

thread_local! {
  /// The memory reference to the NamedRule repository.
  static DB: RefCell<StableBTreeMap<NamedRuleKey, NamedRule, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(NAMED_RULE_MEMORY_ID))
    )
  });

  static CACHE: RefCell<Cache<NamedRuleId, NamedRule>> = RefCell::new(Cache::new(NamedRuleRepository::MAX_CACHE_SIZE));

}

lazy_static! {
    pub static ref NAMED_RULE_REPOSITORY: Arc<NamedRuleRepository> =
        Arc::new(NamedRuleRepository::default());
}

#[derive(Default, Debug)]
pub struct NamedRuleRepository {
    unique_index: UniqueIndexRepository,
}

impl NamedRuleRepository {
    pub const MAX_CACHE_SIZE: usize = 100_000;

    /// Checks if every named rule in the repository is in the cache.
    fn use_only_cache(&self) -> bool {
        self.len() <= Self::MAX_CACHE_SIZE
    }

    /// Builds the cache from the stable memory repository.
    ///
    /// This method should only be called during init or upgrade hooks to ensure that the cache is
    /// up-to-date with the repository and that we have enough instructions to rebuild the cache.
    pub fn build_cache(&self) {
        if self.len() > Self::MAX_CACHE_SIZE {
            print(format!(
                "Only the first {} named rules will be added to the cache, the reposity has {} named rules.",
                Self::MAX_CACHE_SIZE,
                NAMED_RULE_REPOSITORY.len(),
            ));
        }

        CACHE.with(|cache| {
            cache.borrow_mut().clear();

            DB.with(|db| {
                for (_, named_rule) in db.borrow().iter().take(Self::MAX_CACHE_SIZE) {
                    cache.borrow_mut().insert(named_rule.id, named_rule);
                }
            });
        });
    }

    // Returns the named rule associated with the given name if it exists.
    pub fn find_by_name(&self, name: &str) -> Option<NamedRuleId> {
        self.unique_index
            .get(&UniqueIndexKey::NamedRuleName(format_unique_string(name)))
    }

    pub fn exists_unique(&self, name: &str) -> Option<NamedRuleId> {
        let key = NamedRule::to_unique_index_key(name);
        self.unique_index.get(&key)
    }
}

impl StableDb<NamedRuleKey, NamedRule, VirtualMemory<Memory>> for NamedRuleRepository {
    fn with_db<F, R>(f: F) -> R
    where
        F: FnOnce(&mut StableBTreeMap<NamedRuleKey, NamedRule, VirtualMemory<Memory>>) -> R,
    {
        DB.with(|m| f(&mut m.borrow_mut()))
    }
}

impl Repository<NamedRuleKey, NamedRule, VirtualMemory<Memory>> for NamedRuleRepository {
    fn list(&self) -> Vec<NamedRule> {
        let mut rules = Vec::with_capacity(self.len());

        if self.use_only_cache() {
            CACHE.with(|cache| {
                cache
                    .borrow()
                    .iter()
                    .for_each(|(_, rule)| rules.push(rule.clone()))
            });
        } else {
            Self::with_db(|db| {
                db.iter().for_each(|(_, rule)| rules.push(rule));
            });
        }

        rules
    }

    fn get(&self, key: &NamedRuleKey) -> Option<NamedRule> {
        DB.with(|m| {
            let maybe_cache_hit = CACHE.with(|cache| cache.borrow().get(&key.id).cloned());

            match self.use_only_cache() {
                true => maybe_cache_hit,
                false => maybe_cache_hit.or_else(|| m.borrow().get(key)),
            }
        })
    }

    fn insert(&self, key: NamedRuleKey, value: NamedRule) -> Option<NamedRule> {
        DB.with(|m| {
            CACHE.with(|cache| cache.borrow_mut().insert(key.id, value.clone()));

            let prev = m.borrow_mut().insert(key, value.clone());

            self.save_entry_indexes(&value, prev.as_ref());

            prev
        })
    }

    fn remove(&self, key: &NamedRuleKey) -> Option<NamedRule> {
        DB.with(|m| {
            CACHE.with(|cache| cache.borrow_mut().remove(&key.id));

            let prev = m.borrow_mut().remove(key);

            if let Some(prev) = &prev {
                self.remove_entry_indexes(prev);
            }

            prev
        })
    }
}

impl IndexedRepository<NamedRuleKey, NamedRule, VirtualMemory<Memory>> for NamedRuleRepository {
    fn remove_entry_indexes(&self, value: &NamedRule) {
        let index = value.to_unique_index();
        self.unique_index.remove(&index.0);
    }

    fn add_entry_indexes(&self, value: &NamedRule) {
        let index = value.to_unique_index();
        self.unique_index.insert(index.0, index.1);
    }

    fn clear_indexes(&self) {
        CACHE.with(|cache| cache.borrow_mut().clear());

        self.unique_index
            .clear_when(|key| matches!(key, UniqueIndexKey::NamedRuleName(_)));
    }

    fn save_entry_indexes(&self, value: &NamedRule, previous: Option<&NamedRule>) {
        if let Some(prev) = previous {
            self.remove_entry_indexes(prev);
        }

        self.add_entry_indexes(value);
    }
}

#[cfg(test)]
mod test {
    use orbit_essentials::{model::ModelKey, repository::Repository};

    use crate::{models::NamedRule, repositories::NamedRuleRepository};

    #[test]
    fn test_crud() {
        let repository = NamedRuleRepository::default();
        let named_rule = NamedRule {
            id: [0; 16],
            name: "test".to_string(),
            description: Some("test description".to_string()),
            rule: crate::models::RequestPolicyRule::AutoApproved,
        };

        assert!(repository.get(&named_rule.key()).is_none());

        repository.insert(named_rule.key(), named_rule.clone());

        assert!(repository.get(&named_rule.key()).is_some());
        assert!(repository.remove(&named_rule.key()).is_some());
        assert!(repository.get(&named_rule.key()).is_none());
    }

    #[test]
    fn test_find_by_name() {
        let repository = NamedRuleRepository::default();
        let named_rule = NamedRule {
            id: [0; 16],
            name: "test".to_string(),
            description: Some("test description".to_string()),
            rule: crate::models::RequestPolicyRule::AutoApproved,
        };

        assert!(repository.find_by_name(&named_rule.name).is_none());

        repository.insert(named_rule.key(), named_rule.clone());

        assert!(repository.find_by_name(&named_rule.name).is_some());
    }
}
