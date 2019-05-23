#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;



mod config;

//fn watch() -> notify::Result<()> {
//    // Create a channel to receive the events.
//    let (tx, rx) = channel();
//
//    // Automatically select the best implementation for your platform.
//    // You can also access each implementation directly e.g. INotifyWatcher.
//    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(2)));
//
//    // Add a path to be watched. All files and directories at that path and
//    // below will be monitored for changes.
//    try!(watcher.watch("/home/adelvecchio/Documents/workspace/rocket_dynamic_router/src/config/routes/", RecursiveMode::Recursive));
//
//    // This is a simple loop, but you may want to use more complex logic here,
//    // for example to handle I/O.
//    loop {
//        match rx.recv() {
//            Ok(event) => println!("{:?}", event),
//            Err(e) => println!("watch error: {:?}", e),
//        }
//    }
//}
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[get("/")]
fn index2() -> &'static str {
    "Hello, world!"
}


fn rocket() -> rocket::error::LaunchError {
    rocket::ignite().mount("/", routes![index]).launch();
    rocket::ignite().mount("/", routes![index2]).launch()
}

fn main() {
//    if let Err(e) = watch() {
//        println!("error: {:?}", e)
//    }
    config::parse_router::parse_router()
}