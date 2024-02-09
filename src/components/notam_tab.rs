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
use yew::{function_component, html, Html};

#[function_component(NotamTab)]
pub fn notam_tab() -> Html {
    html! {
        <div>
          <div class="subtitle">
            {"Navigation Warnings"}
          </div>
          <div class="block">
            {"The PDFs below show a summary of the navigation warning NOTAMs relevant to cross country gliding. The PDFs are refreshed during the day at approximately ten minutes to the hour."}
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
