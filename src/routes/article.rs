use std::fs;

use crate::ui::{Pane, Stack, Terminal};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use leptos::*;
use leptos::{leptos_dom::logging::console_log, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_router::{use_params, use_params_map};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct FrontMatter {
    title: String,
    description: String,
}

struct Article {
    title: String,
    descriptin: String,
    body: String,
}

#[derive(Params, PartialEq)]
struct PostParams {
    slug: String,
}

#[component]
pub fn ArticlePage() -> impl IntoView {
    let params = use_params::<PostParams>();
    let slug = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.slug.clone())
                .unwrap_or_default()
        })
    };
    let file_path = format!(
        "{}/content/{}.md",
        std::env::current_dir().unwrap().display(),
        slug()
    );
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let matter = Matter::<YAML>::new();
    let result = matter.parse(contents.as_str());
    let front_matter: FrontMatter = result.data.unwrap().deserialize().unwrap();
    let html_content = markdown::to_html(result.content.as_str());

    console_log(format!("Gonna load post ID {}", contents).as_str());
    view! {
        <div class="flex flex-col gap-6 mx-3 pb-3 mt-10">
            <Stack>
                <h2 class="text-5xl font-dos text-foam">{front_matter.title}</h2>
                <h3 class="text-xl font-dos text-subtle">{front_matter.description}</h3>
                <hr class="border-subtle border-dashed" />
                <div class="article" inner_html=html_content></div>
                <hr class="border-subtle border-dashed" />
            </Stack>
        </div>
    }
}
