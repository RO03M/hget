use gpui::*;
use hget_core::executor::HttpResponse;

use crate::components::selectable_text::SelectableText;

pub struct ResponsePane {
    id: ElementId,
    selectable_text: Entity<SelectableText>,
    response: Option<HttpResponse>,
    pending_update: bool,
}

impl ResponsePane {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            id: "response-pane".into(),
            selectable_text: cx.new(|cx| SelectableText::new("".into(), window, cx)),
            response: None,
            pending_update: false,
        }
    }

    pub fn set_response(&mut self, cx: &mut Context<Self>, response: HttpResponse) {
        println!("{:?}", response);
        self.response = Some(response.clone());
        self.pending_update = true;
        cx.notify();
        // cx.notify(I need the entityid here);
    }
}

impl Render for ResponsePane {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(response) = self.response.clone() else {
            return div().child("Make a request!");
        };

        if self.pending_update {
            self.selectable_text.update(cx, |this, cx| {
                this.set_text(window, cx, response.clone().body.into());
            });

            self.pending_update = false;
        }

        return div().w_full().h_full().child(self.selectable_text.clone());
    }
}
