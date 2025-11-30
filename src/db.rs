use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

pub struct DbCluster {
    pub name: String,
    pub tables: HashSet<String>,
    pub read_only: bool,
    pub primary:bool,
    pub latency: Duration,
}

impl DbCluster {
    pub fn new(name: String, latency: Duration,primary:bool) -> DbCluster {
        DbCluster {
            name,
            tables: HashSet::new(),
            read_only: false,
            latency: latency,
            primary:primary
        }
    }
}

pub type SchemaDomain = HashMap<String, Vec<String>>;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum operation {
    Write,
    Read,
}
pub fn simulateDbAccess(cluster: Arc<Mutex<DbCluster>>, query_type: operation) -> Duration {
    let start = Instant::now();
    let cluster_guard = cluster.lock().unwrap();
    if query_type == operation::Write && cluster_guard.read_only {
        println!(
            "[[ERROR]] write failed on {} (Read-Only Mode) - 500 Error",
            cluster_guard.name
        );
        thread::sleep(Duration::from_secs(1));
        return start.elapsed();
    }
    thread::sleep(cluster_guard.latency);
    start.elapsed()
}
