use dioxus::prelude::*;
use dioxus_router::Link;

pub fn NavBar(cx: Scope) -> Element {
    let show_mobile_menu = use_state(cx, || false);

    render!(
        div {
            class: "fixed flex flex-col",
            header {
                class: "flex flex-row place-content-between items-center w-screen bg-zinc-900 px-8 h-16 border-b border-zinc-700",
                Logo {}
                Menu {
                    on_click: move |_: MouseEvent| show_mobile_menu.set(!show_mobile_menu)
                }
            }
            show_mobile_menu.then(|| rsx!(
                MobileMenu {}
            ))
        }
    )
}

pub fn MobileMenu(cx: Scope) -> Element {
    render!(
        div {
            class: "sticky flex flex-col w-screen bg-zinc-900 lg:hidden text-gold-100 font-['Barlow']",
            Link {
                to: "/camere",
                class: "flex items-center hover:text-white hover:bg-gold-100 px-8 h-16",
                "Camere"
            }
            Link {
                to: "/preturi",
                class: "flex items-center hover:text-white hover:bg-gold-100 px-8 h-16",
                "Preturi"
            }
            Link {
                to: "/contact",
                class: "flex items-center hover:text-white hover:bg-gold-100 px-8 h-16",
                "Contact"
            }
        }
    )
}

#[inline_props]
pub fn Menu<'a>(cx: Scope<'a>, on_click: EventHandler<'a, MouseEvent>) -> Element<'a> {
    render!(
        div {
            class: "hidden lg:flex lg:flex-row gap-8 text-gold-100 font-['Barlow']",
            Link {
                to: "/camere",
                class: "hover:text-white",
                "Camere"
            }
            Link {
                to: "/preturi",
                class: "hover:text-white",
                "Preturi"
            }
            Link {
                to: "/contact",
                class: "hover:text-white",
                "Contact"
            }
        }
        button {
            class: "lg:hidden h-full",
            onclick: move |event| on_click.call(event),
            img {
                class: "h-4/5",
                src: "assets/menu.svg"
            }
        }
    )
}

pub fn Logo(cx: Scope) -> Element {
    render!(
        Link {
            to: "/",
            class: "h-full",
            img {
                class: "h-full",
                src: "assets/logo.png"
            }
        }
    )
}

pub fn Footer(cx: Scope) -> Element {
    render!(
        div {
            class: "flex flex-col gap-2 h-16 py-2 text-gold-100 items-center border-t-2 border-zinc-600 ",
            p {
                "© 2023 Vila Doua Minuni"
            }
            p {
                "Designed by "
                a {
                    class: "text-zinc-200",
                    href: "https://www.github.com/arqalite",
                    "arqalite"
                }
            }
        }
    )
}
