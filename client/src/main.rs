use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{yew::styled_component, Style};

const STYLE_FILE: &str = include_str!("style.css");

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/hello")]
    HelloWorld,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(HelloWorld)]
fn hello() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Hello world!" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

#[function_component(NotFound)]
fn not_found() -> Html {
    let window = web_sys::window().unwrap();
    let path = window.location().pathname().unwrap();
    html! {
        <div>
            <h1>{ "404" }</h1>
            <p>{ format!("No such page: {}", path) }</p>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <> <h1>{ "Home" }</h1> <p>{ "This is a website" }</p></> },
        Route::HelloWorld => html! {
            <HelloWorld />
        },
        Route::NotFound => html! {<NotFound />},
    }
}

#[styled_component(App)]
fn app() -> Html {
    let stylesheet:Style = Style::new(STYLE_FILE).unwrap();
    html! {
        <div class={stylesheet}>
            <BrowserRouter>
                <header class="header">
                    <div class="menu">
                        <div class="menu" style="display: inline-block; padding: 5px;"><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></div> 
                        <div class="menu" style="display: inline-block; padding: 5px;"><Link<Route> to={Route::HelloWorld}>{ "HelloWorld" }</Link<Route>></div>
                    </div>
                </header>
                <div class="main">
                    <Switch<Route> render={switch} />
                </div>
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
