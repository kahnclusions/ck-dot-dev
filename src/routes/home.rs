use leptos::*;

use crate::ui::{Stack, Terminal};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Stack class="gap-6".into()>
            <div class="flex flex-row justify-center mt-3">
                <img src="/images/me.png" class="w-[200px] h-[200px]  aspect-square object-cover object-top rounded-lg" />
            </div>
            <Terminal title="whoami".into() class="mx-3".into()>
                <div class="flex flex-col sm:flex-row gap-3">
                <Stack>
                    <p>"My name is Chris. I'm a software developer and tech lead from Canada who's washed ashore in Taipei, Taiwan."</p>
                </Stack>
                </div>
            </Terminal>
        </Stack>
    }
}
