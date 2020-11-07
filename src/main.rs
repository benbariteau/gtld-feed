extern crate reqwest;
extern crate itertools;
extern crate jsonfeed;
extern crate chrono;

use std::collections::HashSet;
use std::fs::File;
use std::io::{Read, Write};
use itertools::{sorted, join};
use chrono::Utc;

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

    let newest_tlds: Vec<&str> = sorted(new_tlds.difference(&old_tlds)).map(|v| *v).collect();

    if newest_tlds.is_empty() {
        return;
    }

    // TODO: take in path to json feed file
    let old_json_feed_file = File::open("feed.json").unwrap();
    let old_json_feed = jsonfeed::from_reader(old_json_feed_file).unwrap();

    let mut items = old_json_feed.items.clone();
    // remove the last item
    items.truncate(19);

    let mut item_builder = jsonfeed::Item::builder();
    let last_id: usize = items[0].id.parse().unwrap();
    item_builder.id = Some(format!("{}", last_id + 1));
    item_builder.date_published = Some(Utc::now().to_rfc3339());
    let item = item_builder
        .title(format!("New TLDs {}", Utc::today().format("%F")))
        .content_text(
            format!("<ul>{}</ul>",
                    join(newest_tlds.iter().map(|tld| format!("<li>{}</li>", tld)), "\n")
                    )
            )
        .build().unwrap();

    items.insert(0, item);

    let mut new_json_feed = old_json_feed.clone();
    new_json_feed.items = items;

    let new_json_feed_file = File::create("feed.json").unwrap();
    jsonfeed::to_writer(new_json_feed_file, &new_json_feed).unwrap();

    let mut new_tlds_file = File::create("old_tlds.txt").unwrap();
    new_tlds_file.write(&join(sorted(&new_tlds), "\n").into_bytes()).unwrap();
}
