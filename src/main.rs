extern crate futures;
extern crate tokio_core;
extern crate tokio_tungstenite;
extern crate tungstenite;
extern crate dotenv;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::env;

use futures::stream::Stream;
use futures::Future;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tungstenite::protocol::Message;

use tokio_tungstenite::accept_async;


fn main() {
    dotenv::dotenv().ok();

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
    env::var("PORT").unwrap().parse::<u16>().unwrap());

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let socket = TcpListener::bind(&addr, &handle).unwrap();

    let srv = socket.incoming().for_each(|(stream, addr)| {
        accept_async(stream).and_then(move |ws_stream| {
            println!("New WebSocket connection: {}", addr);

            let (sink, stream) = ws_stream.split();

            let ws_reader = stream.for_each(move |message: Message| {
                println!("Received a message from {}: {}", addr, message);
                Ok(())
            });

            Ok(())
        }).or_else(|_| {
            Ok(())
        })
    });

    core.run(srv).unwrap();
}
