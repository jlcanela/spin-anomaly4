use leptos::*;
use leptos_oidc::*;
use leptos_meta::*;
use leptos_router::*;

use url::Url;

mod auth0;

use crate::auth0::MakeAuth0;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/main.css"/>

        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

        <Router>
            <AppWithRouter/>
        </Router>
    }
}

fn main() {
    mount_to_body(|| view! { <App />})
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Home"/>
        <h1>Home</h1>

        // Your Pome Page without authentication
    }
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
            <div>
                <Authenticated unauthenticated=move || {
                    view! {
                        <LoginLink class="text-login">Sign in</LoginLink>
                    }
                }>
                    <LogoutLink class="text-logout">Sign Out</LogoutLink>
                </Authenticated>
            </div>
            
            <Routes>
                <Route path="/" view=move || view! { <Home/> }/>
                
                // This is an example route for your profile, it will render
                // loading if it's still loading, render unauthenticated if it's
                // unauthenticated and it will render the children, if it's
                // authenticated
                <Route
                path="/profile"
                view=move || {
                    view! {
                        <Authenticated
                        loading=move || view! { <Loading/> }
                        unauthenticated=move || view! { <Unauthenticated/> }
                        >
                        <Profile/>
                    </Authenticated>
                }
                }
                />
            </Routes>
        </MakeAuth0>
    }
}

