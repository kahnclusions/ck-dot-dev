use crate::ui::{Link, Terminal};
use leptos::*;

#[component]
pub fn ArticlesPage() -> impl IntoView {
    // TODO make this page dynamic
    view! {
        <div class="flex flex-col gap-6 mx-3">
            <Terminal title="cat /dev/urandom | echo".into()>
                <ul class="list-disc ml-5">
                    <li>
                        <Link href="/article/2024-05-15--leptos-and-nix".into()>Building Leptos apps with Nix, Fenix, and Crane</Link>
                        <p>Published on 16 May 2024</p>
                    </li>
                </ul>
            </Terminal>
        </div>
    }
}
