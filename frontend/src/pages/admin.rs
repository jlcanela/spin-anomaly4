use leptos::*;
use leptos_oidc::*;
use leptonic::prelude::*;

use crate::api::Api;


async fn init_game(access_token: String) {
    let _ = expect_context::<Api>().init_game(&access_token).await;
}

async fn clear_game(access_token: String) {
    expect_context::<Api>().clear_game(&access_token).await
}

#[component]
pub fn Admin() -> impl IntoView {

    let token = ||use_context::<Auth>().expect("No auth context").id_token().expect("No access token");
    let init = create_action( move |_: &String| {
        let t = token();
        init_game(t) 
    });
    let clear = create_action( move |_: &String| {
        let t = token();
        clear_game(t) 
    });

    view! {
        <Authenticated>
            <div id="game-content">
                <Tabs mount=Mount::Once>
                    <Tab name="games" label="Jeux".into_view()>
                        <h1>Jeux</h1>
                        <Button on_click=move |_| {
                            let dummy = "".to_string();
                            init.dispatch(dummy.clone());
                        }>Init</Button>
                        <Button on_click=move |_| {
                            let dummy = "".to_string();
                            clear.dispatch(dummy.clone());
                        }>Clear</Button>
                    </Tab>
                    <Tab name="players" label="Joueurs".into_view()>
                        <h1>Joueurs</h1>
                    </Tab>
                    //<Tab name="star-map" label="Star Map".into_view()>"Star Map"</Tab>
                    //<Tab name="diplomacy" label="Diplomacy".into_view()>"Diplomacy"</Tab>
                </Tabs>
            </div>   
        <div id="admin">

        </div>
        </Authenticated>            
    }
}