use yew::prelude::*;

#[function_component]
pub fn Projects() -> Html {
    html! {
        <section class="sm:px-16 px-6 sm:py-16 py-10 max-w-7xl mx-auto relative z-0">
            <span class="mt-[-100px] pb-[100px] block" id="projects">{'\u{00a0}'}</span>
            <div>
                <p class="sm:text-[18px] text-[14px] text-subtext1 uppercase tracking-wider">{"My work"}</p>
                <h2 class="font-black md:text-[60px] sm:text-[50px] xs:text-[40px] text-[30px]">{"Projects."}</h2>
            </div>
            {"WIP"}
        </section>
    }
}
