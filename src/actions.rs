use std::error::Error;
use std::io;

use crate::hosts::{add_hosts_to_file, create_backups, flush_dns_cache, rollback_hosts_file, HOSTS_FILE_PATH};

enum Action {
    ModifyHostsFile,
    Rollback,
}

pub fn perform_action(blocked_domains: &[String]) -> Result<(), Box<dyn Error>> {
    // Create backups of the hosts file
    let backups = create_backups(HOSTS_FILE_PATH)?;

    println!("Successfully created backups: {}", backups.join(", "));

    // Prompt user for action
    let action = prompt_action()?;

    match action {
        Action::ModifyHostsFile => {
            // Modify hosts file to block domains
            add_hosts_to_file(blocked_domains)?;

            // Flush DNS cache
            flush_dns_cache()?;

            // Prompt user to test connection
            println!("Do you want to test the connection? (y/n): ");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            match input.trim() {
                "y" => test_connections(blocked_domains),
                "n" => println!("Program will continue without testing the connection."),
                _ => println!("Invalid input. Please enter y or n."),
            }
        }
        Action::Rollback => {
            // Rollback hosts file modifications
            rollback_hosts_file(&backups)?;
        }
    }

    Ok(())
}

fn prompt_action() -> Result<Action, Box<dyn Error>> {
    loop {
        println!("Choose an action:");
        println!("1. Modify hosts file to block domains");
        println!("2. Rollback hosts file modifications (restore backups)");
        println!("Enter the number of your choice:");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => return Ok(Action::ModifyHostsFile),
            "2" => return Ok(Action::Rollback),
            _ => println!("Invalid input. Please enter 1 or 2."),
        }
    }
}

fn test_connections(domains: &[String]) {
    println!("Testing connections...");
    for domain in domains {
        let url = format!("https://{}", domain);
        match reqwest::blocking::get(&url) {
            Ok(response) if response.status().is_success() => {
                println!("Not blocked connection to {}", domain)
            }
            _ => println!("Blocked connection to {}", domain),
        }
    }
    println!("Testing complete.");
}
