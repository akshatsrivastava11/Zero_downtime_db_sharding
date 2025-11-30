use std::{sync::Arc, thread, time::{Duration, Instant}};

use crate::{CLUSTER_A, CLUSTER_B, db::{operation, simulateDbAccess}};

pub fn write_cutover()->Duration{
    println!("Starting physical write cutover (Moving repos / issues)");
    let start=Instant::now();
    let mut handles=vec![];
    let cluster_a_clone=Arc::clone(&CLUSTER_A);
    let cluster_b_clone=Arc::clone(&CLUSTER_B);
    //enable read only mode for cluster A primary
    {
        CLUSTER_A.lock().unwrap().read_only=true;
        println!("1. Cluster A set to read-only");
    }

    //simulate a write attempt over to cluster a that will fail / hang
    handles.push(thread::spawn(move || {
        simulateDbAccess(cluster_a_clone, operation::Write);
    }));    

    //read the last executed mysql query from mysql gtid from the cluster A primarily 
    thread::sleep(Duration::from_millis(5));
    println!(" 2. Read the last executed GTID ");

    //poll clusterB to verify that the last executed gtid has arrived
    thread::sleep(Duration::from_millis(10));
    println!(" 3. Verified replication sync on clusterB");

    //stop replication on the cluster B primary from cluster A
    {
        let mut a=CLUSTER_A.lock().unwrap();
        let mut b=CLUSTER_B.lock().unwrap();
        a.tables.remove("repositories");
        a.tables.remove("issues");
        b.tables.insert("repositories".to_string());
        b.tables.insert("issues".to_string());
        b.primary=true;
        println!(" 4. Stopped replication on clusterB");
    }


    //update proxy sql routing to route the traffic back to cluster B
    CLUSTER_B.lock().unwrap().latency=Duration::from_millis(20);

    CLUSTER_A.lock().unwrap().primary=false;
    
    //wait for the simulated failing write from step 1 to complete 
    for handle in handles{
        handle.join().unwrap();
    
    }
    let cutover_time=start.elapsed();
    println!("Cutover complete . Total critical downtime : {:?} ",cutover_time);
    cutover_time
}   