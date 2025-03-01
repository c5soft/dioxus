use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch_cfg(app, |cfg| {
        cfg.with_window(|w| w.with_title("BorderLess Demo").with_decorations(false))
    });
}

fn app(cx: Scope) -> Element {
    let window = dioxus::desktop::use_window(&cx);

    cx.render(rsx!(
        link { href:"https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css", rel:"stylesheet" }
        header {
            class: "text-gray-400 bg-gray-900 body-font",
            onmousedown: move |_| window.drag(),
            div {
                class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center",
                a { class: "flex title-font font-medium items-center text-white mb-4 md:mb-0",
                    span { class: "ml-3 text-xl", "Dioxus"}
                }
                nav { class: "md:ml-auto flex flex-wrap items-center text-base justify-center" }
                button {
                    class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0",
                    onmousedown: |evt| evt.cancel_bubble(),
                    onclick: move |_| window.minimize(true),
                    "Minimize"
                }
                button {
                    class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0",
                    onmousedown: |evt| evt.cancel_bubble(),
                    onclick: move |_| window.close(),
                    "Close"
                }
            }
        }
    ))
}
