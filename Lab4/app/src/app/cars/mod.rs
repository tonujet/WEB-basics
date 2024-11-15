pub mod comp;
pub mod ssf;

#[cfg(feature = "ssr")]
mod dal;

use crate::app::cars::ssf::{CreateCar, DeleteCar, GetCar, ListCars, UpdateCar};
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString, VariantNames};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Car {
    pub id: i64,
    pub production_date: chrono::NaiveDate,
    pub brand: CarBrand,
    pub color: String,
    pub state: CarState,
    pub owner_name: String,
}

impl From<CarDto> for Car {
    fn from(
        CarDto {
            id,
            production_date,
            brand,
            color,
            state,
            owner_name,
        }: CarDto,
    ) -> Self {
        Car {
            id,
            production_date,
            brand,
            color,
            state,
            owner_name,
        }
    }
}

use chrono::{NaiveDate, Utc};
use validator::{Validate, ValidationError};
use validator_derive::Validate;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate, Default)]
pub struct CarDto {
    pub id: i64,

    #[validate(custom(function = "validate_production_date"))]
    pub production_date: NaiveDate,

    pub brand: CarBrand,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Color must be between 1 and 100 characters"
    ))]
    pub color: String,

    pub state: CarState,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Owner name must be between 1 and 100 characters"
    ))]
    pub owner_name: String,
}

fn validate_production_date(date: &NaiveDate) -> Result<(), ValidationError> {
    let today = Utc::now().naive_utc().date();
    if *date > today {
        return Err(ValidationError::new("production_date_invalid")
            .with_message("Date exceeded current one".into()));
    }
    Ok(())
}

impl From<Car> for CarDto {
    fn from(
        Car {
            id,
            production_date,
            brand,
            color,
            state,
            owner_name,
        }: Car,
    ) -> Self {
        CarDto {
            id,
            production_date,
            brand,
            color,
            state,
            owner_name,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct CreateCarDto {
    #[validate(custom(function = "validate_production_date"))]
    production_date: NaiveDate,

    brand: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Color must be between 1 and 100 characters"
    ))]
    color: String,

    state: CarState,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Owner name must be between 1 and 100 characters"
    ))]
    owner_name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateCarDto {
    id: i64,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Color must be between 1 and 100 characters"
    ))]
    color: String,

    state: CarState,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Owner name must be between 1 and 100 characters"
    ))]
    owner_name: String,
}

trait EnumToSelect: IntoEnumIterator + Display {
    fn html_select_options() -> View {
        Self::iter()
            .map(|variant| {
                let variant = variant.to_string();
                view! {<option value={variant.clone()}>{variant}</option>}
            })
            .collect_view()
    }
}

// TODO test to the end
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    EnumIter,
    Display,
    EnumString,
    VariantNames,
    Default,
)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "TEXT"))]
pub enum CarState {
    #[default]
    FactoryNew,
    MinimalWear,
    FieldTested,
    WellWorn,
    BattleScarred,
}

impl EnumToSelect for CarState {}

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    EnumIter,
    Display,
    EnumString,
    VariantNames,
    Default,
)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "TEXT"))]
pub enum CarBrand {
    #[default]
    Audi,
    BMW,
    Mercedes,
    Toyota,
    Honda,
    Ford,
    Tesla,
    Volkswagen,
    Nissan,
    Chevrolet,
    Opel,
}

impl EnumToSelect for CarBrand {}

#[derive(leptos::Params, PartialEq, Debug)]
struct CarUrlParams {
    id: usize,
}

#[derive(Clone)]
pub struct CarCrud {
    pub create_car: MultiAction<CreateCar, Result<CarDto, ServerFnError>>,
    pub delete_car: Action<DeleteCar, Result<CarDto, ServerFnError>>,
    pub update_car: Action<UpdateCar, Result<CarDto, ServerFnError>>,
    pub list_cars: Action<ListCars, Result<Vec<CarDto>, ServerFnError>>,
    pub get_car: Action<GetCar, Result<CarDto, ServerFnError>>,
}

impl Default for CarCrud {
    fn default() -> Self {
        Self {
            create_car: create_server_multi_action::<CreateCar>(),
            delete_car: create_server_action::<DeleteCar>(),
            update_car: create_server_action::<UpdateCar>(),
            list_cars: create_server_action::<ListCars>(),
            get_car: create_server_action::<GetCar>(),
        }
    }
}
