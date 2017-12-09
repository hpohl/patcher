extern crate ws;

use std::env;


fn main() {
    ws::listen(("127.0.0.1", env::var("PORT").unwrap().parse::<u16>().unwrap()), |out| {
        move |msg| {
            println!("{}", msg);
            out.send(msg)
        }
    }).unwrap();
}
