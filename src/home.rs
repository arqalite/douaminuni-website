use dioxus::prelude::*;
use crate::common::Separator;

pub fn HomePage(cx: Scope) -> Element {
    render!(
        section {
            class: "flex flex-col text-white mt-16",
            HeroBanner {},
            MainDescription {},
            RoomsDescription {},
            ToursDescription {},
        }
    )
}

pub fn HeroBanner(cx: Scope) -> Element {
    render!(
        img {
            src: "assets/banner.jpg",
            class: "w-max"
        }
    )
}

pub fn MainDescription(cx: Scope) -> Element {
    render!(
        section {
            class: "flex flex-col items-center font-['Barlow'] bg-zinc-800 mt-8 gap-4 px-6",
            h1 {
                class: "text-4xl text-gold-100",
                "Vila Doua Minuni"
            },
            span {
                class: "text-center text-zinc-200 lg:w-3/5",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque id sem turpis. Duis aliquet ipsum ut eros iaculis malesuada. Nullam ornare mi pulvinar nisl pulvinar, ac pulvinar leo commodo. Fusce."
            }
            Separator {}
        }
    )
}

pub fn RoomsDescription(cx: Scope) -> Element {
    render!(
        section {
            class: "flex flex-col items-center font-['Barlow'] bg-zinc-800 my-4 gap-4 px-6",
            h1 {
                class: "text-3xl text-gold-100",
                "Cazare"
            },
            span {
                class: "text-center text-zinc-200 lg:w-3/5",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque id sem turpis. Duis aliquet ipsum ut eros iaculis malesuada. Nullam ornare mi pulvinar nisl pulvinar, ac pulvinar leo commodo. Fusce."
            }
            div {
                class: "flex flex-col lg:flex-row gap-4 py-4",
                div {
                    class: "flex flex-col gap-4",
                    span {
                        class: "text-gold-100 text-xl border-zinc-600 border-b-2 w-max pb-2",
                        "Camere duble"
                    },
                    img {
                        class: "mx-8 rounded-lg",
                        src: "assets/camera_1.jpg"
                    }
                }
                div {
                    class: "flex flex-col gap-4",
                    span {
                        class: "text-gold-100 text-xl border-zinc-600 border-b-2 w-max pb-2",
                        "Camere triple"
                    }
                    img {
                        class: "mx-8 rounded-lg",
                        src: "assets/camera_2.jpg"
                    }
                }
                div {
                    class: "flex flex-col gap-4",
                    span {
                        class: "text-gold-100 text-xl border-zinc-600 border-b-2 w-max pb-2",
                        "Camere de 4 locuri"
                    }
                    img {
                        class: "mx-8 rounded-lg",
                        src: "assets/camera_4.jpg"
                    }
                }
            },
            Separator {}
        }
    )
}

pub fn ToursDescription(cx: Scope) -> Element {
    render!(
        section {
            class: "flex flex-col items-center font-['Barlow'] bg-zinc-800 my-4 gap-4 px-6",
            h1 {
                class: "text-3xl text-gold-100",
                "Excursii montane"
            },
            span {
                class: "text-center text-zinc-200 lg:w-3/5",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque id sem turpis. Duis aliquet ipsum ut eros iaculis malesuada. Nullam ornare mi pulvinar nisl pulvinar, ac pulvinar leo commodo. Fusce."
            }
            div {
                class: "flex flex-col lg:flex-row gap-4 py-4",
                div {
                    class: "flex flex-col gap-4",
                    span {
                        class: "text-gold-100 text-xl border-zinc-600 border-b-2 w-max pb-2",
                        "Babele"
                    },
                    img {
                        class: "mx-8 rounded-lg",
                        src: "assets/camera_1.jpg"
                    }
                }
                div {
                    class: "flex flex-col gap-4",
                    span {
                        class: "text-gold-100 text-xl border-zinc-600 border-b-2 w-max pb-2",
                        "Sfinxul"
                    }
                    img {
                        class: "mx-8 rounded-lg",
                        src: "assets/camera_2.jpg"
                    }
                }
                div {
                    class: "flex flex-col gap-4",
                    span {
                        class: "text-gold-100 text-xl border-zinc-600 border-b-2 w-max pb-2",
                        "Lacul Bolboci"
                    }
                    img {
                        class: "mx-8 rounded-lg",
                        src: "assets/camera_4.jpg"
                    }
                }
            }
        }
    )
}
