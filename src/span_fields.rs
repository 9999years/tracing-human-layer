use std::fmt;

use itertools::Itertools;

use crate::Style;
use crate::HumanFields;

#[derive(Debug)]
pub struct StyledSpanFields {
    pub(crate) style: Style,
    pub(crate) fields: HumanFields,
}

impl fmt::Display for StyledSpanFields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.fields.is_empty() {
            write!(
                f,
                "{}{}{}",
                self.style.style_span_name("{"),
                self.fields
                    .iter()
                    .map(|(name, value)| self.style.style_field(name, value))
                    .join(" "),
                self.style.style_span_name("}"),
            )?;
        }
        Ok(())
    }
}
