use yew::prelude::*;
use yew_icons::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub theme_toggle: Callback<MouseEvent>,
    pub dark: bool,
}

pub struct Header {}

impl Component for Header {
    type Message = ();

    type Properties = HeaderProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let icon_id = if ctx.props().dark {
            IconId::FontAwesomeSolidSun
        } else {
            IconId::FontAwesomeSolidMoon
        };
        let onclick = ctx.props().theme_toggle.clone();

        html! {
            <div>
                <nav class="sm:px-16 px-6 w-full flex items-center py-5 fixed top-0 z-20 bg-crust">
                    <div class="w-full flex justify-between items-center max-w-7xl mx-auto">
                        <a class="flex items-center gap-2" href="/">
                            <Icon class="object-contain" icon_id={IconId::FontAwesomeSolidComputer}/>
                            <p class="text-xl font-bold text-s">{"Julius Kreutz"}</p>
                        </a>
                        <ul class="list-none hidden sm:flex flex-row gap-10">
                            <li class="text-secondary hover:text-maroon text-lg font-medium cursor-pointer">
                                <a href="#about">{"About"}</a>
                            </li>
                            <li class="text-secondary hover:text-maroon text-lg font-medium cursor-pointer">
                                <a href="#work">{"Work"}</a>
                            </li>
                            <li class="text-secondary hover:text-maroon text-lg font-medium cursor-pointer">
                                <a href="#contact">{"Contact"}</a>
                            </li>
                        </ul>
                        <Icon class="text-lavender hover:text-mauve cursor-pointer" {icon_id} {onclick}/>
                    </div>
                </nav>
                <section class="relative w-full h-screen mx-auto bg-mantle">
                    <div class="sm:px-16 px-6 absolute inset-0 top-32 max-w-7xl mx-auto flex flex-row items-start gap-5">
                        <div class="flex flex-col justify-center items-center mt-5">
                            <div class="w-5 h-5 rounded-full bg-mauve"></div>
                            <div class="w-1 sm:h-80 h-40 bg-gradient-to-b from-mauve"></div>
                        </div>
                        <div>
                            <h1 class="font-black lg:text-7xl sm:text-6xl xs:text-5xl text-4xl lg:leading-[98px] mt-2">
                                {"Hi, I'm "}<span class="text-mauve">{"Julius"}</span>
                            </h1>
                            <p class="font-medium lg:text-[30px] sm:text-[26px] xs:text-[20px] text-[16px] lg:leading-[40px] mt-2 text-white-100">
                                {"I develop a variaty of applications with"}<br class="sm:block hidden"/>{" focus on a performant and safe backend"}
                            </p>
                        </div>
                    </div>
                    <div class="absolute xs:bottom-10 bottom-32 w-full flex justify-center items-center">
                        <a href="#about">
                            <Icon class="animate-bounce w-12 h-14" icon_id={IconId::FontAwesomeSolidCircleArrowDown}/>
                        </a>
                    </div>
                </section>
            </div>
        }
    }
}
