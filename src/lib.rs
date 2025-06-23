use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod data;

use components::{AboutView, ContactView, NotFoundView, ProjectsView, ResumeView, Sidebar};
use data::Profile;

pub const BASE_URL: &str = "{API_SERVER}";

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    About,
    #[at("/resume")]
    Resume,
    #[at("/projects")]
    Projects,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::About => html! { <AboutView /> },
        Route::Resume => html! { <ResumeView /> },
        Route::Projects => html! { <ProjectsView /> },
        Route::Contact => html! { <ContactView /> },
        Route::NotFound => html! { <NotFoundView /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let mobile_menu_open = use_state(|| false);
    let current_route = use_route::<Route>().unwrap_or(Route::About);

    let profile_str = include_str!("datafile/profile.json");
    let profile: Profile = serde_json::from_str(profile_str).expect("valid json");

    let toggle_mobile_menu = {
        let mobile_menu_open = mobile_menu_open.clone();
        Callback::from(move |_| {
            mobile_menu_open.set(!*mobile_menu_open);
        })
    };

    let close_mobile_menu = {
        let mobile_menu_open = mobile_menu_open.clone();
        Callback::from(move |_| {
            mobile_menu_open.set(false);
        })
    };

    let on_navigate = {
        let mobile_menu_open = mobile_menu_open.clone();
        Callback::from(move |_| {
            mobile_menu_open.set(false);
        })
    };

    html! {
        <BrowserRouter>
            <div class="container">
                // Mobile menu button
                <button
                    class="mobile-menu-btn"
                    onclick={toggle_mobile_menu}
                >
                    <i class={if *mobile_menu_open { "fas fa-times" } else { "fas fa-bars" }}></i>
                </button>

                // Mobile overlay
                if *mobile_menu_open {
                    <div class="mobile-overlay" onclick={close_mobile_menu.clone()}></div>
                }

                // Desktop sidebar
                <Sidebar
                    profile={profile.clone()}
                    current_route={current_route.clone()}
                    is_mobile={false}
                    is_open={true}
                    on_navigate={on_navigate.clone()}
                />

                // Mobile sidebar
                if *mobile_menu_open {
                    <Sidebar
                        profile={profile.clone()}
                        current_route={current_route.clone()}
                        is_mobile={true}
                        is_open={*mobile_menu_open}
                        on_navigate={on_navigate}
                    />
                }

                // Main content
                <main class={if *mobile_menu_open { "main-content main-content-mobile" } else { "main-content" }}>
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
