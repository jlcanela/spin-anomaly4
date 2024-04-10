use leptonic::prelude::*;
use leptos::*;
use leptos_oidc::*;
use crate::components::auth_button::LoginLink;

#[component]
pub fn Register() -> impl IntoView {

    let (accept_register, set_accept_register) = create_signal(false);
    let disable_register = move || !accept_register.get();
    view! {       
        <Stack spacing=Size::Em(1.0)>
        <div style="margin: 0 8em 0 8em">
            <p>"Ce site est un projet personnel privé. Seules les personnes habilitées ont le droit de s'enregistrer et de jouer."</p>
            <p>"Si vous vous enregistrez, vous pouvez le faire soit en utilisant un compte google, soit en fournissant un email et un mot de passe."</p>
            <p>"Votre email sera utilisé par le fournisseur d'authentification"<a href="https://auth0.com/">Auth0</a>" pour vous envoyer un email de confirmation. Auth0 fourni également un service de réinitialisation de mot de passe et de suppression de compte."</p>
            <p>"Le site en lui même n'utilise votre email que pour l'associer à votre compte de jeu, il ne vous enverra pas de spam."</p>
            <p>"Le fournisseur d'authentification "<a href="https://auth0.com/">Auth0</a>" peut conserver votre adresse IP pendant une durée indéfinie"</p>
            <p>"En cas de tentative de piratage ou d'abus, toutes les informations vous concernant seront transmises aux autorités compétentes."</p>
                                    
        </div>
        <div>
            "J'accepte ces conditions et je m'enregistre: "
            <Checkbox checked=accept_register set_checked=set_accept_register />
            <LoginLink disabled=Signal::derive(disable_register)>"S'enregistrer"</LoginLink>  
        </div>
        </Stack>
    }
}

#[component]
pub fn Description() -> impl IntoView {
    view! {
        <Skeleton style="padding: 2em;" animated=false>
            <div>
                <div>
                    <p>"Vous aimez les jeux simples à jouer et à comprendre où l'habilité peut faire la différence ?"</p>
                    <p>"Vous n'aimez ou ne pouvez pas passer des heures à jouer à des jeux de stratégie temps réel."</p>
                    <p>"Vous en avez assez de jouer contre un ordinateur stupide."</p>
                    <p>"Vous aimez avoir le temps d'user de diplomatie avec d'autres joueurs ?"</p>
                    <p>"ALORS, la 4e Anomalie est un jeu fait pour vous !"</p>
                    <p> "La quatrième anomalie est un jeu entièrement gratuit qui se situe à mi chemin entre le jeu par correspondance et le jeu en temps réel. Il y a pas de notion de tour de jeu et vous jouez à votre rythme tout en conservant les mêmes chances de gagner."</p>
                </div>
                <div>
                    <h2>"L'histoire"</h2>
                    <p>"Vous allez voir c'est très original"</p>
                    <p>"C'est le chaos. L'empereur vient de mourir et le système politique sclérosé vient de s'effondrer sur lui même. Vous, qui êtes le gouverneur d'une planète, vous êtes persuadé que vous seuls pourrez ramener la grandeur passée de l'empire. Le seul problème est que les autres gouverneurs ne sont même pas au courant. Vous allez être obligé de recourir à la force."</p>
                    <p>"Si vous voulez en lire plus au sujet de l'histoire cliquez là mais vous n'êtes pas obligé de le faire pour jouer."</p>
                </div>
                <div>
                    <h2>"Le principe"</h2>
                    <p>"Chaque heure qui passe vous apporte des points de pouvoir politique que vous pouvez utiliser pour réaliser des actions dans le jeu. Chaque action à un coût politique propre et nécessite également des ressources."</p>
                    <p>"Les étoiles que vous contrôlez produisent des vaisseaux que vous pouvez déplacer ou transformer pour améliorer les capacités de vos étoiles."</p>
                    <p>"Vous pouvez échanger des messages avec les autres joueurs pour nouer des alliances ou bien jouer de désinformation."</p>
                    <p>"Pour animer la partie, le jeu est ponctué d'anomalies dont je vous laisse la surprise."</p>
                </div>
                <div>
                    <h2>"Les actions"</h2>
                    <ul>
                        <li>"DEVELOPPER la capacité d'un système"</li>
                        <li>"DEPLACER des vaisseaux pour renforcer vos étoiles ou attaquer celles de votre voisin"</li>
                        <li>"PRODUIRE des vaisseaux sur vos étoiles"</li>
                        <li>"COLONISER des étoiles" </li>
                        <li>"PILLER une de vos étoiles pour produire plus de vaisseaux"</li>
                        <li>"envoyer un MESSAGE à un autre joueur."</li>
                    </ul>
                    <p>"De nouveaux ordres viennent s'ajouter au fur et à mesure de votre développement."</p>
                </div>
                <div>
                    <h2>"Les anomalies"</h2>
                    <p>"Quand la bête qui montre les dents lorsqu'elle est heureuse viendra, le vieux royaume des sages Prolarch' succombera."</p>
                    <p>"Plus tard, lorsque la chose ne mordra plus, elle se révoltera."</p> 
                    <p>"Les sages enfants viendront."</p>
                    <p>"Vainqueurs, ils instilleront le poison, la chose alors s'assagira ou disparaîtra."</p>
                </div>
                </div>
            </Skeleton>
            
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div style="padding: 3em;">
            <Collapsibles default_on_open=OnOpen::CloseOthers>
                <Stack spacing=Size::Em(3.0)>
                    <Authenticated unauthenticated=move || view! {
                        <Collapsible>
                            <CollapsibleHeader slot>
                                <H2 style="margin: 0em 0em 0em 0em; width: 100%">"S'enregistrer"</H2>
                            </CollapsibleHeader>
                            <CollapsibleBody slot>
                                <Register/>
                            </CollapsibleBody>
                        </Collapsible>
                    }>""</Authenticated>     
                    <Collapsible open=true>
                        <CollapsibleHeader slot>
                            <H2 style="margin: 0em 0em 0em 0em; width: 100%">"Le jeu Anomaly 4"</H2>
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
