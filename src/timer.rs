use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    let duration = use_state(cx, || 10.0);
    let progress = use_state(cx, || 0.0);

    use_future(cx, (), |_| {
        to_owned![duration, progress];
        async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                let new = *progress.current() + 0.1;
                if new <= *duration.current() {
                    progress.set(new);
                } else {
                    progress.set(*duration.current());
                }
            }
        }
    });

    cx.render(rsx! {
        div {
            input { r#type: "number", value: "{progress}" }
            input {
                class: "slider",
                r#type: "range",
                value: "{duration}",
                max: "20",
                min: "1",
                oninput: move |val| {
                    duration.set(val.value.parse::<f32>().unwrap());
                }
            }
            button { onclick: move |_| progress.set(0.0), "Reset" }
        }
    })
}
