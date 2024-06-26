use leptonic::prelude::*;
use leptos::*;
use leptos_oidc::*;
use leptos_use::storage::use_local_storage;
use leptos_use::utils::FromToStringCodec;

use crate::api::Api;
use crate::components::flags::*;
use crate::components::auth_button::{LoginLink, LogoutLink};
use crate::i18n::*;

pub async fn jwks(domain: &String) -> Result<String, String> {
    api::jwks::load_jwks(domain).await
}

pub async fn is_admin(jwks: &String, audience: &String, token: &String) -> bool {
    let admin_nole = "Admin".to_string(); 
    api::jwt::has_role(jwks, audience, token, &admin_nole).await.unwrap_or(false)
}



#[component]
pub fn AdminButton() -> impl IntoView {
    
    let i18n = provide_i18n_context();

    let (local_jwk, local_jwk_set, _) = use_local_storage::<String, FromToStringCodec>("jwk");

    spawn_local(async move {
        let current = local_jwk.get_untracked();

        if current.is_empty() {
            let api = use_context::<Api>();
                if let Some(api) = api {
                    let domain = api.config.auth0_domain;
                    let j = jwks(&domain).await.ok();
                    local_jwk_set.set(j.unwrap().to_string());
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
        let (jwks, audience, token) = p;
        is_admin(&jwks, &audience, &token).await
    });

    view! {
        {
            move || {
                if is_admin_resource.get().unwrap_or(false) {
                    view! {
                        <Link href="/admin">
                            <H1 style="display: inline;">{t!(i18n, menu_admin)}</H1>
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
    let i18n = provide_i18n_context();

    view! {
        <div id="header" style="position: relative">
            <Stack class="header-anomaly4" orientation=StackOrientation::Horizontal spacing=Size::Em(0.6)>
                <Link href="/">
                    <H1>{t!(i18n, game_name)}</H1>
                </Link>
                <Authenticated loading=|| view! { "" } >
                    <Link href="/game">
                    <H1 style="display: inline;">{t!(i18n, the_game)}</H1>
                    </Link>
                    <AdminButton/>
                </Authenticated>

                <Stack spacing=Size::Em(0.2) orientation=StackOrientation::Horizontal>
                    <Stack spacing=Size::Em(0.0) orientation=StackOrientation::Horizontal>
                        <Stack spacing=Size::Em(0.0)>
                            <Button on_click=move |_| {
                                i18n.set_locale(Locale::en);
                            }>
                                <GbFlag/>
                                "EN"
                            </Button>  
                        </Stack>
                        <Stack style="margin-left: 0.1em; margin-right: 1em;" spacing=Size::Em(0.0)>
                            <Button on_click=move |_| {
                                i18n.set_locale(Locale::fr);
                            }>
                            <FrFlag/>
                            "FR"
                            </Button>
                        </Stack>
                    </Stack>
                    <Authenticated unauthenticated=move || {
                        view! {                         
                            <LoginLink>{t!(i18n, btn_connect)}</LoginLink> 
                        }
                    }>          
                    <LogoutLink>{t!(i18n, btn_disconnect)}</LogoutLink>
                    </Authenticated>
                </Stack>
            </Stack>
        </div>
    }
}
