use super::*;

use std::collections::HashMap;
use std::fmt;

use crate::{DynAuthcMech, IntoDynAuthcMech};

pub struct AuthcWithMechs {
    mechs: HashMap<&'static str, Box<dyn DynAuthcMech>>,
}

impl Authc for AuthcWithMechs {
    fn select_mech(&self, mech_name: &str) -> Option<Box<dyn Procedure>> {
        if let Some(authc) = self.mechs.get(mech_name) {
            Some(authc.start_procedure())
        } else {
            None
        }
    }
}

impl fmt::Debug for AuthcWithMechs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let () = write!(f, "AuthcWithMechs [")?;
        for mech_name in self.mechs.keys() {
            let () = write!(f, " {:?}", mech_name)?;
        }
        let () = write!(f, " ]")?;
        Ok(())
    }
}

impl AuthcWithMechs {
    pub fn create() -> Self {
        Self { mechs: Default::default() }
    }

    pub fn add_mech<M>(&mut self, mech: M) -> &mut Self
    where
        M: AuthcMech,
    {
        let mech = mech.into_dyn();
        let _ = self.mechs.insert(M::name(), mech);
        self
    }

    pub fn with_mech<M>(mut self, mech: M) -> Self
    where
        M: AuthcMech,
    {
        self.add_mech(mech);
        self
    }
}
