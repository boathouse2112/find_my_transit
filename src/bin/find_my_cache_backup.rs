use std::{
    cmp, env,
    fs::{self, File},
    io::Write,
    path::Path,
};

use directories::UserDirs;
use time::OffsetDateTime;

const FIND_MY_ITEMS_RELATIVE_PATH: &str = "Library/Caches/com.apple.findmy.fmipcore/Items.data";
const CACHE_BACKUP_DIRECTORY: &str = "cache_backups";

/// Start by reading the Find My items data file
fn main() {
    let user_dirs = UserDirs::new().unwrap();
    let find_my_items_path = user_dirs.home_dir().join(FIND_MY_ITEMS_RELATIVE_PATH);
    let find_my_items = fs::read_to_string(find_my_items_path.clone()).expect(
        format!(
            "should be able to find My Items data at {:?}",
            find_my_items_path
        )
        .as_str(),
    );
    let cache_backups_dir = fs::read_dir(CACHE_BACKUP_DIRECTORY).expect(
        format!(
            "should be able to find cache backup directory at {:?}/{:?}",
            env::current_dir().unwrap(),
            CACHE_BACKUP_DIRECTORY
        )
        .as_str(),
    );
    let mut cache_backups: Vec<_> = cache_backups_dir.map(|file| file.unwrap()).collect();
    cache_backups
        .sort_unstable_by_key(|file| cmp::Reverse(file.metadata().unwrap().modified().unwrap()));
    let most_recent_backup = cache_backups.first();

    // If the current cache differs from the most recent backup, or if there are no backups, write a new backup file
    let write_new_backup = most_recent_backup.map_or(true, |most_recent_backup| {
        let backup_contents = fs::read_to_string(most_recent_backup.path()).expect(
            format!(
                "should be able to read most_recent_backup at {:?}/{:?}",
                env::current_dir().unwrap(),
                most_recent_backup.path()
            )
            .as_str(),
        );
        backup_contents != find_my_items
    });

    if write_new_backup {
        let timestamp = OffsetDateTime::now_utc().to_string();
        let backup_file_path =
            Path::new(CACHE_BACKUP_DIRECTORY).join(format!("{}.json", timestamp));
        println!(
            "Writing new cache backup at {:?}",
            env::current_dir().unwrap().join(backup_file_path.clone())
        );
        let mut backup_file = File::create(backup_file_path.clone()).expect(
            format!(
                "should be able to create backup file at {:?}{:?}",
                env::current_dir().unwrap(),
                backup_file_path
            )
            .as_str(),
        );
        backup_file
            .write_all(find_my_items.as_bytes())
            .expect("should be able to write to backup file");
    } else {
        println!("Not writing new cache backup");
    }
}
