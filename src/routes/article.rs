use std::fs;

use crate::ui::Stack;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use leptos::*;
use leptos_meta::Script;
use leptos_router::use_params;
use leptos_router::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct FrontMatter {
    title: String,
    description: String,
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
    let paths: Vec<_> = fs::read_dir(format!(
        "{}/content/",
        std::env::current_dir().unwrap().display()
    ))
    .unwrap()
    .map(|path| path.unwrap().path().display().to_string())
    .collect();

    let file_path = format!(
        "{}/content/{}.md",
        std::env::var("CONTENT_PATH")
            .unwrap_or(std::env::current_dir().unwrap().display().to_string()),
        slug()
    );

    if !paths.contains(&file_path) {
        return view! {
            <div class="flex flex-col gap-6 mx-3 pb-3 mt-10">
                <Stack>
                    <h2 class="text-5xl font-dos text-foam">"Oops, that article couldn't be located."</h2>
                </Stack>
            </div>
        };
    }

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let matter = Matter::<YAML>::new();
    let result = matter.parse(contents.as_str());
    let front_matter: FrontMatter = result.data.unwrap().deserialize().unwrap();
    let html_content = markdown::to_html(result.content.as_str());

    view! {
        <div class="flex flex-col gap-6 mx-3 mt-8">
            <Script src="/js/prism.js" />
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
