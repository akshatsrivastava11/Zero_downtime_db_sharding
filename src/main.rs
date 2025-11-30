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
    let avg_pre_latency = pre_partition_latency / 100;
    println!("Done.\nAverage Read Latency on ClusterA ('mysql1'): {:?}", avg_pre_latency);
    println!("```````````````````````````````````````````");
    println!("\n ##Virtual Partition Validation ");
    query_linter(&["users", "avatars"]);          
    // query_linter(&["users", "repositories"]);     
    transaction_linter(&["repositories", "issues"]); 

    let cutover_time=write_cutover();

    println!("\n## Post-Partitioning Results");
    println!("Critical Write Downtime (Cutover Duration): {:?}", cutover_time);

    CLUSTER_A.lock().unwrap().latency = Duration::from_millis(20); 

    println!("\n[POST-PARTITION] Simulating 100 reads to Cluster A (Reduced Load)...");
    let mut post_a_latency = Duration::new(0, 0);
    for _ in 0..100 {
        post_a_latency += simulateDbAccess(Arc::clone(&CLUSTER_A), operation::Read);
    }

    let avg_post_a_latency = post_a_latency / 100;
    println!("Done.\nNew Average Read Latency on ClusterA: {:?} (reduced from {:?})", avg_post_a_latency, avg_pre_latency);
    println!("\n[POST-PARTITION] Simulating 100 reads to Cluster B (New Primary)...");
    let mut post_b_latency = Duration::new(0, 0);
    for _ in 0..100 {
        post_b_latency += simulateDbAccess(Arc::clone(&CLUSTER_B), operation::Read);
    }
    let avg_post_b_latency = post_b_latency / 100;
    println!("Done.\nNew Average Read Latency on ClusterB: {:?}", avg_post_b_latency);
    
    println!("\nBy vertically partitioning, the load on the original cluster was significantly reduced, lowering the **average access time** (latency) per query and improving overall stability.");
}
