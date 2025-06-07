use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

/// Configuration trait for the Proof of Existence Module
/// Extends the system configuration with content type requirements
pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
}

/// Proof of Existence Module implementation
/// Enables users to register and manage ownership claims on content
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// Storage mapping content to its owner
    /// Supports multiple claims per account but one owner per claim
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    /// Initializes a new Proof of Existence Module instance
    pub fn new() -> Self {
        Self { claims: BTreeMap::new() }
    }

    /// Retrieves the current owner of a specific claim
    /// Returns None if no claim exists for the content
    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }
}

#[macros::call]
impl<T: Config> Pallet<T> {
    /// Registers a new content claim for the specified caller
    /// Returns error if content is already claimed by another user
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        if self.claims.contains_key(&claim) {
            return Err("this content is already claimed");
        }
        self.claims.insert(claim, caller);
        Ok(())
    }

    /// Removes an existing content claim
    /// Requires caller to be the current owner of the claim
    /// Returns error if claim doesn't exist or caller isn't the owner
    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let owner = self.get_claim(&claim).ok_or("claim does not exist")?;
        if caller != *owner {
            return Err("this content is owned by someone else");
        }
        self.claims.remove(&claim);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut poe = super::Pallet::<TestConfig>::new();
        assert_eq!(poe.get_claim(&"Hello, world!"), None);
        assert_eq!(poe.create_claim("alice", "Hello, world!"), Ok(()));
        assert_eq!(poe.get_claim(&"Hello, world!"), Some(&"alice"));
        assert_eq!(
            poe.create_claim("bob", "Hello, world!"),
            Err("this content is already claimed")
        );
        assert_eq!(poe.revoke_claim("alice", "Hello, world!"), Ok(()));
        assert_eq!(poe.create_claim("bob", "Hello, world!"), Ok(()));
    }
}