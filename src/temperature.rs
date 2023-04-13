use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    let celcius = use_state(cx, || 0.0);
    let farenheight = use_state(cx, || 0.0);

    cx.render(rsx! {
        div {
            input {
                r#type: "number",
                value: "{celcius}",
                onchange: move |e| {
                    if let Ok(value) = e.value.parse::<f32>() {
                        celcius.set(value);
                        farenheight.set(value * 1.8 + 32.0);
                    }
                }
            }
            label { " Celsius =" }
            input {
                r#type: "number",
                value: "{farenheight}",
                onchange: move |e| {
                    if let Ok(value) = e.value.parse::<f32>() {
                        farenheight.set(value);
                        celcius.set((value - 32.0) / 1.8);
                    }
                }
            }
            label { "Farhenheit" }
        }
    })
}
