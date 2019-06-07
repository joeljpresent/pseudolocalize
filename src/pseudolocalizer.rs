use crate::transform::transform_str;

pub struct Pseudolocalizer<'a> {
    str_start: &'a str,
    str_end: &'a str
}

impl<'a> Pseudolocalizer<'a> {
    pub fn new() -> Self {
        Pseudolocalizer {
            str_start: "[!!! ",
            str_end: " !!!]"
        }
    }

    pub fn transform(&self, string: &str) -> String {
        format!("{}{}{}",
            self.str_start,
            transform_str(string),
            self.str_end)
    }

    fn str_start(&self) -> &str {
        &self.str_start
    }

    fn str_end(&self) -> &str {
        &self.str_end
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