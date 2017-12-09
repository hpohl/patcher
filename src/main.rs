extern crate websocket;
extern crate futures;
extern crate tokio_core;

use std::env;
use std::fmt::Debug;

use websocket::message::{Message, OwnedMessage};
use websocket::server::{InvalidConnection};
use websocket::async::{Server};

use tokio_core::reactor::{Handle, Core};
use futures::{Future, Sink, Stream};


fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let server = Server::bind
        (("0.0.0.0", env::var("PORT").unwrap().parse::<u16>().unwrap()), &handle).unwrap();

    let f = server.incoming()
        .map_err(|InvalidConnection { error, .. }| error)
        .for_each(|(upgrade, addr)| {
            println!("Got a connection from: {}", addr);

            let f = upgrade
                .accept()
                .and_then(|(s, _)| s.send(Message::text("Hello World!").into()))
                .and_then(|s| {
                    let (sink, stream) = s.split();
                    stream
                        .take_while(|m| Ok(!m.is_close()))
                        .filter_map(|m| {
                            println!("Message from Client: {:?}", m);
                            match m {
                                OwnedMessage::Ping(p) => Some(OwnedMessage::Pong(p)),
                                OwnedMessage::Pong(_) => None,
                                _ => Some(m),
                            }
                        })
                    .forward(sink)
                        .and_then(|(_, sink)| {
                            sink.send(OwnedMessage::Close(None))
                        })
                });

            spawn_future(f, "Client Status", &handle);
            Ok(())
        });

    core.run(f).unwrap();
}

fn spawn_future<F, I, E>(f: F, desc: &'static str, handle: &Handle)
where F: Future<Item = I, Error = E> + 'static,
      E: Debug
{
    handle.spawn(f.map_err(move |e| println!("{}: '{:?}'", desc, e))
                 .map(move |_| println!("{}: Finished.", desc)));
}
