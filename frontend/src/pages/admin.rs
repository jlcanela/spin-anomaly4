use leptos::*;
use leptos_oidc::*;
use leptonic::prelude::*;

use crate::api::api::Api;


async fn init_game() {
    expect_context::<Api>().init_game().await
}

#[component]
pub fn Admin() -> impl IntoView {

    let init = create_action( move |_: &String| init_game() );

    view! {
        <Authenticated>
        <div id="admin">
            <h1>Admin</h1>
            <Button on_click=move |_| {
                let dummy = "".to_string();
                init.dispatch(dummy.clone());
            }>Init</Button>
        </div>
        </Authenticated>            
    }
}