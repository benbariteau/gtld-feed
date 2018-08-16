extern crate reqwest;

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    // TODO: take in path to old tlds
    let mut old_tlds_file = File::open("old_tlds.txt").unwrap();
    let mut old_tlds_bytes = vec![];
    old_tlds_file.read_to_end(&mut old_tlds_bytes).unwrap();
    let old_tlds_text = String::from_utf8(old_tlds_bytes).unwrap();
    let old_tlds: HashSet<&str> = old_tlds_text.split("\n").map(|tld| tld.trim()).collect();

    let new_tlds_text = reqwest::get("https://data.iana.org/TLD/tlds-alpha-by-domain.txt").unwrap().text().unwrap();
    // split on newline and then skip the first line, which is always the generated time comment
    let new_tlds: HashSet<&str> = new_tlds_text.split("\n").skip(1).map(|tld| tld.trim()).collect();

    for tld in new_tlds.difference(&old_tlds) {
        println!("{}", tld);
    }
}
