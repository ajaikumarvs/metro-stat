use iced::{
    executor,
    theme::{self, Theme},
    widget::{button, column, container, row, text},
    Application, Color, Command, Element, Length, Settings, Subscription, Alignment,
};
use rand::Rng;
use std::time::{Duration, Instant};

struct MetroApp {
    stations: Vec<Station>,
    current_station_index: usize,
    start_time: Instant,
    last_blink_time: Instant,
    is_blinking: bool,
    is_rerouting: bool,
    terminal_waiting: bool,
    rerouting_message: bool,
    countdown_to_reroute: bool,
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
        let mut rng = rand::thread_rng();

        let stations = vec![
            Station { name: String::from("Central Station"), time_to_next: rng.gen_range(5..=8) },
            Station { name: String::from("City Square"), time_to_next: rng.gen_range(5..=8) },
            Station { name: String::from("Riverside"), time_to_next: rng.gen_range(5..=8) },
            Station { name: String::from("University"), time_to_next: rng.gen_range(5..=8) },
            Station { name: String::from("Tech Park"), time_to_next: rng.gen_range(5..=8) },
            Station { name: String::from("Old Town"), time_to_next: rng.gen_range(5..=8) },
            Station { name: String::from("Market Street"), time_to_next: rng.gen_range(5..=8) },
            Station { name: String::from("Harbor"), time_to_next: rng.gen_range(5..=8) },
            Station { name: String::from("Sunset Boulevard"), time_to_next: rng.gen_range(5..=8) },
            Station { name: String::from("Terminal"), time_to_next: 0 }, // Final destination
        ];

        (
            MetroApp {
                stations,
                current_station_index: 0,
                start_time: Instant::now(),
                last_blink_time: Instant::now(),
                is_blinking: false,
                is_rerouting: false,
                terminal_waiting: false,
                rerouting_message: false,
                countdown_to_reroute: false,
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
                let elapsed_time = Instant::now().duration_since(self.start_time);

                if self.terminal_waiting {
                    if elapsed_time >= Duration::from_secs(5) {
                        self.terminal_waiting = false;
                        self.rerouting_message = true;
                        self.start_time = Instant::now();
                    }
                } else if self.rerouting_message {
                    if elapsed_time >= Duration::from_secs(4) {
                        self.rerouting_message = false;
                        self.countdown_to_reroute = true;
                        self.start_time = Instant::now();
                    }
                } else if self.countdown_to_reroute {
                    if elapsed_time >= Duration::from_secs(4) {
                        self.countdown_to_reroute = false;
                        self.is_rerouting = true;
                        self.start_time = Instant::now();
                    }
                } else {
                    let current_station = &self.stations[self.current_station_index];
                    if elapsed_time >= Duration::from_secs(current_station.time_to_next as u64) {
                        if self.is_rerouting {
                            if self.current_station_index > 0 {
                                self.current_station_index -= 1;
                                self.start_time = Instant::now();
                            }
                            if self.current_station_index == 0 {
                                self.is_rerouting = false;
                            }
                        } else if self.current_station_index < self.stations.len() - 1 {
                            self.current_station_index += 1;
                            self.start_time = Instant::now();
                            if self.current_station_index == self.stations.len() - 1 {
                                self.terminal_waiting = true;
                            }
                        }
                    }

                    // Determine if blinking should occur and update blinking state
                    if !self.is_rerouting && self.current_station_index < self.stations.len() - 1 {
                        let time_left = self.stations[self.current_station_index].time_to_next as i64 - elapsed_time.as_secs() as i64;
                        if time_left <= 2 {
                            if Instant::now().duration_since(self.last_blink_time) >= Duration::from_millis(250) {
                                self.is_blinking = !self.is_blinking;
                                self.last_blink_time = Instant::now();
                            }
                        } else {
                            self.is_blinking = false;
                        }
                    } else {
                        self.is_blinking = false;
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
                if !self.is_rerouting && self.current_station_index < self.stations.len() - 1 {
                    if self.is_blinking {
                        if Instant::now().duration_since(self.last_blink_time) < Duration::from_millis(125) {
                            Color::from_rgb(1.0, 0.0, 0.0) // Red
                        } else {
                            Color::from_rgb(0.0, 1.0, 0.0) // Green
                        }
                    } else {
                        Color::from_rgb(0.0, 1.0, 0.0) // Normal color (green)
                    }
                } else if self.is_rerouting {
                    Color::from_rgb(0.0, 0.0, 1.0) // Rerouting color (blue)
                } else {
                    Color::from_rgb(0.0, 1.0, 0.0) // Normal color (green)
                }
            } else if index < self.current_station_index {
                Color::from_rgb(0.5, 0.5, 0.5) // Past station color
            } else {
                Color::from_rgb(1.0, 1.0, 1.0) // Future station color
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

        let eta_text = if self.terminal_waiting {
            "Arrived at Terminal. Rerouting in 5 seconds...".to_string()
        } else if self.rerouting_message {
            "Next destination: Sunset Boulevard in 4 seconds...".to_string()
        } else if self.countdown_to_reroute {
            let time_left = 4 - Instant::now().duration_since(self.start_time).as_secs() as i64;
            format!("Countdown: {} seconds...", time_left.max(0))
        } else if self.is_rerouting && self.current_station_index < self.stations.len() - 1 {
            let time_left = self.stations[self.current_station_index].time_to_next as i64 - Instant::now().duration_since(self.start_time).as_secs() as i64;
            let eta_message = format!("ETA to next station: {} seconds", time_left.max(0));

            let next_station_name = if self.current_station_index > 0 {
                self.stations[self.current_station_index - 1].name.clone()
            } else {
                "N/A".to_string()
            };
            format!("{} - Heading to {}", eta_message, next_station_name)
        } else if self.current_station_index < self.stations.len() - 1 {
            let time_left = self.stations[self.current_station_index].time_to_next as i64 - Instant::now().duration_since(self.start_time).as_secs() as i64;
            let eta_message = format!("ETA to next station: {} seconds", time_left.max(0));

            if time_left <= 2 && !self.is_rerouting {
                let next_station_name = self.stations[self.current_station_index + 1].name.clone();
                format!("{} - Arriving soon at {}", eta_message, next_station_name)
            } else {
                eta_message
            }
        } else {
            "Arrived at final destination".to_string()
        };

        let content = column![
            text("Metro Line").size(28),
            row(elements).spacing(10),
            text(eta_text).size(20)
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
