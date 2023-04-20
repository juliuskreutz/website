use yew::prelude::*;

mod content;
mod header;

use content::*;
use header::*;

struct App {
    dark: bool,
}

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { dark: false }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let theme_toggle = ctx.link().callback(|_| ());
        let dark = self.dark;

        html! {
            <div class={classes!("h-full", if dark { "macchiato" } else { "latte" })}>
                <Header {theme_toggle} {dark}/>
                <Content/>
            </div>
        }
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        self.dark = !self.dark;

        true
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
