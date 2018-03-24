extern crate jsonrpc_http_server;

use jsonrpc_http_server::*;
use jsonrpc_http_server::jsonrpc_core::*;

fn main() {
	  let mut io = IoHandler::default();
	  io.add_method("say_hello", |_| {
		    Ok(Value::String(":tada:hello".into()))
	  });

	  let server = ServerBuilder::new(io)
		    .cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Null]))
		    .start_http(&"127.0.0.1:3030".parse().unwrap())
		    .expect("Unable to start RPC server");

	  server.wait();
}
