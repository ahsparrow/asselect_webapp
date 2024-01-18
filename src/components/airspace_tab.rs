use crate::state::{AirType, Settings};
use crate::AirspaceSetting;
use web_sys::HtmlInputElement;
use yew::{function_component, html, Callback, Event, Html, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub settings: Settings,
    pub gliding_sites: Vec<String>,
    pub callback: Callback<AirspaceSetting>,
}

#[function_component(AirspaceTab)]
pub fn airspace_tab(props: &Props) -> Html {
    let onchange = props.callback.reform(|e: Event| {
        let element = e.target_unchecked_into::<HtmlInputElement>();
        let name = element.name();
        let value = element.value();

        AirspaceSetting { name, value }
    });

    let set = &props.settings;

    let gliding_sites = || {
        props
            .gliding_sites
            .iter()
            .map(|name| {
                html! {
                    <option selected={Some(name) == set.home.as_ref()} >{name}</option>
                }
            })
            .collect::<Html>()
    };

    html! {
        <div class="box">
          <div class="columns">
            <div class="column is-one-third">
              <div class="field">
                <label class="label">
                  {"ATZ"}
                  <div class="control">
                    <div class="select is-fullwidth">
                      <select name="atz" onchange={onchange.clone()}>
                        <option value="classd" selected={set.atz == AirType::ClassD}>{"Class D"}</option>
                        <option value="ctr" selected={set.atz == AirType::Ctr}>{"Control Zone"}</option>
                      </select>
                    </div>
                  </div>
                </label>
              </div>
            </div>

            <div class="column is-one-third">
              <div class="field">
                <label class="label">
                  {"ILS Feather"}
                  <div class="control">
                    <div class="select is-fullwidth">
                      <select name="ils" onchange={onchange.clone()}>
                        <option value="atz" selected={set.ils.is_none()}>{"As ATZ"}</option>
                        <option value="classf" selected={set.ils == Some(AirType::ClassF)}>{"Class F"}</option>
                        <option value="classg" selected={set.ils == Some(AirType::ClassG)}>{"Class G"}</option>
                      </select>
                    </div>
                  </div>
                </label>
              </div>
            </div>
          </div>

          <div class="columns">
            <div class="column is-one-third">
              <div class="field">
                <label class="label">
                  {"Non-ATZ Airfield"}
                  <div class="control">
                    <div class="select is-fullwidth">
                      <select name="unlicensed" onchange={onchange.clone()}>
                        <option value="exclude" selected={set.unlicensed.is_none()}>{"No"}</option>
                        <option value="classf" selected={set.unlicensed == Some(AirType::ClassF)}>{"Class F"}</option>
                        <option value="classg" selected={set.unlicensed == Some(AirType::ClassG)}>{"Class G"}</option>
                      </select>
                    </div>
                  </div>
                </label>
              </div>
            </div>

            <div class="column is-one-third">
              <div class="field">
                <label class="label">
                  {"Microlight Airfield"}
                  <div class="control">
                    <div class="select is-fullwidth">
                      <select name="microlight" onchange={onchange.clone()}>
                        <option value="exclude" selected={set.microlight.is_none()}>{"No"}</option>
                        <option value="classf" selected={set.microlight == Some(AirType::ClassF)}>{"Class F"}</option>
                        <option value="classg" selected={set.microlight == Some(AirType::ClassG)}>{"Class G"}</option>
                      </select>
                    </div>
                  </div>
                </label>
              </div>
            </div>
          </div>

          <div class="columns">
            <div class="column is-one-third">
              <div class="field">
                <label class="label">
                  {"Gliding Airfield"}
                  <div class="control">
                    <div class="select is-fullwidth">
                      <select name="gliding" onchange={onchange.clone()}>
                        <option value="exclude" selected={set.gliding.is_none()}>{"No"}</option>
                        <option value="gsec" selected={set.gliding == Some(AirType::Gliding)}>{"Gliding Sector"}</option>
                        <option value="classf" selected={set.gliding == Some(AirType::ClassF)}>{"Class F"}</option>
                        <option value="classg" selected={set.gliding == Some(AirType::ClassG)}>{"Class G"}</option>
                      </select>
                    </div>
                  </div>
                </label>
              </div>
            </div>

            <div class="column is-one-third">
              <div class="field">
                <label class="label">
                  {"Exclude Home Airfield"}
                  <div class="control">
                    <div class="select is-fullwidth">
                      <select name="home" onchange={onchange.clone()}>
                        <option selected={set.home.is_none()}>{"No"}</option>
                        { gliding_sites() }
                      </select>
                    </div>
                  </div>
                </label>
              </div>
            </div>
          </div>
        </div>
    }
}
