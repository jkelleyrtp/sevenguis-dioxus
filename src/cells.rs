use dioxus::prelude::*;
use std::sync::Arc;

static CHARS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

// we're very careful to update this only when we need to
// so we don't trigger a re-render of the whole table
// It's fine to re-render the whole table, but this example doesn't want to take any shortcuts
pub fn app(cx: Scope) -> Element {
    let rows = use_ref(cx, || RowManager::new(cx.schedule_update_any()));

    render! {
        div { display: "flex", flex_direction: "column",
            for x in 0..99 {
                div { display: "flex", flex_direction: "row",
                    for y in 0..26 {
                        Cell { rows: rows.clone(), x: x, y: y }
                    }
                }
            }
        }
    }
}

#[inline_props]
fn Cell(cx: Scope, rows: UseRef<RowManager>, x: usize, y: usize) -> Element {
    let value = rows.read().get_cell(*x, *y);

    render! {
        div { border: "1px solid black", padding: "20px",
            input {
                oninput: move |e| rows.write_silent().handle_input(*x, *y, e.value.clone()),
                value: "{value}"
            }
        }
    }
}

struct RowManager {
    rows: Vec<Vec<(usize, ScopeId)>>,
    update_any: Arc<dyn Fn(ScopeId)>,
}

impl RowManager {
    fn new(update_any: Arc<dyn Fn(ScopeId)>) -> Self {
        let rows = vec![vec![(0, ScopeId(0)); 26]; 100];
        Self { rows, update_any }
    }

    fn handle_input(&self, x: usize, y: usize, value: String) {
        let row = &self.rows[x];
        let cell = &row[y];
        let scope = cell.1;
        (self.update_any)(scope);
    }

    fn get_cell(&self, x: usize, y: usize) -> usize {
        self.rows[x][y].0
    }
}
