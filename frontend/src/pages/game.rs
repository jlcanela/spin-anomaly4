use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn Game() -> impl IntoView {
    view! {
        <div style="padding: 2em;">

            <Skeleton height=Size::Em(50.0) animated=false>
                <H2>Le Jeu</H2>
            </Skeleton>
        </div>
    }
}