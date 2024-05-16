use leptos::*;
use leptos_router::Outlet;

use crate::ui::icons::{AddPerson, CopyDocument, TextIcon};
use crate::ui::MenuItem;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <main class="bg-base text-text h-full w-full" dir="ltr">
            <div class="max-w-[960px] mx-auto flex flex-col gap-6">
                <header class="p-3 box-content flex flex-row gap-3 items-center justify-between">
                    <h1 class="text-4xl text-gold font-display">
                        <a href="/">"ck·dev"</a>
                    </h1>
                    <nav class=" font-dos text-2xl flex flex-row gap-2">
                        <p>
                            <MenuItem label="blog".into() href="/articles".into()>
                                <TextIcon />
                            </MenuItem>
                        </p>
                        <p>
                            <MenuItem label="work".into() href="/portfolio".into()>
                                <CopyDocument  />
                            </MenuItem>
                        </p>
                        <p>
                            <MenuItem label="contact".into() href="/contact".into()>
                                <AddPerson />
                            </MenuItem>
                        </p>
                    </nav>
                </header>
                <Outlet />
                <footer class="font-display px-3 pb-3 text-center">"© 2024 ck - made in Canada + Taiwan"</footer>
            </div>
        </main>
    }
}
