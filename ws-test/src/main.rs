use actix::{Actor, StreamHandler, AsyncContext, ActorContext};
use actix::prelude::*;
use actix_web::{body::MessageBody, web::{self, Bytes}, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use rand::{prelude::*, random};

const DATA_SIZE: usize = 1024 * 1024;

#[derive(Message)]
#[rtype(result = "()")]
struct SendNextData;

struct TestWebSocket {
    hb: Instant,
    waiting_for_ack: bool,
}

impl TestWebSocket {
    fn new() -> Self {
        Self { hb: Instant::now(), waiting_for_ack: false }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(Duration::from_secs(10), |act, ctx| {
            if Instant::now().duration_since(act.hb) > Duration::from_secs(20) {
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }

    fn send_data(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        if self.waiting_for_ack {
            return;
        }

        let mut rng = rand::rng();
        let mut data = vec![0x00; DATA_SIZE];
        rng.fill(&mut data[..]);
        
        ctx.binary(data);

        self.waiting_for_ack = true;
    }
}

impl Actor for TestWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.send_data(ctx);
    }
}

impl Handler<SendNextData> for TestWebSocket {
    type Result = ();

    fn handle(&mut self, _msg: SendNextData, ctx: &mut Self::Context) {
        self.send_data(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for TestWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(_)) | Ok(ws::Message::Binary(_)) => {
                self.waiting_for_ack = false;
                ctx.address().do_send(SendNextData);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

async fn websocket_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(TestWebSocket::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/ws", web::get().to(websocket_route)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
