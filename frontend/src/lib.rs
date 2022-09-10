mod owner;
mod pet;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Root,

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
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render = {Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Root => html! {
            <>
            {"123"}
            </>
        },
        Route::NotFound => html! {
            <h1> {"404:"} <br/> {"The page is not found"}  </h1>
        },
        _ => html! {"Other"},
    }
}
