use leptonic::prelude::*;
use leptos::*;
use leptos_oidc::*;

use crate::components::flags::*;
use crate::components::auth_button::{LoginLink, LogoutLink};

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
#[component]
pub fn Header() -> impl IntoView {
   
    view! {
        <div id="header" style="position: relative">
            <Stack class="header-anomaly4" orientation=StackOrientation::Horizontal spacing=Size::Em(0.6)>
                <Link href="/">
                    <H1>Anomaly4</H1>
                </Link>
               
                <Authenticated loading=|| view! { "" } >
                    <Link href="/game">
                    <H1 style="display: inline;">Le Jeu</H1>
                    </Link>
                </Authenticated>
            
                <Stack spacing=Size::Em(0.2) orientation=StackOrientation::Horizontal>
                    <Stack spacing=Size::Em(0.2) orientation=StackOrientation::Horizontal>
                        <Stack  style="margin-right: 1em;" spacing=Size::Em(0.2)>
                            <GbFlag/>
                            "EN"
                        </Stack>
                        <Stack spacing=Size::Em(0.2)>
                            <FrFlag/>
                            "FR"
                        </Stack>
                    </Stack>
                    <Authenticated unauthenticated=move || {
                        view! {                         
                            <LoginLink>"Se Connecter"</LoginLink> 
                        }
                    }>          
                    <LogoutLink>"Se Déconnecter"</LogoutLink>
                    </Authenticated>
                </Stack>
            </Stack>
        </div>
    }
}
