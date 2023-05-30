use yew::prelude::*;

#[function_component]
pub fn Landing() -> Html {
    html! {
        <section class="relative w-full h-screen mx-auto bg-mantle">
            <div class="sm:px-16 px-6 absolute inset-0 top-[120px] max-w-7xl mx-auto flex flex-row items-start gap-5">
                <div class="flex flex-col justify-center items-center mt-5">
                    <div class="w-5 h-5 rounded-full bg-mauve"></div>
                    <div class="w-1 sm:h-80 h-40 bg-gradient-to-b from-mauve"></div>
                </div>
                <div>
                    <h1 class="font-black lg:text-[80px] sm:text-[60px] text-[40px] lg:leading-[98px] mt-2">
                        {"Hi, I'm "}<span class="text-mauve">{"Julius"}</span>
                    </h1>
                    <p class="font-medium lg:text-[30px] sm:text-[26px] text-[16px] lg:leading-[40px] mt-2">
                        {"I develop a variety of applications with"}<br class="sm:block hidden"/>{" focus on a performant and safe backend"}
                    </p>
                </div>
            </div>
            <div class="absolute xs:bottom-10 bottom-32 w-full flex justify-center items-center">
                <a href="#about" aria-label="Scroll down">
                    <svg class="animate-bounce w-12 h-14" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">{"<!--! Font Awesome Pro 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. -->"}<path fill="currentColor" d="M256 0a256 256 0 1 0 0 512A256 256 0 1 0 256 0zM135 241c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0l87 87 87-87c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9L273 345c-9.4 9.4-24.6 9.4-33.9 0L135 241z"/></svg>
                </a>
            </div>
        </section>
    }
}
