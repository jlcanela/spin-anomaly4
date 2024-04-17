use leptonic::prelude::*;
use leptos::*;
use leptos_oidc::*;
use leptos_use::storage::use_local_storage;
use leptos_use::utils::FromToStringCodec;

use crate::api::api::Api;
use crate::components::flags::*;
use crate::components::auth_button::{LoginLink, LogoutLink};

pub async fn jwks(domain: &String) -> Result<String, String> {
    api::jwks::load_jwks(domain).await
}

pub async fn is_admin(jwks: &String, audience: &String, token: &String) -> bool {
    let admin_nole = "Admin".to_string(); 
    api::jwt::has_role(jwks, audience, token, &admin_nole).await.unwrap_or(false)
}



#[component]
pub fn AdminButton() -> impl IntoView {
    
    let (local_jwk, local_jwk_set, _) = use_local_storage::<String, FromToStringCodec>("jwk");

    spawn_local(async move {
        let current = local_jwk.get_untracked();

        if current.is_empty() {
            let api = use_context::<Api>();
            match api {
                Some(api) => {
                    let domain = api.config.auth0_domain;
                    let j = jwks(&domain).await.ok();
                    local_jwk_set.set(j.unwrap().to_string());
                },
                None => {}
            }
        }
        
    });

    let params = move || {
        let auth = use_context::<Auth>()?;
        let api = use_context::<Api>()?;
        let id_token = auth.id_token()?;
        let audience = api.config.client_id;
        let jwks = local_jwk.get();
        Some((jwks, audience, id_token))
    };

    let memo = create_memo(move |m: Option<&(String, String, String)>| {
        match m {
            Some(pp) => pp.clone(),
            None => params().unwrap()
        }
    });

    let is_admin_resource = create_resource(move ||memo.get(),   move|p| async {
        match p {
            (jwks, audience, token) => {
                let res = is_admin(&jwks, &audience, &token).await;
                res
            }
        }
    });

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
