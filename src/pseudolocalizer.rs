/// A configurable struct which can
/// pseudolocalize strings.
pub struct Pseudolocalizer<'a> {
    prefix: &'a str,
    suffix: &'a str,
    transform_str: fn(s: &str) -> String
}

impl<'a> Pseudolocalizer<'a> {

    /// Create a default `Pseudolocalizer`.
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

    /// Set the prefix, i.e. the string with which 
    /// pseudolocalized strings shall start.
    pub fn with_prefix(mut self, prefix: &'a str) -> Self {
        self.prefix = prefix;
        self
    }

    /// Set the suffix, i.e. the string with which 
    /// pseudolocalized strings shall end.
    pub fn with_suffix(mut self, suffix: &'a str) -> Self {
        self.suffix = suffix;
        self
    }

    /// Set the transform function which can transform an input string
    /// into a string with similar-looking characters.
    pub fn with_transform_function(mut self, 
            transform_function: fn(&str) -> String) -> Self {
        self.transform_str = transform_function;
        self
    }

    /// Get the string slice with which pseudolocalized strings shall start.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Get the string slice with which pseudolocalized strings shall end.
    pub fn suffix(&self) -> &str {
        &self.suffix
    }

    /// Get the function used for pseudolocalizing strings.
    pub fn transform_function(&self) -> &(fn(s: &str) -> String) {
        &self.transform_str
    }
}

#[cfg(test)]
mod tests {
    use super::Pseudolocalizer;

    #[test]
    fn default_pseudolocalizer() {
        let pl = Pseudolocalizer::new();
        let s = pl.transform("Voix ambiguë d'un cœur qui, au zéphyr, préfère les jattes de kiwi.");
        assert_eq!(s, "[!!! Ṿøíẍ άɱƃíǧûë ď'ûñ çœûŕ ʠûí, άû ẓéƥℏƴŕ, ƥŕéƒèŕë łëŝ ĵάțțëŝ ďë ķíẅí. !!!]");
    }

    #[test]
    fn change_affixes() {
        let pl = Pseudolocalizer::new()
            .with_prefix("<< ")
            .with_suffix(" »");
        let s = pl.transform("Voyez le brick géant que j'examine près du wharf.");
        assert_eq!(s, "<< Ṿøƴëẓ łë ƃŕíçķ ǧéάñț ʠûë ĵ'ëẍάɱíñë ƥŕèŝ ďû ẅℏάŕƒ. »");
    }

    #[test]
    fn no_affixes() {
        let s = Pseudolocalizer::new()
            .with_prefix("")
            .with_suffix("")
            .transform("J'ouvre quinze woks de gypse aux méchants bas-de-plafond.");
        assert_eq!(s, "Ĵ'øûṽŕë ʠûíñẓë ẅøķŝ ďë ǧƴƥŝë άûẍ ɱéçℏάñțŝ ƃάŝ-ďë-ƥłάƒøñď.");
    }
}