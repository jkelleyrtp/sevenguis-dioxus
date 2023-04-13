mod cells;
mod circles;
mod counter;
mod crud;
mod flight;
mod temperature;
mod timer;

fn main() {
    dioxus_desktop::launch(app);
}

use dioxus::prelude::*;
use dioxus_desktop::use_window;

fn app(cx: Scope) -> Element {
    let win = use_window(cx);

    render! {
        div { display: "flex", flex_direction: "column",
            h1 { "Dioxus 7 guis" }
            button { onclick: launch(win, counter::app), "1. Counter" }
            button { onclick: launch(win, temperature::app), "2. Temperature" }
            button { onclick: launch(win, flight::app), "3. Flight Booker" }
            button { onclick: launch(win, timer::app), "4. Timer" }
            button { onclick: launch(win, crud::app), "5. CRUD" }
            button { onclick: launch(win, circles::app), "6. Circles" }
            button { onclick: launch(win, cells::app), "7. Cells" }
        }
    }
}

fn launch(
    window: &dioxus_desktop::DesktopContext,
    c: fn(Scope) -> Element,
) -> impl Fn(MouseEvent) + '_ {
    move |_| {
        window.new_window(VirtualDom::new(c), Default::default());
    }
}
