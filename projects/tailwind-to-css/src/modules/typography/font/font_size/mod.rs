use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontSize {
    name: String,
}

impl Display for TailwindFontSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "text-{}", self.name)
    }
}

impl TailwindInstance for TailwindFontSize {}
