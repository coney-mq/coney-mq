use super::*;

impl AmqpException {
    pub fn new<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self {
            condition: Default::default(),
            caused_by: Default::default(),
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
    pub fn with_props(self, props: AmqpFrameProps) -> Self {
        Self {
            caused_by: props,
            ..self
        }
    }
    pub fn with_condition(self, condition: Condition) -> Self {
        Self { condition, ..self }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn condition(&self) -> Condition {
        self.condition
    }
    pub fn props(&self) -> AmqpFrameProps {
        self.caused_by
    }

    pub fn is_soft(&self) -> bool {
        self.condition.is_soft()
    }
    pub fn is_hard(&self) -> bool {
        self.condition.is_hard()
    }
}
