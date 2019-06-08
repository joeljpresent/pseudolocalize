pub(crate) fn crop_string(base_length: usize, extension: &str, percentage: u32) -> String {
    let increased_length = (percentage as usize) * base_length / 100;
    let mut extended_string = String::new();
    let mut i = increased_length;
    while i > 0 {
        let copied = extended_string.strncpy(extension, i);
        i -= copied;
    }
    extended_string
}

trait CStringFunctions {
    fn strncpy(&mut self, src: &str, n: usize) -> usize;
}

impl CStringFunctions for String {
    /// Copy `n` characters for `src` and append them to this string.
    /// Return the number of actual copied characters.
    fn strncpy(&mut self, src: &str, n: usize) -> usize {
        let mut copied = 0;
        for i in src.chars().take(n) {
            self.push(i);
            copied += 1;
        }
        copied
    }
}

#[cfg(test)]
mod tests {
    use super::crop_string;

    #[test]
    fn same_as_base_string() {
        let s = crop_string(10, "🤗洗い熊のJérôme😃🙃", 100);
        assert_eq!(s, "🤗洗い熊のJérôm");
    }

    #[test]
    fn half_of_base_string() {
        let s = crop_string(10, "🤗洗い熊のJérôme😃🙃", 50);
        assert_eq!(s, "🤗洗い熊の");
    }

    #[test]
    fn twice_base_string() {
        let s = crop_string(10, "🤗洗い熊のJérôme😃🙃", 200);
        assert_eq!(s, "🤗洗い熊のJérôme😃🙃🤗洗い熊のJé");
    }
}