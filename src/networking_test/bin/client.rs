use catan_rust::networking_test::bootstrap;
use catan_rust::networking_test::client::run_client;

fn main() {
    let join_code = std::env::args().nth(1).expect("join code");
    let (socket, server_addr) =
        bootstrap::join("127.0.0.1:4000", &join_code);
    run_client(socket, server_addr);
}
