# MCP Server with Combined Tools

This project implements a Model Context Protocol (MCP) server in Rust using the `poem` framework. The server provides multiple tools for managing counters, adding numbers, validating IP addresses, and checking if an IP is in a CIDR range. The tools are exposed via a Server-Sent Events (SSE) endpoint, allowing clients to interact with the server in real-time.

## Features

- **Counter**: 
  - Increment, decrement, and get the current value of a counter.
  
- **Adder**: 
  - Add two integers together.
  
- **IP Validator**: 
  - Validate if a given string is a valid IPv4 address.
  
- **CIDR Checker**: 
  - Check if an IP address is within a specified CIDR range.

## Endpoints

- `/sse`: A Server-Sent Events (SSE) endpoint that exposes the tools in real-time.

## Tools

- **Increment**: Increment the counter by 1.
- **Decrement**: Decrement the counter by 1.
- **Get Value**: Retrieve the current counter value.
- **Add**: Add two numbers together.
- **Is Valid IPv4**: Check if a string is a valid IPv4 address.
- **Is IP in CIDR**: Check if an IP address is within a CIDR range.

## Setup

### Requirements

- Rust 1.60 or higher
- Cargo (Rust's package manager and build system)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/mcp-server.git
   cd mcp-server
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Run the server:

   ```bash
   cargo run
   ```

The server will start and listen on `http://127.0.0.1:8000`.

### Dependencies

- `poem`: A web framework for Rust.
- `poem_mcpserver`: A library for implementing MCP servers with tools and SSE endpoints.
- `tokio`: Asynchronous runtime for Rust.

## Example Usage

Once the server is running, you can connect to the `/sse` endpoint via an SSE client. Here's an example using JavaScript in the browser:

```javascript
const eventSource = new EventSource('http://127.0.0.1:8000/sse');

eventSource.onmessage = function(event) {
    console.log('Received data:', event.data);
};
```

## License

This project is licensed under the MIT License. See the LICENSE file for more details.
