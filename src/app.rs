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

fn generate_random_response() -> String {
    let mut rng = thread_rng();
    let num_words = rng.gen_range(5..10);
    (0..num_words)
        .map(|_| SAD_WORDS.choose(&mut rng).unwrap().to_owned())
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Participant {
    User,
    SadGpt,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Chat {
    content: RwSignal<String>,
    belongs_to: RwSignal<Participant>,
}

#[component]
fn Chat(cx: Scope, chat: Chat) -> impl IntoView {
    view! {
        cx,
        <p>{chat.content}</p>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let (chats, set_chats) = create_signal(
        cx,
        vec![Chat {
            belongs_to: create_rw_signal(cx, Participant::SadGpt),
            content: create_rw_signal(
                cx,
                "waaaa waaaa bahahaha moan sob sob waaaa waaaa?".to_string(),
            ),
        }],
    );

    view! { cx,
        <main class="bg-spt-bg min-h-screen">
            <ul>
                <For
                    each=chats
                    key=|chat| chat.content
                    view= move |cx, chat: Chat| view! { cx, <Chat chat /> }
                />
            </ul>
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
