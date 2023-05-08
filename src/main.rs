use yew::prelude::*;
use yew_router::prelude::*;

mod home;
mod news;
use home::HomeApp;
use news::NewsApp;

#[derive(Clone, Routable, PartialEq)]
enum MainRoute {
    #[at("/")]
    Home,
    #[at("/news")]
    News,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! { <HomeApp /> },
        MainRoute::News => html! { <NewsApp /> },
        MainRoute::NotFound => html! {<h1>{"Not Found"}</h1>},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<MainRoute> render={switch_main} />
        </BrowserRouter>
    }
}

fn main(){
    yew::Renderer::<App>::new().render();
}
