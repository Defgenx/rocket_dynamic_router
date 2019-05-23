#![allow(missing_docs)]

extern crate toml;
extern crate rustc_serialize;

use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;
use toml::Value;
use rustc_serialize::Decodable;
use rustc_serialize::Decoder;


const ROUTING_FILE: &str = "routes.toml";

#[derive(RustcDecodable,Debug)]
struct TRoute {
    name : Option<String>,
    url : Option<String>,
    method : Option<String>,
    http_method : Option<u32>
}

#[derive(RustcDecodable,Debug)]
struct TRouteVec {
    route    : Option<Vec<TRoute>>
}

pub fn parse_router() {
//    let exe_path = match env::current_exe() {
//        Ok(exe_path) => exe_path.display(),
//        Err(e) => return Err(e),
//    };
    let exe_path = env::current_exe().unwrap();
    let mut routing_file = format!("{}{}{}", exe_path.display().to_string(), "/", ROUTING_FILE);
    let content = fs::read_to_string(routing_file)
        .expect("Something went wrong reading the file");
    let tml :TRouteVec = toml::decode_str(&content).unwrap();
    for x in tml.route.unwrap() {
        println!("X: {:?}", x);
    }
}