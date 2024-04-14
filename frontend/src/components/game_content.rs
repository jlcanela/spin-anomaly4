use std::vec;

use api::Star;
use leptonic::prelude::*;
use leptos::*;

use crate::api::api::Api;

async fn fetch() -> Vec<Star> {
    expect_context::<Api>().fetch().await
}

async fn produce_order(star_id: i32) {
    expect_context::<Api>().produce_order(star_id.clone()).await
}

#[component]
pub fn OwnedStars() -> impl IntoView {

    let (current_star_id, set_current_star_id) = create_signal(Option::<u32>::None);
    let (confirm_order, set_confirm_order) = create_signal(false);

    let (version, set_version) = create_signal(0);

    let resource_stars = create_resource(move ||version.get(), move |_| fetch() );

    let (nb_shuttles_move, set_nb_shuttles_move) = create_signal(0.0);

    let (nb_shuttles_attack, set_nb_shuttles_attack) = create_signal(0.0);

    let produce = create_action( move |star_id: &i32| produce_order(star_id.clone()) );


    let buttons = move || {
        if current_star_id.get().is_some() {
            view! {
                <div id="buttons">
                    <H2>"Choisissez votre ordre pour '#"{current_star_id}"'"</H2>
                    <Tabs>
                        <Tab name="produce" label=view! { <div>"Produire"</div> }.into_view()>
                            <Stack spacing=Size::Em(1.0) orientation=StackOrientation::Horizontal>
                                <div>
                            "Produire des navettes sur Nogami"<br/>"Coût: 8 Points"
                                </div>
                                <Button on_click=move |_|  {
                                    let star_id = current_star_id.get().unwrap().clone() as i32;
                                    produce.dispatch(star_id);
                                    tracing::info!("Produce order sent");
                                    set_version.update(|v| *v = *v + 1);
                                }>"Produire"</Button>
                            </Stack>
                        </Tab>
                        <Tab name="loot" label=view! { <div>"Piller"</div> }.into_view()>
                            <Stack spacing=Size::Em(1.0) orientation=StackOrientation::Horizontal>
                                <div>
                                "Piller Nogami pour obtenir des navettes"<br/>
                                "Coût: 1 Point"<br/>
                                //"Dev_Max ({3}) updated to: {2}"<br/>
                                //"Dev ({3}) updated to: {2}"
                                </div>
                                <Button on_click=move |_| {
                                    tracing::info!("Loot order sent");
                                }>"Piller"</Button>
                            </Stack>
                        </Tab>
                        <Tab name="develop" label=view! { <div>"Développer"</div> }.into_view()>
                            <Stack spacing=Size::Em(1.0) orientation=StackOrientation::Horizontal>
                                <div>
                                    "Developper économiquement Nogami"<br/>
                                    "Coût: 8 Points & 3 Navettes"<br/>
                                    //"Dev ({2}) updated to: {3}."
                                </div>
                                <Button on_click=move |_| {
                                    tracing::info!("Develop order sent");
                                }>"Développer"</Button>
                            </Stack>
                        </Tab>
                        <Tab name="move" label=view! { <div>"Déplacer"</div> }.into_view()>
                            <Stack spacing=Size::Em(1.0) orientation=StackOrientation::Horizontal>
                                <div>
                                    "Déplacer "{nb_shuttles_move}" navettes de Nogami vers Rokoto"<br/>
                                    "Coût: {13} Points"
                                    <Stack spacing=Size::Em(1.0) orientation=StackOrientation::Horizontal>
                                        "Navettes:"
                                        <NumberInput min=0.0 max=10.0 step=1.0 get=nb_shuttles_move set=set_nb_shuttles_move placeholder="Sélectionnez le nombre de navettes à déplacer"/>
                                    </Stack>
                                </div>
                                <Button on_click=move |_| {
                                    tracing::info!("Move order sent");
                                }>"Déplacer"</Button>
                            </Stack>
                        </Tab>
                        <Tab name="attack" label=view! { <div>"Attaquer"</div> }.into_view()>
                            <Stack spacing=Size::Em(1.0) orientation=StackOrientation::Horizontal>
                                <div>
                                    "Attaquer Rokoto à partir de Nogami en utilisant "{nb_shuttles_attack}" navettes"<br/>
                                    "Coût: {13} Points"
                                    <Stack spacing=Size::Em(1.0) orientation=StackOrientation::Horizontal>
                                        "Navettes:"
                                        <NumberInput min=0.0 max=10.0 step=1.0 get=nb_shuttles_attack set=set_nb_shuttles_attack placeholder="Sélectionnez le nombre de navettes pour attaquer"/>
                                    </Stack>
                                </div>
                                <Button on_click=move |_| {
                                    tracing::info!("Attack order sent");
                                }>"Attaquer"</Button>
                            </Stack>
                        </Tab>

                    </Tabs>

                    <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0)>
                        "Ne pas demander de confirmation pour l'ordre:"
                        <Checkbox checked=confirm_order set_checked=set_confirm_order />
                    </Stack>
                </div>
            }
        } else {
            view! {
                <div></div>
            }
        }
    };

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
                            <For each=move ||resource_stars.get().unwrap_or(vec![]) key=|x| x.clone() children=move |star| view! {
                                <TableRow>
                                    <TableCell>{star.name}</TableCell>
                                    <TableCell>"("{star.x}", "{star.y}")"</TableCell>
                                    <TableCell>{star.shuttles}</TableCell>
                                    <TableCell>{star.dev}</TableCell>
                                    <TableCell>{star.dev_max}</TableCell>
                                    <TableCell><Button on_click=move |_| {
                                        set_current_star_id.set(Some(star.id as u32));
                                        tracing::info!("Star selected: {}", star.id);
                                    }>"Sélectionner"</Button></TableCell>
                                </TableRow>
                            }/>
                        </Transition>
                    </TableBody>
                </Table>
            </TableContainer>
            { buttons }
        </div>    
    }
}

#[component]
pub fn Radar() -> impl IntoView {
    let (get_stars, _) = create_signal(vec!(
        Star { id: 0, name: "Bob".to_string(), owner: "Yellow".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        Star { id: 1, name: "Kevin".to_string(), owner: "Green".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        Star { id: 2, name: "Stuart".to_string(), owner: "Blue".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 }));

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
                        <For each=move ||get_stars.get() key=|x| x.clone() children=move |star| view! {
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
pub fn GameContent() -> impl IntoView {
    view! {
        <Stack spacing=Size::Em(0.0)>
            <H2>"Vos étoiles"</H2>
            <p>"Sélectionner une étoile pour effectuer une action"</p>
            <OwnedStars/>
            <H2>"Radar"</H2>
            <Radar/>
        </Stack>
    }
}