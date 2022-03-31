mod server;
mod router;
mod handler;
fn main() {
    let socket_addr = "localhost:8080";
    let server = server::Server::new(socket_addr);
    server.run();
}
