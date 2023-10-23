use std::process::Command;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_clik = move |_| set_count.update(|count| *count += 1);

    let server_act = create_action(|_| get_ipconfig());

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_clik>"Click Me: " {count}</button>
        <button on:click=move |_| server_act.dispatch(())>"get_ipconfig: "</button>
        <span>{server_act.value()}</span>
    }
}

#[server]
pub async fn get_ipconfig() -> Result<String, ServerFnError> {
    let out = Command::new("ipconfig").output().unwrap().stdout;
    Ok(String::from_utf8(out).unwrap())
}
