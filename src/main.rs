use std::error::Error;

mod hosts;
mod actions;
mod exit;

const BLOCKED_DOMAINS_URL: &str =
    "https://raw.githubusercontent.com/safecy/VRCPrivacyGuard/main/blocked-domains.txt";

fn main() -> Result<(), Box<dyn Error>> {
    // Fetch blocked domains from URL
    let blocked_domains = hosts::fetch_blocked_domains(BLOCKED_DOMAINS_URL)?;

    // Perform user-selected action
    actions::perform_action(&blocked_domains)?;

    // Wait for key press to exit
    exit::wait_for_key_press();

    Ok(())
}
