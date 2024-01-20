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

#[function_component(AboutTab)]
pub fn about_tab() -> Html {
    html! {
        <div class="content">
          <h3 class="subtitle">{ "Output Format" }</h3>
            <p>{ "ASSelect generates airspace data for the mainland UK in OpenAir format.
              \"Competition\" format is intended for competition organiser use only." }</p>
          <h3 class="subtitle">{ "Airspace Types" }</h3>
            <p><em>{ "Non-ATZ Airfields" }</em>{ " are busy training aerodromes (without an ATZ) as listed
              in section ENR\u{00a0}5.5 of the AIP. Note, the AIP does not include the majority
              of smaller airfields shown on the 1:500000 chart and they are not
              included by ASSelect." }</p>
            <p><em>{ "Gliding Airfields" }</em>{ " are listed in ENR\u{00a0}5.5. ASSelect plots
              them with a radius of 1\u{00a0}nm.  To avoid unnecessary airspace warnings
              ASSelect can exclude your home site." }</p>
            <p><em>{ "Microlight Airfields" }</em>{ " are listed in ENR\u{00a0}5.5 and
              plotted with a radius of 0.5\u{00a0}nm." }</p>
            <p><em>{ "Obstacles" }</em>{ " are listed in ENR\u{00a0}5.4. ASSelect includes only
              obstacles with a height of greater than 600\u{00a0}ft (mainly radio masts)." }</p>
            <p><em>{ "ILS Feathers" }</em>{ " are not strictly an airspace type, but the
              BGA recommend a radio call if you fly in their vicinity. Note the feather
              is symbolic only - the actual instrument approach procedure will extend to the
              side of the feather." }</p>
          <h3 class="subtitle">{ "Local Agreements" }</h3>
            <p>{ "BGA letters of agreement are listed on the " }
              <a class="text-primary" href="https://members.gliding.co.uk/library/loas/">{ "BGA website" }</a>
              { ". With the exception of the Cambridge RAZ a local club briefing is
              required before using any local agreement." }</p>
          <h3 class="subtitle">{ "Radio Frequencies" }</h3>
            <p>{ "Radio frequencies are stored using the OpenAir \"AF\" record type. In addition ASSelect
              can add them to the airspace name. "}</p>
          <h3 class="subtitle">{ "Temporary Restrictions, RA(T)" }</h3>
            <p>{ "RA(T)s are detailed in Mauve AICs on the "}
              <a class="text-primary"
                 href="https://nats-uk.ead-it.com/cms-nats/opencms/en/Publications/Aeronautical-Information-Circulars-AICs/mauve-aics/">
                 { "NATS AIS website" }</a>
              {". RA(T)s can be downloaded separately from the rest of the airspace." }</p>
          <h3 class="subtitle">{ "Altitude Overlay" }</h3>
            <p>{ "ASSelect can generate a graphical overlay showing the base of controlled airspace.
              The overlay is encoded as Class B airspace." }</p>
          <h3 class="subtitle">{ "Data" }</h3>
            <p>{ "Airspace data is updated every four weeks - see " }
              <a class="text-primary"
                href="https://nats-uk.ead-it.com/cms-nats/export/sites/default/en/Publications/publication-schedule/10-year-AIRAC.pdf">
                {" Schedule "}</a>
              { ". This site is normally updated approximately two weeks in advance of
              the effective date" }</p>
            <p>{ "You can download the ASSelect airspace database from "}
              <a class="text-primary" href="https://github.com/ahsparrow/airspace">{ "GitHub" }</a></p>
          <h3 class="subtitle">{ "Contact" }</h3>
            <address>{ "Comments, corrections and complaints to: " }
              <a href="mailto:web@asselect.uk">{ "Alan Sparrow" }</a>
            </address>
        </div>
    }
}
