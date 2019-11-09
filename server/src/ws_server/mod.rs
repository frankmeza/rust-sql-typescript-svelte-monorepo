use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

/// Define http actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<ws::Message, ws::ProtocolError> for MyWs {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}

fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

pub fn start() {
    HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}

// use ws::listen;

// pub fn start() {
//     // Listen on an address and call the closure for each connection
//     println!("Server got here");
//     if let Err(error) = listen("127.0.0.1:8888", |out| {
//         // The handler needs to take ownership of out, so we use move
//         move |msg| {
//             // Handle messages received on this connection
//             println!("Server got message '{}'. ", msg);

//             // Use the out channel to send messages back
//             out.send(msg)
//         }
//     }) {
//         // Inform the user of failure
//         println!("Failed to create WebSocket due to {:?}", error);
//     }
// }
