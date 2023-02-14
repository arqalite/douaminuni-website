use dioxus::prelude::*;
use gloo_console::log;
use crate::common::RoomType;
use crate::common::Separator;

pub fn RoomsPage(cx: Scope) -> Element {
    render!(
        section {
            class: "flex flex-col items-center font-['Barlow'] bg-zinc-800 mt-16 gap-2 py-4 px-8",
            h1 {
                class: "text-4xl text-gold-100",
                "Camere"
            },
            Separator {},
            RoomsList {}
        }
    )
}

pub fn RoomsList(cx: Scope) -> Element {
    let room_list = crate::common::construct_rooms();
    let room_type: &UseState<Option<RoomType>> = use_state(cx, || None);

    let set_room_filter = move |evt: Event<FormData>| {
        if evt.data.value == "Duble" {
            room_type.set(Some(RoomType::Double));
        } else if evt.data.value == "Triple" {
            room_type.set(Some(RoomType::Triple));
        } else if evt.data.value == "4 locuri" {
            room_type.set(Some(RoomType::Quadruple));
        } else {
            room_type.set(None);
        }
    };

    let mut room_filter = RoomType::Double;
    let mut is_filter = false;
    if room_type.is_some() {
        room_filter = room_type.unwrap();
        is_filter = true;
    };

    render!(
        section {
            div {
                class: "flex flex-row mb-4 lg:justify-end",
                p {
                    class: "text-gold-100",
                    "Filtrati dupa tipul camerei:"
                }
                select {
                    name: "camere",
                    id: "camere",
                    onchange: set_room_filter,
                    class: "",
                    option {
                        "Toate camerele"
                    },
                    option {
                        "Duble"
                    },
                    option {
                        "Triple"
                    },
                    option {
                        "4 locuri"
                    }
                }
            }
            div {
                class: "flex flex-col lg:grid lg:grid-cols-3 w-full gap-4",
                for room in room_list.iter() {
                    if room.size == room_filter || !is_filter {
                        rsx!(
                            div {
                                class: "flex flex-col gap-6 py-2 w-full",
                                span {
                                    class: "text-gold-100 text-xl border-zinc-600 border-b-2 w-max",
                                    "{room.name}"
                                }
                                div {
                                    class: "flex flex-row justify-center items-center",
                                    button {
                                        class: "absolute w-max left-8 bg-black/60",
                                        onclick: |_| {
                                            log!("haha");
                                        },
                                        img {
                                            class: "",
                                            src: "assets/chevron-left.svg"
                                        },
                                    }
                                    img {
                                        class: "rounded-lg w-full",
                                        src: room.photos[room.index],
                                    }
                                    button {
                                        class: "absolute w-max right-8 bg-black/60",
                                        img {
                                            class: "",
                                            src: "assets/chevron-right.svg"
                                        },
                                    }
                                }
                            }
                        )
                    }
                }
            }
        }
    )
}
