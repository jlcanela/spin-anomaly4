use leptonic::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use url::Url;

mod components;
mod pages;
mod api;

use crate::components::auth0::auth;
use crate::components::error_template::{AppError, ErrorTemplate};
use crate::components::header::Header;
use crate::pages::game::Game;
use crate::pages::home::Home;
use crate::pages::admin::Admin;

leptos_i18n::load_locales!();

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

fn main() {
    use tracing_subscriber_wasm::MakeConsoleWriter;

    tracing_subscriber::fmt()
    .compact()
    .with_file(true)
    .with_line_number(true)
    .without_time()
    .with_writer(MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG))
    .init();

    let base_url = base_url();
    let config_url = format!("{}/api/config", base_url);    

    spawn_local(  async move {
        // Fetch config and initialize leptos_oidc before starting the application
        // It is necessary to avoid crash later in the application 
        // as leptos_oidc components can use expect_context::<Auth>(); and crash of not yet initialized
        auth(base_url.clone(), config_url.clone()).await;
        mount_to_body(|| view! { <App /> });
    });

}

#[component]
pub fn AppWithRouter() -> impl IntoView {

    view! {
        <Header/>
        <Routes>
            <Route path="/"      view=|| view! { <Home/> } />
            <Route path="/game"  view=|| view! { <Game/> } />
            <Route path="/admin" view=|| view! { <Admin/> }
            />
        </Routes>
    }
}

