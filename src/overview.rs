use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct LanguageCardProps {
    language: AttrValue,
    children: Children,
}

#[function_component]
fn LanguageCard(props: &LanguageCardProps) -> Html {
    html! {
        <div class="sm:w-[250px] w-full group" data-tilt-scale={"1.1"} data-tilt={""}>
            <div class="w-full bg-gradient-to-tr from-mauve to-green p-[2px] rounded-[20px]">
                <div class="bg-surface0 rounded-[20px] py-5 px-12 min-h-[280px] flex justify-evenly items-center flex-col">
                    <svg class="w-16 h-16" viewBox="0 0 128 128">{props.children.clone()}</svg>
                    <h3 class="text-[20px] font-bold text-center">{props.language.clone()}</h3>
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn Overview() -> Html {
    html! {
        <section class="sm:px-16 px-6 sm:py-16 py-10 max-w-7xl mx-auto relative z-0">
            <span class="mt-[-100px] pb-[100px] block" id="about">{'\u{00a0}'}</span>
            <div>
                <p class="sm:text-[18px] text-[14px] text-subtext1 uppercase tracking-wider">{"Introduction"}</p>
                <h2 class="font-black md:text-[60px] sm:text-[50px] xs:text-[40px] text-[30px]">{"Overview."}</h2>
            </div>
            <p class="mt-4 text-secondary text-[17px] max-w-3xl leading-[30px]">
                {"I'm a skilled software developer with experience in multiple technologies and expertise in their frameworks. I'm a quick learner and collaborate closely with clients to create efficient, scalable, and safe solutions that solve real-world problems. Let's work together to bring your ideas to life!"}
            </p>
            <div class="mt-20 flex flex-wrap gap-10">
                <LanguageCard language="Rust">
                    <path class="fill-text group-hover:fill-peach" d="M62.271 10.88c-.189.11-.982 1.248-1.763 2.529-1.96 3.217-1.982 3.219-4.615.448-1.713-1.802-2.127-2.132-2.679-2.128-.359.002-.812.124-1.008.271-.195.147-.748 1.317-1.228 2.6-1.099 2.939-1.152 3.034-1.761 3.151-.375.071-1.097-.331-2.828-1.574-1.278-.919-2.532-1.67-2.786-1.67-1.054 0-1.351.576-1.853 3.593-.638 3.836-.616 3.823-4.074 2.252-1.396-.633-2.72-1.152-2.943-1.152-.223 0-.646.24-.939.533-.532.533-.533.535-.388 3.468l.146 2.936-.555.297c-.492.263-.831.231-3.009-.284-2.843-.671-3.443-.653-4.019.122l-.421.566.565 2.421c.31 1.331.609 2.613.665 2.848.055.234-.04.609-.212.832-.284.367-.586.4-3.217.36-4.453-.07-4.706.312-2.866 4.328.585 1.275 1.064 2.433 1.064 2.572 0 .734-.585 1.001-3.098 1.411-1.406.229-2.628.417-2.716.417-.088 0-.352.192-.586.426-.765.765-.548 1.483 1.187 3.932 2.161 3.05 2.157 3.061-1.413 4.427-4.06 1.553-4.142 1.936-1.051 4.868 2.879 2.73 2.882 2.69-.377 4.739-2.469 1.551-2.507 1.588-2.57 2.429-.076 1.023-.058 1.041 2.89 2.842 2.915 1.78 2.915 1.834.054 4.541-3.077 2.91-2.982 3.335 1.081 4.868 3.55 1.339 3.555 1.355 1.39 4.405-1.227 1.729-1.618 2.449-1.618 2.983 0 .999.52 1.254 3.627 1.776 2.617.441 3.2.7 3.2 1.422 0 .148-.48 1.316-1.067 2.594-1.826 3.977-1.618 4.308 2.704 4.308 4.025 0 3.918-.123 3.051 3.507-.654 2.736-.664 3.26-.072 3.851.453.454 1.307.403 3.978-.236 2.04-.487 2.398-.521 2.871-.268l.54.289-.146 2.935c-.145 2.934-.144 2.936.388 3.469.293.293.722.533.952.533.23 0 1.554-.516 2.943-1.147 3.447-1.565 3.425-1.578 4.061 2.246.504 3.031.798 3.594 1.874 3.594.267 0 1.494-.72 2.728-1.6 2.167-1.546 2.729-1.788 3.306-1.421.149.094.727 1.364 1.284 2.822.819 2.144 1.119 2.702 1.575 2.92.868.416 1.405.082 3.445-2.14 2.463-2.683 2.564-2.67 4.575.589 2.221 3.598 2.796 3.59 5.073-.073 1.962-3.156 1.939-3.154 4.591-.384 1.761 1.838 2.136 2.131 2.73 2.131.379 0 .832-.142 1.005-.316.174-.174.75-1.459 1.28-2.855.53-1.397 1.079-2.613 1.221-2.703.561-.357 1.142-.106 3.306 1.43 1.274.905 2.473 1.6 2.758 1.6 1.058 0 1.44-.751 1.88-3.703.376-2.517.452-2.758.947-3.009.487-.247.779-.164 3.063.873 1.389.63 2.713 1.146 2.943 1.146.23 0 .666-.247.967-.549l.549-.548-.151-2.815c-.144-2.688-.131-2.832.298-3.22.441-.399.486-.397 2.952.166 2.986.682 3.543.7 4.104.139.548-.548.542-.668-.208-3.831-.841-3.548-.954-3.422 3.088-3.422 2.755 0 3.062-.039 3.413-.426.586-.648.447-1.39-.732-3.903-.595-1.266-1.078-2.418-1.074-2.56.02-.747.607-1.002 3.32-1.443 1.66-.269 2.902-.581 3.127-.784.754-.681.477-1.567-1.244-3.98-2.157-3.024-2.148-3.053 1.306-4.326 4.136-1.524 4.254-2.032 1.159-4.973-2.867-2.724-2.868-2.709.272-4.637 3.796-2.33 3.802-2.855.067-5.173-3.212-1.993-3.21-1.965-.331-4.699 3.088-2.934 3.004-3.318-1.057-4.871-3.584-1.371-3.595-1.405-1.417-4.394 1.297-1.78 1.618-2.371 1.618-2.981 0-1.066-.478-1.305-3.622-1.813-2.627-.424-3.205-.682-3.205-1.429 0-.142.48-1.285 1.067-2.542 1.149-2.461 1.31-3.446.66-4.035-.349-.316-.817-.361-3.321-.32-2.62.044-2.955.007-3.318-.358-.397-.399-.393-.455.227-3.042.76-3.17.763-3.247.138-3.834-.634-.596-1.03-.586-3.941.099-2.121.5-2.472.533-2.954.275l-.547-.293.151-2.926.152-2.925-.547-.547c-.301-.301-.728-.547-.95-.547-.221 0-1.538.523-2.926 1.161-2.318 1.067-2.567 1.138-3.068.876-.5-.262-.583-.52-1.01-3.127-.493-3.016-.798-3.603-1.869-3.603-.254 0-1.513.755-2.798 1.678-2.11 1.516-2.393 1.659-2.919 1.476-.435-.152-.688-.483-.997-1.306-.229-.606-.667-1.774-.975-2.595-.622-1.656-.969-2.027-1.901-2.027-.52 0-.991.374-2.679 2.127-2.653 2.756-2.663 2.755-4.614-.445-.78-1.279-1.595-2.421-1.812-2.537-.488-.262-1.062-.261-1.511.002m2.418 9.635c2.311 1.645 1.082 5.512-1.752 5.512-2.75 0-4.135-3.313-2.171-5.194 1.108-1.062 2.697-1.191 3.923-.318m-2.906 10.214c1.515.576 2.137.23 5.596-3.104l2.599-2.506 1.1.146c3.45.458 10.312 3.472 14.255 6.261 3.623 2.564 8.438 7.786 10.49 11.377l.439.769-1.944 4.38c-1.07 2.409-1.945 4.633-1.945 4.944 0 .717.47 1.851.923 2.226.191.159 2.006 1.033 4.033 1.942l3.684 1.654.145.937c.187 1.221.212 4.22.042 5.072l-.133.666h-2.103c-2.439 0-2.251-.218-2.383 2.774-.096 2.169-.62 3.368-1.812 4.144-1.942 1.267-5.149 1.037-6.509-.466-.209-.231-.615-1.392-.903-2.581-.841-3.473-1.971-5.423-4.241-7.32-.717-.599-1.303-1.158-1.303-1.243 0-.084.788-.748 1.752-1.473 3.51-2.646 5.528-5.726 5.75-8.777.423-5.819-4.213-11.243-11.109-13.001-1.635-.417-2.333-.43-22.56-.43-11.48 0-20.873-.075-20.873-.166 0-.215 2.551-2.691 4.054-3.933 4.127-3.412 9.488-6.097 15.04-7.531l1.92-.497 2.728 2.766c1.501 1.521 2.972 2.857 3.268 2.97M27.432 48.526c1.257.823 1.772 2.891 1.03 4.134-1.148 1.924-4.056 2.005-5.205.145-1.671-2.702 1.547-6.001 4.175-4.279m74.05.105c3.288 2.005.74 6.937-2.78 5.38-2.35-1.04-2.425-4.252-.127-5.424.959-.489 2.061-.472 2.907.044M37.12 60.907v12.266H26.276l-.43-1.866c-.846-3.675-1.202-7.477-.989-10.591l.149-2.188 3.728-1.672c2.339-1.048 3.843-1.847 4.037-2.144.848-1.293.767-2.217-.423-4.845l-.556-1.227h5.328v12.267m31.22-11.733c2.322.604 3.549 1.833 3.552 3.556.002 1.265-.625 2.059-2.18 2.761-1.101.498-1.276.51-8.219.578l-7.093.068v-7.284h6.355c4.964 0 6.625.07 7.585.321m-2.396 17.602c1.151.32 2.512 1.32 3.21 2.359.733 1.092 1.162 2.512 2.178 7.216.858 3.976 1.41 5.276 2.956 6.968 1.915 2.095 1.471 2.014 11.037 2.014 4.581 0 8.328.073 8.328.163 0 .161-3.155 3.891-3.291 3.891-.039 0-1.687-.345-3.662-.767-5.577-1.191-5.778-1.051-7.058 4.926l-.823 3.84-.743.366c-1.24.612-5.27 1.872-7.359 2.302-3.452.71-7.209.95-10.511.671-5.629-.477-13.083-2.661-13.374-3.92-.062-.267-.437-1.995-.832-3.841-.396-1.846-.877-3.597-1.069-3.891-.923-1.408-1.894-1.495-6.164-.55-1.617.358-3.028.65-3.136.65-.203 0-3.204-3.47-3.204-3.704 0-.073 7.128-.158 15.84-.188l15.84-.054.057-5.627c.04-3.973-.015-5.714-.187-5.92-.192-.232-1.214-.293-4.91-.293H54.4V66.56l5.387.001c2.962.001 5.733.098 6.157.215M41.536 92.365c2.519 1.535 1.311 5.557-1.668 5.554-3.055-.002-4.187-3.987-1.584-5.575.861-.525 2.374-.515 3.252.021m46.126.168c1.235.905 1.646 2.788.881 4.042-2.009 3.295-7.033.676-5.355-2.791.825-1.703 3.018-2.317 4.474-1.251" fill-rule="evenodd"></path>
                </LanguageCard>
                <LanguageCard language="Java">
                    <path class="fill-text group-hover:fill-blue" d="M47.617 98.12s-4.767 2.774 3.397 3.71c9.892 1.13 14.947.968 25.845-1.092 0 0 2.871 1.795 6.873 3.351-24.439 10.47-55.308-.607-36.115-5.969zm-2.988-13.665s-5.348 3.959 2.823 4.805c10.567 1.091 18.91 1.18 33.354-1.6 0 0 1.993 2.025 5.132 3.131-29.542 8.64-62.446.68-41.309-6.336z"></path><path class="fill-text group-hover:fill-red" d="M69.802 61.271c6.025 6.935-1.58 13.17-1.58 13.17s15.289-7.891 8.269-17.777c-6.559-9.215-11.587-13.792 15.635-29.58 0 .001-42.731 10.67-22.324 34.187z"></path><path class="fill-text group-hover:fill-blue" d="M102.123 108.229s3.529 2.91-3.888 5.159c-14.102 4.272-58.706 5.56-71.094.171-4.451-1.938 3.899-4.625 6.526-5.192 2.739-.593 4.303-.485 4.303-.485-4.953-3.487-32.013 6.85-13.743 9.815 49.821 8.076 90.817-3.637 77.896-9.468zM49.912 70.294s-22.686 5.389-8.033 7.348c6.188.828 18.518.638 30.011-.326 9.39-.789 18.813-2.474 18.813-2.474s-3.308 1.419-5.704 3.053c-23.042 6.061-67.544 3.238-54.731-2.958 10.832-5.239 19.644-4.643 19.644-4.643zm40.697 22.747c23.421-12.167 12.591-23.86 5.032-22.285-1.848.385-2.677.72-2.677.72s.688-1.079 2-1.543c14.953-5.255 26.451 15.503-4.823 23.725 0-.002.359-.327.468-.617z"></path><path class="fill-text group-hover:fill-red" d="M76.491 1.587S89.459 14.563 64.188 34.51c-20.266 16.006-4.621 25.13-.007 35.559-11.831-10.673-20.509-20.07-14.688-28.815C58.041 28.42 81.722 22.195 76.491 1.587z"></path><path class="fill-text group-hover:fill-blue" d="M52.214 126.021c22.476 1.437 57-.8 57.817-11.436 0 0-1.571 4.032-18.577 7.231-19.186 3.612-42.854 3.191-56.887.874 0 .001 2.875 2.381 17.647 3.331z"></path>
                </LanguageCard>
                <LanguageCard language="Python">
                    <path class="fill-text group-hover:fill-blue" d="M63.391 1.988c-4.222.02-8.252.379-11.8 1.007-10.45 1.846-12.346 5.71-12.346 12.837v9.411h24.693v3.137H29.977c-7.176 0-13.46 4.313-15.426 12.521-2.268 9.405-2.368 15.275 0 25.096 1.755 7.311 5.947 12.519 13.124 12.519h8.491V67.234c0-8.151 7.051-15.34 15.426-15.34h24.665c6.866 0 12.346-5.654 12.346-12.548V15.833c0-6.693-5.646-11.72-12.346-12.837-4.244-.706-8.645-1.027-12.866-1.008zM50.037 9.557c2.55 0 4.634 2.117 4.634 4.721 0 2.593-2.083 4.69-4.634 4.69-2.56 0-4.633-2.097-4.633-4.69-.001-2.604 2.073-4.721 4.633-4.721z" transform="translate(0 10.26)"></path><path class="fill-text group-hover:fill-yellow" d="M91.682 28.38v10.966c0 8.5-7.208 15.655-15.426 15.655H51.591c-6.756 0-12.346 5.783-12.346 12.549v23.515c0 6.691 5.818 10.628 12.346 12.547 7.816 2.297 15.312 2.713 24.665 0 6.216-1.801 12.346-5.423 12.346-12.547v-9.412H63.938v-3.138h37.012c7.176 0 9.852-5.005 12.348-12.519 2.578-7.735 2.467-15.174 0-25.096-1.774-7.145-5.161-12.521-12.348-12.521h-9.268zM77.809 87.927c2.561 0 4.634 2.097 4.634 4.692 0 2.602-2.074 4.719-4.634 4.719-2.55 0-4.633-2.117-4.633-4.719 0-2.595 2.083-4.692 4.633-4.692z" transform="translate(0 10.26)"></path>
                </LanguageCard>
                <LanguageCard language="Python">
                    <path class="fill-text group-hover:brightness-125 group-hover:fill-blue" d="M115.4 30.7L67.1 2.9c-.8-.5-1.9-.7-3.1-.7-1.2 0-2.3.3-3.1.7l-48 27.9c-1.7 1-2.9 3.5-2.9 5.4v55.7c0 1.1.2 2.4 1 3.5l106.8-62c-.6-1.2-1.5-2.1-2.4-2.7z"></path><path class="fill-text group-hover:fill-blue" d="M10.7 95.3c.5.8 1.2 1.5 1.9 1.9l48.2 27.9c.8.5 1.9.7 3.1.7 1.2 0 2.3-.3 3.1-.7l48-27.9c1.7-1 2.9-3.5 2.9-5.4V36.1c0-.9-.1-1.9-.6-2.8l-106.6 62z"></path><path class="fill-surface0" d="M85.3 76.1C81.1 83.5 73.1 88.5 64 88.5c-13.5 0-24.5-11-24.5-24.5s11-24.5 24.5-24.5c9.1 0 17.1 5 21.3 12.5l13-7.5c-6.8-11.9-19.6-20-34.3-20-21.8 0-39.5 17.7-39.5 39.5s17.7 39.5 39.5 39.5c14.6 0 27.4-8 34.2-19.8l-12.9-7.6z"></path>
                </LanguageCard>
                <LanguageCard language="C#">
                    <path class="fill-text group-hover:brightness-125 group-hover:fill-mauve" d="M115.4 30.7L67.1 2.9c-.8-.5-1.9-.7-3.1-.7-1.2 0-2.3.3-3.1.7l-48 27.9c-1.7 1-2.9 3.5-2.9 5.4v55.7c0 1.1.2 2.4 1 3.5l106.8-62c-.6-1.2-1.5-2.1-2.4-2.7z"></path><path class="fill-text group-hover:fill-mauve" d="M10.7 95.3c.5.8 1.2 1.5 1.9 1.9l48.2 27.9c.8.5 1.9.7 3.1.7 1.2 0 2.3-.3 3.1-.7l48-27.9c1.7-1 2.9-3.5 2.9-5.4V36.1c0-.9-.1-1.9-.6-2.8l-106.6 62z"></path><path class="fill-surface0" d="M85.3 76.1C81.1 83.5 73.1 88.5 64 88.5c-13.5 0-24.5-11-24.5-24.5s11-24.5 24.5-24.5c9.1 0 17.1 5 21.3 12.5l13-7.5c-6.8-11.9-19.6-20-34.3-20-21.8 0-39.5 17.7-39.5 39.5s17.7 39.5 39.5 39.5c14.6 0 27.4-8 34.2-19.8l-12.9-7.6zM97 66.2l.9-4.3h-4.2v-4.7h5.1L100 51h4.9l-1.2 6.1h3.8l1.2-6.1h4.8l-1.2 6.1h2.4v4.7h-3.3l-.9 4.3h4.2v4.7h-5.1l-1.2 6h-4.9l1.2-6h-3.8l-1.2 6h-4.8l1.2-6h-2.4v-4.7H97zm4.8 0h3.8l.9-4.3h-3.8l-.9 4.3z"></path>
                </LanguageCard>
                <LanguageCard language="Haskell">
                    <path class="fill-text group-hover:fill-mauve" d="M0 110.2L30.1 65 0 19.9h22.6L52.7 65l-30.1 45.1H0z"></path><path class="fill-text group-hover:fill-lavender" d="M30.1 110.2L60.2 65 30.1 19.9h22.6l60.2 90.3H90.4L71.5 81.9l-18.8 28.2H30.1z"></path><path class="fill-text group-hover:fill-pink" d="M102.9 83.8l-10-15.1H128v15.1h-25.1zM87.8 61.3l-10-15.1H128v15.1H87.8z"></path>
                </LanguageCard>
                <LanguageCard language="Kotlin">
                    <path class="fill-text group-hover:fill-blue" d="M0 0h65.4L0 64.4z"></path><path class="fill-text group-hover:fill-blue" d="M128 128L64.6 62.6 0 128z"></path><path class="fill-text group-hover:fill-peach" d="M0 128L128 0H64.6L0 63.7z"></path>
                </LanguageCard>
                <LanguageCard language="Go">
                    <path class="fill-text group-hover:fill-blue" d="M11.156 54.829c-.243 0-.303-.122-.182-.303l1.273-1.637c.12-.182.424-.303.666-.303H34.55c.243 0 .303.182.182.364l-1.03 1.576c-.121.181-.424.363-.606.363zM2.004 60.404c-.242 0-.303-.12-.182-.303l1.273-1.636c.121-.182.424-.303.667-.303h27.636c.242 0 .364.182.303.364l-.485 1.454c-.06.243-.303.364-.545.364zM16.67 65.98c-.242 0-.302-.182-.181-.364l.848-1.515c.122-.182.364-.363.607-.363h12.12c.243 0 .364.181.364.424l-.12 1.454c0 .243-.243.425-.425.425zM79.58 53.738c-3.819.97-6.425 1.697-10.182 2.666-.91.243-.97.303-1.758-.606-.909-1.03-1.576-1.697-2.848-2.303-3.819-1.878-7.516-1.333-10.97.91-4.121 2.666-6.242 6.605-6.182 11.514.06 4.849 3.394 8.849 8.182 9.516 4.121.545 7.576-.91 10.303-4 .545-.667 1.03-1.394 1.636-2.243H56.064c-1.272 0-1.575-.788-1.151-1.818.788-1.879 2.242-5.03 3.09-6.606.183-.364.607-.97 1.516-.97h22.06c-.12 1.637-.12 3.273-.363 4.91-.667 4.363-2.303 8.363-4.97 11.878-4.364 5.758-10.06 9.333-17.273 10.303-5.939.788-11.454-.364-16.302-4-4.485-3.394-7.03-7.879-7.697-13.454-.788-6.606 1.151-12.546 5.151-17.758 4.303-5.636 10-9.212 16.97-10.485 5.697-1.03 11.151-.363 16.06 2.97 3.212 2.121 5.515 5.03 7.03 8.545.364.546.122.849-.606 1.03z"></path><path class="fill-text group-hover:fill-blue" d="M99.64 87.253c-5.515-.122-10.546-1.697-14.788-5.334-3.576-3.09-5.818-7.03-6.545-11.697-1.091-6.848.787-12.909 4.909-18.302 4.424-5.819 9.757-8.849 16.97-10.122 6.181-1.09 12-.484 17.272 3.091 4.788 3.273 7.757 7.697 8.545 13.515 1.03 8.182-1.333 14.849-6.97 20.546-4 4.06-8.909 6.606-14.545 7.757-1.636.303-3.273.364-4.848.546zm14.424-24.485c-.06-.788-.06-1.394-.182-2-1.09-6-6.606-9.394-12.363-8.06-5.637 1.272-9.273 4.848-10.606 10.545-1.091 4.727 1.212 9.515 5.575 11.454 3.334 1.455 6.667 1.273 9.879-.363 4.788-2.485 7.394-6.364 7.697-11.576z"></path>
                </LanguageCard>
            </div>
        </section>
    }
}
