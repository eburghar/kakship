use style_builder::MyStyleBuilder;
use yew::{html, Classes, Component, ComponentLink, Html, ShouldRender};
use yew_ansi::AnsiStatic;

mod style_builder;

const OUTPUT: &str = include_str!("../../../assets/cargo-expand.txt");

// create a type alias for our custom ansi renderer.
type MyAnsi = AnsiStatic<MyStyleBuilder>;

pub struct Model;
impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <MyAnsi
                class=Classes::from("ansi-container")
                no_default_style=true
                text=OUTPUT
            />
        }
    }
}

pub fn main() {
    yew::start_app::<Model>();
}
