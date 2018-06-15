#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate regex;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate serde;


use std::collections::HashMap;
use std::io::{Write, Read};
use std::process::{Command, Stdio};

use rocket_contrib::{Json, Value};
use regex::Regex;

#[derive(Serialize)]
struct Position {
    line: u8,
    column: u8
}

#[derive(Serialize)]
struct TouistError {
    message: String,
    start: Position,
    end: Position
}

#[derive(FromForm)]
struct TouistInput {
    source: String,
    solver: String
}

// ROCKET_BASE allows to set the appropriate URL base instead of the (default) / path.
lazy_static! {
    static ref BASE : String = std::env::var("ROCKET_BASE").unwrap_or("/".to_string());
}

fn parse_error(error: String) -> Option<TouistError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"line (?P<line>[0-9]+), col (?P<colstart>[0-9]+)-(?P<colend>[0-9]+): error: (?P<message>[\s\S]*)").unwrap();
    }

    for capture in RE.captures(error.as_str()) {
        let line = capture.name("line").map_or(0, |m| { m.as_str().parse::<u8>().unwrap() });
        let colstart = capture.name("colstart").map_or(0, |m| { m.as_str().parse::<u8>().unwrap() });
        let tmp = capture.name("colend").map_or(0, |m| { m.as_str().parse::<u8>().unwrap() });
        let colend = if tmp == 1 { 255 } else { tmp };

        return Some(TouistError {
            message: capture.name("message").map_or(String::from(""), |m| String::from(m.as_str())),
            start: Position { line: line, column: colstart },
            end: Position { line: line, column: colend }
        })
    }
    return None
}

#[get("/")]
fn index() -> Json<Value> {
    Json(json!({
        "solve": format!("{}/solve?source={{touist_code}}&solver={{solver}}", BASE.trim_right_matches("/")),
        "latex": format!("{}/latex?source={{touist_code}}&solver={{solver}}", BASE.trim_right_matches("/"))
    }))
}

#[get("/ping")]
pub fn ping() -> Json<Value> {
    Json(json!({
        "pong": true
    }))
}

#[get("/healthcheck")]
pub fn healthcheck() -> Json<Value> {
    Json(json!({"message": "All good"}))
}

#[get("/latex?<touist_input>")]
fn latex(touist_input: TouistInput) -> Json<Value> {
    let process =
        Command::new("./external/touist")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("-")
            .arg("--latex")
            .arg("--linter")
            .arg(format!("--{}", touist_input.solver).as_str())
            .spawn().unwrap();

    let _ = process.stdin.unwrap().write_all(touist_input.source.as_bytes());

    let mut stdout = String::new();
    let _ = process.stdout.unwrap().read_to_string(&mut stdout);

    let mut stderr = String::new();
    let _ = process.stderr.unwrap().read_to_string(&mut stderr);

    Json(json!({
        "latex": format!(r"\begin{{gathered}}{}\end{{gathered}}", stdout),
        "error": parse_error(stderr)
    }))
}

#[get("/solve?<touist_input>")]
fn solve(touist_input: TouistInput) -> Json<Value> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<value>[0|1]) (?P<key>\S*)").unwrap();
    }

    let process =
        Command::new("./external/touist")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("-")
            .arg("--solve")
            .arg(format!("--{}", touist_input.solver).as_str())
            .arg("--limit")
            .arg("1000")
            .spawn().unwrap();

    let _ = process.stdin.unwrap().write_all(touist_input.source.as_bytes());

    let mut stdout = String::new();
    let _ = process.stdout.unwrap().read_to_string(&mut stdout);

    let mut stderr = String::new();
    let _ = process.stderr.unwrap().read_to_string(&mut stderr);

    let mut models = Vec::new();

    let parts: Vec<&str> = stdout.split("==== ").collect();
    let len = parts.len();

    if len > 1 {
        for part in &parts[0..len] {
            let mut model : HashMap<String, bool> = HashMap::new();
            if !part.to_string().starts_with("model") { continue }
            for capture in RE.captures_iter(part) {
                model.insert(
                    capture.name("key").map_or(String::from(""), |m| String::from(m.as_str())),
                    capture.name("value").map_or(false, |m| m.as_str() == "1")
                );
            }
            models.push(model);
        }
    }

    Json(json!({
        "models": models,
        "error": parse_error(stderr)
    }))
}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn main() {
    rocket::ignite()
        .mount(&BASE, routes![index, latex, solve, ping, healthcheck,])
        .mount("/", routes![ping, healthcheck])
        .catch(errors![not_found])
        .launch();
}
