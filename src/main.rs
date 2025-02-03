use iced::{
    widget::{button, column, container, row, text},
    window, Element, Font, Length, Size, Task,
}; // GUI
use msgbox::IconType; // Message box shower
use rand::prelude::*; // Random number generator
use std::{
    sync::{
        atomic::{AtomicBool, AtomicI32, Ordering},
        Arc,
    },
    thread,
    time::Duration,
}; // Some std library

#[derive(Default)]
pub struct RandomChoose {
    button_text: String,
    value: Arc<AtomicI32>,
    choose: Arc<AtomicBool>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Pause,
    Reset,
    ValueUpdated(i32),
}

impl RandomChoose {
    pub fn new() -> (Self, Task<Message>) {
        let value = Arc::new(AtomicI32::new(0));
        let choose = Arc::new(AtomicBool::new(false));
        let button_text = String::from("开始");
        
        // Return it
        let app = Self {
            button_text,
            value,
            choose,
        };

        (app, Task::none())
    }

    pub fn title(&self) -> String {
        String::from("RandomChoose")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        println!("Update func's value: {}", self.value.load(Ordering::SeqCst));
        match message {
            Message::Start => {
                self.choose.store(true, Ordering::SeqCst);
                self.button_text = String::from("暂停");

                // Thread spawn
                self.choose();

                // End of thread spawn
                // thread::sleep(Duration::from_millis(50));
                Task::none()
            }
            Message::Pause => {
                self.choose.store(false, Ordering::SeqCst);
                self.button_text = String::from("开始");
                Task::none()
            }
            Message::Reset => {
                if self.choose.load(Ordering::SeqCst) {
                    msgbox::create("错误", "还在抽选, 请先暂停再重置!!!", IconType::Error).unwrap();
                    return Task::none();
                }
                self.value.store(0, Ordering::SeqCst);
                msgbox::create("提示", "成功重置!!!", IconType::Info).unwrap();
                Task::none()
            }
            Message::ValueUpdated(new_value) => {
                self.value.store(new_value, Ordering::SeqCst);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        println!("View func's value: {}", self.value.load(Ordering::SeqCst));
        column![
            container(text(format!("选中了: {}", self.value.load(Ordering::SeqCst))).size(50))
                .width(500)
                .height(250)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            container(row![
                container(
                    button(text(&self.button_text).center())
                        .on_press(if self.choose.load(Ordering::SeqCst) {
                            Message::Pause
                        } else {
                            Message::Start
                        })
                        .width(Length::Fill)
                        .height(Length::Fill)
                )
                .padding(30),
                container(
                    button("重置")
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
        let value = self.value.clone();
        let guard = self.choose.clone();
        thread::Builder::new()
            .name(String::from("random"))
            .spawn(move || {
                let mut rng = rand::rng();
                loop {
                    if !guard.load(Ordering::SeqCst) {
                        break;
                    }
                    let randnum = rng.random_range(1..=52);
                    value.store(randnum, Ordering::SeqCst);
                    thread::sleep(Duration::from_millis(150));
                }
            })
            .unwrap();
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
        size: Size::new(630.0, 400.0),
        min_size: Some(Size::new(630.0, 400.0)),
        ..Default::default()
    })
    .default_font(Font::with_name("微软雅黑"))
    .run()
}
