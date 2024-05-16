pub mod icons;

use leptos::*;

#[component]
pub fn Container(children: Children, class: Option<String>) -> impl IntoView {
    let class = format!(
        "px-3 w-full lg:max-w-[700px] mx-auto {}",
        class.unwrap_or("".to_string())
    );
    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="p-3 border border-4 border-slate-500 bg-slate-50 dark:bg-slate-950">
            {children()}
        </div>
    }
}

#[component]
pub fn Stack(#[prop(default = "".to_string())] class: String, children: Children) -> impl IntoView {
    let class = format!("{} {}", "h-full w-full flex flex-col gap-3", class);
    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn Image(src: String, #[prop(default = "".to_string())] class: String) -> impl IntoView {
    let class = format!(
        "{} {}",
        "flex flex-col gap-3 absolute inset-0 object-cover w-full h-full max-w-none", class
    );
    view! { <img class=class src=src /> }
}

#[component]
pub fn Panel(#[prop(default = "".to_string())] class: String, children: Children) -> impl IntoView {
    let class = format!("{} {}", "w-full h-full relative shrink-0 snap-start", class);
    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn Pane(
    title: String,
    #[prop(default = None)] image: Option<String>,
    #[prop(default = "".to_string())] class: String,
    children: Children,
) -> impl IntoView {
    match image {
        Some(src) => {
            let class = format!(
                "{} {}",
                "border-[4px] bg-surface border border-hl-high text-2xl p-3 font-mono text-xl flex flex-col sm:flex-row gap-3", class
            );
            view! {
                <div class=class>
                    <div class="">
                    <p class="font-dos text-2xl text-foam">{title}</p>
                    {children()}
                    </div>
                    <div class="sm:w-[200px] grow shrink-0"><img src={src} class="sm:w-[200px] object-cover" /></div>
                </div>
            }
        }
        None => {
            let class = format!(
                "{} {}",
                "border-[4px] bg-surface border border-hl-high text-2xl p-3 font-mono text-xl",
                class
            );
            view! {
                <div class=class>
                    <p class="font-dos text-foam text-2xl">{title}</p>
                    {children()}
                </div>
            }
        }
    }
}

#[component]
pub fn Link(
    href: String,
    #[prop(default = false)] newtab: bool,
    #[prop(default = "".to_string())] rel: String,
    #[prop(default = "".to_string())] class: String,
    children: Children,
) -> impl IntoView {
    let class = format!("{} {}", "text-gold underline", class);
    let target = if newtab { "_blank " } else { "" };
    view! {
        <a href=href class=class rel=rel target=target>{children()}</a>
    }
}

#[component]
pub fn Terminal(
    title: String,
    #[prop(default = "".to_string())] class: String,
    children: Children,
) -> impl IntoView {
    let class = format!(
        "{} {}",
        "border-[4px] border bg-surface border-hl-high text-2xl p-3 font-mono text-xl", class
    );
    view! {
        <div class=class>
            <p class="font-dos"><span class="text-pine">"$ "</span><span class="text-foam">{title}</span></p>
            {children()}
        </div>
    }
}

#[component]
pub fn MenuItem(
    label: String,
    href: String,
    #[prop(default = "".to_string())] class: String,
    children: Children,
) -> impl IntoView {
    let class = format!(
        "{} {}",
        "h-[40px] flex flex-col items-center justify-center gap-0", class
    );
    view! {
        <a class=class href=href>
            <span class="">
                {children()}
            </span>
            <span class="text-sm">{label}</span>
        </a>
    }
}
