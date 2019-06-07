use pseudolocalize::{Pseudolocalizer, PseudolocalizerBuilder};

fn main() {
    let sz = PseudolocalizerBuilder::new()
        .with_prefix("ëë ")
        .build()
        .transform("Spongebob | La città delle bolle | Nickelodeon Italia");
    
    //let pll: Pseudolocalizer = PseudolocalizerBuilder::new().build();
    //let szs = pll.transform("pq ça marche pas???");
    
    //gp.transform("wiii");
    let mut plb = PseudolocalizerBuilder::new();
    let pl = plb
        .with_prefix("<<< ")
        .with_suffix(" »»»")
        .build();
        
    let s = pl.transform("The quick brown fox jumps over the lazy dog");
    let z = pl.transform("Je me baladais sur l'avenue, la...");
    println!("{}\n{}\n{}", sz, s, z);
}