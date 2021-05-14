use crate::style::{ClassStyle, InlineStyle, StyleBuilder};
use std::{borrow::Borrow, marker::PhantomData, rc::Rc};
use yew::{html, Classes, Component, ComponentLink, Html, Properties, ShouldRender};

const CSS_ANSI_CONTAINER: &str = "font-family:monospace;";

/// Props that can be passed to the [`AnsiRenderer`] component.
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AnsiProps<S: Clone> {
    /// Classes to add to the root element. (Optional)
    #[prop_or_default]
    pub class: Classes,
    /// Content to render. (Required)
    pub text: S,
    /// Whether to disable the inline style applied to the root component. (Optional)
    #[prop_or_default]
    pub no_default_style: bool,
}

/// Component for rendering text containing ANSI escape codes.
///
/// This takes two type arguments, `Text` and `Builder`.
/// `Text` is the type that is passed to [`AnsiProps::text`]. It can be any type that implements [`Borrow<str>`][Borrow].
/// `Builder` specifies the [`StyleBuilder`]. You probably want to use [`InlineStyle`].
///
/// Unless you have special requirements you should use one of the helper types instead of this:
/// - [`Ansi`]
/// - [`AnsiRc`]
/// - [`AnsiStatic`]
///
/// See [`AnsiProps`] for the props that can be passed to this component.
#[derive(Debug)]
pub struct AnsiRenderer<Text, Builder>
where
    Text: Clone,
    Builder: StyleBuilder,
{
    props: AnsiProps<Text>,
    segments: Vec<(ClassStyle, String)>,
    _builder: PhantomData<Builder>,
}
impl<Text, Builder> AnsiRenderer<Text, Builder>
where
    Text: Borrow<str> + Clone,
    Builder: StyleBuilder,
{
    fn update_segments(&mut self) {
        let s = &self.props.text;
        self.segments.clear();

        for (effect, content) in crate::get_sgr_segments(s.borrow()) {
            self.segments
                .push((effect.to_class_style::<Builder>(), content.to_owned()))
        }
    }

    fn render_segment((class_style, content): &(ClassStyle, String)) -> Html {
        // TODO update to use optional attributes when they land
        let class = class_style.class.clone();
        let style = class_style.style.clone().unwrap_or_default();
        html! {
            <span class=class style=style>
                { content }
            </span>
        }
    }
}
impl<Text, Builder> Component for AnsiRenderer<Text, Builder>
where
    Text: Borrow<str> + Clone + PartialEq + 'static,
    Builder: StyleBuilder + 'static,
{
    type Message = ();
    type Properties = AnsiProps<Text>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mut instance = Self {
            props,
            segments: Vec::new(),
            _builder: PhantomData::default(),
        };
        instance.update_segments();
        instance
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let update_segments = self.props.text != props.text;

        let should_render = if self.props == props {
            false
        } else {
            self.props = props;
            true
        };

        if update_segments {
            self.update_segments();
        }

        should_render
    }

    fn view(&self) -> Html {
        let props = &self.props;
        let style = if props.no_default_style {
            ""
        } else {
            CSS_ANSI_CONTAINER
        };
        html! {
            <pre class=props.class.clone() style=style>
                { for self.segments.iter().map(Self::render_segment) }
            </pre>
        }
    }
}

/// ANSI component which takes a [`String`].
///
/// See [`AnsiRenderer`] for more details.
pub type Ansi<Builder = InlineStyle> = AnsiRenderer<String, Builder>;
/// ANSI component which takes a [`Rc<String>`][Rc].
///
/// See [`AnsiRenderer`] for more details.
pub type AnsiRc<Builder = InlineStyle> = AnsiRenderer<Rc<String>, Builder>;
/// ANSI component which takes a [`&'static str`][str].
///
/// See [`AnsiRenderer`] for more details.
pub type AnsiStatic<Builder = InlineStyle> = AnsiRenderer<&'static str, Builder>;
