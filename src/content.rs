use yew::prelude::*;

pub struct Content {}

impl Component for Content {
    type Message = ();

    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <main class="h-96 bg-mantle">
            </main>
        }
    }
}
