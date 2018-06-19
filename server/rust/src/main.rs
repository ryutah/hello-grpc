use std::thread;

extern crate grpc;
extern crate protobuf;
extern crate tls_api;

use grpc::{RequestOptions, ServerBuilder, SingleResponse, StreamingResponse};

mod helloworld;
mod helloworld_grpc;

use helloworld::{HelloReply, HelloRequest, MultiGreetReply, MultiGreetRequest};
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

    // XXX It seems not to async response.
    fn get_multi_greet(
        &self,
        _m: RequestOptions,
        req: MultiGreetRequest,
    ) -> StreamingResponse<MultiGreetReply> {
        let name = if req.get_name().len() >= 1 {
            req.get_name().to_string()
        } else {
            String::from("John")
        };

        let it = std::iter::repeat(())
            .enumerate()
            .map(move |(i, _)| {
                std::thread::sleep(std::time::Duration::from_millis(1000));
                let mut rep = MultiGreetReply::new();
                rep.set_index((i + 1) as i32);
                rep.set_message(format!("Hello, {}!!", name));
                rep
            })
            .take(req.get_count() as usize);
        StreamingResponse::iter(it)
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
