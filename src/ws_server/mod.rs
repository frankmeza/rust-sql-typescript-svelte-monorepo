use crate::handlers::AppMessage;
use actix::{Actor, Handler, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

pub struct WebSocket;

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<AppMessage> for WebSocket {
    type Result = Result<AppMessage, Error>;

    fn handle(&mut self, msg: AppMessage, _ctx: &mut Self::Context) -> Self::Result {
        Ok(msg)
    }
}

/// Handler for ws::Message message
impl StreamHandler<ws::Message, ws::ProtocolError> for WebSocket {
    fn started(&mut self, ctx: &mut Self::Context) {
        // let addr = ctx.address();
    }

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
