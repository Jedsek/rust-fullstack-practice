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

// #[wasm_bindgen(start)]
// pub fn run_app() {
//     yew::start_app::<App>();
// }

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
        Route::Home => html! {<h1> {"Home"} </h1>},
        _ => html! {"123"},
    }
}
