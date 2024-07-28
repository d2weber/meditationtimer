use zoon::*;

use std::sync::Arc;

use super::duration::Duration;

pub fn root() -> impl Element {
    Column::new()
        .s(Height::screen())
        .item(
            Column::new()
                .s(Align::center())
                .s(Width::exact(200))
                .s(Gap::both(10))
                .s(Font::new().size(20).center())
                .items_signal_vec(super::timers().signal_vec_cloned().map(timer))
                .item(new_timer())
                .item(remaining())
                .item_signal(super::clk_running().map_bool(
                    || stop_button(super::stop_clk).left_either(),
                    || start_button(super::start_clk).right_either(),
                )),
        )
        .item(
            Link::new()
                .s(Font::new().size(10).color(hsluv!(0, 0, 40)))
                .label("Background vector created by rawpixel.com - www.freepik.com")
                .to("https://www.freepik.com/vectors/background"),
        )
}

fn remaining() -> impl Element {
    Label::new().label_signal(
        super::remaining()
            .signal_cloned()
            .map(|e| e.unwrap_or(Duration::new()))
            .map(|d| {
                let (mins, secs) = d.mins_secs();
                colon_centered_row(label_with_digits(mins), label_with_digits(secs))
            }),
    )
}

fn new_timer() -> impl Element {
    TextInput::new()
        .s(RoundedCorners::all(3))
        .s(Padding::all(10))
        .s(Font::new().center())
        .update_raw_el(|e| e.attr("type", "number"))
        .label_hidden("Add new interval")
        .on_change(|t| super::new_timer().set(t))
        .on_key_down_event(|event| event.if_key(Key::Enter, super::add_timer))
        .placeholder(Placeholder::new("Add new interval"))
        .text_signal(super::new_timer().signal_cloned())
}

use super::{
    select_from, SelectedPart,
    SelectedPart::{Minutes, Seconds},
};

fn timer(duration: Arc<Duration>) -> impl Element {
    //super::is_selected_timer(duration.clone()).map(move |selected| {
    Row::new()
        .s(Background::new().color_signal(
            super::running_timer()
                .signal_cloned()
                .map(clone!((duration) move |t| super::same_as(&t, &duration)))
                .map_bool(
                    move || hsluv!(65, 29, 86, 70),
                    move || hsluv!(39, 90, 60, 90),
                ),
        ))
        .s(Padding::all(10))
        .s(RoundedCorners::all(3))
        .item(
            Row::new()
                .s(Align::new().center_x())
                .item(Label::new().label(":"))
                .element_on_left_signal(view_duration(duration.clone(), Minutes))
                .element_on_right_signal(view_duration(duration.clone(), Seconds)),
        )
        .element_on_right(remove_duration_button(duration.clone()))
}

fn view_duration(
    duration: Arc<Duration>,
    selection: SelectedPart,
) -> impl Signal<Item = impl Element> {
    super::is_selected_timer(select_from(duration.clone(), selection)).map_bool(
        move || Column::new().item(duration_edit()),
        clone!((duration) move || Column::new().item(duration_label(duration.clone(), selection))),
    )
}

fn duration_label(duration: Arc<Duration>, selection: SelectedPart) -> impl Element {
    let duration = duration.clone();
    let number = match selection {
        SelectedPart::Minutes => duration.mins_part(),
        SelectedPart::Seconds => duration.secs_part(),
    };
    Column::new().item(
        Label::new()
            .label(format!("{:02}", number))
            .on_double_click(clone!((duration) move || {
                super::selected_timer().set(Some(select_from(duration.clone(),selection)));
                super::edited_timer().set(Some(number.to_string()));
            })),
    )
}

fn duration_edit() -> impl Element {
    let text_signal = super::edited_timer()
        .signal_cloned()
        .map(Option::unwrap_throw);
    Column::new().item(
        TextInput::new()
            .s(Height::exact(23))
            .s(Width::exact(40))
            .s(Font::new().size(20).center())
            .update_raw_el(|e| e.attr("type", "number"))
            .label_hidden("selected duration")
            .focus(true)
            .on_blur(super::save_edited_timer)
            .on_change(move |text| super::edited_timer().set_neq(Some(text)))
            .on_key_down_event(|event| match event.key() {
                Key::Escape => super::selected_timer().set(None),
                Key::Enter => super::save_edited_timer(),
                _ => (),
            })
            .text_signal(text_signal),
    )
}

fn label_with_digits(number: u32) -> impl Element {
    Label::new().label(format!("{:02}", number))
}

fn colon_centered_row<L: Element, R: Element>(left: L, right: R) -> impl Element {
    Row::new()
        .s(Align::new().center_x())
        .item(Label::new().label(":"))
        .element_on_left(left)
        .element_on_right(right)
}

fn remove_duration_button(duration: Arc<Duration>) -> impl Element {
    Button::new()
        .s(Width::exact(30))
        .s(Height::exact(30))
        .s(Transform::new().move_left(30).move_down(5))
        .s(Font::new().size(25))
        .on_press(move || super::remove_timer(duration.clone()))
        .label("×")
}

fn start_button(on_press: fn()) -> impl Element {
    button("Start", hsluv!(52.1, 53.6, 90.3), hsluv!(149.9, 98.5, 66.8), on_press)
}

fn stop_button(on_press: fn()) -> impl Element {
    button("Stop", hsluv!(12.2, 97.2, 85.8), hsluv!(12.2, 80.7, 55), on_press)
}

fn button(label: &str, bg_color: HSLuv, bg_color_hovered: HSLuv, on_press: fn()) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::all(6))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(move || bg_color_hovered, move || bg_color)))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label(label)
        .on_press(on_press)
}
