use super::*;

#[derive(Debug)]
pub struct AuthcMechPlainConstCreds {
    creds: Arc<HashMap<(String, String), String>>,
}

impl AuthcMechPlainConstCreds {
    pub fn new<I, Login, Password, Identity>(users: I) -> Self
    where
        I: IntoIterator<Item = (Login, Password, Identity)>,
        Login: AsRef<str>,
        Password: AsRef<str>,
        Identity: AsRef<str>,
    {
        let creds = users
            .into_iter()
            .map(|(l, p, i)| {
                ((l.as_ref().to_owned(), p.as_ref().to_owned()), i.as_ref().to_owned())
            })
            .collect();
        let creds = Arc::new(creds);
        Self { creds }
    }
}

impl AuthcMech for AuthcMechPlainConstCreds {
    type Procedure = AuthcProcedurePlainConstCreds;

    fn name() -> &'static str {
        "PLAIN"
    }

    fn start_procedure(&self) -> Self::Procedure {
        AuthcProcedurePlainConstCreds::new(self.creds.clone())
    }
}
