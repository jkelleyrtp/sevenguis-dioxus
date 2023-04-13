use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        div {
            p { "{count}" }
            button { onclick: move |_| count += 1, "Count" }
        }
    })
}
