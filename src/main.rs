use leptos::*;
mod auth;

use auth::auth_ui::AuthUi;

fn main() {
    leptos::mount_to_body(|| view! { <AuthUi/> })
}
