extern crate ws;

use std::rc::Rc;
use std::cell::Cell;

use ws::{listen, Handler, Sender, Result, Message, CloseCode, Handshake};

struct Server {
    out: Sender,
    count: Rc<Cell<u32>>,
}

impl Handler for Server {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // We have a new connection, so we increment the connection counter
        println!("New connection. Current connected: {}", self.count.get() + 1);
        Ok(self.count.set(self.count.get() + 1))
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Echo the message back
        println!("Got message: {}", msg);
        println!("Current connected user: {}", self.count.get());
        // self.out.send(msg)
        self.out.broadcast(msg)
        // self.out.close(CloseCode::Normal)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        // The WebSocket protocol allows for a utf8 reason for the closing state after the
        // close code. WS-RS will attempt to interpret this data as a utf8 description of the
        // reason for closing the connection. I many cases, `reason` will be an empty string.
        // So, you may not normally want to display `reason` to the user,
        // but let's assume that we know that `reason` is human-readable.
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
        println!("Lose connection. Current connected: {}", self.count.get() - 1);
        self.count.set(self.count.get() - 1)
    }
}

fn main() {
  let count = Rc::new(Cell::new(0));

  listen("127.0.0.1:3012", |out| { Server { out: out, count: count.clone() } }).unwrap()
} 