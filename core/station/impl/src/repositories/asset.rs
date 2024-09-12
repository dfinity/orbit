use super::indexes::unique_index::UniqueIndexRepository;
use crate::{
    core::{
        cache::Cache, ic_cdk::api::print, metrics::ASSET_METRICS, with_memory_manager, Memory,
        ASSET_MEMORY_ID,
    },
    models::{indexes::unique_index::UniqueIndexKey, Asset, AssetId},
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::{
    repository::{IndexedRepository, Repository, StableDb},
    types::UUID,
};
use std::{cell::RefCell, sync::Arc};

thread_local! {
  static DB: RefCell<StableBTreeMap<UUID, Asset, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ASSET_MEMORY_ID))
    )
  });

  static CACHE: RefCell<Cache<AssetId, Asset>> = RefCell::new(Cache::new(AssetRepository::MAX_CACHE_SIZE));
}

lazy_static! {
    pub static ref ASSET_REPOSITORY: Arc<AssetRepository> = Arc::new(AssetRepository::default());
}

/// A repository that enables managing assets in stable memory.
#[derive(Default, Debug)]
pub struct AssetRepository {
    unique_index: UniqueIndexRepository,
}

impl StableDb<UUID, Asset, VirtualMemory<Memory>> for AssetRepository {
    fn with_db<F, R>(f: F) -> R
    where
        F: FnOnce(&mut StableBTreeMap<UUID, Asset, VirtualMemory<Memory>>) -> R,
    {
        DB.with(|m| f(&mut m.borrow_mut()))
    }
}

impl IndexedRepository<UUID, Asset, VirtualMemory<Memory>> for AssetRepository {
    fn remove_entry_indexes(&self, entry: &Asset) {
        entry
            .to_unique_indexes()
            .into_iter()
            .for_each(|(index, _)| {
                self.unique_index.remove(&index);
            });
    }

    fn add_entry_indexes(&self, entry: &Asset) {
        entry
            .to_unique_indexes()
            .into_iter()
            .for_each(|(index, id)| {
                self.unique_index.insert(index, id);
            });
    }

    /// Clears all the indexes for the asset.
    fn clear_indexes(&self) {
        CACHE.with(|cache| cache.borrow_mut().clear());

        self.unique_index
            .clear_when(|key| matches!(key, UniqueIndexKey::AssetSymbolBlockchain(_, _)));
    }
}

impl Repository<UUID, Asset, VirtualMemory<Memory>> for AssetRepository {
    fn list(&self) -> Vec<Asset> {
        let mut assets = Vec::with_capacity(self.len());

        if self.use_only_cache() {
            CACHE.with(|cache| {
                cache.borrow().iter().for_each(|(_, asset)| {
                    assets.push(asset.clone());
                });
            });
        } else {
            Self::with_db(|db| {
                db.iter().for_each(|(_, asset)| {
                    assets.push(asset);
                });
            });
        }

        assets
    }

    fn get(&self, key: &AssetId) -> Option<Asset> {
        let maybe_cache_hit = CACHE.with(|cache| cache.borrow().get(key).cloned());

        match self.use_only_cache() {
            true => maybe_cache_hit,
            false => maybe_cache_hit.or_else(|| Self::with_db(|db| db.get(key))),
        }
    }

    fn insert(&self, key: AssetId, value: Asset) -> Option<Asset> {
        DB.with(|m| {
            CACHE.with(|cache| cache.borrow_mut().insert(key, value.clone()));

            let prev = m.borrow_mut().insert(key, value.clone());

            // Update metrics when an asset is upserted.
            ASSET_METRICS.with(|metrics| {
                metrics
                    .iter()
                    .for_each(|metric| metric.borrow_mut().sum(&value, prev.as_ref()))
            });

            self.save_entry_indexes(&value, prev.as_ref());

            prev
        })
    }

    fn remove(&self, key: &AssetId) -> Option<Asset> {
        DB.with(|m| {
            CACHE.with(|cache| cache.borrow_mut().remove(key));

            let prev = m.borrow_mut().remove(key);

            // Update metrics when a asset is removed.
            if let Some(prev) = &prev {
                ASSET_METRICS.with(|metrics| {
                    metrics
                        .iter()
                        .for_each(|metric| metric.borrow_mut().sub(prev))
                });

                self.remove_entry_indexes(prev);
            }

            prev
        })
    }
}

impl AssetRepository {
    /// Currently the cache uses around 100 bytes per entry (UUID, Asset),
    /// so the max cache storage size is around 10MiB.
    pub const MAX_CACHE_SIZE: usize = 100_000;

    /// Checks if every asset in the repository is in the cache.
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
                "Only the first {} assets will be added to the cache, the reposity has {} assets.",
                Self::MAX_CACHE_SIZE,
                self.len(),
            ));
        }

        CACHE.with(|cache| {
            cache.borrow_mut().clear();

            DB.with(|db| {
                for (_, asset) in db.borrow().iter().take(Self::MAX_CACHE_SIZE) {
                    cache.borrow_mut().insert(asset.id, asset);
                }
            });
        });
    }

    pub fn exists_unique(&self, blockchain: &str, symbol: &str) -> Option<AssetId> {
        let key = Asset::to_unique_index_by_symbol_blockchain(symbol, blockchain.to_owned());

        self.unique_index.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::asset_test_utils;

    #[test]
    fn test_crud() {
        let repository = AssetRepository::default();
        let asset = asset_test_utils::mock_asset();

        assert!(repository.get(&asset.id).is_none());

        repository.insert(asset.id.to_owned(), asset.clone());

        assert!(repository.get(&asset.id).is_some());
        assert!(repository.remove(&asset.id).is_some());
        assert!(repository.get(&asset.id).is_none());
    }

    #[test]
    fn test_unqiueness() {
        let repository = AssetRepository::default();
        let asset = asset_test_utils::mock_asset();

        assert!(repository
            .exists_unique(&asset.blockchain.to_string(), &asset.symbol)
            .is_none());

        repository.insert(asset.id.to_owned(), asset.clone());

        assert!(repository.exists_unique("icp", "icp").is_some());

        assert!(repository.exists_unique("icp", "ICP").is_some());

        assert!(repository.exists_unique("icp", "ICP2").is_none());

        assert!(repository.exists_unique("eth", "ICP").is_none());
    }
}
