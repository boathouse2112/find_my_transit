use std::{fs, path::Path};

const FIND_MY_ITEMS_PATH: &str = "~/Library/Caches/com.apple.findmy.fmipcore/Items.data";

/// Start by reading the Find My items data file
fn main() {
    let find_my_items = fs::read_to_string(FIND_MY_ITEMS_PATH);
}
