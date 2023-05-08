use yew::prelude::*;
use yew_router::prelude::*;
use crate::MainRoute;

#[function_component(HomeApp)]
pub fn home_app() -> Html {
    html!{
        <>
        <h1>{"This is the Home page"}</h1>
        <Link<MainRoute> to={MainRoute::News}>{"Click Here To See News"}</Link<MainRoute>>
        </>
    }
}