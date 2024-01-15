use gloo_net::http::Request;
use gloo_net::Error;
use yew::{function_component, html, use_effect_with, use_state, Html};

use yaixm::Yaixm;

mod yaixm;

#[function_component]
fn App() -> Html {
    let yaixm = use_state(|| None);

    // Fetch YAIXM data
    {
        let yaixm = yaixm.clone();

        // use_effect_with((), ...) triggers only on first render of component
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let data = fetch_yaixm().await;
                yaixm.set(data.ok());
            });
            || ()
        });
    }

    // HTML rendering
    match yaixm.as_ref() {
        // Render full interface if YAIXM data is available
        Some(yaixm) => {
            html! {
                yaixm.release.airac_date.as_str()
            }
        }

        None => {
            html! {
                { "No YAIXM" }
            }
        }
    }
}

// Get YAIXM data from server
async fn fetch_yaixm() -> Result<Yaixm, Error> {
    let result = Request::get("yaixm.json").send().await;
    match result {
        Ok(response) => response.json().await,
        Err(e) => Err(e),
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
