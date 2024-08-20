use crate::{Icon, Message, Tab};
use iced::{widget::{Column, Container, Text}, Alignment, Element};
use iced::widget::{Button, Row, TextInput};
use iced_aw::tab_bar::TabLabel;

#[derive(Clone, Debug)]
pub enum ReplacementMessage {
    BaseM64Changed(String),
    ReplacementM64Changed(String),
    OutputM64Changed(String),
    BaseStartChanged(String),
    BaseEndChanged(String),
    ReplacementStartChanged(String),
    ReplacementEndChanged(String),
    BaseFileSearch, // There is probably a better way to do this
    ReplacementFileSearch, //
    OutputFileSearch,
    ReplaceFrames,
}

#[derive(Default)]
pub struct ReplacementTab {
    base_m64: String,
    replacement_m64: String,
    output_m64: String,
    base_start: String,
    base_end: String,
    replacement_start: String,
    replacement_end: String,
}

impl ReplacementTab {
    pub fn new() -> Self {
        ReplacementTab {
            base_m64: String::new(),
            replacement_m64: String::new(),
            output_m64: String::new(),
            base_start: String::new(),
            base_end: String::new(),
            replacement_start: String::new(),
            replacement_end: String::new(),
        }
    }
    pub fn update(&mut self, message: ReplacementMessage) {
        match message {
            ReplacementMessage::BaseFileSearch => {
                self.base_m64 = rfd::FileDialog::new().pick_file()
                    .unwrap_or_else(|| "".parse().unwrap())
                    .to_str().unwrap_or_else(|| "")
                    .to_string();
            }
            ReplacementMessage::ReplacementFileSearch => {
                self.replacement_m64 = rfd::FileDialog::new().pick_file()
                    .unwrap_or_else(|| "".parse().unwrap())
                    .to_str().unwrap_or_else(|| "")
                    .to_string();
            }
            ReplacementMessage::OutputFileSearch => {
                self.output_m64 = rfd::FileDialog::new().pick_file()
                    .unwrap_or_else(|| "".parse().unwrap())
                    .to_str().unwrap_or_else(|| "")
                    .to_string();
            }
            ReplacementMessage::ReplaceFrames => {
                // Replace frames
            }
            ReplacementMessage::BaseM64Changed(value) => {
                self.base_m64 = value
            }
            ReplacementMessage::ReplacementM64Changed(value) => {
                self.replacement_m64 = value
            }
            ReplacementMessage::OutputM64Changed(value) => {
                self.output_m64 = value
            }
            ReplacementMessage::BaseStartChanged(value) => {
                self.base_start = value
            }
            ReplacementMessage::BaseEndChanged(value) => {
                self.base_end = value
            }
            ReplacementMessage::ReplacementStartChanged(value) => {
                self.replacement_start = value
            }
            ReplacementMessage::ReplacementEndChanged(value) => {
                self.replacement_end = value
            }
        }
    }
}

impl Tab for ReplacementTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Replacement")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::CogAlt.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, ReplacementMessage> = Container::new(

            Column::new()
                .spacing(4)
                .push(
                    Row::new()
                        .align_y(Alignment::Start)
                        .spacing(10)
                        .push(Text::new("Input M64:")).align_y(Alignment::Center)
                        .push(TextInput::new("Input M64", &self.base_m64)
                            .on_input(ReplacementMessage::BaseM64Changed).padding(2).width(275))
                        .push(Button::new(Text::new("Find")
                            .align_x(Alignment::Center).align_y(Alignment::Center))
                            .padding(2).width(50).on_press(ReplacementMessage::BaseFileSearch))
                ).align_x(Alignment::End)
                .push(
                    Row::new()
                        .spacing(10)
                        .push(Text::new("Replacement M64:")).align_y(Alignment::Center)
                        .push(TextInput::new("Replacement M64", &self.replacement_m64)
                            .on_input(ReplacementMessage::ReplacementM64Changed).padding(2).width(275))
                        .push(Button::new(Text::new("Find")
                            .align_x(Alignment::Center).align_y(Alignment::Center))
                            .padding(2).width(50).on_press(ReplacementMessage::ReplacementFileSearch))
                )
                .push(
                    Row::new()
                        .spacing(10)
                        .push(Text::new("Output M64:")).align_y(Alignment::Center)
                        .push(TextInput::new("Output M64", &self.output_m64)
                            .on_input(ReplacementMessage::OutputM64Changed).padding(2).width(275))
                        .push(Button::new(Text::new("Find")
                            .align_x(Alignment::Center).align_y(Alignment::Center))
                            .padding(2).width(50).on_press(ReplacementMessage::OutputFileSearch))
                )
                .push(Text::new("").size(20))
                .push(Text::new("Part to erase from base file:").width(427))
                .push(
                    Row::new()
                        .spacing(10)
                        .push(Text::new("Start frame:")).align_y(Alignment::Center)
                        .push(TextInput::new("", &self.base_start.to_string())
                            .on_input(|value| {
                                ReplacementMessage::BaseStartChanged(match value.parse::<u32>() {
                                    Ok(value) => value.to_string(),
                                    Err(_) =>
                                        if value != "" { self.base_start.to_string() }
                                        else { "".to_string() }
                                })
                            }).padding(2).width(50))
                        .push(Text::new("").width(70))  // This is a hack to make the text input align with the button
                        .push(Text::new("End frame:")).align_y(Alignment::Center)
                        .push(TextInput::new("", &self.base_end.to_string())
                            .on_input(|value| {
                                ReplacementMessage::BaseEndChanged(match value.parse::<u32>() {
                                    Ok(value) => value.to_string(),
                                    Err(_) =>
                                        if value != "" { self.base_end.to_string() }
                                        else { "".to_string() }
                                })
                            }).padding(2).width(50))
                        .push(Text::new("").width(50)) // This is a hack to make the text input align with the button
                )
                .push(Text::new("").size(20))
                .push(Text::new("Part to replace over base file:").width(427))
                .push(
                    Row::new()
                        .spacing(10)
                        .push(Text::new("Start frame:")).align_y(Alignment::Center)
                        .push(TextInput::new("", &self.replacement_start.to_string())
                            .on_input(|value| {
                                ReplacementMessage::ReplacementStartChanged(match value.parse::<u32>() {
                                    Ok(value) => value.to_string(),
                                    Err(_) =>
                                        if value != "" { self.replacement_start.to_string() }
                                        else { "".to_string() }
                                })
                            }).padding(2).width(50))
                        .push(Text::new("").width(70))  // This is a hack to make the text input align with the button
                        .push(Text::new("End frame:")).align_y(Alignment::Center)
                        .push(TextInput::new("", &self.replacement_end.to_string())
                            .on_input(|value| {
                                ReplacementMessage::ReplacementEndChanged(match value.parse::<u32>() {
                                    Ok(value) => value.to_string(),
                                    Err(_) =>
                                        if value != "" { self.replacement_end.to_string() }
                                        else { "".to_string() }
                                })
                            }).padding(2).width(50))
                        .push(Text::new("").width(50)) // This is a hack to make the text input align with the button
                ).push(
                    Text::new("").size(20)
                ).push(
                    Button::new(Text::new("Replace frames"))
                        .on_press(ReplacementMessage::ReplaceFrames)
                )

        ).align_x(iced::alignment::Horizontal::Right)
        .into();
        content.map(Message::Replacement)
    }
}