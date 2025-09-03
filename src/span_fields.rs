use std::borrow::Cow;
use std::fmt;

use itertools::Itertools;

use crate::style::IntoConditionalColor;
use crate::HumanFields;
use crate::ShouldColor;
use crate::Style;

#[derive(Debug)]
pub struct StyledSpanFields<'a> {
    pub(crate) style: Cow<'a, Style>,
    pub(crate) fields: HumanFields,
    pub(crate) color: ShouldColor,
}

impl<'a> fmt::Display for StyledSpanFields<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.fields.is_empty() {
            write!(
                f,
                "{}{}{}",
                "{".colored(self.color, self.style.span_name),
                self.fields
                    .iter()
                    .map(|(name, value)| self.style.style_field(self.color, name, value))
                    .join(" "),
                "}".colored(self.color, self.style.span_name)
            )?;
        }
        Ok(())
    }
}
