use actix::{Actor, StreamHandler, AsyncContext, ActorContext};
use actix_web::{body::MessageBody, web::{self, Bytes}, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use rand::{prelude::*, random};

const DATA_SIZE: usize = 1024 * 1024;

struct TestWebSocket {
    hb: Instant,
    counter: usize,
}

impl TestWebSocket {
    fn new() -> Self {
        Self { hb: Instant::now(), counter: 0 }
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
}

impl Actor for TestWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        ctx.run_interval(Duration::from_secs(1), |act, ctx| {
            act.counter += 1;
            let tmp: Vec<u8> = vec![0;DATA_SIZE].iter().map(|_x| random::<u8>()).collect();
            ctx.binary(Bytes::copy_from_slice(&tmp));
        });
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
            Ok(ws::Message::Text(text)) => {
                ctx.text(text);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
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
