use super::*;

pub trait Authc: Send + Sync + 'static {
    fn select_mech(&self, mech_name: &str) -> Option<Box<dyn Procedure>>;
}
