#![feature(proc_macro_hygiene, decl_macro)]

mod game;
mod db;
mod schema;

#[macro_use]
extern crate rocket;
// #[macro_use] extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate diesel;

use rocket_contrib::json::Json;
use game::Game;
use serde_json::Value;

#[get("/")]
fn index() -> &'static str {
    "Game catalog v0.1\ngithub.com/ATechnoHazard/game-catalog"
}

#[post("/create", data = "<game>")]
fn create(game: Json<Game>, conn: db::Connection) -> Json<Value> {
    let insert = Game { id: None, ..game.into_inner() };
    Game::create(&insert, &conn);
    Json(json!({
        "status": 201,
        "message": "Game successfully created"
    }))
}

#[get("/")]
fn read(conn: db::Connection) -> Json<Value> {
    Json(json!(Game::read(&conn)))
}

#[get("/<id>")]
fn read_by_id(id: i32, conn: db::Connection) -> Json<Value> {
    let game = match Game::read_by_id(id, &conn) {
        Ok(g) => g,
        Err(_) => return Json(json!({"status": 409, "message": format!("Game with ID {} not found", id)}))
    };
    Json(json!(game))
}

#[patch("/<id>", data = "<game>")]
fn update(id: i32, game: Json<Game>, conn: db::Connection) -> Json<Value> {
    let update = Game { id: Some(id), ..game.into_inner() };
    if Game::update(id, &update, &conn) {
        Json(json!({
            "status": 200,
            "message": format!("Game with ID {} successfully updated", id)
        }))
    } else {
        Json(json!({
            "status": 500,
            "message": "Error updating game"
        }))
    }
}

#[delete("/<id>")]
fn delete(id: i32, conn: db::Connection) -> Json<Value> {
    if Game::delete(id, &conn) {
        Json(json!({
            "status": 200,
            "message": format!("Game with ID {} successfully deleted", id)
        }))
    } else {
        Json(json!({
            "status": 500,
            "message": "Error deleting game"
        }))
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/game", routes![create, read, read_by_id, update, delete])
        .manage(db::connect())
        .launch();
}