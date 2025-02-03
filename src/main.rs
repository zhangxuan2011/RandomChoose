use iced::{
    widget::{button, column, container, row, text},
    window, Element, Font, Length, Size, Task,
}; // GUI
use rand::prelude::*; // Random number generator

#[derive(Default)]
pub struct RandomChoose {
    list: Vec<i32>,
    value: i32,
}

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Reset,
}

impl RandomChoose {
    pub fn new() -> (Self, Task<Message>) {
        let value = 0;
        let list = (1..=52).collect();

        // Return it
        let app = Self { value, list };

        (app, Task::none())
    }

    pub fn title(&self) -> String {
        String::from("RandomChoose")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Start => {
                self.choose();
            }
            Message::Reset => {
                self.value = 0;
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        column![
            container(text(format!("选中了: {}", self.value)).size(50))
                .width(500)
                .height(250)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            container(row![
                container(
                    button(text("抽选").size(25))
                        .on_press(Message::Start)
                        .width(Length::Fill)
                        .height(Length::Fill)
                )
                .padding(30),
                container(
                    button(text("重置").size(25))
                        .on_press(Message::Reset)
                        .width(Length::Fill)
                        .height(Length::Fill)
                )
                .padding(30),
            ])
            .padding(30)
            .center_x(Length::Fill)
            .center_y(Length::Fill),
        ]
        .into()
    }

    fn choose(&mut self) {
        self.list = (1..=52).collect();
        // Now we've generated a random number list, we can shuffle it
        let mut rng = rand::rng();
        // 5000 times （5s）
        // Shuffle it
        self.list.shuffle(&mut rng);

        // Show it
        let index = rng.random_range(0..self.list.len());
        self.value = self.list[index];
        self.list.remove(index);
    }
}

fn main() -> iced::Result {
    // Run
    iced::application(
        RandomChoose::title,
        RandomChoose::update,
        RandomChoose::view,
    )
    .centered()
    .window(window::Settings {
        size: Size::new(850.0, 550.0),
        min_size: Some(Size::new(850.0, 500.0)),
        ..Default::default()
    })
    .default_font(Font::with_name("微软雅黑"))
    .run()
}
