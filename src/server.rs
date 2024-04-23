use capnp::capability::Promise;
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};

use crate::hello_world_capnp::hello_world;

use futures::AsyncReadExt;
use std::net::ToSocketAddrs;

fn calc_factorial(num: u16) -> u64 {
    match num {
        0 => 1,
        1 => 1,
        _ => calc_factorial(num - 1) * (num as u64),
    }
}

pub struct HelloWorldImpl {
    pub name: String,
    pub message: String
}

impl hello_world::Server for HelloWorldImpl {
    fn say_hello(
        &mut self,
        params: hello_world::SayHelloParams,
        mut results: hello_world::SayHelloResults,
    ) -> Promise<(), ::capnp::Error> {
        let name = params
            .get()
            .unwrap()
            .get_request()
            .unwrap()
            .get_name()
            .unwrap();
        let text_str = name.to_string().unwrap();
        let message = format!("Hello, {text_str}!");
        results.get().init_reply().set_message(message.as_str());

        Promise::ok(())
    }

    fn who_am_i(
        &mut self,
        _: hello_world::WhoAmIParams,
        mut results: hello_world::WhoAmIResults,
    ) -> Promise<(), ::capnp::Error> {
        let message = "I am World!";
        results.get().init_reply().set_message(message);

        Promise::ok(())
    }

    fn multuply(
        &mut self,
        params: hello_world::MultuplyParams,
        mut results: hello_world::MultuplyResults,
    ) -> capnp::capability::Promise<(), capnp::Error> {
        let params = pry!(params.get());
        results.get().set_result(params.get_a() * params.get_b());
        capnp::capability::Promise::ok(())
    }

    fn is_odd(
        &mut self,
        params: hello_world::IsOddParams,
        mut results: hello_world::IsOddResults,
    ) -> capnp::capability::Promise<(), capnp::Error> {
        let number = pry!(params.get()).get_a();
        results.get().set_is_odd(number % 2 != 0);
        capnp::capability::Promise::ok(())
    }

    fn factorial(
        &mut self,
        params: hello_world::FactorialParams,
        mut results: hello_world::FactorialResults,
    ) -> capnp::capability::Promise<(), capnp::Error> {
        let number = pry!(params.get()).get_a();
        let res = calc_factorial(number);
        println!("factorial result: {}", res);
        results.get().set_fact(res);
        capnp::capability::Promise::ok(())
    }
}

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} server ADDRESS[:PORT]", args[0]);
        return Ok(());
    }

    let addr = args[2]
        .to_socket_addrs()?
        .next()
        .expect("could not parse address");

    tokio::task::LocalSet::new()
        .run_until(async move {
            let listener = tokio::net::TcpListener::bind(&addr).await?;
            let hello_world_client: hello_world::Client = capnp_rpc::new_client(HelloWorldImpl {name: String::new(), message: String::new()});

            loop {
                let (stream, _) = listener.accept().await?;
                stream.set_nodelay(true)?;
                let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
                let network = twoparty::VatNetwork::new(
                    reader,
                    writer,
                    rpc_twoparty_capnp::Side::Server,
                    Default::default(),
                );

                let rpc_system = RpcSystem::new(Box::new(network), Some(hello_world_client.clone().client));

                tokio::task::spawn_local(rpc_system);
            }
        })
        .await
}