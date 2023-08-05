use std::net::SocketAddr;

//use axum::{*, routing::get};
use axum::{
    routing::{get},
    Router,
    response::*, extract
};
use idle_game_server::{player::{Player}, items::ItemGenerator};
use idle_game_server::items::Item;

#[tokio::main]
async fn main() {

    let app = Router::new()
    .route("/", get(root))
    .route("/players", get(get_user))
    .route("/items", get(get_item));

    let addr =  SocketAddr::from(([127,0,0,1], 8080));
    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn root() -> &'static str{
    "Hello!"
}

async fn get_user() -> String {
    let player = Player {
        id: 42069,
        name: String::from("Gildehurt")
    };

    player.name
}

async fn generate_item(extract::Json(payload): extract::Json<ItemGenerator>) {
    
}

async fn get_item() -> Json<Item> {

    let item = Item {
        id:1,
        name: String::from("Sword"),
        damage: 5,
        attack_spd: 1.0,
        item_type: String::from("one-hannded")
    };

    Json(item)
}

