use crate::transform::transform_str;
use crate::percent::crop_string;

/// A configurable struct which can
/// pseudolocalize strings.
pub struct Pseudolocalizer<'a> {
    prefix: &'a str,
    suffix: &'a str,
    increase: u32,
    extension_string: &'a str,
    transform_str: fn(s: &str) -> String
}

impl<'a> Pseudolocalizer<'a> {

    /// Create a default `Pseudolocalizer`.
    pub fn new() -> Self {
        Pseudolocalizer {
            prefix: "[!!! ",
            suffix: " !!!]",
            increase: 0,
            extension_string: 
r"Þôřƭèƺ çè Ʋïèúж ωλïƨƙ¥ áú Júϱè βℓôñδ 9úï ƒú₥è ƨúř ƨôñ
îℓè ïñƭéřïèúřè, à çôƭé δè ℓ'áℓçôƲè ôƲôïδè, ôù ℓèƨ βûçλèƨ ƨè çôñƨú₥èñƭ δáñƨ
ℓ'âƭřè, çè 9úï ℓúï ƥèř₥èƭ δè ƥèñƨèř à ℓá çæñôϱéñèƨè δè ℓ'êƭřè δôñƭ ïℓ èƨƭ
9úèƨƭïôñ δáñƨ ℓá çáúƨè á₥βïϱúë èñƭèñδúè à Môÿ, δáñƨ úñ çáƥλářñáü₥ 9úï,
ƥèñƨè-ƭ-ïℓ, δï₥ïñúè çà èƭ ℓà ℓá 9úáℓïƭé δè ƨôñ œúƲřè.",
            transform_str: transform_str
        }
    }

    /// Transform a string into a pseudolocalized string
    /// according to the `Pseudolocalizer`'s configuration.
    pub fn transform(&self, string: &str) -> String {
        format!("{}{}{}{}",
            self.prefix,
            (self.transform_str)(string),
            crop_string(string.chars().count(), self.extension_string, self.increase),
            self.suffix)
    }

    /// Set the extension string, i.e. the string used to
    /// increase the length of pseudolocalized strings
    /// (if the extension value is positive).
    pub fn with_extension_string(mut self, extension: &'a str) -> Self {
        self.extension_string = extension;
        self
    }

    /// Set the increase rate of a string in percent 
    /// (e.g., an increase of 27% means that pseudolocalized strings
    /// shall be 1.27 times longer than the original string).
    pub fn with_increase_percentage(mut self, increase: u32) -> Self {
        self.increase = increase;
        self
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