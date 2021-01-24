#![feature(proc_macro_hygiene, decl_macro)]

use rocket;
use rocket::error::LaunchError;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

/// Launches a rocket_mvc application,
/// guaranteed to call [Rocket::launch](rocket::Rocket::launch).
pub fn launch() -> LaunchError {
  // here is where we'd like to call codegen'd automounts from the build script  
  rocket::ignite()
    .attach(Template::fairing())
    .mount("/", StaticFiles::from("./www"))
    .launch()
}