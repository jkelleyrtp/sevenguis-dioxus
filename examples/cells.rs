use dioxus::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

fn main() {
    dioxus::desktop::launch(app);
}

const COLUMNS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CSS: &str = r##"
.cell {
    margin: 0;
    padding: 0;
    border: 1px solid black;
}
.cell-input {
    margin: 0;
    padding: 0;
    border: 0;
}
.row {
    padding: 0;
}
.table {
    border-collapse: collapse;
}
"##;

fn app(cx: Scope) -> Element {
    use_context_provider(&cx, || CellManager::new(&cx));

    cx.render(rsx! { 
        style { "{CSS}" }
        table {
            class: "table",
            onkeydown: move |key| {},
            thead {
                tr {
                    COLUMNS.chars().map(|c| rsx! { th { "{c}" } })
                }
            }
            tbody { class: "row",
                (0..100).map(|row| rsx! {
                    tr { tabindex: "{row}",
                        COLUMNS.chars().map(|col| rsx! { Cell { col: col, row: row } })
                    }
                })
            }
        }
    })
}

#[inline_props]
fn Cell(cx: Scope, col: char, row: u64) -> Element {
    let manager = use_context::<CellManager>(&cx)?;

    // Register the cell with the manager - any updates will flow to us
    cx.use_hook(|_| manager.write_silent().register(*col, *row, cx.scope_id()));

    let manager_ref = manager.read();
    let value = manager_ref.get_cell(cx.scope_id());

    cx.render(rsx! { 
        td { class: "cell",
            input {
                class: "cell-input",
                r#type: "text",
                value: "{value}",
                width: "20px",
                oninput: move |val| manager.write_silent().set_cell(cx.scope_id(), val.value.clone())
            }
        }
    })
}

struct CellManager {
    entries: HashMap<ScopeId, String>,
    cells: HashMap<(char, u64), ScopeId>,
    depenedents: HashMap<ScopeId, HashSet<ScopeId>>,
    update_any: Arc<dyn Fn(ScopeId)>,
}

impl CellManager {
    fn new(cx: &ScopeState) -> Self {
        Self {
            entries: HashMap::new(),
            cells: HashMap::new(),
            depenedents: HashMap::new(),
            update_any: cx.schedule_update_any(),
        }
    }

    fn register(&mut self, col: char, row: u64, scope: ScopeId) {
        self.cells.insert((col, row), scope);
    }

    fn get_cell(&self, cell: ScopeId) -> &str {
        self.entries.get(&cell).map(|f| f.as_str()).unwrap_or("")
    }

    fn set_cell(&mut self, scope: ScopeId, value: String) {
        if value.starts_with('=') {
            // parse the formula and add the dependents
        } else {
            self.entries.insert(scope, value);
            if let Some(dependents) = self.depenedents.get(&scope) {
                dependents.iter().for_each(|dep| (self.update_any)(*dep));
            }
        }
    }
}
