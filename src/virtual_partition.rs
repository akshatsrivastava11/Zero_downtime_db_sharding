use std::collections::{HashMap, HashSet};

use crate::db::SchemaDomain;


//detect cross-domain join
pub fn query_linter(tables: &[&str]) -> bool {
    let mut domains: SchemaDomain = HashMap::new();
    domains.insert(
        "users_domain".to_string(),
        vec!["users".to_string(), "avatars".to_string()],
    );
    domains.insert(
        "repositories_domain".to_string(),
        vec!["repositories".to_string(), "issues".to_string()],
    );

    let table_to_domain: HashMap<&str, &str> = domains
        .iter()
        .flat_map(|(domain_name, domain_tables)| {
            domain_tables
                .iter()
                .map(move |table| (table.as_str(), domain_name.as_str()))
        })
        .collect();
    println!("{:?}", table_to_domain);
    let mut first_domain: Option<&str> = None;

    for table in tables {
        if let Some(domain) = table_to_domain.get(table) {
            match first_domain {
                None => first_domain = Some(domain),
                Some(fd) if fd != *domain => {
                    //cross-domain join detected
                    println!(
                        "Query Linter Violation: Tables '{:?}' and '{}' belong to different domains.",
                        tables[0], table
                    );
                    return false;
                }
                _ => {} // Same domain, continue
            }
        }
    }
    println!("Query Linter: No cross-domain query violations detected.");
    true
}


//detect cross table transactions
pub fn transaction_linter(tables: &[&str]){
    println!("Checking transaction for safety");
    if !query_linter(tables){
        println!("Transaction linter : cross-domain transaction detected ");
    }
    else{
        println!("Transaction linter : Transaction is safe for partitioning");
    }
}
