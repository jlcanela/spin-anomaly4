use leptonic::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use url::Url;

mod components;
mod pages;
mod api;

use crate::components::auth0::MakeAuth0;
use crate::components::error_template::{AppError, ErrorTemplate};
use crate::components::header::Header;
use crate::pages::game::Game;
use crate::pages::home::Home;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
  //  provide_i18n_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/main.css"/>

        <Stylesheet id="leptos" href="/pkg/leptonic-template-ssr.css"/>
        <Stylesheet href="https://fonts.googleapis.com/css?family=Roboto&display=swap"/>
        //<Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Root default_theme=LeptonicTheme::default()>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
        }>
            <AppWithRouter/>
        </Router>
        </Root>
    }
}

fn main() {
    use tracing_subscriber_wasm::MakeConsoleWriter;

    let _subscriber = tracing_subscriber::fmt()
    .compact()
    .with_file(true)
    .with_line_number(true)
    .without_time()
    .with_writer(MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG))
    .init();

    mount_to_body(|| view! { <App />})
}


/// This will be rendered, if the authentication library is still loading
#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <Title text="Loading"/>
        <h1>Loading</h1>

        // Your Loading Page/Animation
    }
}

/// This will be rendered, if the user is unauthenticated
#[component]
pub fn Unauthenticated() -> impl IntoView {
    view! {
        <Title text="Unauthenticated"/>
        <h1>Unauthenticated</h1>

        // Your Unauthenticated Page
    }
}

/// This will be rendered, if the user is authentication
#[component]
pub fn Profile() -> impl IntoView {
    view! {
        <Title text="Profile"/>
        <h1>Profile</h1>

        // Your Profile Page
    }
}

fn location() -> String {
    document()
        .location()
        .expect("should have location")
        .href()
        .expect("href should be defined")
}


fn base_url() -> String {

    let loc = location();
    let url = Url::parse(&loc).expect("location should be a valid URL");
    let port = url
    .port()
    .map(|port| format!(":{}", port))
    .unwrap_or("".to_string());
   let schema = url.scheme();
   let host = url.host_str().expect("host should be defined");
   
   let root = format!("{}://{}{}", schema, host, port);
   root
}

#[component]
pub fn AppWithRouter() -> impl IntoView {

    let base_url = base_url();
    let config_url = format!("{}/api/config", base_url);

    view! {
        // This is an example for a navbar where you have a login and logout
        // button, based on the state.
        <MakeAuth0 base_url = base_url config_url = config_url loading = || view! { <div>Loading Config</div>}>
            <Header/>
            <Routes>
                <Route path="/" view=move || view! { <Home/> }/>
                
                // This is an example route for your profile, it will render
                // loading if it's still loading, render unauthenticated if it's
                // unauthenticated and it will render the children, if it's
                // authenticated
                <Route
                path="/game"
                view=|| view! { <Game/> }
                />
            </Routes>
        </MakeAuth0>
    }
}

