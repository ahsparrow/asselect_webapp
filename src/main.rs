use futures::try_join;
use gloo::file::{Blob, ObjectUrl};
use gloo::net::{http::Request, Error};
use gloo::storage::{LocalStorage, Storage};
use yew::{
    classes, function_component, html, use_effect_with, use_node_ref, use_reducer, use_state,
    AttrValue, Callback, Html,
};

use convert::openair;
use components::{
    about_tab::AboutTab, airspace_tab::AirspaceTab, extra_panel::ExtraPanel,
    extra_tab::ExtraTab, notam_tab::NotamTab, options_tab::OptionsTab, tabs::Tabs,
};
use state::{Action, State};
use yaixm::{gliding_sites, loa_names, rat_names, wave_names, Yaixm};

mod convert;
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

#[derive(Default)]
pub struct Overlay {
    pub overlay_105: String,
    pub overlay_195: String,
    pub overlay_atzdz: String,
}

#[function_component]
fn App() -> Html {
    // Airspace data
    let yaixm = use_state(|| None);

    // Overlay data
    let overlay = use_state(|| Overlay::default());

    // User interface settings
    let state = use_reducer(|| State {
        settings: LocalStorage::get("settings").unwrap_or_default(),
    });

    // Release modal control
    let show_release = use_state(|| false);

    // Reference for download anchor element
    let anchor_node_ref = use_node_ref();

    // Fetch YAIXM and overlay data
    {
        let yaixm = yaixm.clone();
        let overlay = overlay.clone();

        // use_effect_with((), ...) triggers only on first render of component
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // Get YAIXM data (and trigger page render)
                yaixm.set(fetch_yaixm().await.ok());

                // Get overlay data
                let overlay_105 = fetch_overlay("overlay_105.txt");
                let overlay_195 = fetch_overlay("overlay_195.txt");
                let overlay_atzdz = fetch_overlay("overlay_atzdz.txt");

                let response = try_join!(overlay_105, overlay_195, overlay_atzdz);
                if let Ok((overlay_105, overlay_195, overlay_atzdz)) = response {
                    overlay.set(Overlay {
                        overlay_105,
                        overlay_195,
                        overlay_atzdz,
                    });
                };
            });
            || ()
        });
    }

    // Save airspace callback
    let onsave = {
        let yaixm = yaixm.clone();
        let state = state.clone();
        let anchor_node_ref = anchor_node_ref.clone();

        Callback::from(move |_| {
            // Save settings in local storage
            LocalStorage::set("settings", &state.settings).ok();

            // Create OpenAir data
            let oa = openair(yaixm.as_ref().unwrap(), &state.settings);
            let blob = Blob::new(oa.as_str());
            let object_url = ObjectUrl::from(blob);

            // Trigger a "fake" download
            let anchor_node_ref = anchor_node_ref.cast::<web_sys::HtmlAnchorElement>();
            if let Some(anchor_node_ref) = anchor_node_ref {
                anchor_node_ref.set_href(&object_url);
                anchor_node_ref.click();
            }
        })
    };

    // Release modal callbacks
    let onshow_release = {
        let show_release = show_release.clone();
        Callback::from(move |_| {
            show_release.set(true);
        })
    };

    let onhide_release = {
        let show_release = show_release.clone();
        Callback::from(move |_| {
            show_release.set(false);
        })
    };

    // General settings callback
    let onairspace_set = {
        let state = state.clone();
        Callback::from(move |setting: AirspaceSetting| {
            state.dispatch(Action::Set {
                name: setting.name,
                value: setting.value,
            })
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

    // HTML rendering
    match yaixm.as_ref() {
        // Render full interface if YAIXM data is available
        Some(yaixm) => {
            let airac_date = &yaixm.release.airac_date[..10];
            let release_note = &yaixm.release.note;

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
                AttrValue::Static("Temporary Restrictions"),
                AttrValue::Static("Local Agreements"),
                AttrValue::Static("Wave Boxes"),
            ];

            let tab_names = vec![
                AttrValue::Static("Main"),
                AttrValue::Static("Option"),
                AttrValue::Static("Extra"),
                AttrValue::Static("NOTAM"),
                AttrValue::Static("About"),
            ];

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
                    <NotamTab />
                    <AboutTab />
                  </Tabs>
                </div>

                <div class="container block">
                  <div class="mx-4">
                    <button class="button is-primary" onclick={onsave}>
                      {"Get Airspace"}
                    </button>
                    <a id="airac-button" class="button is-text is-pulled-right" onclick={onshow_release}>
                    {"AIRAC: "}{ airac_date }
                    </a>
                  </div>
                </div>

                <div class={classes!("modal", show_release.then(|| Some("is-active")))}>
                  <div class="modal-background"></div>
                  <div class="modal-content">
                    <div class="box">
                      <h2 class="subtitle">{"Release Details"}</h2>
                      <pre>{ release_note }</pre>
                    </div>
                  </div>
                  <button id="modal-close" class="modal-close is-large" onclick={onhide_release.clone()}></button>
                </div>

                <a ref={anchor_node_ref} id="download" hidden=true download="openair.txt"></a>
                </>
            }
        }

        None => {
            html! {
                { "Loading YAIXM..." }
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

// Get overlay data from server
async fn fetch_overlay(path: &str) -> Result<String, Error> {
    let result = Request::get(path).send().await;
    match result {
        Ok(response) => {
            if response.ok() {
                response.text().await
            } else {
                Ok("* Missing overlay".to_string())
            }
        },
        Err(e) => Err(e),
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
