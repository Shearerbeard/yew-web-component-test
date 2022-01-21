use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            <p>{ "Lets do some more!" }</p>
            <mwc-button label="Add 1" raised="true" icon="code"></mwc-button>
        </main>
    }
}
