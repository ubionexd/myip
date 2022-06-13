use terminal_clipboard;

#[tokio::main]
async fn main() {
    // Attempt to get an IP address and print it.
    if let Some(ip) = public_ip::addr().await {
        println!("Your public IPv4 address is:\n{:?}", ip);
        println!("\nI copied it to your clipboard!");

        // Copy the IP address to the clipboard.
        terminal_clipboard::set_string(ip.to_string()).unwrap();
    } else {
        println!("Failed to get your public IPv4 address.");
    }
}
