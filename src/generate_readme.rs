use std::{fs::File, io::Write};

use rocket::Route;

fn format_route(path: &str, index: usize) -> String {
    format!("**{}**: {}", index, path.replace("<", "\\<").replace(">", "\\>"))
}

pub fn generate_readme(routes: Vec<&Route>) {
    let file = File::create("README.md");
    let uris = routes
        .iter().enumerate()
        .map(|(index, route)| format_route(route.uri.path(), index))
        .collect::<Vec<String>>()
        .join("\n\n");
    let parsed_template = format!(
        "# NUMBERS API

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
