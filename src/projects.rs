use yew::prelude::*;

use crate::Section;

#[derive(Properties, PartialEq)]
struct ProjectCardProps {
    image: AttrValue,
    title: AttrValue,
    description: AttrValue,
    link: AttrValue,
    children: Children,
}

#[function_component]
fn ProjectCard(props: &ProjectCardProps) -> Html {
    html! {

                <a class="bg-surface0 p-5 rounded-2xl sm:w-[360px] w-full" href={props.link.clone()} target="_blank" data-tilt-scale={"1.1"} data-tilt={""}>
                    <div class="relative w-full h-[230px]">
                        <img src={format!("static/{}", props.image)} alt={props.title.clone()} class="w-full h-full object-cover rounded-2xl"/>
                    </div>
                    <div class="mt-5">
                        <h3 class="font-bold text-[24px]">{props.title.clone()}</h3>
                        <p class="mt-2 text-[14px]">{props.description.clone()}</p>
                    </div>
                    <div class="mt-4 flex flex-wrap gap-2">
                        {props.children.clone()}
                    </div>
                </a>
    }
}

#[derive(Properties, PartialEq)]
struct TagProps {
    text: AttrValue,
    color: AttrValue,
}

#[function_component]
fn Tag(props: &TagProps) -> Html {
    html! {
        <p class={classes!("text-[14px]", props.color.to_string())}>{props.text.clone()}</p>
    }
}

#[function_component]
pub fn Projects() -> Html {
    html! {
        <Section id="projects" header0="My work" header1="Projects.">
            <p class="mt-3 text-[17px] max-w-3xl leading-[30px]" style="opacity: 1; transform: none;">
                {"Following public projects showcase my skills and experience through real-world examples of my work. They reflect my ability to solve complex problems, work with different technologies and manage projects effectively."}
            </p>
            <div class="mt-20 flex flex-wrap gap-7">
                <ProjectCard
                    image="website.webp"
                    title="This website"
                    description="The website you are seeing right now."
                    link="https://github.com/juliuskreutz/website"
                >
                    <Tag text="#rust" color="text-peach"/>
                    <Tag text="#tailwind" color="text-pink"/>
                    <Tag text="#yew" color="text-green"/>
                </ProjectCard>
                <ProjectCard
                    image="rwm.webp"
                    title="Rwm"
                    description="A small rust window manager."
                    link="https://github.com/juliuskreutz/rwm"
                >
                    <Tag text="#rust" color="text-peach"/>
                    <Tag text="#linux" color="text-blue"/>
                    <Tag text="#xcb" color="text-teal"/>
                </ProjectCard>
                <ProjectCard
                    image="mandelbrot.webp"
                    title="Mandelbrot"
                    description="A mandelbrot shader that runs on the gpu."
                    link="https://github.com/juliuskreutz/mandelbrot"
                >
                    <Tag text="#rust" color="text-peach"/>
                    <Tag text="#gpu" color="text-green"/>
                    <Tag text="#shader" color="text-red"/>
                </ProjectCard>
                <ProjectCard
                    image="ar.webp"
                    title="ARMarineExperience"
                    description="An ar experience for the marine reasearch exhibition of the german museum."
                    link="https://github.com/museum4punkt0/AR-Erlebniswelt-Meeresforschung-Demo"
                >
                    <Tag text="#unity" color="text-mauve"/>
                    <Tag text="#ar" color="text-green"/>
                    <Tag text="#mobile" color="text-yellow"/>
                </ProjectCard>
                <ProjectCard
                    image="smssender.webp"
                    title="Smssender"
                    description="A utility application, that allows you to send a sms to multiple groups at once controlled from your computer."
                    link="https://github.com/juliuskreutz/smssender"
                >
                    <Tag text="#java" color="text-blue"/>
                    <Tag text="#android" color="text-green"/>
                    <Tag text="#mobile" color="text-yellow"/>
                </ProjectCard>
                <ProjectCard
                    image="snake.webp"
                    title="SnakeOs"
                    description="A bootloader, whose sole purpose is to play snake."
                    link="https://github.com/juliuskreutz/snake_os"
                >
                    <Tag text="#rust" color="text-peach"/>
                    <Tag text="#os" color="text-blue"/>
                    <Tag text="#game" color="text-red"/>
                </ProjectCard>
            </div>
        </Section>
    }
}
