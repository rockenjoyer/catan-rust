use catan_rust::networking_test::{bootstrap, client};
use catan_rust::networking_test::server::run_server;

fn main() {
    let (socket, _) =
        bootstrap::host("127.0.0.1:4000", "ABC123");
    run_server(socket);
}
