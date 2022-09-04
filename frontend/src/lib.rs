mod owner;
mod pet;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,

    #[at("/app/create-owner")]
    CreateOwner,

    #[at("/app/create-pet/{id}")]
    CreatePet { id: i32 },

    #[at("/app/{id}")]
    Detail { id: i32 },

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn app() -> Html {
    html! {}
}
