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
            <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.6)>
                
                <Skeleton animated=false>
                    <Link  href="/">
                        <H2>Anomaly4</H2>
                    </Link>
                </Skeleton>
                <Authenticated loading=|| view! {
                    <Skeleton animated=false>""</Skeleton>

                } >
                    <Skeleton animated=false>
                        <H2 style="display: inline;">Game</H2>
                    </Skeleton>
                </Authenticated>
            
            <Skeleton width=Size::Em(5.0) animated=false>
                <Stack  style="margin-right: 1em;" spacing=Size::Em(0.2)>
                    <GbFlag/>
                    "EN"
                </Stack>
                <Stack spacing=Size::Em(0.2)>
                    <FrFlag/>
                    "FR"
                </Stack>
            </Skeleton>
                    <Authenticated unauthenticated=move || {
                        view! { 
                            <Skeleton  width=Size::Em(8.0) animated=false>
                            <LoginLink>Sign in</LoginLink> 
                        </Skeleton>                       
                        }
                    }>
                    <Skeleton  width=Size::Em(15.0) animated=false>
                    <LogoutLink>Sign out</LogoutLink>
                    </Skeleton>
                    </Authenticated>
        </Stack>
        </div>
    }
}