use log::info;

use crate::common::InfoHash;
use crate::tracker::TorrentTracker;
use crate::tracker_objects::stats::StatsEvent;

impl TorrentTracker {
    pub async fn load_whitelists(&self)
    {
        if let Ok(whitelists) = self.sqlx.load_whitelist().await {
            let mut whitelist_count = 0i64;

            for info_hash in whitelists.iter() {
                self.add_whitelist(*info_hash, true).await;
                whitelist_count += 1;
            }

            info!("Loaded {} whitelists.", whitelist_count);
        }
    }

    pub async fn save_whitelists(&self) -> bool
    {
        let whitelist = self.get_whitelist().await;

        if self.sqlx.save_whitelist(whitelist).await.is_ok() { return true; }

        false
    }

    pub async fn add_whitelist(&self, info_hash: InfoHash, on_load: bool)
    {
        let whitelist_arc = self.whitelist.clone();

        if on_load { whitelist_arc.insert(info_hash, 1i64); } else { whitelist_arc.insert(info_hash, 2i64); }

        self.update_stats(StatsEvent::Whitelist, 1).await;
    }

    pub async fn get_whitelist(&self) -> Vec<(InfoHash, i64)>
    {
        let whitelist_arc = self.whitelist.clone();

        let mut return_list = vec![];
        for item in whitelist_arc.iter() { return_list.push((*item.key(), *item.value())); }

        return_list
    }

    pub async fn remove_flag_whitelist(&self, info_hash: InfoHash)
    {
        let whitelist_arc = self.whitelist.clone();

        if whitelist_arc.get(&info_hash).is_some() { whitelist_arc.insert(info_hash, 0i64); }
        let mut whitelist_count = 0i64;
        for item in whitelist_arc.iter() { if item.value() == &1i64 { whitelist_count += 1; } }

        self.set_stats(StatsEvent::Whitelist, whitelist_count).await;
    }

    pub async fn remove_whitelist(&self, info_hash: InfoHash)
    {
        let whitelist_arc = self.whitelist.clone();

        whitelist_arc.remove(&info_hash);
        let mut whitelist_count = 0i64;
        for item in whitelist_arc.iter() { if item.value() == &1 { whitelist_count += 1; } }

        self.set_stats(StatsEvent::Whitelist, whitelist_count).await;
    }

    pub async fn check_whitelist(&self, info_hash: InfoHash) -> bool
    {
        let whitelist_arc = self.whitelist.clone();

        if whitelist_arc.get(&info_hash).is_some() { return true; }

        false
    }

    pub async fn clear_whitelist(&self)
    {
        let whitelist_arc = self.whitelist.clone();

        whitelist_arc.clear();

        self.set_stats(StatsEvent::Whitelist, 0).await;
    }
}