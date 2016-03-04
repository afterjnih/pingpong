extern crate mio;

use mio::tcp::*;

const SERVER: mio::Token = mio::Token(0);

struct Pong {
    server: TcpListener,
    connections: Slab<Connection>,
}

impl mio::Handler for Pong {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self,
             event_loop: &mio::EventLoop<Pong>,
             token: mio::Token,
             events: mio::EventSet) {
        match token {
            SERVER => {
                assert!(events.is_readable());
                println!("this server socket is ready to accept a connection");
                match self.server.accept() {
                    Ok(Some(connection)) => {
                        println!("accepted a socket, exiting program");
                        event_loop.shutdown();
                    }
                    Ok(None) => {
                        println!("the server socket wasn't actually ready");
                    }
                    Err(e) => {
                        println!("listener.accept() errored: {}", e);
                        event_loop.shutdown();
                    }
                }
            }
            _ => panic!("Received unknown token"),
        }
    }
}

fn main() {
    let address = "0.0.0.0:6567".parse().unwrap();
    let server = TcpListener::bind(&address).unwrap();

    let mut event_loop = mio::EventLoop::new().unwrap();
    event_loop.register(&server, SERVER);

    println!("running pingpong server");
    event_loop.run(&mut Pong { server: server });
}
