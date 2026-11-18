use std::{collections::HashMap, time::Duration};

use iced::{widget::container, window::{Id, Settings, close, drag, icon::from_file_data, maximize, minimize}, *};
use crate::{app::app, components::create_inst::CreateInstance, style::*};
pub mod app;
pub mod components;
pub mod routes;
pub mod style;

struct UKMCL {
    windows: HashMap<Id, Window>,
    current_page: Page,
    shown_text: String,
    typewriter: usize,
    user_name: Option<String>,
    full_text: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    WindowIdReceived(Option<Id>),
    TitleBar(Id, WinMsg),
    Nav(Page),
    Windows(PossibleWindows),
    TypewriterTick,
    UsernameLoaded(Option<String>),
    WindowOpened(Id),
    Focused(Id, bool),
}

#[derive(Debug, Clone)]
pub enum WinMsg {
    Minimize,
    Maximize,
    Close,
    DragWindow,
}

#[derive(Debug, Clone)]
pub enum PossibleWindows {
    OpenCreateWindow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Instances,
    Store,
    Settings,
}

pub struct Window {
    kind: WindowKind,
    is_maxed: bool,
    is_focused: bool,
    info: Option<CreateInstance>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WindowKind {
    Main,
    CreateInstance,
}

impl UKMCL {
    fn new() -> (Self, Task<Message>) {
        let app = Self {
            current_page: Page::Home,
            shown_text: String::new(),
            typewriter: 0,
            user_name: Some(String::new()),
            full_text: "Welcome!".to_string(),
            windows: HashMap::new(),
        };

        let (_, task) = window::open(Settings {
            icon: Some(from_file_data(
                        include_bytes!("../assets/icons/icon.png"), None
                    ).expect("Failed to load window icon")),
            size: Size::new(1366.0, 768.0),
            min_size: Some(Size::new(1366.0, 768.0)),
            resizable: true,
            decorations: false,
            transparent: true,
            position: window::Position::Centered,
            ..Default::default()
        });
        (app, task.map(|id| Message::WindowIdReceived(Some(id))))
    }

    fn subscription(&self) -> Subscription<Message> {
        let focus = event::listen_with(|event, _, id| {
            match event {
                Event::Window(window::Event::Focused) => {
                    Some(Message::Focused(id, true))
                }
                Event::Window(window::Event::Unfocused) => {
                    Some(Message::Focused(id, false))
                }
                _ => None
            }
        });
        
        let typewriter = if self.typewriter < self.full_text.chars().count() {
            time::every(Duration::from_millis(150)).map(|_| Message::TypewriterTick)
        } else {
            Subscription::none()
        };

        Subscription::batch([focus, typewriter])
    }
    
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowIdReceived(Some(id)) => {
                self.windows.insert(id, Window {
                    kind: WindowKind::Main,
                    is_maxed: false,
                    is_focused: false,
                    info: None,
                });
                Task::none()     
            },
            Message::WindowIdReceived(None) => {
                Task::none()
            }
            Message::TitleBar(i, w) => {
                match w {
                    WinMsg::Close => {
                        if let Some(window) = self.windows.get(&i) {
                            match window.kind {
                                WindowKind::Main => close(i).chain(exit()),
                                WindowKind::CreateInstance => close(i),
                            }
                        } else {
                            Task::none()
                        }
                    },
                    WinMsg::DragWindow => drag(i),
                    WinMsg::Minimize => minimize(i, true),
                    WinMsg::Maximize => {
                        if let Some(window) = self.windows.get_mut(&i) {
                            window.is_maxed = !window.is_maxed;
                            maximize(i, window.is_maxed)
                        } else {
                            Task::none()
                        }
                    }
                }
            }
            Message::Nav(p) => {
                self.current_page = p;
                Task::none()
            },
            Message::TypewriterTick => {
                if let Some(c) = self.full_text.chars().nth(self.typewriter) {
                    self.shown_text.push(c);
                    self.typewriter += 1;
                    Task::none()
                } else {
                    Task::none()
                }
            },
            Message::UsernameLoaded(username) => {
                self.user_name = username;
                self.full_text = match &self.user_name {
                    Some(name) => format!("Welcome, {}!", name),
                    None => "Welcome!".to_string(),
                };
                self.shown_text.clear();
                self.typewriter = 0;
                Task::none()
            },
            Message::Windows(m) => {
                match m {
                    PossibleWindows::OpenCreateWindow => {
                        let (_, task) = iced::window::open(
                            Settings {
                                resizable: false,
                                size: Size {
                                    width: 830.0,
                                    height: 450.0,
                                },
                                decorations: false,
                                position: window::Position::Centered,
                                transparent: true,
                                icon: Some(from_file_data(
                                    include_bytes!("../assets/icons/icon.png"), None
                                ).expect("Failed to load window icon")),
                                ..Default::default()
                            }
                        );
                        
                        task.map(Message::WindowOpened)
                    }
                }
            }
            Message::WindowOpened(i) => {
                self.windows.insert(i, Window {
                    kind: WindowKind::CreateInstance,
                    is_maxed: false,
                    is_focused: false,
                    info: None,
                });
                Task::none()
            }
            Message::Focused(i, focused) => {
                if let Some(window) = self.windows.get_mut(&i) {
                    window.is_focused = focused;
                }
                
                Task::none()
            }
        }
    }

    fn view(&self, window: Id) -> Element<'_, Message> {
        let Some(win) = self.windows.get(&window) else {
            return container("unknown window").into();
        };
        let mut inner = container(app(window, win, self.current_page, self.shown_text.clone()))
            .width(Fill)
            .height(Fill);

        if win.is_focused {
            inner = inner.style(|_| container::Style {
                background: Some(Background::Color(UKMCLColors::TitleBar.value())), 
                border: Border {
                    radius: if win.is_maxed { 0.0.into() } else { 12.0.into() },
                    ..Default::default()
                },
                shadow: Shadow {
                    color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.90 },
                    offset: Vector { x: 0.0, y: 0.0 },
                    blur_radius: 10.0,
                },
                ..Default::default()
            });
        } else {
            inner = inner.style(|_| container::Style {
                background: Some(Background::Color(UKMCLColors::TitleBar.value())), 
                border: Border {
                    radius: if win.is_maxed { 0.0.into() } else { 12.0.into() },
                    ..Default::default()
                },
                shadow: Shadow {
                    color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.40 },
                    offset: Vector { x: 0.0, y: 0.0 },
                    blur_radius: 10.0,
                },
                ..Default::default()
            });
        }
        
        let mut outer = container(inner).style(|_| container::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                ..Default::default()
            });

        if !win.is_maxed {
            outer = outer.padding(12);
        } else {
            outer = outer.padding(0);
        }
        
        outer.into()
    }
}

fn main() -> Result {
    daemon(
        UKMCL::new, 
        UKMCL::update, 
        UKMCL::view
    )
    .subscription(UKMCL::subscription)
    .title("UKMCL")
    .style(|_, theme| theme::Style { 
        background_color: Color::TRANSPARENT, 
        text_color: theme.palette().text
    }).run()
}
