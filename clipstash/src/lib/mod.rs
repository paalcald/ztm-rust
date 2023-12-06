pub mod data;
pub mod domain;
pub mod service;
pub mod web;

use data::AppDatabase;
use rocket::{fs::FileServer, Build, Rocket};
use web::renderer::Renderer;

pub fn rocket(config: RocketConfig) -> Rocket<Build> {
    rocket::build()
        .manage::<AppDatabase>(config.database)
        .manage::<Renderer>(config.renderer)
        .mount("/", web::http::routes())
        .mount("/static", FileServer::from("static"))
        .register("/", web::http::catcher::catchers())
}

pub struct RocketConfig {
    pub renderer: Renderer<'static>,
    pub database: AppDatabase,
}
