/// An already configured struct which can
/// pseudolocalize strings.
pub struct Pseudolocalizer<'a> {
    prefix: &'a str,
    suffix: &'a str,
    transform_str: fn(s: &str) -> String
}

impl<'a> Pseudolocalizer<'a> {

    /// Create a default `Pseudolocalizer`.
    /// 
    /// If you want to build a custom `Pseudolocalizer`,
    /// you should use a `PseudolocalizerBuilder`.
    pub fn new() -> Self {
        Pseudolocalizer {
            prefix: "[!!! ",
            suffix: " !!!]",
            transform_str: crate::transform::transform_str
        }
    }

    /// Transform a string into a pseudolocalized string
    /// according to the `Pseudolocalizer`'s configuration.
    pub fn transform(&self, string: &str) -> String {
        format!("{}{}{}",
            self.prefix,
            (self.transform_str)(string),
            self.suffix)
    }

    /// Get the string slice with which pseudolocalized strings shall start.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Get the string slice with which pseudolocalized strings shall end.
    pub fn suffix(&self) -> &str {
        &self.suffix
    }
}

/// A struct which can configure and build a `Pseudolocalizer`.
pub struct PseudolocalizerBuilder<'a> {
    prefix: &'a str,
    suffix: &'a str,
    transform_str: fn(s: &str) -> String
}

impl<'a> PseudolocalizerBuilder<'a> {

    /// Create a default `PseudolocalizerBuilder`.
    pub fn new() -> PseudolocalizerBuilder<'a> {
        PseudolocalizerBuilder {
            prefix: "[!!! ",
            suffix: " !!!]",
            transform_str: crate::transform::transform_str
        }
    }

    /// Set the prefix, i.e. the string with which 
    /// pseudolocalized strings shall start.
    pub fn with_prefix(&mut self, prefix: &'a str) -> &mut Self {
        self.prefix = prefix;
        self
    }

    /// Set the suffix, i.e. the string with which 
    /// pseudolocalized strings shall end.
    pub fn with_suffix(&mut self, suffix: &'a str) -> &mut Self {
        self.suffix = suffix;
        self
    }

    /// Set the transform function which can transform an input string
    /// into a string with similar-looking characters.
    pub fn with_transform_function(&mut self, 
            transform_function: fn(&str) -> String) -> &mut Self {
        self.transform_str = transform_function;
        self
    }

    /// Build the `Pseudolocalizer`.
    pub fn build(&self) -> Pseudolocalizer {
        Pseudolocalizer {
            prefix: self.prefix,
            suffix: self.suffix,
            transform_str: self.transform_str
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Pseudolocalizer;

    #[test]
    fn pseudolocalizer_new_and_transform() {
        let pl = Pseudolocalizer::new();
        let s = pl.transform("Voix ambiguë d'un cœur qui, au zéphyr, préfère les jattes de kiwi.");
        assert_eq!(s, "[!!! Ṿøíẍ άɱƃíǧûë ď'ûñ çœûŕ ʠûí, άû ẓéƥℏƴŕ, ƥŕéƒèŕë łëŝ ĵάțțëŝ ďë ķíẅí. !!!]");
    }
}