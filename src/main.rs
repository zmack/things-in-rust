//! A very simple HTTP server which responds with the plain text "Hello, World!" to every request.

#![crate_id = "hello_world"]

extern crate time;
extern crate http;
extern crate lua;

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::Writer;

use http::server::{Config, Server, Request, ResponseWriter};
use http::headers::content_type::MediaType;

#[deriving(Clone)]
struct HelloWorldServer;

impl Server for HelloWorldServer {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8001 } }
    }

    fn handle_request(&self, r: &Request, w: &mut ResponseWriter) {
        w.headers.date = Some(time::now_utc());
        w.headers.content_type = Some(MediaType {
            type_: StrBuf::from_str("text"),
            subtype: StrBuf::from_str("plain"),
            parameters: vec!((StrBuf::from_str("charset"), StrBuf::from_str("UTF-8")))
        });
        w.headers.server = Some(StrBuf::from_str("Example"));

        let message = format!("Hello, {} !\r\n", r.remote_addr);
        w.headers.content_length = Some(message.len());
        w.write(message.as_bytes()).unwrap();
    }
}

fn main() {
    let mut lua_state = lua::State::new();
    let argv = std::os::args();

    if argv.len() < 2 {
      fail!("Must provide a lua script");
    }

    let file_name = argv[1];
    let path = Path::new(file_name);
      
    lua_state.openlibs();
    lua_state.loadfile(Some(&path));
    lua_state.pcall(0, lua::MULTRET, 0);

    println!("Script result was {}", lua_state.tostring(-1));
    println!("Serving on port {}", 8001);
    HelloWorldServer.serve_forever();
}
