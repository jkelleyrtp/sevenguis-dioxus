/*
The task is to build a frame containing a combobox C with the two options “one-way flight” and “return flight”, two
textfields T1 and T2 representing the start and return date, respectively, and a button B for submitting the selected
flight. T2 is enabled iff C’s value is “return flight”. When C has the value “return flight” and T2’s date is strictly
before T1’s then B is disabled. When a non-disabled textfield T has an ill-formatted date then T is colored red and B is
disabled. When clicking B a message is displayed informing the user of his selection (e.g. “You have booked a one-way
flight on 04.04.2014.”). Initially, C has the value “one-way flight” and T1 as well as T2 have the same (arbitrary)
date (it is implied that T2 is disabled).
*/

use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    let flight_type = use_state(cx, || "one-way".to_string());
    let today = use_state(cx, || chrono::Local::now().format("%Y-%m-%d").to_string());
    let start = use_state(cx, || today.to_string());
    let end = use_state(cx, || today.to_string());

    let return_disabled = matches!(flight_type.get().as_str(), "one-way");

    render! {
        div {
            div {
                label { r#for: "start", "Trip direction: " }
                select {
                    id: "start",
                    name: "trip-start",
                    value: "{flight_type}",
                    onchange: move |e| flight_type.set(e.value.to_string()),
                    option { value: "one-way", "One-way flight" }
                    option { value: "return", "Return flight" }
                }
            }

            div {
                label { r#for: "start", "Start date: " }
                input {
                    r#type: "date",
                    id: "start",
                    name: "trip-start",
                    value: "{start}",
                    min: "{today}",
                    oninput: move |e| start.set(e.value.clone())
                }
            }

            div {
                label { r#for: "start", "Return date: " }
                input {
                    r#type: "date",
                    id: "start",
                    disabled: "{return_disabled}",
                    name: "trip-start",
                    value: "{end}",
                    min: "{start}",
                    oninput: move |e| end.set(e.value.clone())
                }
            }
        }
    }
}
