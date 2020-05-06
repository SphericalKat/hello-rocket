#![feature(proc_macro_hygiene, decl_macro)]

mod game;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

use rocket_contrib::json::Json;
use game::Game;

#[get("/")]
fn index() -> &'static str {
    "Game catalog v0.1\ngithub.com/ATechnoHazard/game-catalog"
}

#[post("/create", data = "<game>")]
fn create_game(mut game: Json<Game>) -> Json<Game> {
    game.id = Some(4);
    game
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/game", routes![create_game])
        .launch();
}