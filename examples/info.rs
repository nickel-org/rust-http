#![feature(collections,core,old_io)]

//! A not-quite-trivial HTTP server which responds to requests by showing the request and response
//! headers and any other information it has.

#![crate_name = "info"]

extern crate time;
extern crate http;

use std::old_io::net::ip::{SocketAddr, Ipv4Addr};
use std::old_io::Writer;

use http::server::{Config, Server, Request, ResponseWriter};
use http::headers::HeaderEnum;
use http::headers::content_type::MediaType;

#[derive(Clone)]
struct InfoServer;

impl Server for InfoServer {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8001 } }
    }

    fn handle_request(&self, r: Request, w: &mut ResponseWriter) {
        w.headers.date = Some(time::now_utc());
        w.headers.content_type = Some(MediaType {
            type_: String::from_str("text"),
            subtype: String::from_str("html"),
            parameters: vec!((String::from_str("charset"), String::from_str("UTF-8")))
        });
        w.headers.server = Some(String::from_str("Rust Thingummy/0.0-pre"));
        w.write_all(b"<!DOCTYPE html><title>Rust HTTP server</title>").unwrap();

        w.write_all(b"<h1>Request</h1>").unwrap();
        let s = format!("<dl>
            <dt>Method</dt><dd>{:?}</dd>
            <dt>Host</dt><dd>{:?}</dd>
            <dt>Request URI</dt><dd>{:?}</dd>
            <dt>HTTP version</dt><dd>{:?}</dd>
            <dt>Close connection</dt><dd>{}</dd></dl>",
            r.method,
            r.headers.host,
            r.request_uri,
            r.version,
            r.close_connection);
        w.write_all(s.as_bytes()).unwrap();
        w.write_all(b"<h2>Extension headers</h2>").unwrap();
        w.write_all(b"<table><thead><tr><th>Name</th><th>Value</th></thead><tbody>").unwrap();
        for header in r.headers.iter() {
            let line = format!("<tr><td><code>{}</code></td><td><code>{}</code></td></tr>",
                               header.header_name(),
                               header.header_value());
            w.write_all(line.as_bytes()).unwrap();
        }
        w.write_all(b"</tbody></table>").unwrap();
        w.write_all(b"<h2>Body</h2><pre>").unwrap();
        w.write_all(r.body.as_slice()).unwrap();
        w.write_all(b"</pre>").unwrap();

        w.write_all(b"<h1>Response</h1>").unwrap();
        let s = format!("<dl><dt>Status</dt><dd>{:?}</dd></dl>", w.status);
        w.write_all(s.as_bytes()).unwrap();
        w.write_all(b"<h2>Headers</h2>").unwrap();
        w.write_all(b"<table><thead><tr><th>Name</th><th>Value</th></thead><tbody>").unwrap();
        {
            let h = w.headers.clone();
            for header in h.iter() {
                let line = format!("<tr><td><code>{}</code></td><td><code>{}</code></td></tr>",
                                header.header_name(),
                                header.header_value());
                w.write_all(line.as_bytes()).unwrap();
            }
        }
        w.write_all(b"</tbody></table>").unwrap();
    }
}

fn main() {
    InfoServer.serve_forever();
}
