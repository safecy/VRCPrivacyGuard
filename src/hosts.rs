use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

pub const HOSTS_FILE_PATH: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";

pub fn fetch_blocked_domains(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?.text()?;
    let domains: Vec<String> = response.lines().map(|s| s.trim().to_string()).collect();
    Ok(domains)
}

pub fn create_backups(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut backups = Vec::new();
    let mut i = 0;
    loop {
        let backup_path = format!("{}.backup-{}", file_path, i);
        if !Path::new(&backup_path).exists() {
            fs::copy(file_path, &backup_path)?;
            backups.push(backup_path);
            break;
        }
        i += 1;
    }
    Ok(backups)
}

pub fn add_hosts_to_file(domains: &[String]) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().append(true).open(HOSTS_FILE_PATH)?;
    writeln!(file, "\n# Added by VRCPrivacyGuard")?;

    for domain in domains {
        writeln!(file, "0.0.0.0 {}", domain)?;
    }

    writeln!(file, "# End of VRCPrivacyGuard hosts file")?;
    Ok(())
}

pub fn rollback_hosts_file(backups: &[String]) -> Result<(), Box<dyn Error>> {
    for backup in backups {
        let original_file = backup.replace(".backup-", "");
        fs::copy(backup, &original_file)?;
        println!("Restored backup: {}", original_file);
    }
    Ok(())
}

pub fn flush_dns_cache() -> Result<(), Box<dyn Error>> {
    std::process::Command::new("ipconfig")
        .arg("/flushdns")
        .output()?;
    Ok(())
}
