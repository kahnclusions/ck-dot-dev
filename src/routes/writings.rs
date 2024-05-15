use crate::ui::{Link, Stack, Terminal};
use leptos::*;

#[component]
pub fn WritingsPage() -> impl IntoView {
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
