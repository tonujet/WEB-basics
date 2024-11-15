use leptos::{server, use_context, ServerFnError};
use crate::app::cars::*;


#[server(ListCars, "/api", "GetJson", "cars/list")]
pub async fn list_cars() -> Result<Vec<CarDto>, ServerFnError> {
    use crate::app::ssr::*;
    let app_state = use_context::<AppState>().expect("State must be passed");
    dal::list_cars(&app_state).await
}

#[server(GetCar, "/api", "GetJson", "cars/:id")]
pub async fn get_car(id: i64) -> Result<CarDto, ServerFnError> {
    use crate::app::ssr::*;
    let app_state = use_context::<AppState>().expect("State must be passed");
    dal::get_car(&app_state, id).await
}

#[server(CreateCar, "/api", "Url", "cars/add")]
pub async fn create_car(dto: CreateCarDto) -> Result<CarDto, ServerFnError> {
    use crate::app::ssr::*;
    dto.validate()?;
    let app_state = use_context::<AppState>().expect("State must be passed");
    dal::create_car(&app_state, dto).await
}

#[server(DeleteCar, "/api", "Url", "cars/delete")]
pub async fn delete_car(id: i64) -> Result<CarDto, ServerFnError> {
    use crate::app::ssr::*;
    let app_state = use_context::<AppState>().expect("State must be passed");
    dal::delete_car(&app_state, id).await
}

#[server(UpdateCar, "/api", "Url", "cars/update")]
pub async fn update_car(dto: UpdateCarDto) -> Result<CarDto, ServerFnError> {
    use crate::app::ssr::*;
    dto.validate()?;
    let app_state = use_context::<AppState>().expect("State must be passed");
    dal::update_car(&app_state, dto).await
}