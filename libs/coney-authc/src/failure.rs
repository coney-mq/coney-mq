#[derive(Debug, Error)]
pub enum AuthcFailure {
    #[error("AuthcFailure::Unimplemented: {0}")]
    Unimplemented(String),

    #[error("AuthcFailure::UnsupportedMechanism: {0}")]
    UnsupportedMechanism(String),

    #[error("AuthcFailure::InvalidCreds")]
    InvalidCreds,
}

impl AuthcFailure {
    pub fn unimplemented(message: &str) -> Self {
        Self::Unimplemented(message.to_owned())
    }

    pub fn invalid_creds() -> Self {
        Self::InvalidCreds
    }

    pub fn unsupported_mechanism(mechanism: &str) -> Self {
        Self::UnsupportedMechanism(mechanism.to_owned())
    }
}
