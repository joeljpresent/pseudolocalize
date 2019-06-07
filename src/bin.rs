use pseudolocalize::Pseudolocalizer;

fn main() {
    let pl = Pseudolocalizer::new();
    let s = pl.transform("The quick brown fox jumps over the lazy dog");
    assert_eq!(s, "[!!! Ŧℏë ʠûíçķ ƃŕøẅñ ƒøẍ ĵûɱƥŝ øṽëŕ țℏë łάẓƴ ďøǧ !!!]");
}