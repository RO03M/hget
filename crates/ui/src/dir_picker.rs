use gpui::*;
use gpui_component::button::Button;

#[derive(IntoElement)]
pub struct DirPicker;

impl RenderOnce for DirPicker {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        return div().flex().justify_center().items_center().child(
            Button::new("dir-picker")
                .label("Open folder")
                .on_click(|event, window, cx| {
                    let path_future = cx.prompt_for_paths(PathPromptOptions {
                        files: false,
                        directories: true,
                        multiple: false,
                        prompt: None,
                    });

                    cx.spawn(async move |cx| {
                        let paths = path_future.await.ok()?.ok()??;
                        let path = paths.first()?.clone();

                        println!("{:?}", path);
                        
                        Some(())
                    }).detach();

                    // cx.spawn(async move |cx| {
                    //     let dir = cx
                    //         .background_executor()
                    //         .spawn(async {
                    //             return rfd::FileDialog::new()
                    //                 .set_directory("/")
                    //                 .pick_folder();
                    //         }).await;

                    //     println!("{:?}", dir.unwrap());
                    // })
                    // .detach();
                }),
        );
    }
}
