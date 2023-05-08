use yew::prelude::*;
use yew_router::prelude::*;
use crate::MainRoute;

#[function_component(NewsApp)]
pub fn news_app() -> Html {
    html!{
        <>
        <h1>{"This is the News page"}</h1>
        <Link<MainRoute> to={MainRoute::Home}>{"Click Here To See The Home Page"}</Link<MainRoute>>
        </>
    }
}
