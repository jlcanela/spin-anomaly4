use leptonic::prelude::*;
use leptos::*;
use leptos_oidc::*;

#[must_use]
#[component(transparent)]
pub fn LoginLink(
    children: Children,
    #[prop(into, optional)] variant: OptionalMaybeSignal<ButtonVariant>,
    #[prop(into, optional)] color: OptionalMaybeSignal<ButtonColor>,
    #[prop(into, optional)] size: OptionalMaybeSignal<ButtonSize>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] active: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionalMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    //#[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView {
    let auth = expect_context::<Auth>();
    let login_url = move || auth.login_url();

    view! {
        <LinkButton 
            variant = variant color=color size=size disabled=disabled active=active id=id class=class style=style
            href=login_url>
            {children()}
        </LinkButton>
    }
}

/// A transparent component representing a logout link.
/// It generates a logout URL and renders a link with the provided children and optional CSS class.
#[must_use]
#[component(transparent)]
pub fn LogoutLink(
    children: Children,
    //#[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView {
    let auth = expect_context::<Auth>();
    let logout_url = move || auth.logout_url();

    view! {
        <LinkButton href=logout_url>
            {children()}
        </LinkButton>
    }
}


