use crate::ui::{Stack, Terminal};
use leptos::*;

#[component]
pub fn ArticlesPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 mx-3 pb-3">
            <Terminal title="writings".into()>
                <Stack>
                    <p>"List of blog posts will go here."</p>
                </Stack>
            </Terminal>
        </div>
    }
}
