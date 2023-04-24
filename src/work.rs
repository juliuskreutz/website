use yew::prelude::*;

use crate::Section;

#[function_component]
pub fn Work() -> Html {
    html! {
        <Section id="work" header0="What I have done so far" header1="Work Experience.">
            <div class="mt-20 gap-7 flex">
                <div class="absolute h-full w-1 bg-text"></div>
                {"Hello World"}
            </div>
        </Section>
    }
}
