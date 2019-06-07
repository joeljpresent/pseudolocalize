use pseudolocalize::Pseudolocalizer;

fn main() {
    let sz = Pseudolocalizer::new()
        .with_prefix("<< ")
        .with_suffix(" »")
        .transform("Voyez le brick géant que j'examine près du wharf.");
    
    let pll: Pseudolocalizer = Pseudolocalizer::new().with_prefix("lol");
    let szs = pll.transform("pq ça marche pas???");
    
    let plb = Pseudolocalizer::new();
    let pl = plb
        .with_prefix("<<< ")
        .with_suffix(" »»»");
        
    let s = pl.transform("The quick brown fox jumps over the lazy dog");
    let z = pl.transform("Je me baladais sur l'avenue, la...");
    println!("{}\n{}\n{}\n{}", szs, sz, s, z);
}