use windows_reactor::*;

fn main() -> Result<()> {
    bootstrap()?;
    App::new().title("Built-in Text Styling").render(app)
}

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        title("Title element"),
        subtitle("Subtitle element"),
        text_block("Regular text with modifiers")
            .font_size(14_f64)
            .foreground(Color::rgb(0, 120, 212)),
        caption("Caption for fine print"),
    ))
    .spacing(8_f64)
    .into()
}
