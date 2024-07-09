# VRCPrivacyGuard

VRCPrivacyGuard is a Rust application for managing blocked domains in the hosts file on Windows systems.

## Overview

This project fetches a list of blocked domains from a specified URL and allows the user to perform actions such as modifying the hosts file to block these domains or rolling back changes by restoring backups.

## Table of Contents

- [VRCPrivacyGuard](#vrcprivacyguard)
  - [Overview](#overview)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Usage](#usage)
  - [Configuration](#configuration)
  - [License](#license)
  - [Acknowledgments](#acknowledgments)
  - [Contributing](#contributing)

## Features

- Fetches blocked domains list from a URL.
- Creates backups of the hosts file before making modifications.
- Modifies the hosts file to block specified domains.
- Rolls back hosts file modifications using previously created backups.
- Flushes DNS cache after modifying the hosts file.
- Provides an option to test connections to blocked domains.

## Usage

1. **Download VRCPrivacyGuard**: 
   - Download `VRCPrivacyGuard.exe` from the latest [release](https://github.com/safecy/VRCPrivacyGuard/releases) on GitHub.

2. **Run with Admin Privileges**:
   - Right-click `VRCPrivacyGuard.exe` and select "Run as administrator" to ensure proper access to modify system files.

3. **Follow Prompts**:
   - Follow the prompts in the command-line interface to select actions:
     - Modify hosts file to block domains.
     - Rollback hosts file modifications (restore backups).

## Configuration

Modify the `BLOCKED_DOMAINS_URL` constant in `main.rs` to change the source of blocked domains list.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to [reqwest](https://crates.io/crates/reqwest) developers for providing a convenient HTTP client library.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
