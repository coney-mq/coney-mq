use std::error::Error as StdError;

use crate::AnyError;

pub trait ErrorReport<Idx> {
    fn error_report(&self) -> String;
}

impl<E> ErrorReport<&dyn StdError> for E
where
    E: StdError,
{
    fn error_report(&self) -> String {
        let mut e: &dyn StdError = self;
        let mut out = format!("- {}", e);

        while let Some(source) = e.source() {
            e = source;
            out.push_str(format!("\n- {}", e).as_str());
        }

        out
    }
}
impl ErrorReport<AnyError> for AnyError {
    fn error_report(&self) -> String {
        let mut e: &dyn StdError = self.as_ref();
        let mut out = format!("- {}", e);

        while let Some(source) = e.source() {
            e = source;
            out.push_str(format!("\n- {}", e).as_str());
        }

        out
    }
}
