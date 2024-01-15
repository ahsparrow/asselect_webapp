use gloo::console::log;
use gloo::net::{http::Request, Error};
use gloo::storage::{LocalStorage, Storage};
use wasm_bindgen::JsValue;
use yew::{function_component, html, use_effect_with, use_reducer, use_state, Callback, Html};

use components::{airspace_tab::AirspaceTab, tabs::Tabs};
use state::{Action, State};
use yaixm::{gliding_sites, Yaixm};

mod components;
mod state;
mod yaixm;

// Callback data structures
pub struct AirspaceSetting {
    pub name: String,
    pub value: String,
}

#[function_component]
fn App() -> Html {
    let yaixm = use_state(|| None);

    let state = use_reducer(|| State {
        settings: LocalStorage::get("settings").unwrap_or_default(),
    });

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

    let onsave = {
        let state = state.clone();
        Callback::from(move |_| {
            // Save settings in local storage
            LocalStorage::set("settings", &state.settings).ok();

            let a = state.settings.clone();
            let object = JsValue::from(format!("{:?}", a));
            log!(object);
        })
    };

    // Airspace settings callback
    let onairspace_set = {
        let state = state.clone();
        Callback::from(move |setting: AirspaceSetting| {
            state.dispatch(Action::Set {
                name: setting.name,
                value: setting.value,
            })
        })
    };

    // HTML rendering
    match yaixm.as_ref() {
        // Render full interface if YAIXM data is available
        Some(yaixm) => {
            let mut gliding_sites = gliding_sites(yaixm);
            gliding_sites.sort();

            let tab_names = vec!["Main".to_string()];

            html! {
                <>
                <header class="hero is-small is-primary block">
                  <div class="hero-body">
                    <div class="container">
                      <div class="title is-4">
                        {"ASSelect - UK Airspace"}
                      </div>
                    </div>
                  </div>
                </header>

                <div class="container block">
                  <Tabs {tab_names}>
                    <AirspaceTab settings={state.settings.clone()} {gliding_sites} callback={onairspace_set.clone()} />
                  </Tabs>
                </div>

                <div class="container block">
                  <div class="mx-4">
                    <button class="button is-primary" onclick={onsave}>
                      {"Save"}
                    </button>
                  </div>
                </div>
                </>
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
