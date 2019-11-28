use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use futures::stream::once;

pub struct WebSocket;

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;

    // fn started(ctx: &mut Self::Context) {
    //     Self::add_stream(once::<Ping, io::Error>(Ok(Ping)), ctx);
    // }
}

/// Handler for ws::Message message
impl StreamHandler<ws::Message, ws::ProtocolError> for WebSocket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        // echoes from the client
        let here_go = format!("WS MSG: {:?}", msg);

        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(here_go),
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub fn start(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WebSocket {}, &req, stream);
    println!("{:?}", resp);
    resp
}
