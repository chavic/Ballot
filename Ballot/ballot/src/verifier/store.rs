use crate::vote::Vote;

use super::verified_voter::{VerifiedVoter, VoterIdentifier};
use store::{ImutableStore, StoreConfig, StoreError};

pub struct VerificationStore {
    pub store: ImutableStore,
}

impl VerificationStore {
    pub fn new(config: StoreConfig) -> Result<Self, StoreError> {
        todo!("Impliment verfication store")
    }

    pub fn add(&self, verified_voter: VerifiedVoter<String>) -> Result<(), StoreError> {
        todo!("Impliment indetified voter add")
    }

    pub fn get<I: VoterIdentifier>(
        &self,
        verified_voter_identifier: I,
    ) -> Result<Option<()>, StoreError> {
        todo!("Impliment get verified voter")
    }

    pub fn update<I: VoterIdentifier>(
        &self,
        verified_voter_identifier: I,
        new_verified_voter: VerifiedVoter<I>,
    ) -> Result<(), StoreError> {
        todo!("Impliment update verified voter")
    }

    pub fn get_votes<I: VoterIdentifier>(
        &self,
        verified_voter_identifier: I,
    ) -> Result<Vec<Vote>, StoreError> {
        todo!("impliment Verified voter votes")
    }
}
