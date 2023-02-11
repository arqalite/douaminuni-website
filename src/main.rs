#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Menu {}
        MainPage {}
    })
}

fn MainPage(cx: Scope) -> Element {
    cx.render(rsx!(
        section {
            class: "flex flex-col py-8 h-full w-screen bg-zinc-800 text-white",
            p {
                "Lorem ipsum sit dolor amet"
            },
            p {
                "This is sample text lalala"
            }
        }
    ))
}

fn NavBar(cx: Scope) -> Element {
    cx.render(
        rsx!(
            header {
                class: "flex flex-row place-content-between h-[10%] w-screen bg-zinc-900 px-16 md:px-8",
                //Logo {}
                //Menu {}
            }
        )
    )
}

fn Menu(cx: Scope) -> Element {

    let open_menu = use_state(&cx, || false);

    let nav_style = if **open_menu {
        "relative lg:block"
    } else {
        "lg:block"
    };

    let button_style = if **open_menu {
        ""
    } else {
        ""
    };
    
    let link_mobile_style = if **open_menu {
        "max-lg:float-none max-lg:block max-lg:text-left"
    } else {
        ""
    };
    

    let link_style = "float-left text-gold-100 text-center p-4 hover:text-white hover:bg-gold-100";

    cx.render(rsx!(
        nav {
            class: "topnav bg-zinc-900 {nav_style} h-[10%]",
            id: "myTopnav",
            a {
                href: "#acasa",
                class: "{link_style} {link_mobile_style} h-full w-max bg-zinc-900",
                img {
                    class: "h-full",
                    src: "assets/logo.webp"
                }
            }
            a {
                href: "#camere",
                class: "{link_style} {link_mobile_style} bg-zinc-900 h-full hidden lg:block",
                "Camere"
            }
            a {
                href: "#preturi",
                class: "{link_style} {link_mobile_style} bg-zinc-900 h-full hidden lg:block",
                "Preturi"
            }
            a {
                href: "#contact",
                class: "{link_style} {link_mobile_style} bg-zinc-900 h-full hidden lg:block",
                "Contact"
            }
            a {
                href: "javascript:void(0)",
                class: "icon {link_style} block right-0 top-0 lg:hidden float-right block h-full",
                onclick: move |_| open_menu.set(!open_menu),
                img {
                    class: "h-8 hover:text-white",
                    src: "assets/menu.svg"
                }
            }
        }
    ))
}