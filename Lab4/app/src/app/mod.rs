mod cars;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use cars::{comp::*};
use crate::app::cars::{CarCrud};

#[cfg(feature = "ssr")]
pub mod ssr {
    use sqlx::SqlitePool;
    use std::sync::Arc;

    #[derive(Clone, Debug)]
    pub struct AppState {
        pub db_pool: Arc<SqlitePool>,
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(CarCrud::default());

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
        <Stylesheet id="leptos" href="/pkg/app.css" />
        <Router>
            <header class="header">
                <div class="header_link">
                    <a href="/">List</a>
                </div>
                <div class="header_link">
                    <a href="/cars/add">Add</a>
                </div>
            </header>
            <main class="body">
                <Routes>
                    <Route path="/" view=ListCarsComp/>
                    <Route path="/cars/list" view=ListCarsComp/>
                    <Route path="/cars/add" view=CreateCarComp/>
                    <Route path="/cars/update/:id" view=UpdateCarComp/>
                </Routes>
            </main>
        </Router>
    }
}
