use crate::app::cars::{Car, CarDto, CreateCarDto, UpdateCarDto};
use crate::app::ssr::AppState;
use leptos::ServerFnError;

pub async fn create_car(
    app_state: &AppState,
    CreateCarDto {
        production_date,
        brand,
        color,
        state,
        owner_name,
    }: CreateCarDto,
) -> Result<CarDto, ServerFnError> {
    let car_state = state.to_string();
    let car_brand = brand.to_string();
    let result = sqlx::query!(
        r#"
            INSERT INTO cars (production_date, brand, color, state, owner_name)
            VALUES (?, ?, ?, ?, ?)
        "#,
        production_date,
        car_brand,
        color,
        car_state,
        owner_name
    )
    .execute(app_state.db_pool.as_ref())
    .await?;
    let id = result.last_insert_rowid();
    get_car(app_state, id).await
}

pub async fn update_car(
    app_state: &AppState,
    UpdateCarDto {
        id,
        color,
        state,
        owner_name,
    }: UpdateCarDto,
) -> Result<CarDto, ServerFnError> {
    let car_state = state.to_string();
    sqlx::query!(
        r#"
            UPDATE cars
            SET color = ?, state = ?, owner_name = ?
            WHERE id = ?
        "#,
        color,
        car_state,
        owner_name,
        id,
    )
    .execute(app_state.db_pool.as_ref())
    .await?;

    get_car(&app_state, id).await
}

pub async fn delete_car(app_state: &AppState, id: i64) -> Result<CarDto, ServerFnError> {
    let car = get_car(app_state, id).await?;
    sqlx::query(
        r#"
        DELETE FROM cars WHERE id = ?
    "#,
    )
    .bind(id)
    .execute(app_state.db_pool.as_ref())
    .await?;
    Ok(car.into())
}

pub async fn list_cars(app_state: &AppState) -> Result<Vec<CarDto>, ServerFnError> {
    let cars: Vec<Car> = sqlx::query_as(
        r#"
        SELECT id, production_date, brand, color, state, owner_name
        FROM cars
    "#,
    )
    .fetch_all(app_state.db_pool.as_ref())
    .await?;
    Ok(cars.into_iter().map(|c| c.into()).collect())
}

pub async fn get_car(app_state: &AppState, id: i64) -> Result<CarDto, ServerFnError> {
    let car: Car = sqlx::query_as(
        r#"
        SELECT id, production_date, brand, color, state, owner_name
        FROM cars
        WHERE id = ?
    "#,
    )
    .bind(id)
    .fetch_one(app_state.db_pool.as_ref())
    .await?;

    Ok(car.into())
}
