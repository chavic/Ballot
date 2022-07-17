use store::{StoreConfig, StoreError};

use super::store::{OptionStore, OptionsIdentifier};
use crate::{
    verifier::{store::VerificationStore, verified_voter::VoterIdentifier},
    vote::Vote,
};
use std::collections::HashMap;

pub struct Ballot<'a, I: OptionsIdentifier> {
    pub verification_store: &'a VerificationStore,
    pub option_stores: HashMap<I, OptionStore<I>>,
}

impl Ballot<'_, String> {
    pub fn new(
        master_config: &StoreConfig,
        options_list: Vec<Option<String>>,
        verification_store: &VerificationStore,
    ) -> Self {
        todo!("Impliment new ballot")
    }

    pub fn vote<I: VoterIdentifier>(
        &self,
        voter_identifier: &I,
        choice: &Vote,
    ) -> Result<(), StoreError> {
        todo!("Add vote")
    }
}
