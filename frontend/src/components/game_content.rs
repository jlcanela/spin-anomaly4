use api::{Order, OrderResult, OrderFailed::*, Star};
use leptonic::prelude::*;
use leptos::{ *};
use uuid::Uuid;
use time;

use crate::api::Api;

async fn post_order(cmd: &Order) -> Result<OrderResult, String> {
    expect_context::<Api>().send_order(cmd).await
}

trait CmdView {
    fn header_view(&self) -> View;
    fn body_view(&self) -> View;
}   

impl CmdView for OrderResult {
    fn header_view(self: &OrderResult) -> View {
        match self {
            OrderResult::Produce{ .. } => view! { <div>"Produire"</div> }.into_view(),
            OrderResult::Loot { .. } => view! { <div>"Piller"</div> }.into_view(),
            OrderResult::Develop { .. } => view! { <div>"Développer"</div> }.into_view(),
            OrderResult::Colonize { .. } => view! { <div>"Coloniser"</div> }.into_view(),
            OrderResult::Move { .. } => view! { <div>"Déplacer"</div> }.into_view(),
            OrderResult::Attack { .. } => view! { <div>"Attaquer"</div> }.into_view(),
            OrderResult::OrderFailed(_) => view! { <div>"Echec"</div> }.into_view(),
        }
    }
    fn body_view(self: &OrderResult) -> View {
        match self {
            OrderResult::Produce { name, produced_shuttles, .. } => view! { <div>{format!("Vous avez produit {} nouveaux vaisseaux sur {}", produced_shuttles, name )} </div> }.into_view(),
            OrderResult::Loot { name, produced_shuttles, .. } => view! { <div>{format!("Vous avez pillé {} vaisseaux sur {}", produced_shuttles, name )} </div> }.into_view(),
            OrderResult::Develop { name, new_dev, .. } => view! { <div>{format!("Vous avez développé {} pour un total de {}", name, new_dev )} </div> }.into_view(),
            OrderResult::Colonize { name, new_dev, .. } => view! { <div>{format!("Vous avez colonisé {} et son niveau de développement est {}", name, new_dev )} </div> }.into_view(),
            OrderResult::Move { name_source, name_destination, moved_shuttles, .. } => view! { <div>{format!("Vous avez déplacé {} vaisseaux de {} vers {}", moved_shuttles, name_source, name_destination )} </div> }.into_view(),
            OrderResult::Attack { name_source, name_destination, attacking_shuttles, lost_shuttles, destroyed_shuttles, .. } => 
                view! { <div>{format!("Vous avez attaqué {} à partir de {} en utilisant {} navettes, vous avez perdu {} navettes et détruit {} navettes", name_destination, name_source, attacking_shuttles, lost_shuttles, destroyed_shuttles )} </div> }.into_view(),
            OrderResult::OrderFailed(NotEnoughPoints)  => view! { <div>"Vous n'avez pas assez de points pour effectuer cette action"</div> }.into_view(),
            OrderResult::OrderFailed(NotEnoughShuttles) => view! { <div>"Vous n'avez pas assez de navettes pour effectuer cette action"</div> }.into_view(),
            OrderResult::OrderFailed(NotEnoughDev) => view! { <div>"Votre étoile n’est pas suffisamment développée pour effectuer cette action"</div> }.into_view(),
            OrderResult::OrderFailed(TooMuchDev) => view! { <div>"Votre étoile est trop développée pour effectuer cette action"</div> }.into_view(),
            OrderResult::OrderFailed(DevShouldBeZero) => view! { <div>"Votre étoile ne devrait pas être développée pour effectuer cette action"</div> }.into_view(),
            OrderResult::OrderFailed(ServiceFailure(_, _)) => view! { <div>"Echec de l'ordre (erreur du jeu)"</div> }.into_view(),
            }
    }
}

#[component]
pub fn Command(
    current_star_id: ReadSignal<Option<u32>>,
    star_name: Signal<String>,
    set_version: WriteSignal<i32>,
) -> impl IntoView {
    let (nb_shuttles_move, set_nb_shuttles_move) = create_signal(0.0);
    let (nb_shuttles_attack, set_nb_shuttles_attack) = create_signal(0.0);

    let issue_order = create_action(move |cmd: &Order| {
        let star_id = current_star_id.get().unwrap() as i32;
        let mut cmd = cmd.clone();
        cmd.with_star_id(star_id);
        async move {
            let order_result = post_order(&cmd).await;
            order_result.map(|or| {
                set_version.update(|v| *v += 1);
                let toasts = expect_context::<Toasts>();
                let toast_variant = match or {
                    OrderResult::OrderFailed(_) => ToastVariant::Error,
                    _ => ToastVariant::Success,
                };
                let toast = Toast {
                    id: Uuid::new_v4(),
                    variant: toast_variant,
                    header: or.header_view(),
                    body: or.body_view(),
                    timeout: ToastTimeout::DefaultDelay, 
                    created_at: time::OffsetDateTime::now_utc(),
                };
                    
                toasts.push(toast);  
            })
        }
    });

    view! {
    <div id="buttons">
        <Tabs>
            <Tab name="produce" label=view! { <div>"Produire"</div> }.into_view()>
                <Stack spacing=Size::Em(1.0) orientation=StackOrientation::Horizontal>
                    <div>
                "Produire des navettes sur "{star_name}<br/>"Coût: 8 Points"
                    </div>
                    <Button on_click=move |_|  {
                        issue_order.dispatch(Order::produce(0));
                    }>"Produire"</Button>
                </Stack>
            </Tab>
            <Tab name="loot" label=view! { <div>"Piller"</div> }.into_view()>
                <Stack spacing=Size::Em(1.0) orientation=StackOrientation::Horizontal>
                    <div>
                    "Piller "{star_name}" pour obtenir des navettes"<br/>
                    "Coût: 1 Point"<br/>
                    //"Dev_Max ({3}) updated to: {2}"<br/>
                    //"Dev ({3}) updated to: {2}"
                    </div>
                    <Button on_click=move |_| {
                        issue_order.dispatch(Order::loot(0));
                    }>"Piller"</Button>
                </Stack>
            </Tab>
            <Tab name="develop" label=view! { <div>"Développer"</div> }.into_view()>
                <Stack spacing=Size::Em(1.0) orientation=StackOrientation::Horizontal>
                    <div>
                        "Developper économiquement "{star_name}<br/>
                        "Coût: 8 Points & 3 Navettes"<br/>
                        //"Dev ({2}) updated to: {3}."
                    </div>
                    <Button on_click=move |_| {
                        issue_order.dispatch(Order::develop(0));
                    }>"Développer"</Button>
                </Stack>
            </Tab>
            <Tab name="colonize" label=view! { <div>"Coloniser"</div> }.into_view()>
                <Stack spacing=Size::Em(1.0) orientation=StackOrientation::Horizontal>
                    <div>
                        "Coloniser "{star_name}<br/>
                        "Coût: 8 Points & 3 Navettes"<br/>
                        //"Dev ({2}) updated to: {3}."
                    </div>
                    <Button on_click=move |_| {
                        issue_order.dispatch(Order::colonize(0));
                    }>"Coloniser"</Button>
                </Stack>
            </Tab>
            <Tab name="move" label=view! { <div>"Déplacer"</div> }.into_view()>
                <Stack spacing=Size::Em(1.0) orientation=StackOrientation::Horizontal>
                    <div>
                        "Déplacer "{nb_shuttles_move}" navettes de "{star_name}" vers Rokoto"<br/>
                        "Coût: {13} Points"
                        <Stack spacing=Size::Em(1.0) orientation=StackOrientation::Horizontal>
                            "Navettes:"
                            <NumberInput min=0.0 max=10.0 step=1.0 get=nb_shuttles_move set=set_nb_shuttles_move placeholder="Sélectionnez le nombre de navettes à déplacer"/>
                        </Stack>
                    </div>
                    <Button disabled=true on_click=move |_| {
                    }>"Déplacer"</Button>
                </Stack>
            </Tab>
            <Tab name="attack" label=view! { <div>"Attaquer"</div> }.into_view()>
                <Stack spacing=Size::Em(1.0) orientation=StackOrientation::Horizontal>
                    <div>
                        "Attaquer Rokoto à partir de "{star_name}" en utilisant "{nb_shuttles_attack}" navettes"<br/>
                        "Coût: {13} Points"
                        <Stack spacing=Size::Em(1.0) orientation=StackOrientation::Horizontal>
                            "Navettes:"
                            <NumberInput min=0.0 max=10.0 step=1.0 get=nb_shuttles_attack set=set_nb_shuttles_attack placeholder="Sélectionnez le nombre de navettes pour attaquer"/>
                        </Stack>
                    </div>
                    <Button disabled=true on_click=move |_| {
                    }>"Attaquer"</Button>
                </Stack>
            </Tab>

        </Tabs>
        </div>
        // <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0)>
        //     "Ne pas demander de confirmation pour l'ordre:"
        //     <Checkbox checked=confirm_order set_checked=set_confirm_order />
        // </Stack>
    }
}

#[component]
pub fn OwnedStars(
    stars: Signal<Option<Vec<Star>>>,
    set_version: WriteSignal<i32>,
) -> impl IntoView {
    let (current_star_id, set_current_star_id) = create_signal(Option::<u32>::None);

    view! {
        <div>
            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>"Nom"</TableHeaderCell>
                            <TableHeaderCell>"Position"</TableHeaderCell>
                            <TableHeaderCell>"Nb vaisseaux"</TableHeaderCell>
                            <TableHeaderCell>"Dev. éco actuel"</TableHeaderCell>
                            <TableHeaderCell>"Dev. éco max"</TableHeaderCell>
                            <TableHeaderCell>"Action"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <Transition>
                            <For each=move ||stars.get().unwrap_or_default() key=|x| x.clone() children=move |star| view! {
                                <TableRow>
                                    <TableCell>{star.name}</TableCell>
                                    <TableCell>"("{star.x}", "{star.y}")"</TableCell>
                                    <TableCell>{star.shuttles}</TableCell>
                                    <TableCell>{star.dev}</TableCell>
                                    <TableCell>{star.dev_max}</TableCell>
                                    <TableCell><Button on_click=move |_| {
                                        set_current_star_id.set(Some(star.id as u32));
                                    }>"Sélectionner"</Button>
                                    </TableCell>
                                </TableRow>
                                {
                                    move || { 
                                        match current_star_id.get() {
                                            Some(id) if id == star.id as u32 => {
                                                view! {
                                                    
                                                            <Command current_star_id=current_star_id set_version=set_version star_name=Signal::derive(move || stars.get().unwrap()[id as usize].name.clone())/>
                                                    
                                                }.into_view()
                                            }
                                            _ => "".into_view()                                                
                                        }
                                    }
                                }                                
                            }/>
                        </Transition>
                    </TableBody>
                </Table>
            </TableContainer>
        </div>
    }
}

#[component]
pub fn Radar(radar: Signal<Option<Vec<Star>>>) -> impl IntoView {
    view! {
        <div>
            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>"Nom"</TableHeaderCell>
                            <TableHeaderCell>"Propriétaire"</TableHeaderCell>
                            <TableHeaderCell>"Position"</TableHeaderCell>
                            <TableHeaderCell>"Nb vaisseaux"</TableHeaderCell>
                            <TableHeaderCell>"Dev. éco actuel"</TableHeaderCell>
                            <TableHeaderCell>"Dev. éco max"</TableHeaderCell>
                            <TableHeaderCell>"Distance en Années Lumière"</TableHeaderCell>
                            <TableHeaderCell>"Coût politique du déplacement"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <For each=move ||radar.get().unwrap_or_default() key=|x| x.clone() children=move |star| view! {
                            <TableRow>
                                <TableCell>{star.name}</TableCell>
                                <TableCell>{star.owner}</TableCell>
                                <TableCell>"("{star.x}", "{star.y}")"</TableCell>
                                <TableCell>{star.shuttles}</TableCell>
                                <TableCell>{star.dev}</TableCell>
                                <TableCell>{star.dev_max}</TableCell>
                                <TableCell>""</TableCell>
                                <TableCell>""</TableCell>
                            </TableRow>
                        }/>
                    </TableBody>
                </Table>
            </TableContainer>
        </div>

    }
}

#[component]
pub fn GameContent(
    stars: Signal<Option<Vec<Star>>>,
    radar: Signal<Option<Vec<Star>>>,
    set_version: WriteSignal<i32>,
) -> impl IntoView {
    view! {
        <Stack spacing=Size::Em(0.0)>
            <H2>"Vos étoiles"</H2>
            <p>"Sélectionner une étoile pour effectuer une action"</p>
            <OwnedStars stars = stars set_version = set_version/>
            <H2>"Radar"</H2>
            <Radar radar = radar />
        </Stack>
    }
}
