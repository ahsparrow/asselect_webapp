use yew::{function_component, html, Html};

#[function_component(NotamTab)]
pub fn notam_tab() -> Html {
    html! {
        <div>
          <div class="subtitle">
            {"Navigation Warnings"}
          </div>
          <div class="block">
            {"The PDFs below show a summary of the navigation warning NOTAMs relevant to cross country gliding. The PDFs are refreshed during the day at approximately ten minutes past the hour."}
          </div>
          <div class="block ml-4">
            <a href="https://navplot.asselect.uk/today_south.pdf" download={""}>
              {"Download Today (England/Wales) PDF"}
            </a>
          </div>
          <div class="block ml-4">
            <a href="https://navplot.asselect.uk/today_north.pdf" download={""}>
              {"Download Today (North England/Scotland) PDF"}
            </a>
          </div>
          <div class="block ml-4">
            <a href="https://navplot.asselect.uk/tomorrow_south.pdf" download={""}>
              {"Download Tomorrow (England/Wales) PDF"}
            </a>
          </div>
          <div class="block ml-4">
            <a href="https://navplot.asselect.uk/tomorrow_north.pdf" download={""}>
              {"Download Tomorrow (North England/Scotland) PDF"}
            </a>
          </div>
        </div>
    }
}
