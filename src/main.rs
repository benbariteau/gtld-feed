extern crate reqwest;

use std::collections::HashSet;

fn main() {
    let new_tlds_text = reqwest::get("https://data.iana.org/TLD/tlds-alpha-by-domain.txt").unwrap().text().unwrap();

    // split on newline and then skip the first line, which is always the generated time comment
    let new_tlds: HashSet<&str> = new_tlds_text.split("\n").skip(1).map(|tld| tld.trim()).collect();

    for tld in new_tlds {
        println!("{}", tld);
    }
}
