use crate::{ExtraCategory, ExtraSetting};
use std::collections::HashSet;
use web_sys::HtmlInputElement;
use yew::{function_component, html, Callback, Event, Html, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub category: ExtraCategory,
    pub names: Vec<String>,
    pub selected: HashSet<String>,
    pub callback: Callback<ExtraSetting>,
}

#[function_component(ExtraPanel)]
pub fn extra_panel(props: &Props) -> Html {
    let category = props.category;
    let onchange = props.callback.reform(move |e: Event| {
        let name = e.target_unchecked_into::<HtmlInputElement>().name();
        let checked = e.target_unchecked_into::<HtmlInputElement>().checked();

        ExtraSetting {
            category,
            name,
            checked,
        }
    });

    html! {
        <div class="columns is-multiline">
        {
            props.names.iter().map(|name| {
                let checked = props.selected.contains(name);
                html!(
                      <div class="column is-one-third">
                        <div class="field">
                        <label class="checkbox">
                          <input type="checkbox" class="mr-2" {checked} name={name.clone()} onchange={onchange.clone()}/>
                          {name}
                        </label>
                        </div>
                      </div>
                    )
            }).collect::<Html>()
        }
        </div>
    }
}
