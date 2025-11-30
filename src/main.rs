mod db;
use std::{
    collections::{HashMap, HashSet}, sync::{Arc, Mutex}, time::Duration
};
mod virtual_partition;
use db::*;
use virtual_partition::*;
mod physical_partition;
use physical_partition::*;

lazy_static::lazy_static! {
    pub static ref CLUSTER_A: Arc<Mutex<DbCluster>> = Arc::new(Mutex::new(DbCluster {
        name: "cluster_a".to_string(), read_only: false,
        latency: Duration::from_millis(50),
        tables: vec!["users", "repositories", "issues", "avatars"].into_iter().map(|s| s.to_string()).collect(),
        primary:true
    }));

    pub static ref CLUSTER_B: Arc<Mutex<DbCluster>> = Arc::new(Mutex::new(DbCluster {
        name: "cluster_b".to_string(), read_only: false,
        latency: Duration::from_millis(5), // Low latency due to low initial load
        tables: HashSet::new(),
        primary:false
    }));

    pub static ref DOMAINS: SchemaDomain = {
        let mut domains = HashMap::new();
        domains.insert("users_domain".to_string(), vec!["users".to_string(), "avatars".to_string()]);
        domains.insert("repositories_domain".to_string(), vec!["repositories".to_string(), "issues".to_string()]);
        domains
    };
}

fn main() {
    println!("Zero Downtime Database Sharding Replication");
    println!("```````````````````````````````````````````");
    println!("Initial State (Single large cluster)");

    println!("[Pre-Partition] Simulating 100 reads to Cluster A");

    let mut pre_partition_latency = Duration::new(0, 0);

    for _ in 0..100 {
        pre_partition_latency += simulateDbAccess(Arc::clone(&CLUSTER_A), operation::Read);
    }
    println!(
        "pre_partition_latency:{}",
        pre_partition_latency.as_millis()
    );
    println!("```````````````````````````````````````````");
    println!("\n ##Virtual Partition Validation ");
}
