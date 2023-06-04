use yew::prelude::*;

mod landing;
mod links;
mod nav;
mod overview;
mod projects;
mod work;

use landing::*;
use links::*;
use nav::*;
use overview::*;
use projects::*;
use work::*;

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
                <Projects/>
                <Work/>
                <Links/>

                <script src="https://cdnjs.cloudflare.com/ajax/libs/vanilla-tilt/1.8.0/vanilla-tilt.min.js" integrity="sha512-RX/OFugt/bkgwRQg4B22KYE79dQhwaPp2IZaA/YyU3GMo/qY7GrXkiG6Dvvwnds6/DefCfwPTgCXnaC6nAgVYw==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
            </div>
        }
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        self.dark = !self.dark;

        true
    }
}

#[derive(Properties, PartialEq)]
struct SectionProps {
    id: AttrValue,
    header0: AttrValue,
    header1: AttrValue,
    children: Children,
}

#[function_component]
fn Section(props: &SectionProps) -> Html {
    html! {
        <section class="sm:px-16 px-6 sm:py-16 py-10 max-w-7xl mx-auto relative z-0">
            <span class="mt-[-100px] pb-[100px] block" id={props.id.clone()}>{'\u{00a0}'}</span>
            <div>
                <p class="sm:text-[18px] text-[14px] text-subtext1 uppercase tracking-wider">{props.header0.clone()}</p>
                <h2 class="font-black md:text-[60px] sm:text-[50px] xs:text-[40px] text-[30px]">{props.header1.clone()}</h2>
            </div>
            {props.children.clone()}
        </section>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
