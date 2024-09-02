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
                            <Link href="/article/2024-09-02--deploying-leptos-with-nix".into()>Deploy your Rust app to Nix</Link>
                            <p>Published on 2 September 2024</p>
                        </li>
                        <li>
                            <Link href="/article/2024-05-15--leptos-and-nix".into()>Building Leptos apps with Nix</Link>
                            <p>Published on 16 May 2024</p>
                        </li>
                        <li>
                            <Link href="/article/2024-05-14--leptos-asset-hashes".into()>How to add cache-busting asset hashes to your Leptos app
    </Link>
                            <p>Published on 14 May 2024</p>
                        </li>
                    </ul>
                </Terminal>
            </div>
        }
}
