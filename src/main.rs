use gloo::console::log;
use gloo::net::{http::Request, Error};
use gloo::storage::{LocalStorage, Storage};
use wasm_bindgen::JsValue;
use yew::{function_component, html, use_effect_with, use_reducer, use_state, Callback, Html};

use components::{
    airspace_tab::AirspaceTab,
    extra_panel::ExtraPanel,
    extra_tab::ExtraTab,
    options_tab::OptionsTab,
    tabs::Tabs,
};
use state::{Action, State};
use yaixm::{gliding_sites, loa_names, rat_names, wave_names, Yaixm};

mod components;
mod state;
mod yaixm;

// Callback data structures
pub struct AirspaceSetting {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ExtraCategory {
    Rat,
    Loa,
    Wave,
}

pub struct ExtraSetting {
    pub category: ExtraCategory,
    pub name: String,
    pub checked: bool,
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

    // RAT/LOA/Wave setting callback
    let onextra_set = {
        let state = state.clone();
        Callback::from(move |setting: ExtraSetting| match setting.category {
            ExtraCategory::Rat => state.dispatch(Action::SetRat {
                name: setting.name,
                checked: setting.checked,
            }),
            ExtraCategory::Loa => state.dispatch(Action::SetLoa {
                name: setting.name,
                checked: setting.checked,
            }),
            ExtraCategory::Wave => state.dispatch(Action::SetWave {
                name: setting.name,
                checked: setting.checked,
            }),
        })
    };

    // RAT/LOA/Wave clear callback
    let onextra_clear = {
        let state = state.clone();
        Callback::from(move |category: ExtraCategory| match category {
            ExtraCategory::Rat => state.dispatch(Action::ClearRat),
            ExtraCategory::Loa => state.dispatch(Action::ClearLoa),
            ExtraCategory::Wave => state.dispatch(Action::ClearWave),
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

            let rat_selected = state.settings.rat.clone();
            let rat_names = rat_names(yaixm);

            let loa_selected = state.settings.loa.clone();
            let loa_names = loa_names(yaixm);

            let wave_selected = state.settings.wave.clone();
            let mut wave_names = wave_names(yaixm);
            wave_names.sort();

            let extra_names = vec![
                "Temporary Restrictions".to_string(),
                "Local Agreements".to_string(),
                "Wave Boxes".to_string(),
            ];

            let tab_names = vec!["Main".to_string(), "Option".to_string(), "Extra".to_string()];

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
                    <OptionsTab settings={state.settings.clone()} callback={onairspace_set.clone()} />
                    <ExtraTab names={extra_names} categories={vec![ExtraCategory::Rat, ExtraCategory::Loa, ExtraCategory::Wave]} on_clear={onextra_clear.clone()}>
                      <ExtraPanel category={ExtraCategory::Rat} names={rat_names} selected={rat_selected} callback={onextra_set.clone()}/>
                      <ExtraPanel category={ExtraCategory::Loa} names={loa_names} selected={loa_selected} callback={onextra_set.clone()}/>
                      <ExtraPanel category={ExtraCategory::Wave} names={wave_names} selected={wave_selected} callback={onextra_set.clone()}/>
                    </ExtraTab>
                  </Tabs>
                </div>

                <div class="container block">
                  <div class="mx-4">
                    <button class="button is-primary" onclick={onsave}>
                      {"Get Airspace"}
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
