use yew::prelude::*;

use crate::Section;

#[derive(Properties, PartialEq)]
struct PointProps {
    text: AttrValue,
}

#[function_component]
fn Point(props: &PointProps) -> Html {
    html! {
        <li class="text-[14px] pl-1 tracking-wider">{props.text.clone()}</li>
    }
}

#[derive(Properties, PartialEq)]
struct WorkCardProps {
    title: AttrValue,
    sub_title: AttrValue,
    icon: AttrValue,
    duration: AttrValue,
    children: Children,
}

#[function_component]
fn WorkCard(props: &WorkCardProps) -> Html {
    html! {
        <div class="vertical-timeline-element">
            <span class="vertical-timeline-element-icon bg-surface0">
                <div class="flex justify-center items-center w-full h-full">
                    <img src={format!("static/{}", props.icon)} class="w-[60%] h-[60%] object-contain"/>
                </div>
            </span>
            <div class="vertical-timeline-element-content bg-surface0">
                <div class="vertical-timeline-element-content-arrow border-y-8 border-y-transparent border-r-8 border-r-text"></div>
                <div>
                    <h3 class="text-[24px] font-bold">{props.title.clone()}</h3>
                    <p class="text-subtext0 text-[16px] font-semibold" style="margin: 0px;">{props.sub_title.clone()}</p>
                </div>
                <ul class="mt-5 list-disc ml-5 space-y-2">
                    {props.children.clone()}
                </ul>
                <span class="vertical-timeline-element-date">{props.duration.clone()}</span>
            </div>
        </div>
    }
}

#[function_component]
pub fn Work() -> Html {
    html! {
        <Section id="work" header0="What I have done so far" header1="Work Experience.">
            <div class="mt-20 gap-7 flex">
                <div class="vertical-timeline vertical-timeline--two-columns before:bg-text">
                    <WorkCard
                        title="Android Developer"
                        sub_title="Freelancing"
                        icon="android.png"
                        duration="Januar 2016 - Oktober 2018"
                    >
                        <Point text="Developing and maintaining android applications using Android Studio and other related technologies."/>
                        <Point text="Collaborating with other students to learn and improve working as a team."/>
                        <Point text="Implementing responsive design and ensuring cross device compatibility."/>
                    </WorkCard>
                    <WorkCard
                        title="Ar/Xr/Vr Developer"
                        sub_title="Dexperio"
                        icon="dexperio.png"
                        duration="Oktober 2018 - May 2023"
                    >
                        <Point text="Developing and maintaining applications using primarily Unity and other related technologies."/>
                        <Point text="Leveraging capabilities of specialized platforms or creating solutions cross platform."/>
                        <Point text="Collaborating with cross-functional teams including designers, product managers, and other developers to create high-quality products."/>
                        <Point text="Implementing performant and adapted solutions to suit the customers needs."/>
                    </WorkCard>
                </div>
            </div>
        </Section>
    }
}
