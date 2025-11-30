mod db;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
mod virtual_partition;
use db::*;
use virtual_partition::*;
mod physical_partition;
use physical_partition::*;
fn main() {
    println!("Zero Downtime Database Sharding Replication");
    println!("```````````````````````````````````````````");
    println!("Initial State (Single large cluster)");
    let ClusterA = Arc::new(Mutex::new(DbCluster::new(
        "ClusterA".to_string(),
        Duration::from_millis(100),
    )));

    println!("[Pre-Partition] Simulating 100 reads to Cluster A");

    let mut pre_partition_latency = Duration::new(0, 0);

    for _ in 0..100 {
        pre_partition_latency += simulateDbAccess(Arc::clone(&ClusterA), operation::Read);
    }
    println!(
        "pre_partition_latency:{}",
        pre_partition_latency.as_millis()
    );
    println!("```````````````````````````````````````````");
    println!("\n ##Virtual Partition Validation ");
}
