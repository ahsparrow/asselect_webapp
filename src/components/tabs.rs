use web_sys::HtmlElement;
use yew::{
    classes, function_component, html, use_state, Callback, Children, Html, MouseEvent, Properties,
    TargetCast,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tab_names: Vec<String>,
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
