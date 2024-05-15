use crate::ui::{Stack, Terminal};
use leptos::*;

#[component]
pub fn WritingPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 mx-3 pb-3">
            <Terminal title="writing".into()>
                <Stack>
                    <p>"This page will become the single blog post view."</p>
                </Stack>
            </Terminal>
        </div>
    }
}
