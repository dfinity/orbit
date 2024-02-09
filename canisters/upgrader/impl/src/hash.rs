use crate::LocalRef;
use mockall::automock;
use sha2::{Digest, Sha256};

#[automock]
pub trait Hash: 'static + Sync + Send {
    fn hash(&self, data: &[u8]) -> Vec<u8>;
}

impl<T: Hash> Hash for LocalRef<T> {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        self.with(|h| h.borrow().hash(data))
    }
}

impl Hash for Box<dyn Hash> {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        (**self).hash(data)
    }
}

pub struct Sha256Hasher;

impl Hash for Sha256Hasher {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut h = Sha256::new();
        h.update(data);
        h.finalize().to_vec()
    }
}
