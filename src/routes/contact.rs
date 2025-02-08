use crate::ui::{Link, Stack, Terminal};
use leptos::*;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 mx-3">
            <Terminal title="contact".into()>
                <Stack>
                    <p>"With experience as a tech lead, and in front-end and full-stack software development delivering projects from zero to production, write to me and let me know how I can help you get your ideas to market robustly and securely."</p>
                    <p>"I work on a project basis, or on an ongoing basis with daily, weekly, or monthly rates tailored to your needs."</p>
                    <p>"mailto: "<Link href="mailto:hello@0x75.net".into()>"hello@0x75.net"</Link></p>
                </Stack>
            </Terminal>
        </div>
    }
}
