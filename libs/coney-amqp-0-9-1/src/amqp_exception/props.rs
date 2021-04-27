#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Props {
    pub channel_id: u16,
    pub class_id: u16,
    pub method_id: u16,
}

impl std::fmt::Display for Props {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}|{}:{}]",
            self.channel_id, self.class_id, self.method_id
        )
    }
}
