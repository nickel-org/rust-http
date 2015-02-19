#![feature(collections,old_io)]

//! A very simple HTTP server which responds to only one connection with the plain text "Hello, World!".

#![crate_name = "one_time_server"]

extern crate time;
extern crate http;

use std::old_io::net::ip::{SocketAddr, Ipv4Addr};
use std::old_io::Writer;

use http::server::{Config, Server, Request, ResponseWriter};
use http::headers::content_type::MediaType;

#[derive(Clone)]
struct HelloWorldServer;

impl Server for HelloWorldServer {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8001 } }
    }

    fn handle_request(&self, _r: Request, w: &mut ResponseWriter) {
        w.headers.date = Some(time::now_utc());
        w.headers.content_length = Some(14);
        w.headers.content_type = Some(MediaType {
            type_: String::from_str("text"),
            subtype: String::from_str("plain"),
            parameters: vec!((String::from_str("charset"), String::from_str("UTF-8")))
        });
        w.headers.server = Some(String::from_str("Example"));

        w.write_all(b"Hello, World!\n").unwrap();
    }
}

fn main() {
    match HelloWorldServer.serve_once(true, None) {
        Ok(_) => println!("done serving"),
        Err(e) => println!("failed to serve: {}", e)
    }
}
