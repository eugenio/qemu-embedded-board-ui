use crate::schematic_serializer::*;
use rand::Rng;
use iced::{Container,Sandbox,Button,Length,Settings,Text,Svg,Element,Scrollable,Column,button,scrollable};
mod schematic_serializer;

struct App{
	column_number: u8,
	led_number: u8,
	text: Text,
	led_figure_name: String,
	cont: Content,
	scroll: scrollable::State,
	on_button : button::State,
	off_button : button::State,
}

struct Content{
	on_button_text: String,
	off_button_text: String,
	led_colors: Vec<String>,
	text: String,
}

impl Content{
	fn new( _on_button_text: String, _off_button_text: String) -> Self { Content {
		text: "Led is off".to_string(),
		on_button_text:	"ON".to_string(),
		off_button_text: "OFF".to_string(),
		led_colors: vec!("led_on_blue.svg".to_string(),"led_on_green.svg".to_string(),"led_on_red.svg".to_string()),
		} 
	}
	pub fn led_on_color_set(led_colours: Vec<String>)-> String {
		
		let mut rng = rand::thread_rng();
		let i = rng.gen_range(0..3);
		led_colours[i].to_string()
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Message{
	OnPressed,
	OffPressed,
}

pub fn main() -> iced::Result {
	
	App::run(
				Settings::default(),
			)
}



const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");

impl Sandbox for App {
	type Message = Message;
	fn new() -> App { App { 
					column_number:  5,
					led_number: 3,
					text: Text::new("Led is off"),
					led_figure_name: String::from("led_off.svg"),
					cont: Content::new("ON".to_string(),"OFF".to_string()),
					scroll: scrollable::State::new(),
					on_button: button::State::new() ,
					off_button: button::State::new(),

				} 
			}

	fn title(&self) -> String {
		format!("{}-{}", NAME, VERSION)
	}

	fn update(&mut self, message: Message) {
		match message {
			Message::OnPressed => {
				self.cont.text = String::from("Led is on");
				self.led_figure_name = String::from(Content::led_on_color_set(self.cont.led_colors.to_vec()));
			}
			Message::OffPressed => {
				self.cont.text = String::from("Led is off");
				self.led_figure_name = String::from("led_off.svg");

			}
		}
	}

	fn view(&mut self) -> Element<Message>{
		let on_button_text = &self.cont.on_button_text;
		let off_button_text = &self.cont.off_button_text;
		let text = &self.cont.text;
		let led_figure_name = &self.led_figure_name;
		let col = Column::new()
				.push(Text::new(text))
				.push(Svg::from_path(format!("{}/resources/{}",env!("CARGO_MANIFEST_DIR"),String::from(led_figure_name))).width(Length::Shrink).height(Length::Shrink))
				.push(Button::new( &mut self.on_button, Text::new( on_button_text)).on_press(Message::OnPressed))
				.push(Button::new( &mut self.off_button, Text::new( off_button_text)).on_press(Message::OffPressed))
				.push(Text::new(get_file_content()));
		Scrollable::new(&mut self.scroll).padding(40).push(Container::new(col).width(Length::Fill).center_x()).into()
		}

}