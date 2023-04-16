// use std::sync::Mutex;

// use serde::Serialize;

#[macro_use]
extern crate rocket;

// static SERVERS: Mutex<Vec<GameServer>> = Mutex::new(vec![]);

// #[derive(Serialize)]
// struct GameServer {
//     name: String,
//     id: String,
// }

// #[get("/servers")]
// fn get_servers() -> Vec<String> {
//     let servers = SERVERS.lock().unwrap();
//     servers
//         .into_iter()
//         .map(|server| server.name)
//         .collect::<Vec<String>>()
//         .clone()
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![])
}
