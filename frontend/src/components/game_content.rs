use std::vec;

use leptonic::prelude::*;

use leptos::*;
//use leptos_oidc::*;

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Star {
    pub id: i32,
    pub name: String,
    pub owner: String,
    pub x: i32,
    pub y: i32,
    pub shuttles: i32,
    pub dev: i32,
    pub dev_max: i32,
}

#[component]
pub fn OwnedStars() -> impl IntoView {

    let (current_star_id, set_current_star_id) = create_signal(Option::<u32>::None);
    let (confirm_order, set_confirm_order) = create_signal(false);

    let (get_stars, _) = create_signal(vec!(
        Star { id: 0, name: "Bob".to_string(), owner: "Yellow".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        Star { id: 1, name: "Kevin".to_string(), owner: "Green".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 },
        Star { id: 2, name: "Stuart".to_string(), owner: "Blue".to_string(), x: 2, y: 3, shuttles: 4, dev: 5, dev_max: 6 }));

    let buttons = move || {
        if current_star_id.get().is_some() {
            view! {
                <div id="buttons">
                    <H2>"Choisissez votre ordre pour '#"{current_star_id}"'"</H2>
                    <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.0)>
                        <Button on_click=move |_| {}>"Produire"</Button>
                        <Button on_click=move |_| {}>"Piller"</Button>
                        <Button on_click=move |_| {}>"Développer"</Button>
                        <Button on_click=move |_| {}>"Déplacer"</Button>
                        <Button on_click=move |_| {}>"Attaquer"</Button>
                    </Stack>
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
                        <For each=move ||get_stars.get() key=|x| x.clone() children=move |star| view! {
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
        <Stack spacing=Size::Em(2.0)>
            <H2>"Vos étoiles"</H2>
            <p>"Sélectionner une étoile pour effectuer une action"</p>
            <Skeleton>
                <OwnedStars/>
            </Skeleton>
            <H2>"Radar"</H2>
            <Skeleton>
                <Radar/>
            </Skeleton>
        </Stack>
    }
}