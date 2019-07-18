#![recursion_limit="128"]

extern crate stdweb;
#[macro_use]
extern crate yew;

use stdweb::web::Date;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

struct Model {
    value: i64,
}

enum Msg {
    DoIt,
    Reset,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => self.value = self.value + 1,
            Msg::Reset => self.value = 0
        }
        true
    }
}

struct Users {
    users: Table
}

struct User {
    first_name: String,
    last_name: String,
}

impl Component for Users {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            users: Table::new()
        }
    }

        fn update(&mut self, msg: Self::Message) -> ShouldRender {
            true
        }
}

impl Renderable<Users> for Users {
    fn view(&self) -> Html<Self>{
        html! {
            <div>
                <input type="text" name="firstname"></input>
                <button >{ "+1" }</button>
            </div>
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <button onclick=|_| Msg::DoIt,>{ "+1" }</button>
                <button onclick=|_| Msg::Reset,>{ "Reset Value" }</button>
                <p>{ self.value }</p>
                <table style={ "border: 1px solid black;" }>
                    <th>
                        <td>{ "First Name" }</td>
                        <td>{ "Last Name" }</td>
                    </th>
                </table>
                <Users />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
