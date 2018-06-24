use std::iter;
use std::sync::Arc;
use std::thread;
use std::time;

extern crate futures;
use futures::*;

extern crate grpcio;
extern crate protobuf;

use grpcio::*;

mod helloworld;
mod helloworld_grpc;

use helloworld::*;
use helloworld_grpc::*;

#[derive(Clone)]
struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(&self, ctx: RpcContext, req: HelloRequest, sink: UnarySink<HelloReply>) {
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
        ctx: RpcContext,
        req: MultiGreetRequest,
        sink: ServerStreamingSink<MultiGreetReply>,
    ) {
        let name = if req.get_name().len() > 0 {
            req.get_name().to_owned()
        } else {
            "John".to_owned()
        };

        let messages = iter::repeat(())
            .enumerate()
            .map(move |(i, _)| {
                thread::sleep(time::Duration::from_secs(1));
                let mut reply = MultiGreetReply::new();
                reply.set_index((i + 1) as i32);
                reply.set_message(format!("Hello {}!!", name));
                (reply, WriteFlags::default())
            })
            .take(req.get_count() as usize);

        let f = sink
            .send_all(stream::iter_ok::<_, grpcio::Error>(messages))
            .map(|_| {})
            .map_err(move |e| println!("failed to reply: {:?}", e));

        ctx.spawn(f)
    }

    fn cli_stream_say_hello(
        &self,
        ctx: RpcContext,
        stream: RequestStream<HelloRequest>,
        reply: ClientStreamingSink<HelloReply>,
    ) {
        let f = stream
            .map(|req| {
                println!("Receive: {:?}", req);
                if req.get_name().len() > 0 {
                    req.get_name().to_owned()
                } else {
                    "John".to_owned()
                }
            })
            .fold(String::new(), |sum, name| {
                Ok(format!("{}, {}", sum, name)) as Result<String>
            })
            .and_then(move |name| {
                let mut rep = HelloReply::new();
                rep.set_message(format!("Hello {}!!", name));
                reply.success(rep)
            })
            .map_err(|e| println!("failed to reply: {:?}", e));

        ctx.spawn(f);
    }
}

fn main() {
    let greet_service = create_greeter(GreeterService);
    let mut server = ServerBuilder::new(Arc::new(grpcio::Environment::new(4)))
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
