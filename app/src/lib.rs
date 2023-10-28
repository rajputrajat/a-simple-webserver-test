use cfg_if::cfg_if;
use leptos::{html::Input, *};
use leptos_meta::*;
use leptos_router::*;
use std::process::Command;
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
    let server_rig_ip = create_action(|ip: &String| get_ifconfig(ip.clone()));
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

    let input_element: NodeRef<Input> = create_node_ref();
    let on_submit_ip = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element.get().expect("<input> to exist").value();
        println!("{value}");
        server_rig_ip.dispatch(value);
    };

    view! {
        <h1>"ipconfig"</h1>
        <button on:click=move |_| server_act.dispatch(())>"get_ipconfig: "</button>
        <form on:submit=on_submit_ip>
            <input type="text" node_ref=input_element />
            <input type="submit" value="Submit"/>
        </form>
        <div>
            {values}
            <p>"ipinfo of the rig is: " {server_rig_ip.value()}</p>
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
    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            //println!("from wasm");
            Ok("from wasm".to_owned())
        }else {
            use rig_info::RigInfoClient;
            use tarpc::{client, context, tokio_serde::formats::Json};

            println!("{rig_ip}");
            let mut transport = tarpc::serde_transport::tcp::connect((rig_ip, 8001), Json::default);
            transport.config_mut().max_frame_length(usize::MAX);
            match transport.await {
                Ok(trnsprt) => {
                    let client = RigInfoClient::new(client::Config::default(), trnsprt).spawn();
                    match client.ip(context::current()).await {
                        Ok(ip) => return Ok("ip".to_owned()),
                        Err(_e) => {}
                    }
                }
                Err(_e) => {}
            }
        Ok("failed".to_owned())
        }
    }
}
