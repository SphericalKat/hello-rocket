use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::schema::games;
use diesel::result::Error;

#[table_name = "games"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct Game {
    pub id: Option<i32>,
    pub name: String,
    pub developer: String,
    pub is_goty: bool,
}

impl Game {
    pub fn create(game: &Game, conn: &PgConnection) {
        diesel::insert_into(games::table)
            .values(game)
            .execute(conn)
            .expect("Error creating new game");
    }

    pub fn read(conn: &PgConnection) -> Vec<Game> {
        games::table.order(games::id.asc()).load::<Game>(conn).unwrap()
    }

    pub fn read_by_id(id: i32, conn: &PgConnection) -> Result<Game, Error> {
        let game = games::table.find(id).get_result::<Game>(conn)?;
        Ok(game)
    }

    pub fn update(id: i32, game: &Game, conn: &PgConnection) -> bool {
        diesel::update(games::table.find(id)).set(game).execute(conn).is_ok()
    }

    pub fn delete(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(games::table.find(id)).execute(conn).is_ok()
    }
}