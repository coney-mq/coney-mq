use super::*;

#[async_trait::async_trait]
pub trait Procedure: Send + Sync + 'static {
    async fn response(&mut self, response: &str) -> Result<ProcedureReply, AuthcFailure>;
}

pub enum ProcedureReply {
    Challenge(String),
    Success(String),
    Failure,
}

impl ProcedureReply {
    pub fn challenge(challenge: String) -> Self {
        Self::Challenge(challenge)
    }

    pub fn success(identity: String) -> Self {
        Self::Success(identity)
    }

    pub fn failure() -> Self {
        Self::Failure
    }
}
