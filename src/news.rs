use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(NewsApp)]
pub fn news_app() -> Html {
    html!{
        <>
        <h1>{"This is the News page"}</h1>
        <Link<Route> to={Route::Home}>{"Click Here To See The Home Page"}</Link<Route>>
        </>
    }
}
