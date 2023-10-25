use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use rig_info::RigInfoClient;
use std::process::Command;
use tarpc::{client, context, tokio_serde::formats::Json};

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
    let server_act = create_action(|_| get_ipconfig());
    //let server_rig_ip = create_action(|_| get_ifconfig());
    let values = create_memo(move |_| {
        let v = server_act.value().get();
        v.map(|rv| {
            rv.map(|v| {
                v.iter()
                    .map(|s| view! {<span><br/><code>{s}</code></span> })
                    .collect_view()
            })
        })
    });

    view! {
        <h1>"ipconfig"</h1>
        <button on:click=move |_| server_act.dispatch(())>"get_ipconfig: "</button>
        <button on:click=move |_| server_act.dispatch(())>"get_ipconfig: "</button>
        <div>
            {values}
        </div>
    }
}

#[server]
pub async fn get_ipconfig() -> Result<Vec<String>, ServerFnError> {
    let out = Command::new("ipconfig").output().unwrap().stdout;
    let out = String::from_utf8(out).unwrap();
    let out = out.split('\n').map(|s| s.to_owned()).collect::<Vec<_>>();
    Ok(out)
}

#[server]
pub async fn get_ifconfig(rig_ip: String) -> Result<String, ServerFnError> {
    let mut transport = tarpc::serde_transport::tcp::connect((rig_ip, 8001), Json::default);
    transport.config_mut().max_frame_length(usize::MAX);
    let client = RigInfoClient::new(client::Config::default(), transport.await.unwrap()).spawn();
    Ok(client.ip(context::current()).await.unwrap())
}
