use yew::prelude::*;

mod landing;
mod nav;
mod overview;

use landing::*;
use nav::*;
use overview::*;

struct App {
    dark: bool,
}

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { dark: true }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let theme_toggle = ctx.link().callback(|_| ());
        let dark = self.dark;

        html! {
            <div class={classes!("h-full", "bg-base", "text-text", if dark { "macchiato" } else { "latte" })}>
                <Navbar {theme_toggle} {dark}/>
                <Landing/>
                <Overview/>

                <script src="https://cdnjs.cloudflare.com/ajax/libs/vanilla-tilt/1.8.0/vanilla-tilt.min.js" integrity="sha512-RX/OFugt/bkgwRQg4B22KYE79dQhwaPp2IZaA/YyU3GMo/qY7GrXkiG6Dvvwnds6/DefCfwPTgCXnaC6nAgVYw==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
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
