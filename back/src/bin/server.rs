// (c) 2019 Joost Yervante Damad <joost@damad.be>

extern crate actix_web;

use actix_web::{App, HttpRequest, server};

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
