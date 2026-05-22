use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants}, input::{Input, InputState}, table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow}, v_flex
};

#[derive(Clone)]
pub struct KeyValueRow {
    pub key: Entity<InputState>,
    pub value: Entity<InputState>,
}

pub struct KeyValueTable {
    pub rows: Vec<KeyValueRow>,
}

impl KeyValueTable {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let row = KeyValueRow {
            key: cx.new(|cx| InputState::new(window, cx).placeholder("Key")),
            value: cx.new(|cx| InputState::new(window, cx).placeholder("Value")),
        };

        Self { rows: vec![row] }
    }

    pub fn add_empty_row(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let row = KeyValueRow {
            key: cx.new(|cx| InputState::new(window, cx).placeholder("Key")),
            value: cx.new(|cx| InputState::new(window, cx).placeholder("Value")),
        };

        self.rows.push(row);
        cx.notify();
    }

    pub fn set_rows(&mut self, window: &mut Window, cx: &mut Context<Self>, values: Vec<(String, String)>) {
        self.rows.clear();

        for (key, value) in values {
            self.rows.push(KeyValueRow {
                key: cx.new(|cx| InputState::new(window, cx).placeholder("Key").default_value(key)),
                value: cx.new(|cx| InputState::new(window, cx).placeholder("Value").default_value(value)),
            });
        }

        cx.notify();
    }

    pub fn drop_rows(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.rows = Vec::new();
        cx.notify();
    }

    pub fn read_rows_as_vec(&self, cx: &App) -> Vec<(SharedString, SharedString)> {
        let mut rows: Vec<(SharedString, SharedString)> = Vec::new();

        for row in self.rows.clone() {
            let key = row.key.read(cx).value();
            let value = row.value.read(cx).value();

            rows.push((key, value));
        }

        return rows;
    }
}

impl Render for KeyValueTable {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .child(
                Table::new()
                    .child(
                        TableHeader::new().child(
                            TableRow::new()
                                .child(TableHead::new().child("Key"))
                                .child(TableHead::new().child("Value")),
                        ),
                    )
                    .child(TableBody::new().children(self.rows.iter().map(|row| {
                        TableRow::new()
                            .child(TableCell::new().child(Input::new(&row.key)))
                            .child(TableCell::new().child(Input::new(&row.value)))
                    })))
            )
            .child(
                Button::new("add-row")
                    .ghost()
                    .label("+ Add row")
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.add_empty_row(window, cx);
                    }))
            )
    }
}
