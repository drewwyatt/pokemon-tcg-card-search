use yew::prelude::*;

pub struct SearchBar {}

pub struct Msg {}

impl Component for SearchBar {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        SearchBar {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <input placeholder="This doesn't do anything yet" />
        }
    }
}
