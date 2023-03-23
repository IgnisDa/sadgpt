use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use rand::{seq::SliceRandom, thread_rng, Rng};

const SAD_WORDS: [&str; 4] = ["waaaa", "bahahaha", "sob", "moan"];

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/sadgpt.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move |cx| view! { cx, <Home/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let mut rng = thread_rng();
    let num_words = rng.gen_range((5..10));
    let first_chat = (0..num_words)
        .map(|_| SAD_WORDS.choose(&mut rng).unwrap().to_owned())
        .collect::<Vec<_>>()
        .join(" ");
    let (chats, set_chats) = create_signal(cx, vec![first_chat]);

    view! { cx,
        <main class="bg-spt-bg min-h-screen">
            <button
                class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                "Something's here | "
                {move || if count() == 0 {
                    "Click me!".to_string()
                } else {
                    count().to_string()
                }}
                " | Some more text"
            </button>
        </main>
    }
}
