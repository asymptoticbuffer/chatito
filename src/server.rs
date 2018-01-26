use tokio_proto::TcpServer;
use transport::LineProto;

use service::Echo;

pub fn run_server() {

    // Specify the localhost address
    let addr = "0.0.0.0:12345".parse().unwrap();

    // The builder requires a protocol and an address
    let server = TcpServer::new(LineProto, addr);

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
	println!("About to start serving");
    server.serve(|| Ok(Echo));
}
