use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        text_block("Hello from Reactor!").font_size(24f64).bold(),
        text_block("No XAML. No data binding. Just Rust"),
    ))
    .spacing(12f64)
    .into()
}

fn main() -> Result<()> {
    bootstrap()?;
    App::new().title("Hello World").render(app)
}
