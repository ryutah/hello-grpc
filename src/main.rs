extern crate futures;

use std::thread;

extern crate grpc;
extern crate protobuf;
extern crate tls_api;
extern crate tls_api_native_tls;

mod helloworld;
mod helloworld_grpc;

struct GreetImpl;

impl helloworld_grpc::Greeter for GreetImpl {
    fn say_hello(
        &self,
        _m: grpc::RequestOptions,
        req: helloworld::HelloRequest,
    ) -> grpc::SingleResponse<helloworld::HelloReply> {
        let name = if req.get_name().len() >= 1 {
            req.get_name()
        } else {
            "John"
        };

        let mut reply = helloworld::HelloReply::new();
        reply.set_message(format!("Hello, {}!!!", name));

        grpc::SingleResponse::completed(reply)
    }
}

fn main() {
    let mut builder: grpc::ServerBuilder<tls_api_native_tls::TlsAcceptor> =
        grpc::ServerBuilder::new();
    builder
        .http
        .set_addr("127.0.0.1:8080")
        .expect("failed to listen addr");
    builder.add_service(helloworld_grpc::GreeterServer::new_service_def(GreetImpl));

    let _server = builder.build().expect("failed to build server");

    println!("greeter server started on port {}", 8080);

    loop {
        thread::park();
    }
}
