use leptos::*;
use leptos_router::Outlet;

use crate::ui::icons::{AddPerson, CopyDocument};
use crate::ui::MenuItem;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <main class="bg-base text-text h-full w-full" dir="ltr">
            <div class="max-w-[1000px] mx-auto">
                <header class="p-3 box-content flex flex-row gap-3 items-center justify-between">
                    <h1 class="text-4xl text-gold font-display">
                        <a href="/">"ck·dev"</a>
                    </h1>
                        <nav class=" font-dos text-2xl flex flex-row gap-3">
                    <p>
                        <MenuItem label="portfolio".into() href="/portfolio".into()>
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
                <footer class="font-display p-3 text-center mt-3">"© 2024 ck - made in Canada + Taiwan"</footer>
            </div>
        </main>
    }
}
