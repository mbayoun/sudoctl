use clap::{Parser, Subcommand};
use colored::*;
use nix::unistd::{Uid, ROOT};
use std::fs::{self, OpenOptions};
use std::io::Write;

#[derive(Parser)]
#[command(name = "sudoctl")]
#[command(about = "A tool to manage sudo permissions for users", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Grant { username: String },
    Revoke { username: String },
    Check { username: String },
}

fn is_root() -> bool {
    Uid::current() == ROOT
}

fn grant_sudo(username: &str) {
    if !is_root() {
        eprintln!(
            "{}",
            "Error: You need root privileges to grant sudo permission.".red()
        );
        return;
    }

    let sudoers_line = format!("{} ALL=(ALL) NOPASSWD:ALL", username);
    let sudoers_path = "/etc/sudoers.d";

    let path = format!("{}/{}", sudoers_path, username);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path)
        .expect("Failed to open sudoers file");

    writeln!(file, "{}", sudoers_line).expect("Failed to write to sudoers file");
    println!(
        "{}",
        format!("Sudo permission granted to '{}'.", username).green()
    );
}

fn revoke_sudo(username: &str) {
    if !is_root() {
        eprintln!(
            "{}",
            "Error: You need root privileges to grant sudo permission.".red()
        );
        return;
    }

    let path = format!("/etc/sudoers.d/{}", username);
    match fs::remove_file(&path) {
        Ok(_) => println!(
            "{}",
            format!("Sudo permission revoked from '{}'.", username).green()
        ),
        Err(e) => eprintln!(
            "{}",
            format!(
                "Failed to revoke sudo permission from '{}': {}",
                username, e
            )
            .red()
        ),
    }
}

fn check_sudo(username: &str) {
    let path = format!("/etc/sudoers.d/{}", username);

    if fs::metadata(&path).is_ok() {
        println!(
            "{}",
            format!("User '{}' has sudo permission.", username).green()
        );
    } else {
        println!(
            "{}",
            format!("User '{}' doesn't have sudo permission.", username).red()
        );
    }
}
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Grant { username } => {
            grant_sudo(username);
        }
        Commands::Revoke { username } => revoke_sudo(username),
        Commands::Check { username } => check_sudo(username),
    }
}
