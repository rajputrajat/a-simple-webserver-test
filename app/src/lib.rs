use leptos::{html::Input, *};
use leptos_meta::*;
use leptos_router::*;
// use rig_info::RigInfoClient;
use std::process::Command;
//use tarpc::{client, context, tokio_serde::formats::Json};
use web_sys::SubmitEvent;

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

    // let (signal_linux_ip, set_signal_linux_ip) = create_signal("set ip".to_owned());
    // let input_element: NodeRef<Input> = create_node_ref();
    // let on_submit_ip = move |ev: SubmitEvent| {
    //     ev.prevent_default();
    //     let value = input_element().expect("<input> to exist").value();
    //     set_signal_linux_ip(value);
    // };

    view! {
        <h1>"ipconfig"</h1>
        <button on:click=move |_| server_act.dispatch(())>"get_ipconfig: "</button>
        // <form on:submit=on_submit_ip>
        //     <input type="text" value=signal_linux_ip node_ref=input_element />
        //     <input type="submit" value="Submit"/>
        // </form>
        <div>
            {values}
            // <p>"ipinfo of the rig is: " {signal_linux_ip}</p>
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

// #[server]
// pub async fn get_ifconfig(rig_ip: String) -> Result<String, ServerFnError> {
//     let mut transport = tarpc::serde_transport::tcp::connect((rig_ip, 8001), Json::default);
//     transport.config_mut().max_frame_length(usize::MAX);
//     let client = RigInfoClient::new(client::Config::default(), transport.await.unwrap()).spawn();
//     Ok(client.ip(context::current()).await.unwrap())
// }
