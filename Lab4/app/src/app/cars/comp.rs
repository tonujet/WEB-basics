use crate::app::cars::ssf::*;
use crate::app::cars::{CarBrand, CarCrud, CarDto, CarState, CarUrlParams, EnumToSelect};
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use std::collections::HashMap;
use validator::{Validate, ValidationErrors};

#[component]
pub fn ListCarsComp() -> impl IntoView {
    let CarCrud {
        create_car,
        delete_car,
        update_car,
        ..
    } = use_context::<CarCrud>().expect("Must be provided CRUD operations for car");

    let cars = create_resource(
        move || {
            (
                create_car.version().get(),
                delete_car.version().get(),
                update_car.version().get(),
            )
        },
        move |_| list_cars(),
    );

    let submissions = create_car.submissions();

    view! {
        <div>
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <div class="list">
                    {move || {
                        let pending_cars = move || {
                            submissions()
                                .into_iter()
                                .filter(|submission| submission.pending().get())
                                .map(|submission| {
                                    view! {
                                        <div class="list_block">
                                            <div class="block_loading">Loading</div>
                                            {move || submission.input.get().map(|car| car.dto.brand)}
                                        </div>
                                    }
                                })
                                .collect_view()
                        };
                        let existing_cars = {
                            move || {
                                cars()
                                    .map(move |cars| match cars {
                                        Ok(cars) => {
                                            if !(cars.is_empty()) {
                                                cars.into_iter().map(move |car| {to_car_comp(car)}).collect_view()
                                            } else {
                                                view! {<div class="list_empty">"There aren't any loaded cars"</div>}.into_view()
                                            }
                                        }
                                        Err(e) => {
                                            view! {<pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view()
                                        }
                                    }).unwrap_or_default()
                            }
                        };
                        view! {{existing_cars} {pending_cars}}
                    }}
                </div>
            </Transition>
        </div>
    }
}

pub fn to_car_comp(car: CarDto) -> impl IntoView {
    let car_crud = use_context::<CarCrud>().expect("Must be provided CRUD operations for car");

    view! {
        <div class="list_block">
            <div class="block_field">id: {car.id}</div>
            <div class="block_field">Brand: {car.brand.to_string()}</div>
            <div class="block_field">Owner name: {car.owner_name}</div>
            <div class="block_field">State: {car.state.to_string()}</div>
            <div class="block_field">Color: {car.color}</div>
            <div class="block_field">Production date: {car.production_date.format("%-d %B %C%y").to_string()}</div>
            <div class="block_field block_buttons">
                <div class="block_button delete_button">
                    <ActionForm action=car_crud.delete_car>
                        <input type="hidden" name="id" value=car.id />
                        <input type="submit" value="❌" />
                    </ActionForm>
                </div>
                <div class="block_button update_button">
                    <Form
                        action=""
                        on:submit=move |ev| {
                            ev.prevent_default();
                            use_navigate()(
                                format!("/cars/update/{}", car.id).as_ref(),
                                Default::default(),
                            );
                        }
                    >
                        <input type="submit" value="✏️" />
                    </Form>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn CreateCarComp() -> impl IntoView {
    let car_crud = use_context::<CarCrud>().expect("Must be provided CRUD operations for car");
    let (errs, set_errs) = create_signal(HashMap::new());

    view! {
        <div>
            <MultiActionForm
                action=car_crud.create_car
                on:submit=move |ev| {
                   match CreateCar::from_event(&ev) {
                        Ok(func) => {
                            validate_form(ev, func.dto.validate(), set_errs);
                        }
                        Err(err) => {
                            logging::log!("TODO this validation {:?}", err);
                            // TODO general error handling
                            ev.prevent_default();
                        }
                    }
                }
                class="form"
            >
                <div class="form_raw">
                    <label for="brand">{"Brand:"}</label>
                    <select name="dto[brand]" id="brand">
                        {CarBrand::html_select_options()}
                    </select>

                    <Show
                        when=move || { errs().get("brand").is_some() }
                        fallback=|| view! {}
                    >
                        <span class="error">{errs().get("brand").unwrap()}</span>
                    </Show>
                </div>

                <div class="form_raw">
                    <label for="color">{"Color:"}</label>
                    <input type="text" name="dto[color]" id="color"/>
                    <Show
                        when=move || { errs().get("color").is_some() }
                        fallback=|| view! {}
                    >
                        <span class="error">{errs().get("color").unwrap()}</span>
                    </Show>
                </div>

                <div class="form_raw">
                    <label for="owner_name">{"Name of owner:"}</label>
                    <input type="text" name="dto[owner_name]" id="owner_name"/>
                    <Show
                        when=move || { errs().get("owner_name").is_some() }
                        fallback=|| view! {}
                    >
                        <span class="error">{errs().get("owner_name").unwrap()}</span>
                    </Show>
                </div>

                <div class="form_raw">
                    <label for="production_date">{"Production year:"}</label>
                    <input type="date" min="1970-01-01" value="2024-01-01" name="dto[production_date]" id="production_date"/>
                    <Show
                        when=move || { errs().get("production_date").is_some() }
                        fallback=|| view! {}
                    >
                        <span class="error">{errs().get("production_date").unwrap()}</span>
                    </Show>
                </div>

                <div class="form_raw">
                    <label for="state">{"Car state:"}</label>
                    <select name="dto[state]" id="state">
                        {CarState::html_select_options()}
                    </select>
                    <Show
                        when=move || { errs().get("state").is_some() }
                        fallback=|| view! {}
                    >
                        <span class="error">{errs().get("state").unwrap()}</span>
                    </Show>
                </div>

                <div class="form_raw form_submit">
                    <input type="submit" value="Create" />
                </div>
            </MultiActionForm>
        </div>
    }
}

#[component]
pub fn UpdateCarComp() -> impl IntoView {
    let car_crud = use_context::<CarCrud>().expect("Must be provided CRUD operations for car");
    let params = use_params::<CarUrlParams>();
    let car_id =
        move || params.with(|params| params.as_ref().map(|params| params.id).unwrap_or_default());

    // TODO this properly
    // let car = create_resource(
    //     move || car_crud.update_car.version().get(),
    //     move |_| get_car(car_id() as i64),
    // );
    // let car = get_car(car_id() as i64);
    let (errs, set_errs) = create_signal(HashMap::new());

    view! {
        <div>
            <ActionForm
                class="form"
                action=car_crud.update_car
                on:submit=move |ev| {
                    match UpdateCar::from_event(&ev) {
                        Ok(func) => {
                            validate_form(ev, func.dto.validate(), set_errs);
                        }
                        Err(err) => {
                            logging::log!("TODO this validation {:?}", err);
                            // TODO general error handling
                            ev.prevent_default();
                        }
                    }
                }
            >
                <input type="hidden" name="dto[id]" value=car_id/>
                <div class="form_raw">
                    <label for="color">Color:</label>
                    <input type="text" name="dto[color]" id="color" value=""/>
                    <Show
                        when=move || { errs().get("color").is_some() }
                        fallback=|| view! {}
                    >
                        <span class="error">{errs().get("color").unwrap()}</span>
                    </Show>
                </div>

                <div class="form_raw">
                    <label for="owner_name">Name of owner: </label>
                    <input type="text" name="dto[owner_name]" id="owner_name" value=""/>
                    <Show
                        when=move || { errs().get("owner_name").is_some() }
                        fallback=|| view! {}
                    >
                        <span class="error">{errs().get("owner_name").unwrap()}</span>
                    </Show>
                </div>
                <div class="form_raw">
                    <label for="state">Car state: </label>
                    <select name="dto[state]" id="state">
                        {CarState::html_select_options()}
                    </select>
                    <Show
                        when=move || { errs().get("state").is_some() }
                        fallback=|| view! {}
                    >
                        <span class="error">{errs().get("state").unwrap()}</span>
                    </Show>
                </div>
                <div class="form_raw form_submit">
                    <input type="submit" value="Update" />
                </div>
            </ActionForm>
        </div>
    }
}

fn validate_form(
    ev: SubmitEvent,
    res: Result<(), ValidationErrors>,
    set_errors: WriteSignal<HashMap<String, String>>,
) {
    let mut errors = HashMap::new();
    match res {
        Ok(_) => {
            logging::log!("Create form validated successfully");
        }
        Err(validation_errs) => {
            for (field, error_list) in validation_errs.field_errors() {
                for error in error_list {
                    let mess = error.message.clone();
                    if let Some(mess) = mess {
                        errors.insert(field.to_string(), mess.to_string());
                    }
                }
            }
            ev.prevent_default();
        }
    }
    set_errors(errors);
}
