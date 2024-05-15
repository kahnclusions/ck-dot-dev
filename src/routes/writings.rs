use crate::ui::{Link, Stack, Terminal};
use leptos::*;

#[component]
pub fn WritingsPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 mx-3 pb-3">
            <Terminal title="writings".into()>
                <Stack>
                    <p>"Whether you need someone to manage execution and delivery of your project, or an IC or consultant to join your existing team, write to me and let me know how I can help you get your ideas to market robustly and securely."</p>
                    <p>"I work on a project basis, or on an ongoing basis with daily, weekly, or monthly rates tailored to your needs."</p>
                    <p>"mailto: "<Link href="mailto:hello@ck.dev".into()>"hello@ck.dev"</Link></p>
                </Stack>
            </Terminal>
        </div>
    }
}
