use api::{Player, Situation};
use leptonic::prelude::*;
use leptos::*;

use crate::{api::Api, components::game_content::GameContent};

async fn situation() -> Option<Situation> {
    expect_context::<Api>().situation().await
}

#[component] 
fn PlayerStatus(player: Signal<Player>) -> impl IntoView {
    view! {

        <div class="status-border">
            <div>Governeur: { move || player.get().name }                    
            </div>
            <div>|</div>
            <div>{ move || player.get().points } Points
            </div>
            <div>|</div>
            <div> prochain point dans 28h 12mn 00s
            </div>
        </div>
    }
}

#[component]
pub fn Game() -> impl IntoView {

    let (version, set_version) = create_signal(0);

    let resource_situation = create_resource(move ||version.get(), move |_| situation() );

    let stars = Signal::derive(move || {
        let situation = resource_situation.get().flatten()?;
        Some(situation.stars)
   });

    let radar = Signal::derive(move || {
        let situation = resource_situation.get().flatten()?;
        Some(situation.radar)
    });

    let player = Signal::derive(move || {
        match resource_situation.get().flatten() {
            Some(situation) => situation.player,
            None => Player::default(),
        }
    });

    view! {
        <div id="game">
        <Stack spacing=Size::Em(0.0)> 
            <Stack id="status" spacing=Size::Em(0.0) orientation=StackOrientation::Horizontal>
                <PlayerStatus player = player/>
            </Stack>    
            <div id="game-content">
                <Tabs mount=Mount::Once>
                    <Tab name="control-center" label="Centre de Contrôle".into_view()><GameContent stars = stars radar = radar set_version = set_version/></Tab>
                    //<Tab name="star-map" label="Star Map".into_view()>"Star Map"</Tab>
                    //<Tab name="diplomacy" label="Diplomacy".into_view()>"Diplomacy"</Tab>
                </Tabs>
            </div>                   
        </Stack>
        </div>            
    }
}