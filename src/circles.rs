use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    let circles = use_ref(cx, || CircleManager::default());
    cx.render(rsx! {
        div {
            h1 { "Circles" }
            div {
                button { onclick: move |_| circles.write().undo(), "undo" }
                button { onclick: move |_| circles.write().redo(), "redo" }
            }
            div { padding: "5px", border: "1px solid black",
                svg {
                    width: "100%",
                    height: "100%",
                    onmousedown: move |e| circles.write().add_circle(e),
                    for circle in circles.read().circles.as_slice() {
                        circle {
                            cx: "{circle.x}",
                            cy: "{circle.y}",
                            r: "{circle.r}",
                            fill: "{circle.color}",
                            onclick: move |_| println!("clicked circle")
                        }
                    }
                }
            }
        }
    })
}

#[derive(Default)]
struct CircleManager {
    stack: Vec<Actions>,
    circles: Vec<Circle>,
    index: usize,
}

#[derive(Clone)]
struct Circle {
    x: i32,
    y: i32,
    r: i32,
    color: String,
}

#[derive(Clone)]
enum Actions {
    AddCircle {
        x: i32,
        y: i32,
        r: i32,
        color: String,
    },
    RemoveCircle {
        index: usize,
    },
    AdjustRadius {
        index: usize,
        r: i32,
    },
}

impl CircleManager {
    fn undo(&mut self) {
        if self.index == 0 {
            return;
        }
    }
    fn redo(&mut self) {
        if self.index == self.stack.len() {
            return;
        }
    }
    fn add_circle(&mut self, e: MouseEvent) {
        if self.index < self.stack.len() {
            self.stack.truncate(self.index);
        }

        let (x, y, r, color) = (e.offset_x, e.offset_y, 10, "red");
        self.stack.push(Actions::AddCircle {
            x,
            y,
            r,
            color: color.to_string(),
        });
        self.circles.push(Circle {
            x,
            y,
            r,
            color: color.to_string(),
        });
        self.index += 1;
    }
}
