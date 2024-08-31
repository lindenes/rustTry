use std::time::Duration;
use http_body_util::Full;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use hyper::{Request, Response, Method};
use hyper::body::Bytes;
use hyper::http;
use hyper::header::{HeaderValue, CONTENT_TYPE};
use hyper_tungstenite::{tungstenite, HyperWebsocket};
use tungstenite::Message;
async fn serve_websocket(websocket: HyperWebsocket) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut websocket = websocket.await?;

    // Запускаем задачу для обработки входящих сообщений
    tokio::spawn(async move {
        while let Some(message) = websocket.next().await {
            match message {
                Ok(Message::Text(msg)) => {
                    println!("Received text message: {}", msg);
                },
                Ok(Message::Binary(msg)) => {
                    println!("Received binary message: {:?}", msg);
                },
                Ok(Message::Ping(msg)) => {
                    println!("Received ping message: {:?}", msg);
                },
                Ok(Message::Pong(msg)) => {
                    println!("Received pong message: {:?}", msg);
                },
                Ok(Message::Close(msg)) => {
                    if let Some(msg) = &msg {
                        println!("Received close message with code {} and reason: {}", msg.code, msg.reason);
                    } else {
                        println!("Received close message");
                    }
                    break;
                },
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    break;
                },
                _ => {}
            }
        }
    });

    Ok(())
}
pub async fn get_routes(mut request:Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let path = request.uri().path();

    let response =
        match (path, request.method()) {
            ("/token", &Method::GET) =>{
                let (response, websocket) = hyper_tungstenite::upgrade(&mut request, None).unwrap();
                tokio::spawn(async move {
                    if let Err(e) = serve_websocket(websocket).await {
                        eprintln!("Error in websocket connection: {e}");
                    }
                });

                // Return the response so the spawned future can continue.
                Ok(response)
            },
            ("/version",&Method::GET) =>
                Response::builder()
                    .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
                    .body(Full::from(Bytes::from("{\"version\":\"0.0.1\"}"))),
            _ =>
                Response::builder()
                    .status(404)
                    .body(Full::from(Bytes::from("Not Found")))

        };

     Ok(response?)
}