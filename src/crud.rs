use dioxus::prelude::*;

#[derive(Clone)]
struct Name {
    first: String,
    last: String,
}

pub fn app(cx: Scope) -> Element {
    let names = use_state(cx, || {
        vec![
            Name {
                first: "John".to_string(),
                last: "Doe".to_string(),
            },
            Name {
                first: "Jane".to_string(),
                last: "Doe".to_string(),
            },
        ]
    });

    let filter = use_state(cx, || String::new());

    let selected_name = use_state(cx, || None);

    let selected_user: Option<&Name> = selected_name.get().and_then(|f| names.get().get(f));
    let selected_user_first = selected_user.map(|u| u.first.as_str()).unwrap_or_default();
    let selected_user_last = selected_user.map(|u| u.last.as_str()).unwrap_or_default();

    cx.render(rsx! {
        div {
            div {
                label { r#for: "filter" }
                input {
                    r#type: "text",
                    placeholder: "filter",
                    oninput: move |e| filter.set(e.value.clone())
                }
            }

            div {
                // create
                button {
                    onclick: move |_| {
                        names
                            .make_mut()
                            .push(Name {
                                first: "New".to_string(),
                                last: "Person".to_string(),
                            });
                    },
                    "Add"
                }

                // update

                // delete
                button {
                    onclick: move |_| {
                        if let Some(id) = selected_name.get().clone() {
                            names.make_mut().remove(id);
                            selected_name.set(None);
                        }
                    },
                    "Delete"
                }
            }

            div {
                input {
                    r#type: "text",
                    placeholder: "First Name",
                    value: "{selected_user_first}",
                    oninput: move |e| {
                        if let Some(id) = selected_name.get().clone() {
                            names.make_mut()[id].first = e.value.clone();
                        }
                    }
                }
                input {
                    r#type: "text",
                    placeholder: "Last Name",
                    value: "{selected_user_last}",
                    oninput: move |e| {
                        if let Some(id) = selected_name.get().clone() {
                            names.make_mut()[id].last = e.value.clone();
                        }
                    }
                }
            }

            div {
                names.iter().enumerate().filter_map(|(id, name)| {
                    let bg = if Some(id) == selected_name.get().clone() {
                        "lightblue"
                    } else {
                        "white"
                    };

                    if !name.first.starts_with(filter.get()) {
                        return None;
                    }

                    render! {
                        li {
                            background: "{bg}",
                            onclick: move |_| {
                                if bg == "lightblue" {
                                    selected_name.set(None);
                                } else {
                                    selected_name.set(Some(id))
                                }
                            },
                            "{name.last}, {name.first}"
                        }
                    }
                })
            }
        }
    })
}
