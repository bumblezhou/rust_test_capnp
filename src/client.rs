use crate::hello_world_capnp::hello_world;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use std::net::ToSocketAddrs;

use futures::AsyncReadExt;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 4 {
        println!("usage: {} client HOST:PORT MESSAGE", args[0]);
        return Ok(());
    }

    let addr = args[2]
        .to_socket_addrs()?
        .next()
        .expect("could not parse address");

    let _msg = args[3].to_string();

    tokio::task::LocalSet::new()
        .run_until(async move {
            let stream = tokio::net::TcpStream::connect(&addr).await?;
            stream.set_nodelay(true)?;
            let (reader, writer) =
                tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
            let rpc_network = Box::new(twoparty::VatNetwork::new(
                reader,
                writer,
                rpc_twoparty_capnp::Side::Client,
                Default::default(),
            ));
            let mut rpc_system = RpcSystem::new(rpc_network, None);
            let hello_world: hello_world::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

            tokio::task::spawn_local(rpc_system);

            let start_hello = std::time::Instant::now();
            let mut hello_request = hello_world.say_hello_request();
            let hello_req = hello_request.get();
            hello_req.get_request()?.set_name(_msg);
            let hello_reply = hello_request.send().promise.await?;
            let hello_res = hello_reply.get()?.get_reply()?.get_message();
            println!(
                "say_hello received: {} turn around time is : {}", 
                hello_res.unwrap().to_string().unwrap(),
                start_hello.elapsed().as_millis()
            );

            let start_multuply = std::time::Instant::now();
            let mut request = hello_world.multuply_request();
            let mut req = request.get();
            req.set_a(3);
            req.set_b(4);
            let reply = request.send().promise.await?;
            let rep = reply.get()?.get_result();
            println!(
                "multuply received: 3 * 4 = {} turn around time is : {}", 
                rep,
                start_multuply.elapsed().as_millis()
            );

            let start_is_odd = std::time::Instant::now();
            let mut request_1 = hello_world.is_odd_request();
            let mut req_1 = request_1.get();
            req_1.set_a(reply.get()?.get_result());
            let reply_1 = request_1.send().promise.await?;
            println!(
                "is_odd received: {} turn around time is : {}",
                reply_1.get()?.get_is_odd(),
                start_is_odd.elapsed().as_millis()
            );

            let start_fact = std::time::Instant::now();
            let mut request_1 = hello_world.factorial_request();
            let mut req_1 = request_1.get();
            req_1.set_a(20);
            let reply_1 = request_1.send().promise.await?;
            println!(
                "factorial(20) received: {} turn around time is : {}",
                reply_1.get()?.get_fact(),
                start_fact.elapsed().as_millis()
            );

            Ok(())
        })
        .await
}