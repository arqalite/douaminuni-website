#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Redirect, Route, Router};

mod common;
mod home;
mod rooms;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    render!(
        Router {
            Route {
                to: "/",
                common::NavBar {}
                home::HomePage {},
                common::Footer {}
            },
            Route {
                to: "/camere",
                common::NavBar {},
                rooms::RoomsPage {},
                common::Footer {},
            },
            Redirect {
                from: "",
                to: "/"
            }
        }
    )
}
