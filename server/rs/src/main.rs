use std::sync::Arc;
use std::thread;

extern crate futures;
use futures::*;

extern crate grpcio;
extern crate protobuf;

mod helloworld;
mod helloworld_grpc;

#[derive(Clone)]
struct GreeterService;

impl helloworld_grpc::Greeter for GreeterService {
    fn say_hello(
        &self,
        ctx: grpcio::RpcContext,
        req: helloworld::HelloRequest,
        sink: grpcio::UnarySink<helloworld::HelloReply>,
    ) {
        let name = if req.get_name().len() > 0 {
            req.get_name()
        } else {
            "John"
        };

        let mut reply = helloworld::HelloReply::new();
        reply.set_message(format!("Hello {}!!", name));

        let f = sink
            .success(reply)
            .map_err(move |e| println!("failed to reply: {:?}", e));

        ctx.spawn(f);
    }

    fn get_multi_greet(
        &self,
        ctx: grpcio::RpcContext,
        _req: helloworld::MultiGreetRequest,
        sink: grpcio::ServerStreamingSink<helloworld::MultiGreetReply>,
    ) {
        let messages = std::iter::repeat(())
            .enumerate()
            .map(|(i, _)| {
                std::thread::sleep(std::time::Duration::from_secs(1));
                let mut reply = helloworld::MultiGreetReply::new();
                reply.set_index((i + 1) as i32);
                reply.set_message("This is message".to_string());
                (reply, grpcio::WriteFlags::default())
            })
            .take(10);

        let f = sink
            .send_all(stream::iter_ok::<_, grpcio::Error>(messages))
            .map(|_| {})
            .map_err(move |e| println!("failed to reply: {:?}", e));

        ctx.spawn(f)
    }
}

fn main() {
    let greet_service = helloworld_grpc::create_greeter(GreeterService);
    let mut server = grpcio::ServerBuilder::new(Arc::new(grpcio::Environment::new(4)))
        .register_service(greet_service)
        .bind("[::]", 8080)
        .build()
        .expect("failed to create server");

    server.start();

    println!("Start server on port 8080");

    loop {
        thread::park();
    }
}
