use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindOverscroll {
    kind: Overscroll,
    axis: Option<bool>,
}

#[derive(Clone, Debug)]
enum Overscroll {
    Standard(String),
    Arbitrary(String),
}

impl Display for Overscroll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Standard(s) => write!(f, "{}", s),
            Self::Arbitrary(s) => write!(f, "[{}]", s),
        }
    }
}

impl Display for TailwindOverscroll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            None => write!(f, "overscroll-{}", self.kind),
            Some(true) => write!(f, "overscroll-x-{}", self.kind),
            Some(false) => write!(f, "overscroll-y-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindOverscroll {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = match self.axis {
            None => "overscroll-behavior",
            Some(true) => "overscroll-behavior-x",
            Some(false) => "overscroll-behavior-y",
        };
        css_attributes! {
            class => self.kind
        }
    }
}

impl TailwindOverscroll {
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary, axis: Option<bool>) -> Result<Self> {
        Ok(Self { kind: Overscroll::parse(kind, arbitrary)?, axis })
    }
}

impl Overscroll {
    /// https://tailwindcss.com/docs/overscroll-behavior
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [s] => {
                debug_assert!(Self::check_valid(s));
                Self::Standard(s.to_string())
            },
            [] => {
                debug_assert!(arbitrary.is_some());
                Self::Arbitrary(arbitrary.to_string())
            },
            _ => return syntax_error!("Unknown overscroll instructions: {}", pattern.join("-")),
        };
        Ok(kind)
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/overscroll-behavior#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            // Keyword values
            "auto", "contain", "none", // Global  values
            "inherit", "initial", "revert", "unset",
        ]);
        set.contains(mode)
    }
}