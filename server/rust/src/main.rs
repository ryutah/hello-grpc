use std::thread;

extern crate grpc;
extern crate protobuf;
extern crate tls_api;

use grpc::{RequestOptions, ServerBuilder, SingleResponse};

mod helloworld;
mod helloworld_grpc;

use helloworld::{HelloReply, HelloRequest};
use helloworld_grpc::{Greeter, GreeterServer};

struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(&self, _m: RequestOptions, req: HelloRequest) -> SingleResponse<HelloReply> {
        let name = if req.get_name().len() >= 1 {
            req.get_name()
        } else {
            "John"
        };

        let mut reply = HelloReply::new();
        reply.set_message(format!("Hello, {}!!!", name));

        SingleResponse::completed(reply)
    }
}

fn main() {
    let mut builder = ServerBuilder::new_plain();

    // if want to use ipv4.
    // builder.http.set_addr("127.0.0.1:8080").expect("failed to listen addr");

    builder.http.set_port(8080);
    builder.add_service(GreeterServer::new_service_def(GreeterService));

    let _server = builder.build().expect("failed to build server");

    println!("greeter server started on port {}", 8080);

    loop {
        thread::park();
    }
}
