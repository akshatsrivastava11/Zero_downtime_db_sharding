use std::{sync::Arc, time::Instant};

pub fn write_cutover(){
    println!("Starting physical write cutover (Moving repos / issues)");
    let start=Instant::now();
    let mut handles=vec![];
    let cluster_a_clone=Arc::clone(&cluster_a);
    let cluster_b_clone=Arc::clone(&cluster_b);
    //enable read only mode for cluster A primary
    {
        cluster_a_clone
    }
}   