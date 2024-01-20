// Copyright 2023, Alan Sparrow
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at
// your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//
use crate::ExtraCategory;
use yew::{function_component, html, use_state, AttrValue, Callback, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub names: Vec<AttrValue>,
    pub categories: Vec<ExtraCategory>,
    pub on_clear: Callback<ExtraCategory>,
}

#[function_component(ExtraTab)]
pub fn extra_tab(props: &Props) -> Html {
    let active_panel = use_state(|| 0);

    let onclick = {
        let active_panel = active_panel.clone();
        move |value| active_panel.set(value)
    };

    let onclear = {
        let category = props.categories[*active_panel];
        let onclear = props.on_clear.clone();
        move |_| onclear.emit(category)
    };

    let iter = props.names.iter().zip(props.children.iter()).enumerate();
    let panels = iter
        .map(|(n, (name, child))| {
            let onclick = onclick.clone();
            html! {
                <div class="card block">
                  <header class="card-header is-clickable" onclick={move |_| onclick(n)}>
                    <p class="card-header-title">{ name }</p>
                    {
                      if n == *active_panel {
                        html! {
                          <div class="card-header-icon">
                            <input class="button is-info is-light is-small ml-2" type="button" onclick={onclear.clone()} value="Clear" />
                          </div>
                        }
                      } else {
                          html! ()
                      }
                    }
                  </header>

                  <div class="card-content" hidden={n != *active_panel}>
                    { child }
                  </div>
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div>
          { panels }
        </div>
    }
}
