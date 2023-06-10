use find_my_transit::FindMyCacheData;
use std::{cmp, env, fs};

const CACHE_BACKUP_DIRECTORY: &str = "cache_backups";

fn main() {
    // Get most recent cache backup
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

    // Parse the .json file into cache data
    if let Some(most_recent_backup_path) = most_recent_backup {
        let cache_data_json = fs::read_to_string(most_recent_backup_path.path()).expect(&format!(
            "Can't read cache backup to string: {:?}",
            most_recent_backup_path.path()
        ));

        let cache_data: Vec<FindMyCacheData> = serde_json::from_str(&cache_data_json).unwrap();
        println!("{:?}", cache_data);
    }
}
