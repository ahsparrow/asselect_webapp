use crate::state::{AirType, Format, Overlay, Settings};
use crate::AirspaceSetting;
use web_sys::HtmlInputElement;
use yew::{function_component, html, Callback, Event, Html, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub settings: Settings,
    pub callback: Callback<AirspaceSetting>,
}

#[function_component(OptionsTab)]
pub fn options_tab(props: &Props) -> Html {
    let onchange = props.callback.reform(|e: Event| {
        let name = e.target_unchecked_into::<HtmlInputElement>().name();
        let value = e.target_unchecked_into::<HtmlInputElement>().value();

        AirspaceSetting { name, value }
    });

    let set = &props.settings;

    html! {
        <div class="box">
          <div class="columns">
            <div class="column is-one-third">
              <div class="field">
                <label class="label">
                  {"Format"}
                  <div class="control">
                    <div class="select is-fullwidth">
                      <select name="format" onchange={onchange.clone()}>
                        <option value="openair" selected={set.format == Format::OpenAir}>{"OpenAir"}</option>
                        <option value="ratonly" selected={set.format == Format::RatOnly}>{"RA(T) only"}</option>
                        <option value="competition" selected={set.format == Format::Competition}>{"Competition"}</option>
                      </select>
                    </div>
                  </div>
                </label>
              </div>
            </div>

            <div class="column is-one-third">
              <div class="field">
                <label class="label">
                  {"Maximum Level"}
                  <div class="control">
                    <div class="select is-fullwidth">
                      <select name="max_level" onchange={onchange.clone()}>
                        <option value="600" selected={set.max_level == 600}>{"Unlimited"}</option>
                        <option value="195" selected={set.max_level == 195}>{"FL195"}</option>
                        <option value="125" selected={set.max_level == 125}>{"FL125"}</option>
                        <option value="105" selected={set.max_level == 105}>{"FL105"}</option>
                        <option value="65" selected={set.max_level == 65}>{"FL65"}</option>
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
                  {"HIRTA/GVS"}
                  <div class="control">
                    <div class="select is-fullwidth">
                      <select name="hirta_gvs" onchange={onchange.clone()}>
                        <option value="exclude" selected={set.hirta_gvs.is_none()}>{"No"}</option>
                        <option value="danger" selected={set.hirta_gvs == Some(AirType::Danger)}>{"Danger"}</option>
                        <option value="restricted" selected={set.hirta_gvs == Some(AirType::Restricted)}>{"Restricted"}</option>
                      </select>
                    </div>
                  </div>
                </label>
              </div>
            </div>

            <div class="column is-one-third">
              <div class="field">
                <label class="label">
                  {"Obstacle"}
                  <div class="control">
                    <div class="select is-fullwidth">
                      <select name="obstacle" onchange={onchange.clone()}>
                        <option value="exclude" selected={set.obstacle.is_none()}>{"No"}</option>
                        <option value="danger" selected={set.obstacle == Some(AirType::Danger)}>{"Danger"}</option>
                        <option value="classf" selected={set.obstacle == Some(AirType::ClassF)}>{"Class F"}</option>
                        <option value="classg" selected={set.obstacle == Some(AirType::ClassG)}>{"Class G"}</option>
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
                  {"Radio Frequency"}
                  <div class="control">
                    <div class="select is-fullwidth">
                      <select name="radio" onchange={onchange.clone()}>
                        <option value="no" selected={!set.radio}>{"No"}</option>
                        <option value="yes" selected={set.radio}>{"Append to name"}</option>
                      </select>
                    </div>
                  </div>
                </label>
              </div>
            </div>

            <div class="column is-one-third">
              <div class="field">
                <label class="label">
                  {"Altitude Overlay"}
                  <div class="control">
                    <div class="select is-fullwidth">
                      <select name="overlay" onchange={onchange.clone()}>
                        <option value="no" selected={set.overlay.is_none()}>{"No"}</option>
                        <option value="fl105" selected={set.overlay == Some(Overlay::FL105)}>{"Bases to FL105"}</option>
                        <option value="fl65" selected={set.overlay == Some(Overlay::FL65)}>{"Bases to FL65"}</option>
                        <option value="atzdz" selected={set.overlay == Some(Overlay::AtzDz)}>{"Bases to FL105 and ATZ/DZ"}</option>
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
