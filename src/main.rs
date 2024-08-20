#![feature(ascii_char)]
#![feature(int_roundings)]

mod api;
mod gui;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Container, Text};
use iced::{Element, Font, Length, keyboard, keyboard::key, Task, widget, Subscription};
use iced::event::{self, Event};
use iced_aw::{TabLabel, Tabs};
use gui::settings::{style_from_index, SettingsMessage, SettingsTab, TabBarPosition};
use gui::counter::{CounterMessage, CounterTab};
use crate::gui::replacement_tab::{ReplacementMessage, ReplacementTab};
const HEADER_SIZE: u16 = 32;
const TAB_PADDING: u16 = 16;
const ICON_BYTES: &[u8] = include_bytes!("../fonts/icons.ttf");
const ICON: Font = Font::with_name("icons");

enum Icon {
    User,
    Heart,
    Calc,
    CogAlt,
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::User => '\u{E800}',
            Icon::Heart => '\u{E801}',
            Icon::Calc => '\u{F1EC}',
            Icon::CogAlt => '\u{E802}',
        }
    }
}

fn main() -> iced::Result {
    iced::application("Tabs example", M64Editor::update, M64Editor::view)
        .font(iced_aw::BOOTSTRAP_FONT_BYTES)
        .font(ICON_BYTES)
        .subscription(M64Editor::subscription)
        .run()
}

#[derive(Default)]
struct M64Editor {
    active_tab: TabId,
    counter_tab: CounterTab,
    settings_tab: SettingsTab,
    replacement_tab: ReplacementTab,
}
#[derive(Clone, PartialEq, Eq, Debug, Default)]
enum TabId {
    #[default]
    Login,
    Ferris,
    Counter,
    Settings,
    Replacement,
}

#[derive(Clone, Debug)]
enum Message {
    TabSelected(TabId),
    Counter(CounterMessage),
    Settings(SettingsMessage),
    Replacement(ReplacementMessage),
    TabClosed(TabId),
    Event(Event),
}

impl M64Editor {
    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                                    key: keyboard::Key::Named(key::Named::Tab),
                                    modifiers,
                                    ..
                                }) => {
                    if modifiers.shift() {
                        widget::focus_next()
                    } else {
                        widget::focus_next()
                    }
                }
                _ => {Task::none()}
            },

            Message::TabSelected(selected) => {self.active_tab = selected; Task::none()},
            Message::Counter(message) => {self.counter_tab.update(message); Task::none()},
            Message::Settings(message) => {self.settings_tab.update(message); Task::none()},
            Message::TabClosed(id) => {println!("Tab {:?} event hit", id); Task::none()},
            Message::Replacement(message) => {self.replacement_tab.update(message); Task::none()},
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let position = self
            .settings_tab
            .settings()
            .tab_bar_position
            .unwrap_or_default();
        let theme = self
            .settings_tab
            .settings()
            .tab_bar_theme
            .unwrap_or_default();

        Tabs::new(Message::TabSelected)
            .tab_icon_position(iced_aw::tabs::Position::Bottom)
            .push(
                TabId::Counter,
                self.counter_tab.tab_label(),
                self.counter_tab.view(),
            )
            .push(
                TabId::Settings,
                self.settings_tab.tab_label(),
                self.settings_tab.view(),
            )
            .push(
                TabId::Replacement,
                self.replacement_tab.tab_label(),
                self.replacement_tab.view(),
            )
            .set_active_tab(&self.active_tab)
            .tab_bar_style(style_from_index(theme))
            .tab_bar_height(Length::Fixed(30.0f32))
            .tab_bar_width(Length::Fixed(400.0f32))
            .icon_font(ICON)
            .tab_bar_position(match position {
                TabBarPosition::Top => iced_aw::TabBarPosition::Top,
                TabBarPosition::Bottom => iced_aw::TabBarPosition::Bottom,
            })

            .into()
    }
}

trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(3)
            .push(Text::new(""))
            .push(self.content())
            .align_x(iced::Alignment::Center);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Left)
            .align_y(Vertical::Top)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}