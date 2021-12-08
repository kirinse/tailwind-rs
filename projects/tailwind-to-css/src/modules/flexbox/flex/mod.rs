use super::*;
use crate::TailwindDisplay;

pub(crate) mod flex_direction;
pub(crate) mod flex_wrap;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFlex {
    kind: KeywordOnly,
}

impl Display for TailwindFlex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "flex-{}", self.kind)
    }
}

impl TailwindInstance for TailwindFlex {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "flex" => self.kind.get_properties()
        }
    }
}

impl TailwindFlex {
    pub fn adapt(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match pattern {
            // https://tailwindcss.com/docs/display#flex
            // This won't happen
            [] if arbitrary.is_none() => TailwindDisplay::from("flex").boxed(),
            // https://tailwindcss.com/docs/flex#arbitrary-values
            [] => TailwindFlex::parse_arbitrary(arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/flex-direction
            ["row"] => TailwindFlexDirection::from("row").boxed(),
            ["row", "reverse"] => TailwindFlexDirection::from("row-reverse").boxed(),
            ["col"] => TailwindFlexDirection::from("column").boxed(),
            ["col", "reverse"] => TailwindFlexDirection::from("column-reverse").boxed(),
            ["direction", rest @ ..] => TailwindFlexDirection::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/flex-wrap
            ["wrap"] => TailwindFlexWrap::from("wrap").boxed(),
            ["wrap", "reverse"] => TailwindFlexWrap::from("wrap-reverse").boxed(),
            ["nowrap"] => TailwindFlexWrap::from("nowrap").boxed(),
            // https://tailwindcss.com/docs/flex
            _ => Self::parse(pattern, arbitrary)?.boxed(),
        };
        Ok(out)
    }
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<TailwindFlex> {
        Ok(TailwindFlex { kind: KeywordOnly::parser("flex", &Self::check_valid)(pattern, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: KeywordOnly::parse_arbitrary(arbitrary)? })
    }
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["auto", "inherit", "initial", "initial", "none", "revert", "unset"]);
        set.contains(mode)
    }
}
