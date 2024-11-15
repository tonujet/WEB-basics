use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    let html = "<p>This HTML will be injected.</p>";
    let values = vec![0, 1, 2];

    // create a list of 5 signals
    let length = 5;
    let counters = (1..=length).map(|idx| create_signal(idx));

    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button on:click=move |_| set_count.update(|n| *n += 1)>{count}</button>
                </li>
            }
        })
        .collect_view();

    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);

    let (name, set_name) = create_signal("Test".to_string());

    let (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;


    let (first, set_first) = create_signal("First".to_string());
    let (last, set_last) = create_signal("Last".to_string());
    let (use_last, set_use_last) = create_signal(true);

    let stop = watch(
        move || use_last.get(),
        move |num, prev_num, _| {
            logging::log!("Number: {}; Prev: {:?}", num, prev_num);
        },
        false,
    );

    // this will add the name to the log
    // any time one of the source signals changes
    create_effect(move |_| {
        logging::log!("{}",
            if use_last() {
                format!("{} {}", first(), last())
            } else {
                first()
            },
        )
    });

    view! {
        <button on:click=move |_| {
            set_use_last.update(|v| *v = !*v)
        }>Check console</button>
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count.update(|n| *n += 10);
            }
            class:red=move || count() % 2 == 1
            // set the `style` attribute
            style="position: absolute"
            // and toggle individual CSS properties with `style:`
            style:left=move || format!("{}px", count() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", count(), 100)
            style:max-width="400px"
                // Set a CSS variable for stylesheet use
            style=("--columns", count)
        >
        "Click me: "
            // on stable, this is move || count.get();
            {count}
        </button>
        <p>
            {double_count}
        </p>

        <div inner_html=html/>
        <ProgressBar progress=count/>
        <ProgressBar progress=Signal::derive(double_count)/>

        <p>{values.clone()}</p>
        <ul>
            {values.into_iter()
                .map(|n| view! { <li>{n}</li>})
                .collect::<Vec<_>>()}
        </ul>

        <ul>{counter_buttons}</ul>
        <DynamicList initial_length=10/>

        <button on:click=move |_| {
            set_data.update(|data| {
                for row in data {
                    row.value *= 2;
                }
            });
            // log the new value of the signal
            logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>

        <div>
            <For
                each=move || data().into_iter().enumerate()
                key=|(_, state)| state.key.clone()
                children=move |(index, _)| {
                    let value = create_memo(move |_| {
                    data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
                    });
                    view! {
                    <p>{value}</p>
                    }
                }
            />
        </div>
        <input type="text"
            on:input=move |e| {
                set_name(event_target_value(&e))
            }
            prop:value=name
        />

        <p>Name is {name} </p>

        <div>
        <button on:click=move |_| {
               set_value.update(|v| *v += 1)
        }>
        Increment value: {value}
        </button>
        <p>
        {move || if is_odd() {
            Some("Odd")
        } else {
            None
        }}
        </p>
        </div>
        <NumericInput/>
    }
}

#[component]
fn ProgressBar(
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! { <progress max=max value=progress /> }
}

#[component]
fn DynamicList(
    /// The number of counters to begin with.
    initial_length: usize,
) -> impl IntoView {
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        let sig = create_signal(next_counter_id + 1);
        set_counters.update(move |counters| counters.push((next_counter_id, sig)));
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>"Add Counter"</button>
            <ul>
                <For
                    each=counters
                    key=|counter| counter.0
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button on:click=move |_| {
                                    set_count.update(|n| *n += 1)
                                }>{count}</button>
                                <button on:click=move |_| {
                                    set_counters
                                        .update(|counters| {
                                            counters
                                                .retain(|(counter_id, (signal, _))| {
                                                    if counter_id == &id {
                                                        signal.dispose();
                                                    }
                                                    counter_id != &id
                                                })
                                        });
                                }>"Remove"</button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn test() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;

    view! {
        <main>
            {move || match is_odd() {
                true if value() == 1 => {
                    // returns HtmlElement<Pre>
                    view! { <pre>"One"</pre> }.into_any()
                },
                false if value() == 2 => {
                    // returns HtmlElement<P>
                    view! { <p>"Two"</p> }.into_any()
                }
                // returns HtmlElement<Textarea>
                _ => view! { <textarea>{value()}</textarea> }.into_any()
            }}
        </main>
    }
}


#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    // when input changes, try to parse a number from the input
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input=on_input/>
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors as strings, if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect_view()
                            }
                        </ul>
                    </div>
                }
            >
                <p>"You entered " <strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}
