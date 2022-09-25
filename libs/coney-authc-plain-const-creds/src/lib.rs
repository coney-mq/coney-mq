use std::collections::HashMap;
use std::sync::Arc;

use ::authc::{AuthcFailure, AuthcMech, Procedure, ProcedureReply};

mod mech;
pub use mech::AuthcMechPlainConstCreds;

mod procedure;
pub use procedure::AuthcProcedurePlainConstCreds;
