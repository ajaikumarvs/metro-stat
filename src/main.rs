use iced::{
    executor,
    theme::{self, Theme},
    widget::{button, column, container, row, text},
    Application, Color, Command, Element, Length, Settings, Subscription, Alignment,
};
use std::time::{Duration, Instant};

struct MetroApp {
    stations: Vec<Station>,
    current_station_index: usize,
    start_time: Instant,
}

#[derive(Debug, Clone)]
struct Station {
    name: String,
    time_to_next: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
}

struct StationButtonStyle {
    color: Color,
}

impl button::StyleSheet for StationButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(self.color)),
            border_radius: 15.0.into(),
            shadow_offset: iced::Vector::default(),
            text_color: Color::BLACK,
            ..Default::default()
        }
    }
}

impl Application for MetroApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            MetroApp {
                stations: vec![
                    Station { name: String::from("Central Station"), time_to_next: 5 },
                    Station { name: String::from("City Square"), time_to_next: 4 },
                    Station { name: String::from("Riverside"), time_to_next: 6 },
                    Station { name: String::from("University"), time_to_next: 0 },
                ],
                current_station_index: 0,
                start_time: Instant::now(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Metro Line")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => {
                if Instant::now().duration_since(self.start_time) >= Duration::from_secs(5) {
                    if self.current_station_index < self.stations.len() - 1 {
                        self.current_station_index += 1;
                        self.start_time = Instant::now();
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let station_width = 50.0;
        let line_height = 30.0;

        let mut elements: Vec<Element<Message>> = vec![];

        for (index, station) in self.stations.iter().enumerate() {
            let color = if index == self.current_station_index {
                Color::from_rgb(0.0, 1.0, 0.0)
            } else if index < self.current_station_index {
                Color::from_rgb(0.5, 0.5, 0.5)
            } else {
                Color::from_rgb(1.0, 1.0, 1.0)
            };

            let station_circle = button(text(""))
                .style(theme::Button::Custom(Box::new(StationButtonStyle { color })))
                .width(Length::Fixed(station_width))
                .height(Length::Fixed(line_height));

            elements.push(
                column![
                    station_circle,
                    text(&station.name).size(12)
                ]
                .into(),
            );

            if index < self.stations.len() - 1 {
                elements.push(
                    text("-")
                        .size(24)
                        .width(Length::Fixed(20.0))
                        .height(Length::Fixed(line_height))
                        .into(),
                );
            }
        }

        let eta = if self.current_station_index < self.stations.len() - 1 {
            let time_left = 5 - Instant::now().duration_since(self.start_time).as_secs();
            format!("ETA: {} seconds", time_left)
        } else {
            "Arrived at final destination".to_string()
        };

        let content = column![
            text("Metro Line").size(28),
            row(elements).spacing(10),
            text(eta).size(20)
        ]
        .spacing(20)
        .padding(20)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(100)).map(|_| Message::Tick)
    }
}

fn main() -> iced::Result {
    MetroApp::run(Settings::default())
}
