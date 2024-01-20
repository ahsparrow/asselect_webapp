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
use web_sys::HtmlElement;
use yew::{
    classes, function_component, html, use_state, AttrValue, Callback, Children, Html, MouseEvent,
    Properties, TargetCast,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tab_names: Vec<AttrValue>,
    pub children: Children,
}

#[function_component(Tabs)]
pub fn tabs(props: &Props) -> Html {
    let tab = use_state(|| "tab-0".to_string());

    let onclick = {
        let tab = tab.clone();
        Callback::from(move |e: MouseEvent| {
            let id: String = e
                .target_unchecked_into::<HtmlElement>()
                .id()
                .parse()
                .unwrap();
            tab.set(id);
        })
    };

    let tab_id = |id: usize| format!("tab-{}", id);

    let tabs = || -> Html {
        props
            .tab_names
            .iter()
            .enumerate()
            .map(|(id, t)| {
                html! {
                    <li class={classes!((*tab == tab_id(id)).then_some("is-active"))}>
                      <a id={tab_id(id)}>
                        {t}
                      </a>
                    </li>
                }
            })
            .collect()
    };

    let panel = || -> Html {
        props
            .children
            .iter()
            .enumerate()
            .map(|(id, p)| {
                html! {
                    <div hidden={tab_id(id) != *tab}>
                      {p}
                    </div>
                }
            })
            .collect::<Html>()
    };

    html! {
        <>
        <nav class="tabs">
          <ul {onclick}>
            { tabs() }
          </ul>
        </nav>
        <div class="mx-4">
          { panel() }
        </div>
        </>
    }
}
