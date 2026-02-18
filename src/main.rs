use clap::Parser;
use colored::*;
use std::net::{TcpStream, ToSocketAddrs};
use std::process::Command;
use std::time::Duration;

#[derive(Parser)]
struct Args {
    #[arg(short = 's', default_value_t = 1)]
    start: u32,
    #[arg(short = 'L')]
    launch_ssh: bool,
    #[arg(short = 'u')]
    user: Option<String>,
}

fn main() {
    let args = Args::parse();

    for i in args.start..=630 {
        let hostname = format!("ensipc{i:03}.ensimag.fr",);
        let addr = format!("{}:22", hostname);

        // Check if DNS resolves (machine exists)
        let resolved = match addr.to_socket_addrs() {
            Ok(r) => r,
            Err(_) => continue,
        };

        // Try connecting to port 22 with a 300ms timeout
        let timeout = Duration::from_millis(300);
        if TcpStream::connect_timeout(&resolved.into_iter().next().unwrap(), timeout).is_ok() {
            println!("First awake machine: {}", hostname);

            if args.launch_ssh {
                eprintln!(
                    "\nConnecting to {}...\nYou may now enter your SSH credentials.\n",
                    hostname.blue()
                );

                let target = if let Some(user) = &args.user {
                    format!("{}@{}", user, hostname)
                } else {
                    hostname.clone()
                };

                Command::new("ssh")
                    .arg(target)
                    .status()
                    .expect("Failed to launch ssh");
            }

            return;
        }
    }

    println!("No awake machines found.");
}
