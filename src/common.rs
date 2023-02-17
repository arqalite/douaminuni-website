use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Separator(cx: Scope) -> Element {
    render!(
        span {
            class: "border-zinc-700 w-full border mt-4"
        }
    )
}

pub fn NavBar(cx: Scope) -> Element {
    let show_mobile_menu = use_state(cx, || false);

    render!(
        div {
            class: "z-10 fixed flex flex-col",
            header {
                class: "flex flex-row place-content-between items-center w-screen bg-zinc-900 px-8 h-16",
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
            class: "flex flex-col gap-2 h-16 my-4 py-2 text-gold-100 items-center border-t-2 border-zinc-600 ",
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

pub struct Room<'a> {
    pub name: &'a str,
    pub size: RoomType,
    pub index: usize,
    pub photos: Vec<&'a str>
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RoomType {
    Double,
    Triple,
    Quadruple
}

pub fn construct_rooms() -> Vec<Room<'static>> {
    vec![
        Room {
            name: "Camera 1",
            size: RoomType::Triple,
            index: 0,
            photos: vec!["assets/camera_1.jpg","assets/camera_2.jpg","assets/camera_4.jpg"]
        },
        Room {
            name: "Camera 2",
            size: RoomType::Double,
            index: 0,
            photos: vec!["assets/empty.jpg"]

        },
        Room {
            name: "Camera 3",
            size: RoomType::Quadruple,
            index: 0,
            photos: vec!["assets/empty.jpg"]
        },
        Room {
            name: "Camera 4",
            size: RoomType::Triple,
            index: 0,
            photos: vec!["assets/empty.jpg"]
        },
        Room {
            name: "Camera 5",
            size: RoomType::Triple,
            index: 0,
            photos: vec!["assets/empty.jpg"]
        },
        Room {
            name: "Camera 6",
            size: RoomType::Triple,
            index: 0,
            photos: vec!["assets/empty.jpg"]
        },
        Room {
            name: "Camera 7",
            size: RoomType::Triple,
            index: 0,
            photos: vec!["assets/empty.jpg"]
        },
        Room {
            name: "Camera 8",
            size: RoomType::Double,
            index: 0,
            photos: vec!["assets/empty.jpg"]
        },
        Room {
            name: "Camera 9",
            size: RoomType::Double,
            index: 0,
            photos: vec!["assets/empty.jpg"]
        },
        Room {
            name: "Camera 10",
            size: RoomType::Triple,
            index: 0,
            photos: vec!["assets/empty.jpg"]
        },
        Room {
            name: "Camera 11",
            size: RoomType::Quadruple,
            index: 0,
            photos: vec!["assets/empty.jpg"]
        },
        Room {
            name: "Camera 12",
            size: RoomType::Double,
            index: 0,
            photos: vec!["assets/empty.jpg"]
        },
        Room {
            name: "Camera 14",
            size: RoomType::Triple,
            index: 0,
            photos: vec!["assets/empty.jpg"]
        },
        Room {
            name: "Camera 15",
            size: RoomType::Double,
            index: 0,
            photos: vec!["assets/empty.jpg"]
        },
        Room {
            name: "Camera 16",
            size: RoomType::Double,
            index: 0,
            photos: vec!["assets/empty.jpg"]
        },
        Room {
            name: "Camera 17",
            size: RoomType::Triple,
            index: 0,
            photos: vec!["assets/empty.jpg"]
        },
        Room {
            name: "Camera 18",
            size: RoomType::Double,
            index: 0,
            photos: vec!["assets/empty.jpg"]
        },
    ]
}
