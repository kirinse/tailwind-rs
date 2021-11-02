use super::*;

mod margin;
mod padding;
mod space;

mod builder;
mod display;
#[cfg(test)]
mod test;
pub use self::margin::TailwindMargin;

#[derive(Copy, Clone, Debug)]
pub enum SizeKind {
    Auto,
    Reverse,
    Unit(f32),
    Px(f32),
    Rem(f32),
}

#[doc = include_str ! ("spacing.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindSpacing {
    kind: TailwindSpacingKind,
    size: SizeKind,
}

#[derive(Copy, Clone, Debug)]
pub enum TailwindSpacingKind {
    Padding,
    PaddingL,
    PaddingR,
    PaddingT,
    PaddingB,
    PaddingX,
    PaddingY,
    Margin,
    MarginL,
    MarginR,
    MarginT,
    MarginB,
    MarginX,
    MarginY,
    SpaceBetweenX,
    SpaceBetweenY,
}

#[derive(Clone, Debug)]
pub struct SpacingSystem {
    inner: HashMap<String, Spacing>,
}

impl Default for SpacingSystem {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl SpacingSystem {
    /// Builtin ranges
    /// https://tailwindcss.com/docs/screens
    pub fn builtin() -> Self {
        let mut new = Self::default();
        new.register("sm".to_string(), 640);
        new.register("md".to_string(), 768);
        new.register("lg".to_string(), 1024);
        new.register("xl".to_string(), 1280);
        new.register("2xl".to_string(), 1536);
        return new;
    }

    #[inline]
    pub fn register(&mut self, name: String, width: usize) -> Option<Spacing> {
        self.inner.insert(name, Spacing { width })
    }
}

#[derive(Clone, Debug)]
pub struct Spacing {
    /// min-width
    /// unit: px
    width: usize,
}

impl Display for Spacing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "@media (min-width: {}px) {{", self.width)?;
        f.write_str("}")
    }
}
