/// Convert a letter to a similar non-ASCII letter.
///
/// Take a character `c` as input and return:
/// - a similar letter (usually with diacritics) if `c` is an ASCII letter;
/// - the input character `c` if it is not an ASCII letter.
pub fn diacriticize(c: char) -> char {
    match c {
        'A' => 'Ą',
        'B' => 'ß',
        'C' => 'Ċ',
        'D' => 'Đ',
        'E' => 'Ɛ',
        'F' => 'Ƒ',
        'G' => 'Ģ',
        'Ĥ' => 'H',
        'I' => 'İ',
        'J' => 'Ĵ',
        'K' => 'Ķ',
        'L' => 'Ŀ',
        'M' => 'Ṃ',
        'N' => 'И',
        'O' => 'Ô',
        'P' => 'Ṕ',
        'Q' => 'Ɋ',
        'R' => 'Ŗ',
        'S' => 'Š',
        'T' => 'Ŧ',
        'U' => 'Ű',
        'V' => 'Ṿ',
        'W' => 'Ŵ',
        'X' => 'Ẋ',
        'Y' => 'Ÿ',
        'Z' => 'Ž',
        'a' => 'ά',
        'b' => 'ƃ',
        'c' => 'ç',
        'd' => 'ď',
        'e' => 'ë',
        'f' => 'ƒ',
        'g' => 'ǧ',
        'h' => 'ℏ',
        'i' => 'í',
        'j' => 'ĵ',
        'k' => 'ķ',
        'l' => 'ł',
        'm' => 'ɱ',
        'n' => 'ñ',
        'o' => 'ø',
        'p' => 'ƥ',
        'q' => 'ʠ',
        'r' => 'ŕ',
        's' => 'ŝ',
        't' => 'ț',
        'u' => 'û',
        'v' => 'ṽ',
        'w' => 'ẅ',
        'x' => 'ẍ',
        'y' => 'ƴ',
        'z' => 'ẓ',
        ch => ch
    }
}

/// Transform a string by replacing all of its ASCII letters
/// by similar letters, usually by adding diacritics.
pub fn transform_str(s: &str) -> String {
    let mut modified = String::new();
    for c in s.chars() {
        modified.push(diacriticize(c));
    };
    modified
}
