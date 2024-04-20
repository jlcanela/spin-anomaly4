use leptonic::prelude::*;
use leptos::*;
use leptos_oidc::*;
use crate::components::auth_button::LoginLink;
use crate::i18n::*;

#[component]
pub fn Register() -> impl IntoView {
    let i18n = provide_i18n_context();

    let (accept_register, set_accept_register) = create_signal(false);
    let disable_register = move || !accept_register.get();
    view! {       
        <div class="home-text">
            <p>{t!(i18n, register.disclaimer)}</p>
            <p>{t!(i18n, register.data_usage_1)}</p>
            <p>{t!(i18n, register.data_usage_2)}</p>
            <p>{t!(i18n, register.data_usage_3)}</p>
            <p>{t!(i18n, register.data_usage_4)}</p>
            <p>{t!(i18n, register.abuse_warning)}</p>
            <div style="block:inline">
                {t!(i18n, register.accept_terms)}
                <Checkbox checked=accept_register set_checked=set_accept_register />
                <LoginLink disabled=Signal::derive(disable_register)> {t!(i18n, register.register)}</LoginLink>  
            </div>
        </div>
    }
}

#[component]
pub fn Description() -> impl IntoView {
    let i18n = provide_i18n_context();
    view! {
        <div style="padding: 2em;">
            <div>
                <p>{t!(i18n, description.call_to_action_1)}</p>
                <p>{t!(i18n, description.call_to_action_2)}</p>
                <p>{t!(i18n, description.call_to_action_3)}</p>
                <p>{t!(i18n, description.call_to_action_4)}</p>
                <p>{t!(i18n, description.call_to_action_5)}</p>
                <p>{t!(i18n, description.call_to_action_6)}</p>
          </div>
            <div>
                <h2>{t!(i18n, story.title)}</h2>
                <p>{t!(i18n, story.desc_1)}</p>
                <p>{t!(i18n, story.desc_2)}</p>
                <p>{t!(i18n, story.desc_3)}</p>
            </div>
            <div>
                <h2>{t!(i18n, principle.title)}</h2>
                <p>{t!(i18n, principle.desc_1)}</p>
                <p>{t!(i18n, principle.desc_2)}</p>
                <p>{t!(i18n, principle.desc_3)}</p>
                <p>{t!(i18n, principle.desc_4)}</p>
            </div>
            <div>
                <h2>{t!(i18n, actions.title)}</h2>
                <ul>
                    <li>{t!(i18n, actions.develop)}</li>
                    <li>{t!(i18n, actions.attack)}</li>
                    <li>{t!(i18n, actions.produce)}</li>
                    <li>{t!(i18n, actions.colonize)}</li>
                    <li>{t!(i18n, actions.loot)}</li>
                    <li>{t!(i18n, actions.message)}</li>
                </ul>
                <p>{t!(i18n, actions.new_actions)}</p>
            </div>
            <div>
                <h2>{t!(i18n, anomalies.title)}</h2>
                <p>{t!(i18n, anomalies.anomaly_1)}</p>
                <p>{t!(i18n, anomalies.anomaly_2)}</p>
                <p>{t!(i18n, anomalies.anomaly_3)}</p>
                <p>{t!(i18n, anomalies.anomaly_4)}</p>
            </div>
        </div>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    
    let i18n = provide_i18n_context();


    view! {
        <div id="home">
            <Collapsibles default_on_open=OnOpen::CloseOthers>
                <Stack spacing=Size::Em(3.0)>
                    <Authenticated unauthenticated=move || view! {
                        <Collapsible>
                            <CollapsibleHeader slot>
                                <H2 style="margin: 0em 0em 0em 0em; width: 100%">{t!(i18n, register.title)}</H2>
                            </CollapsibleHeader>
                            <CollapsibleBody slot>
                                <Register/>
                            </CollapsibleBody>
                        </Collapsible>
                    }>""</Authenticated>     
                    <Collapsible open=true>
                        <CollapsibleHeader slot>
                            <H2 style="margin: 0em 0em 0em 0em; width: 100%">{t!(i18n, description.title)}</H2>
                        </CollapsibleHeader>
                        <CollapsibleBody slot>
                            <Description/>
                        </CollapsibleBody>
                    </Collapsible>
                </Stack>
            </Collapsibles>
        </div>
    }
}
