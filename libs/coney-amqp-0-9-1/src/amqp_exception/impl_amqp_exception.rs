use super::*;

impl AmqpException {
    pub fn new<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self {
            condition: Default::default(),
            props: Default::default(),
            message: message.into(),
            source: None,
        }
    }
    pub fn with_source<E: Into<AnyError>>(self, source: E) -> Self {
        Self {
            source: Some(source.into()),
            ..self
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn condition(&self) -> Condition {
        self.condition
    }
    pub fn props(&self) -> Props {
        self.props
    }

    pub fn is_soft(&self) -> bool {
        self.condition.is_soft()
    }
    pub fn is_hard(&self) -> bool {
        self.condition.is_hard()
    }
}
