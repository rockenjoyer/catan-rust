use catan_rust::networking::client::run_client;

fn main() {
    let server_addr = "127.0.0.1:5000";
    run_client(server_addr);
}
