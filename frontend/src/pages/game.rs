use leptonic::prelude::*;
use leptos::*;

use crate::components::game_content::GameContent;

#[component]
pub fn Game() -> impl IntoView {
    view! {
        <div id="game">
        <Stack spacing=Size::Em(0.0)> 
            <Stack id="status" spacing=Size::Em(0.0) orientation=StackOrientation::Horizontal>
                <div class="status-border">
                    <div>Governeur: ABC
                    </div>
                    <div>|</div>
                    <div>18 Points
                    </div>
                    <div>|</div>
                    <div> prochain point dans 28h 12mn 00s
                    </div>
                </div>
            </Stack>    
            <div id="game-content">
                <Tabs mount=Mount::Once>
                    <Tab name="control-center" label="Centre de Contrôle".into_view()><GameContent/></Tab>
                    //<Tab name="star-map" label="Star Map".into_view()>"Star Map"</Tab>
                    //<Tab name="diplomacy" label="Diplomacy".into_view()>"Diplomacy"</Tab>
                </Tabs>
            </div>                   
        </Stack>
        </div>            
    }
}