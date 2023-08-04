use axum::{response::Html, routing::get, Json, Router};
use serde::Serialize;

const SERVICE_ADDR: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let routes = Router::new()
        .route("/", get(get_inicio))
        .route("/pessoa", get(get_pessoa));

    let app = axum::Server::bind(&SERVICE_ADDR.parse().unwrap());
    println!("ouvindo em http://{}", SERVICE_ADDR);
    app.serve(routes.into_make_service()).await.unwrap();
}

async fn get_inicio() -> Html<&'static str> {
    // println!(" GET / (get_inicio)");

    Html("Oi! tudo bem...")
}

#[derive(Serialize)]
struct Pessoa {
    nome: String,
}

async fn get_pessoa() -> Json<Pessoa> {
    // println!(" GET /pessoa (get_pessoa)");

    Json(Pessoa {
        nome: "Jeudi".to_owned(),
    })
}
