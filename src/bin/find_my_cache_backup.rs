use std::{
    cmp,
    fs::{self, File},
    io::Write,
    path::Path,
};

use directories::UserDirs;
use time::OffsetDateTime;

const FIND_MY_ITEMS_RELATIVE_PATH: &str = "Library/Caches/com.apple.findmy.fmipcore/Items.data";
const CACHE_BACKUP_DIRECTORY: &str = "cache_backups";
const ERROR_LOG_PATH: &str = "/logs/error.log";

/// Start by reading the Find My items data file
fn main() {
    let user_dirs = UserDirs::new().unwrap();
    let find_my_items_path = user_dirs.home_dir().join(FIND_MY_ITEMS_RELATIVE_PATH);
    let Ok(find_my_items) = fs::read_to_string(find_my_items_path.clone()) else {
        log_error(format!("Can't find Find My items data: {:?}", find_my_items_path).as_str())
    };

    let cache_backups_dir = fs::read_dir(CACHE_BACKUP_DIRECTORY).unwrap();
    let mut cache_backups: Vec<_> = cache_backups_dir.map(|file| file.unwrap()).collect();
    cache_backups
        .sort_unstable_by_key(|file| cmp::Reverse(file.metadata().unwrap().modified().unwrap()));
    let most_recent_backup = cache_backups.first();

    // If the current cache differs from the most recent backup, or if there are no backups, write a new backup file
    let write_new_backup = most_recent_backup.map_or(true, |most_recent_backup| {
        let Ok(backup_contents) = fs::read_to_string(most_recent_backup.path()) else {
             log_error(format!("Can't read most recent backup: {:?}", most_recent_backup.path()).as_str())
        };
        backup_contents != find_my_items
    });

    if write_new_backup {
        let timestamp = OffsetDateTime::now_utc().to_string();
        let backup_file_path =
            Path::new(CACHE_BACKUP_DIRECTORY).join(format!("{}.json", timestamp));
        let Ok(mut backup_file) = File::create(backup_file_path) else { log_error("Can't create backup path."); };
        let Ok(_) = backup_file.write_all(find_my_items.as_bytes()) else {
            log_error("Can't write to backup file")
        };
    }
}

/// Log the given error and exit with 1
fn log_error(err: &str) -> ! {
    let mut err_log = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(ERROR_LOG_PATH)
        .unwrap();
    let timestamp = OffsetDateTime::now_utc().to_string();
    let err_message = format!("{}:\t{}", timestamp, err);
    err_log.write(err_message.as_bytes()).unwrap();
    std::process::exit(1);
}
