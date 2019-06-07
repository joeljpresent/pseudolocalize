/// A struct which can pseudolocalize strings with
/// some configuration.
pub struct Pseudolocalizer<'a> {
    prefix: &'a str,
    suffix: &'a str,
    transform_str: fn(s: &str) -> String
}

impl<'a> Pseudolocalizer<'a> {

    /// Create a default pseudolocalizer.
    pub fn new() -> Self {
        Pseudolocalizer {
            prefix: "[!!! ",
            suffix: " !!!]",
            transform_str: crate::transform::transform_str
        }
    }

    /// Transform a string into a pseudolocalized string
    /// according to the pseudolocalizer's configuration.
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