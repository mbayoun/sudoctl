# sudoctl Documentation

## Overview

**sudoctl** is a command-line tool designed to manage sudo permissions for users on a Linux system. It allows users to grant, revoke, and check sudo permissions for specified usernames. This tool is especially useful for system administrators who need to manage user privileges efficiently.

### Author

- **Name:** Murad Bayoun

### Version

- **Current Version:** 1.0

### Release Date

- **Released on:** July 6, 2024

## Features

- **Grant Sudo Permission:** Assigns sudo privileges to a specified user.
- **Revoke Sudo Permission:** Removes sudo privileges from a specified user.
- **Check Sudo Permission:** Verifies if a specified user has sudo privileges.

## Installation

To use sudoctl, you need to have Rust installed. If you don't have Rust installed, you can download and install it from [rust-lang.org](https://www.rust-lang.org/).

After ensuring Rust is installed, you can build the tool using the following commands:

```bash
git clone <repository-url>
cd <repository-directory>
cargo build --release
```

Replace `<repository-url>` with the URL of the repository where sudoctl is hosted and `<repository-directory>` with the directory name of the cloned repository.

## Usage

### Command Structure

```bash
sudoctl <COMMAND> <USERNAME>
```

### Commands

1. **Grant Sudo Permission**
    - **Command:** `grant`
    - **Description:** Grants sudo permission to the specified username.
    - **Usage:**
      ```bash
      sudoctl grant <username>
      ```
    - **Example:**
      ```bash
      sudoctl grant alice
      ```

2. **Revoke Sudo Permission**
    - **Command:** `revoke`
    - **Description:** Revokes sudo permission from the specified username.
    - **Usage:**
      ```bash
      sudoctl revoke <username>
      ```
    - **Example:**
      ```bash
      sudoctl revoke bob
      ```

3. **Check Sudo Permission**
    - **Command:** `check`
    - **Description:** Checks if the specified username has sudo permission.
    - **Usage:**
      ```bash
      sudoctl check <username>
      ```
    - **Example:**
      ```bash
      sudoctl check carol
      ```

### Root Privileges

Certain commands require root privileges to execute. Ensure you run the tool with sufficient permissions when performing grant or revoke operations:

```bash
sudo sudoctl <command> <username>
```

## Examples

### Granting Sudo Permission

```bash
sudo sudoctl grant alice
```

### Revoking Sudo Permission

```bash
sudo sudoctl revoke bob
```

### Checking Sudo Permission

```bash
sudoctl check carol
```