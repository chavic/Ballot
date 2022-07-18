use store::{ImutableStore, StoreConfig};

pub trait OptionsIdentifier {}

pub struct Option<I: OptionsIdentifier> {
    identifier: I,
}

pub struct OptionStore<I: OptionsIdentifier> {
    option: Option<I>,
    store: ImutableStore,
}

impl OptionStore<String> {
    pub fn new(config: StoreConfig) -> Self {
        todo!("Impliment candidate store")
    }
}

impl OptionsIdentifier for String {}
