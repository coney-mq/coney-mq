use super::*;

use ::authc::util;

const CHALLENGE_EMPTY: &[&str] = &[];

#[derive(Debug)]
pub struct AuthcProcedurePlainConstCreds {
    creds: Arc<HashMap<(String, String), String>>,
}

impl AuthcProcedurePlainConstCreds {
    pub fn new(creds: Arc<HashMap<(String, String), String>>) -> Self {
        Self { creds }
    }
}

#[async_trait::async_trait]
impl Procedure for AuthcProcedurePlainConstCreds {
    async fn response(&mut self, response: &str) -> Result<ProcedureReply, AuthcFailure> {
        let response = util::split_zero(response);
        if response.len() == 0 {
            log::trace!("empty response. Challenging...");
            Ok(ProcedureReply::challenge(util::join_zero(CHALLENGE_EMPTY)))
        } else if response.len() == 2 {
            let login = response[0];
            let password = response[1];

            if let Some(identity) = self.creds.get(&(login.to_owned(), password.to_owned())) {
                log::trace!("identity: {:?}", identity);
                Ok(ProcedureReply::success(identity.to_owned()))
            } else {
                log::trace!("invalid creds [1]");
                Ok(ProcedureReply::failure())
            }
        } else {
            log::trace!("invalid creds [2]");
            Err(AuthcFailure::invalid_creds())
        }
    }
}
