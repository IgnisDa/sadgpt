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
    let apply_classes = if matches!(chat.belongs_to.get(), Participant::User) {
        "bg-spt-user"
    } else {
        "bg-spt-system"
    };

    view! {
        cx,
        <div class={format!("{apply_classes} text-lg py-6 px-4")}>
            <p class="text-spt-white max-w-lg mx-auto">{chat.content}</p>
        </div>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
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
            <div class="absolute bottom-6 w-full">
                <form id="form" class="w-2/3 mx-auto flex items-center justify-center space-x-4">
                    <input
                        type="text"
                        placeholder="Type your message here"
                        class="bg-[#40414f] border-0 text-[#ececf1] rounded-md w-full text-lg p-2"
                        autocomplete="off"
                    />
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="w-6 h-6 text-spt-white"
                    >
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M6 12L3.269 3.126A59.768 59.768 0 0121.485 12 59.77 59.77 0 013.27 20.876L5.999 12zm0 0h7.5"
                     />
                    </svg>
                </form>
            </div>
        </main>
    }
}
