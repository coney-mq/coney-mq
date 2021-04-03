use super::*;

pub trait AuthcMech: Send + Sync + 'static {
    type Procedure: Procedure;

    fn name() -> &'static str;
    fn start_procedure(&self) -> Self::Procedure;
}

pub trait DynAuthcMech: Send + Sync + 'static {
    fn name(&self) -> &str;
    fn start_procedure(&self) -> Box<dyn Procedure>;
}

pub trait IntoDynAuthcMech: AuthcMech + Sized {
    fn into_dyn(self) -> Box<dyn DynAuthcMech> {
        create_dyn(self)
    }
}
impl<M> IntoDynAuthcMech for M where M: AuthcMech + Sized {}

pub fn create_dyn<M>(mech: M) -> Box<dyn DynAuthcMech>
where
    M: AuthcMech,
{
    Box::new(DynAuthcMechImpl { mech })
}

#[derive(Debug)]
struct DynAuthcMechImpl<M> {
    mech: M,
}

impl<M> DynAuthcMech for DynAuthcMechImpl<M>
where
    M: AuthcMech,
{
    fn name(&self) -> &str {
        M::name()
    }

    fn start_procedure(&self) -> Box<dyn Procedure> {
        Box::new(self.mech.start_procedure())
    }
}
