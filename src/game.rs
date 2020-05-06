#[derive(Serialize, Deserialize)]
pub struct Game {
    pub id: Option<i32>,
    pub name: String,
    pub developer: String,
    pub is_goty: bool
}