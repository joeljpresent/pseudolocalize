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
            string,
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
    fn kk() {
        let pl = Pseudolocalizer::new();
        let s = pl.transform("caca");
        assert_eq!(s, "[!!! caca !!!]");
    }
}