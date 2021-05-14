use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_ansi::AnsiStatic;

const OUTPUT: &str = include_str!("../../../assets/cargo-expand.txt");

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
            <AnsiStatic text=OUTPUT />
        }
    }
}

pub fn main() {
    yew::start_app::<Model>();
}
