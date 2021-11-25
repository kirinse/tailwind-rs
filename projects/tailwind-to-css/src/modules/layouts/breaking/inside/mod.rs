use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBreakInside {
    kind: String,
}

impl<T> From<T> for TailwindBreakInside
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl Display for TailwindBreakInside {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "break-inside-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBreakInside {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "break-inside" => self.kind
        }
    }
}

impl TailwindBreakInside {
    /// <https://tailwindcss.com/docs/break-inside>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: NeverArbitrary::parser("break-inside", &check_valid)(pattern, arbitrary)? })
    }
}

/// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-inside#syntax>
fn check_valid(mode: &str) -> bool {
    let set = BTreeSet::from_iter(vec![
        // Keyword values
        "auto",
        "avoid",
        // Page break values
        "avoid-page",
        // Column break values
        "avoid-column",
        // Region break values
        "avoid-region",
        // Global values
        "inherit",
        "initial",
        "revert",
        "unset",
    ]);
    set.contains(mode)
}
