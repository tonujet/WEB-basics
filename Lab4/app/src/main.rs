use std::sync::Arc;

#[cfg(feature = "ssr")]
mod ssr {
    pub use actix_files::*;
    pub use actix_web::*;
    pub use leptos::*;
    pub use leptos_actix::*;

    #[get("/style.css")]
    pub async fn css() -> impl Responder {
        NamedFile::open_async("./style.css").await
    }
}

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use self::ssr::*;
    use ::app::db;
    use app::app::*;
    use app::app::ssr::AppState;

    let db_conn = db::connect_and_migrate("sqlite:cars.db").await;

    // Setting this to None means we'll be using cargo-leptos and its env vars.
    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    let app_state = AppState {
        db_pool: Arc::new(db_conn),
    };

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;
        let routes = &routes;

        App::new()
            .service(css)
            .leptos_routes_with_context(
                leptos_options.to_owned(),
                routes.to_owned(),
                {
                    let state = app_state.clone();
                    move || provide_context(state.clone())
                },
                App,
            )
            .service(Files::new("/", site_root))
    })
    .bind(addr)?
    .run()
    .await
}
