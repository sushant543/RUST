use tokio::net::TcpListener;
use axum::{routing::post,Json,Router};
use serde::{Deserialize,Serialize};
use std::net::SocketAddr;
#[derive(Debug,Deserialize)]
struct Greetreq{
    name: String
}
#[derive(Debug,Serialize)]
struct Greetres{
    message: String,
}
async fn greet(Json(payload): Json<Greetreq>)-> Json<Greetres>{
 let reply = Greetres{message:format!("HY,{}!",payload.name),};
Json(reply)
}
#[tokio::main]
async fn main() {
    let app = Router::new().route("/greet", post(greet));
    let addr: SocketAddr = "0.0.0.0:8080".parse().expect("Invalid address format");
    println!("Attempting to bind to: {}", addr);
    
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    println!("Server is up and running :): {}", addr);
    
    axum::serve(listener, app).await.unwrap();}
