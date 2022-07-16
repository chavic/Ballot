pub trait VoterIdentifier {}

pub struct VerifiedVoter<I: VoterIdentifier> {
    pub identifier: I,
}

impl VoterIdentifier for String {}
