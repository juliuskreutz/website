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
            <header class="flex bg-crust justify-end">
                <Icon class="text-lavender hover:text-mauve cursor-pointer" {icon_id} {onclick}/>
            </header>
        }
    }
}
