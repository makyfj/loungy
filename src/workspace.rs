use std::{path::PathBuf, sync::Arc};

use gpui::*;

use crate::{
    commands::root::list::Root,
    query::{TextInput, TextModel},
    theme::Theme,
};

pub struct Workspace {
    query: TextInput,
    state: Model<StateModel>,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        let view = cx.new_view(|cx| {
            let query = TextInput::new(cx, String::from(""));
            cx.set_global::<Query>(Query {
                inner: query.model.clone(),
            });
            let root: AnyView = Root::build(cx).into();
            let state = cx.new_model(|_cx| StateModel { root });
            cx.set_global::<State>(State {
                inner: state.clone(),
            });
            Workspace { query, state }
        });
        view
    }
}

pub struct Query {
    pub inner: Model<TextModel>,
}

impl Global for Query {}

pub struct StateModel {
    pub root: AnyView,
}

pub struct State {
    pub inner: Model<StateModel>,
}

impl Global for State {}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .full()
            .flex()
            .flex_col()
            .bg(theme.base)
            //.rounded_xl()
            //.border_2()
            //.border_color(theme.crust)
            .text_color(theme.text)
            .child(
                div()
                    .child(self.query.clone())
                    .text_lg()
                    .px_4()
                    .py_3()
                    .w_full()
                    .border_b_1()
                    .border_color(theme.mantle),
            )
            .child(div().child(self.state.read(cx).root.clone()).p_2())
            .child(
                div()
                    .absolute()
                    .bottom_0()
                    .left_0()
                    .right_0()
                    .bg(theme.mantle)
                    .w_full()
                    .border_t_1()
                    .border_color(theme.crust)
                    .px_4()
                    .py_2()
                    .text_color(theme.subtext0)
                    .text_xs()
                    .flex()
                    .child(
                        div()
                            .mr_2()
                            .on_mouse_down(MouseButton::Left, |ev, cx| {
                                Theme::change(catppuccin::Flavour::Latte, cx);
                            })
                            .child("Latte"),
                    )
                    .child(
                        div()
                            .mr_2()
                            .on_mouse_down(MouseButton::Left, |ev, cx| {
                                Theme::change(catppuccin::Flavour::Mocha, cx);
                            })
                            .child("Mocha"),
                    )
                    .child(
                        div()
                            .mr_2()
                            .on_mouse_down(MouseButton::Left, |ev, cx| {
                                Theme::change(catppuccin::Flavour::Frappe, cx);
                            })
                            .child("Frappe"),
                    )
                    .child(
                        div()
                            .mr_2()
                            .on_mouse_down(MouseButton::Left, |ev, cx| {
                                Theme::change(catppuccin::Flavour::Macchiato, cx);
                            })
                            .child("Macchiato"),
                    ),
            )
    }
}
