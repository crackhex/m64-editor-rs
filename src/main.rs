#![feature(ascii_char)]
#![feature(int_roundings)]
#![windows_subsystem = "windows"]

use crate::delegate::Delegate;
use druid::widget::prelude::*;
use druid::widget::{Align, Axis, Button, Controller, CrossAxisAlignment, Flex,
                    Label, Tabs, TabsEdge, TabsPolicy, TabsTransition, TextBox, ViewSwitcher};
use druid::{AppDelegate, AppLauncher, Data, Lens, Selector, UnitPoint, Widget, WidgetExt, WindowDesc};
use std::any::Any;

mod api;
mod delegate;

pub const OPEN_FILE: Selector = Selector::new("app.open-file");
pub const SAVE_FILE: Selector = Selector::new("app.save-file");
pub const QUIT_APP: Selector = Selector::new("app.quit-app");

#[derive(Data, Clone, Lens)]
struct TabConfig {
    axis: Axis,
    edge: TabsEdge,
    transition: TabsTransition,
}


#[derive(Data, Clone, Lens)]
struct AppState {
    tab_config: TabConfig,
    first_tab_name: String,
    input_m64: String,
    replacement_m64: String,
    output_m64: String,
    input_start: String,
    input_end: String,
    output_start: String,
    output_end: String,
}


fn build_root_widget() -> impl Widget<AppState> {
    fn group<T: Data, W: Widget<T> + 'static>(text: &str, w: W) -> impl Widget<T> {
        Flex::row()
            .with_child(Label::new(text).fix_width(100.0))
            .with_flex_child(w, 1.0)
    }

    let vs = ViewSwitcher::new(
        |app_s: &AppState, _| app_s.tab_config.clone(),
        |_, _, _| Box::new(build_tab_widget(&TabConfig {
            axis: Axis::Horizontal,
            edge: TabsEdge::Leading,
            transition: TabsTransition::Instant,
        })),
    );
    Flex::column().with_flex_child(vs, 1.0).fix_width(400.0)
}


fn build_tab_widget(tab_config: &TabConfig) -> impl Widget<AppState> {
    let control_dynamic = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("Control dynamic tabs"))
        .with_spacer(20.).with_spacer(20.);

    let first_static_tab = Flex::column()
        .with_flex_child(Flex::row()
                             .with_child(Label::new("Rename tab:")), 1.0)
        .with_child(TextBox::new().lens(AppState::first_tab_name));

    let replacement_tab = {
        let input_m64_row = Flex::row()
            .with_child(
                Flex::column().with_child(
                    Label::new("Input M64:")
                        .with_text_size(14.0)
                        .align_horizontal(UnitPoint::RIGHT)
                        .align_vertical(UnitPoint::CENTER)
                        .fix_width(120.0)
                )
            )
            .with_child(
                Flex::column()
                    .fix_width(10.0))  // Spacer
            .with_flex_child(
                TextBox::new()
                    .fix_width(300.0)
                    .lens(AppState::input_m64), 1.0)
            .with_spacer(10.0)
            .with_child(
                Button::new("...")
                    .fix_height(26.0)
                    .on_click(|ctx, data: &mut AppState, _| {
                        data.output_m64 = data.input_m64.clone();
                    })
            );

        let replacement_m64_row = Flex::row()
            .with_child(
                Flex::column().with_child(
                    Label::new("Replacement M64:")
                        .with_text_size(14.0)
                        .align_horizontal(UnitPoint::RIGHT)
                        .align_vertical(UnitPoint::CENTER)
                        .fix_width(120.0)
                )
            ).with_spacer(10.0)
            .with_flex_child(
                TextBox::new()
                    .fix_width(300.0)
                    .lens(AppState::replacement_m64),
                1.0)
            .with_spacer(10.0)
            .with_child(
                Button::new("...")
                    .fix_height(26.0)
                    .on_click(|ctx, data: &mut AppState, _| {
                        data.output_m64 = data.input_m64.clone();
                    })
            );
        let output_m64_row = Flex::row()
            .with_child(
                Flex::column().with_child(
                    Label::new("Output M64:")
                        .with_text_size(14.0)
                        .align_horizontal(UnitPoint::RIGHT)
                        .align_vertical(UnitPoint::CENTER)
                        .fix_width(120.0)
                ))
            .with_spacer(10.0)
            .with_flex_child(
                TextBox::new()
                    .fix_width(300.0)
                    .lens(AppState::output_m64),
                1.0)
            .with_spacer(10.0)
            .with_child(
                Button::new("...")
                    .fix_height(26.0)
                    .on_click(|ctx, data: &mut AppState, _| {
                        data.output_m64 = data.input_m64.clone();
                    })
            );
        let m64_col = Flex::column()
            .with_child(input_m64_row).with_spacer(4.0)
            .with_child(replacement_m64_row).with_spacer(4.0)
            .with_child(output_m64_row).with_spacer(4.0)
            .align_horizontal(UnitPoint::LEFT)
            .align_vertical(UnitPoint::TOP);

        let erase_row = Flex::row()
            .with_child(Label::new("First frame:"))
            .with_spacer(10.0)
            .with_flex_child(TextBox::<String>::new()
                                 .lens(AppState::input_start), 1.0)
            .with_spacer(10.0)
            .with_child(Label::new("Last frame:"))
            .with_spacer(10.0)
            .with_child(TextBox::new()
                .lens(AppState::input_end)
                .fix_width(50.0));

        let container = Flex::column()
            .with_child(m64_col)
            .with_spacer(20.0)
            .with_child(Flex::row()
                .with_spacer(10.0)
                .with_child(Label::new("Part to erase from base file:"))
                .align_left())
            .with_child(erase_row)
            .padding(15.0);
        container
    };
    let main_tabs = Tabs::new()
        .with_axis(tab_config.axis)
        .with_edge(tab_config.edge)
        .with_transition(tab_config.transition)
        .with_tab("Header", first_static_tab)
        .with_tab("Settings", control_dynamic)
        .with_tab("Replacement", replacement_tab);

    Align::left(main_tabs)
}

pub fn main() {

    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("M64 Editor v2.0")
        .window_size((550.0, 400.0))
        .resizable(false);

    // create the initial app state
    let initial_state = AppState {
        tab_config: TabConfig {
            axis: Axis::Horizontal,
            edge: TabsEdge::Leading,
            transition: TabsTransition::Instant,
        },
        first_tab_name: "First tab".into(),
        input_m64: String::new(),
        replacement_m64: String::new(),
        output_m64: String::new(),
        input_start: String::new(),
        input_end: String::new(),
        output_start: String::new(),
        output_end: String::new(),
    };

    // start the application
    let launcher = AppLauncher::with_window(main_window);
    launcher
        .log_to_console()
        .delegate(Delegate::new())
        .launch(initial_state)
        .expect("Failed to launch application");
}