use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route, Server};
use poem_mcpserver::{sse::sse_endpoint, tool::Text, McpServer, Tools};
use std::net::{Ipv4Addr, IpAddr};
use std::str::FromStr;

/// Combined tool that contains counter, adder, IP validator, and CIDR checker functionality
struct CombinedTool {
    count: i32,
}

/// This tool provides multiple functionalities:
///
/// Counter: Increment, decrement, and get the value of a counter.
/// Adder: Add two numbers together.
/// IP Validator: Check if a string is a valid IPv4 address.
/// CIDR Checker: Check if an IP address is within a CIDR range.
#[Tools]
impl CombinedTool {
    /// Increment the counter by 1
    async fn increment(&mut self) -> Text<i32> {
        self.count += 1;
        Text(self.count)
    }
    
    /// Decrement the counter by 1
    async fn decrement(&mut self) -> Text<i32> {
        self.count -= 1;
        Text(self.count)
    }
    
    /// Get the current counter value
    async fn get_value(&self) -> Text<i32> {
        Text(self.count)
    }
    
    /// Add two numbers together
    /// 
    /// Parameters:
    /// - a: First number
    /// - b: Second number
    async fn add(&self, a: i32, b: i32) -> Text<i32> {
        Text(a + b)
    }
    
    /// Check if a string is a valid IPv4 address
    /// 
    /// Parameters:
    /// - ip_str: The IP address string to validate
    async fn is_valid_ipv4(&self, ip_str: String) -> Text<bool> {
        // Try to parse the string as an IPv4 address
        match ip_str.parse::<Ipv4Addr>() {
            Ok(_) => Text(true),
            Err(_) => Text(false),
        }
    }
    
    /// Check if an IP address is within a CIDR range
    /// 
    /// Parameters:
    /// - ip_str: The IP address to check
    /// - cidr_str: The CIDR range (e.g., "192.168.1.0/24")
    async fn is_ip_in_cidr(&self, ip_str: String, cidr_str: String) -> Text<bool> {
        // Parse the IP address
        let ip = match IpAddr::from_str(&ip_str) {
            Ok(ip) => ip,
            Err(_) => return Text(false),
        };
        
        // Only support IPv4 for simplicity
        let ip_v4 = match ip {
            IpAddr::V4(ipv4) => ipv4,
            IpAddr::V6(_) => return Text(false),
        };
        
        // Parse the CIDR notation
        let parts: Vec<&str> = cidr_str.split('/').collect();
        if parts.len() != 2 {
            return Text(false);
        }
        
        // Parse the network address
        let network_str = parts[0];
        let network_ip = match Ipv4Addr::from_str(network_str) {
            Ok(ip) => ip,
            Err(_) => return Text(false),
        };
        
        // Parse the prefix length
        let prefix_len: u8 = match parts[1].parse() {
            Ok(len) if len <= 32 => len,
            _ => return Text(false),
        };
        
        // Convert IPs to u32 for bitwise operations
        let ip_u32: u32 = ip_v4.octets().iter().fold(0u32, |acc, &octet| (acc << 8) | octet as u32);
        let network_u32: u32 = network_ip.octets().iter().fold(0u32, |acc, &octet| (acc << 8) | octet as u32);
        
        // Create the subnet mask
        let mask = if prefix_len == 0 {
            0u32
        } else {
            !0u32 << (32 - prefix_len)
        };
        
        // Check if the IP is in the subnet
        let result = (ip_u32 & mask) == (network_u32 & mask);
        
        Text(result)
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000");
    
    let app = Route::new()
        .at(
            "/sse",
            sse_endpoint(|_| McpServer::new().tools(CombinedTool { count: 0 })),
        )
        .with(Cors::new());
    
    println!("Server running at http://127.0.0.1:8000");
    Server::new(listener).run(app).await
}
