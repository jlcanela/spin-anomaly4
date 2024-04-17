use leptonic::prelude::*;
use leptos::*;
use leptos_oidc::*;

use crate::api::api::Api;
use crate::components::flags::*;
use crate::components::auth_button::{LoginLink, LogoutLink};

pub async fn is_admin(args: (String, String,  String)) -> bool {
    let (domain, audience , token) = args;
    api::jwt::has_role(domain, audience, token, "Admin".to_string()).await.unwrap_or(false)
}



#[component]
pub fn AdminButton() -> impl IntoView {
    
    let is_admin_params = || {
        let auth = use_context::<Auth>()?;
        let api = use_context::<Api>()?;
        let id_token = auth.id_token()?;
        let domain = api.config.auth0_domain;
        let audience = api.config.client_id;
        Some((domain, audience, id_token))
    };

    let is_admin_resource = create_resource(is_admin_params,   move|p| async {
        match p {
            Some(params) => {
                let res = is_admin(params).await;
                res
            },
            None => false
        }
    }  );

    view! {
        {
            move || {
                if is_admin_resource.get().unwrap_or(false) {
                    view! {
                        <Link href="/admin">
                            <H1 style="display: inline;">Admin</H1>
                        </Link>
                    }.into_view()
                } else {
                    view! { "" }.into_view()
                }        
            }
        }
    }
}

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
                    <AdminButton/>
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
