use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            <p>{ "Lets do some more!" }</p>
            <div>
                <paper-button raised={"true"}><iron-icon icon="check"></iron-icon>{ "OK" }</paper-button>
                <paper-button raised={"true"}><iron-icon icon="clear"></iron-icon>{ "CANCEL" }</paper-button>
            </div>

            <p><paper-checkbox>{ "Checkbox" }</paper-checkbox></p>

            <p><paper-toggle-button></paper-toggle-button></p>

            <paper-tabs selected={ true }>
            <paper-tab>{ "TAB 1" }</paper-tab>
            <paper-tab>{ "TAB 2" }</paper-tab>
            <paper-tab>{ "TAB 3" }</paper-tab>
            </paper-tabs>
        </main>
    }
}
