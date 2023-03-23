use leptos::*;
use leptos::{ev::SubmitEvent, html::Input};
use leptos_meta::*;
use leptos_router::*;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::time::Duration;

const INFO_TEXT: &str = r#"
Hi! I'm Ignisda. I made this site to learn new technologies. It was inspired
by <a href="https://catgpt.wvd.io" _target="_blank">CatGPT</a>. To be clear:
this site does not actually use ChatGPT or any other form of AI. It just returns
a random number of sad words. Nothing is done with your input either. If you
want to know more, <a href="https://github.com/ignisda/sadgpt" target="_blank">
check out the code on Github</a>, or send me a message on
<a href="https://twitter.com/IgnisDa" target="_blank">Twitter</a>!
"#;

const SAD_WORDS: [&str; 7] = [
    "waaaa", "whimper", "sign", "sniff", "bahahaha", "sob", "moan",
];

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/sadgpt.css"/>
        <Link rel="shortcut icon" type_="image/png" href="/sadgpt.png"/>
        <Link rel="preload" as_="image" href="/sadgpt.png" />
        <Link rel="preload" as_="image" href="/user.png" />
        <Link rel="preload" as_="image" href="/creator.png" />
        <Title text="SadGPT" />
        <Router>
            <Routes>
                <Route path="/" view=  move |cx| view! { cx, <Home/> }/>
            </Routes>
        </Router>
    }
}

fn create_chat(cx: Scope, content: String, belongs_to: Participant) -> Chat {
    Chat {
        belongs_to: create_rw_signal(cx, belongs_to),
        content: create_rw_signal(cx, content),
    }
}

fn generate_random_response() -> String {
    let mut rng = thread_rng();
    let num_words = rng.gen_range(10..20);
    (0..num_words)
        .map(|_| SAD_WORDS.choose(&mut rng).unwrap().to_owned())
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Participant {
    Creator,
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
    let (apply_classes, img_src) = match chat.belongs_to.get() {
        Participant::User => ("bg-spt-user", "/user.png"),
        Participant::SadGpt => ("bg-spt-system", "/sadgpt.png"),
        Participant::Creator => ("bg-spt-creator", "/creator.png"),
    };

    view! {
        cx,
        <div
            class={format!("{apply_classes} text-lg py-6 px-4")}
        >
            <div
                class="w-11/12 lg:w-3/5 mx-auto prose flex items-center justify-start items-center space-x-4"
            >
                <img src=img_src class="w-8 h-8 !mb-0 rounded-md"/>
                <p class="text-spt-white !mt-0" inner_html={chat.content} />
            </div>
        </div>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    let (input_disabled, set_input_disabled) = create_signal(cx, false);
    let (chats, set_chats) = create_signal(
        cx,
        vec![create_chat(
            cx,
            "waaaa sniff bahahaha moan sob sob whimper?".to_string(),
            Participant::SadGpt,
        )],
    );

    let input_element: NodeRef<Input> = create_node_ref(cx);
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_input_disabled(true);
        let value = input_element().expect("<input> to exist").value();
        let chat = if value == ":info" {
            set_chats.update(|c| {
                c.push(create_chat(cx, INFO_TEXT.to_owned(), Participant::Creator));
            });
        } else {
            set_chats.update(|c| {
                c.push(create_chat(cx, value, Participant::User));
            });
            let new_chat = generate_random_response();
            let chat = create_chat(cx, format!("{new_chat}."), Participant::SadGpt);
            set_timeout(
                move || {
                    set_chats.update(|c| {
                        c.push(chat);
                    });
                    set_input_disabled(false);
                },
                Duration::from_secs(1),
            );
        };
    };

    view! { cx,
        <main class="bg-spt-bg min-h-screen">
            <div class="text-spt-white py-5 text-center">
                <h1 class="text-6xl font-semibold">"SadGPT"</h1>
                <p class="italic text-sm">"What if ChatGPT was sad?"</p>
            </div>
            <div class="pb-40">
                <ul>
                    <For
                        each=chats
                        key=|chat| chat.content
                        view= move |cx, chat: Chat| view! { cx, <Chat chat /> }
                    />
                </ul>
            </div>
            <div class="fixed bottom-6 w-full space-y-5">
                <form
                    class="w-4/5 md:w-2/3 mx-auto flex items-center justify-center space-x-4"
                    on:submit=on_submit
                >
                    <input
                        type="text"
                        node_ref=input_element
                        placeholder="Type your message here"
                        class="bg-[#40414f] border-0 text-[#ececf1] rounded-md w-full text-lg p-2"
                        autocomplete="off"
                        disabled=input_disabled
                    />
                    <button type="submit">
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
                    </button>
                </form>
                <p class="text-spt-white text-center">
                    "This site was created by "
                    <a href="https://www.twitter.com/IgnisDa" target="_blank">"IgnisDa. "</a>
                    "Type `:info` to learn more."
                </p>
            </div>
        </main>
    }
}
