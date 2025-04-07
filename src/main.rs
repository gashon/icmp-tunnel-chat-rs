use p2p_icmp_chat::network::connection::Connection;
use pnet::util::ParseCidrError;
use std::io::{self, BufRead, Write};
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <destination_ip>", args[0]);
        std::process::exit(1);
    }
    
    let ip_addr = match IpAddr::from_str(&args[1]) {
        Ok(IpAddr::V4(ip)) => ip,
        Ok(_) => {
            eprintln!("Error: Only IPv4 addresses are supported");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error parsing IP address: {}", e);
            std::process::exit(1);
        }
    };
    
    println!("ICMP Chat - Connected to {}", ip_addr);
    println!("Type a message and press Enter to send. Press Ctrl+C to quit.");
    
    // Create connection
    let connection = Arc::new(Mutex::new(Connection::new(ip_addr)));
    
    // Clone connection for receiver thread
    let receiver_connection = connection.clone();
    
    // Start receiver thread
    let receiver_handle = thread::spawn(move || {
        let conn = receiver_connection.lock().unwrap();
        
        println!("Listening for incoming messages...");
        
        // Listen for incoming messages
        conn.listen(|message| {
            println!("\nReceived: {}", message);
            print!("> ");
            io::stdout().flush().unwrap();
        });
    });
    
    // Input loop for sender
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut input = String::new();
    
    print!("> ");
    io::stdout().flush()?;
    
    while stdin_lock.read_line(&mut input)? > 0 {
        let message = input.trim().to_string();
        
        if !message.is_empty() {
            let mut conn = connection.lock().unwrap();
            conn.send_payload(message.into_bytes())?;
            println!("Message sent!");
        }
        
        input.clear();
        print!("> ");
        io::stdout().flush()?;
    }
    
    // We shouldn't reach here under normal operation since Ctrl+C will terminate
    receiver_handle.join().unwrap();
    
    Ok(())
}