use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub theme_toggle: Callback<MouseEvent>,
    pub dark: bool,
}

#[function_component]
pub fn Navbar(props: &NavbarProps) -> Html {
    let onclick = props.theme_toggle.clone();
    let icon = if props.dark {
        html!(<svg class="w-9 h-9 text-lavender hover:text-mauve cursor-pointer" {onclick} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">{"<!--! Font Awesome Pro 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. -->"}<path fill="currentColor" d="M361.5 1.2c5 2.1 8.6 6.6 9.6 11.9L391 121l107.9 19.8c5.3 1 9.8 4.6 11.9 9.6s1.5 10.7-1.6 15.2L446.9 256l62.3 90.3c3.1 4.5 3.7 10.2 1.6 15.2s-6.6 8.6-11.9 9.6L391 391 371.1 498.9c-1 5.3-4.6 9.8-9.6 11.9s-10.7 1.5-15.2-1.6L256 446.9l-90.3 62.3c-4.5 3.1-10.2 3.7-15.2 1.6s-8.6-6.6-9.6-11.9L121 391 13.1 371.1c-5.3-1-9.8-4.6-11.9-9.6s-1.5-10.7 1.6-15.2L65.1 256 2.8 165.7c-3.1-4.5-3.7-10.2-1.6-15.2s6.6-8.6 11.9-9.6L121 121 140.9 13.1c1-5.3 4.6-9.8 9.6-11.9s10.7-1.5 15.2 1.6L256 65.1 346.3 2.8c4.5-3.1 10.2-3.7 15.2-1.6zM160 256a96 96 0 1 1 192 0 96 96 0 1 1 -192 0zm224 0a128 128 0 1 0 -256 0 128 128 0 1 0 256 0z"/></svg>)
    } else {
        html!(<svg class="w-9 h-9 text-lavender hover:text-mauve cursor-pointer" {onclick} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">{"<!--! Font Awesome Pro 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. -->"}<path fill="currentColor" d="M223.5 32C100 32 0 132.3 0 256S100 480 223.5 480c60.6 0 115.5-24.2 155.8-63.4c5-4.9 6.3-12.5 3.1-18.7s-10.1-9.7-17-8.5c-9.8 1.7-19.8 2.6-30.1 2.6c-96.9 0-175.5-78.8-175.5-176c0-65.8 36-123.1 89.3-153.3c6.1-3.5 9.2-10.5 7.7-17.3s-7.3-11.9-14.3-12.5c-6.3-.5-12.6-.8-19-.8z"/></svg>)
    };

    html! {
        <nav class="sm:px-16 px-6 w-full flex items-center py-5 fixed top-0 z-20 bg-crust">
            <div class="w-full flex justify-between items-center max-w-7xl mx-auto">
                <a class="flex items-center gap-2" href="/">
                    <svg class="w-9 h-9 object-contain" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512">{"<!--! Font Awesome Pro 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. -->"}<path fill="currentColor" d="M384 96V320H64L64 96H384zM64 32C28.7 32 0 60.7 0 96V320c0 35.3 28.7 64 64 64H181.3l-10.7 32H96c-17.7 0-32 14.3-32 32s14.3 32 32 32H352c17.7 0 32-14.3 32-32s-14.3-32-32-32H277.3l-10.7-32H384c35.3 0 64-28.7 64-64V96c0-35.3-28.7-64-64-64H64zm464 0c-26.5 0-48 21.5-48 48V432c0 26.5 21.5 48 48 48h64c26.5 0 48-21.5 48-48V80c0-26.5-21.5-48-48-48H528zm16 64h32c8.8 0 16 7.2 16 16s-7.2 16-16 16H544c-8.8 0-16-7.2-16-16s7.2-16 16-16zm-16 80c0-8.8 7.2-16 16-16h32c8.8 0 16 7.2 16 16s-7.2 16-16 16H544c-8.8 0-16-7.2-16-16zm32 160a32 32 0 1 1 0 64 32 32 0 1 1 0-64z"/></svg>
                    <p class="text-[18px] font-bold">{"Julius Kreutz"}</p>
                </a>
                <ul class="list-none hidden sm:flex flex-row gap-10">
                    <li class="text-secondary hover:text-mauve text-[18px] font-medium cursor-pointer">
                        <a href="#about">{"About"}</a>
                    </li>
                    <li class="text-secondary hover:text-mauve text-[18px] font-medium cursor-pointer">
                        <a href="#projects">{"Projects"}</a>
                    </li>
                    <li class="text-secondary hover:text-mauve text-[18px] font-medium cursor-pointer">
                        <a href="#work">{"Work"}</a>
                    </li>
                    <li class="text-secondary hover:text-mauve text-[18px] font-medium cursor-pointer">
                        <a href="#contact">{"Contact"}</a>
                    </li>
                </ul>
                {icon}
            </div>
        </nav>
    }
}
