use std::{fs::File, io::Write};

use rocket::Route;

pub fn generate_readme(routes: Vec<&Route>) {
    let file = File::create("README.md");
    let uris = routes
        .iter()
        .map(|route| route.uri.path())
        .collect::<Vec<&str>>()
        .join("\n\n");
    let parsed_template = format!("# NUMBERS API

Note: README.md Generated automatically

## Description

Basically, this is project for learning Rocket framework and Rust

## Routes

{}
",
        uris
    );
    let _ = file.unwrap().write(parsed_template.as_bytes());
}
