use leptos::*;
use crate::auth::auth_api::show_auth_num;

#[component]
pub fn AuthUi() -> impl IntoView {
    // our source signal: some synchronous, local state
    let (count, set_count) = create_signal(0);

    // our resource
    let async_data = create_resource(
        count,
        // every time `count` changes, this will run
        |value| async move {
            if value == 0 {
                return;
            }
            logging::log!("loading data from API");
            show_auth_num().await.unwrap();
            // load_data(value).await
        },
    );

    view! {
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count(3);
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            {move || count()}
        </button>
    }
}
