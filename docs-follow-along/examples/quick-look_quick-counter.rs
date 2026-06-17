use windows_reactor::*;

fn main() -> Result<()> {
    bootstrap()?;
    App::new().title("Quick Counter").render(app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_u8);
    let set_count_2 = set_count.clone();

    hstack((
        button("-1").on_click(move || set_count.call(count - 1)),
        text_block(format!("{count}"))
            .font_size(20_f64)
            .semibold()
            .width(40_f64),
        button("+1").on_click(move || set_count_2.call(count + 1)),
    ))
    .spacing(8_f64)
    .padding(24_f64)
    .into()
}
