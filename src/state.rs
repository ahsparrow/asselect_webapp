use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::rc::Rc;
use yew::Reducible;

// Airspace types
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AirType {
    ClassA,
    ClassB,
    ClassC,
    ClassD,
    ClassE,
    ClassF,
    ClassG,
    Danger,
    Cta,
    Ctr,
    Gliding,
    Matz,
    Other,
    Prohibited,
    Restricted,
    Rmz,
    Tmz,
}

// Output format
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Format {
    OpenAir,
    RatOnly,
    Competition,
}

// Altutude layer overlay
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Overlay {
    None,
    FL105,
    FL65,
    AtzDz,
}

// Settings
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Settings {
    pub atz: AirType,
    pub ils: Option<AirType>,
    pub unlicensed: Option<AirType>,
    pub microlight: Option<AirType>,
    pub gliding: Option<AirType>,
    pub home: Option<String>,
    pub hirta_gvs: Option<AirType>,
    pub obstacle: bool,
    pub max_level: u16,
    pub radio: bool,
    pub format: Format,
    pub overlay: Overlay,
    #[serde(default)]
    pub loa: HashSet<String>,
    #[serde(default)]
    pub rat: HashSet<String>,
    #[serde(default)]
    pub wave: HashSet<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            atz: AirType::Ctr,
            ils: None,
            unlicensed: None,
            microlight: None,
            gliding: None,
            home: None,
            hirta_gvs: None,
            obstacle: false,
            max_level: 660,
            radio: false,
            format: Format::OpenAir,
            overlay: Overlay::None,
            loa: HashSet::new(),
            rat: HashSet::new(),
            wave: HashSet::new(),
        }
    }
}

// Application state
#[derive(Debug, Default, PartialEq)]
pub struct State {
    pub settings: Settings,
}

// State actions
pub enum Action {
    Set { name: String, value: String },
    SetLoa { name: String, checked: bool },
    SetRat { name: String, checked: bool },
    SetWave { name: String, checked: bool },
    ClearLoa,
    ClearRat,
    ClearWave,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut set = self.settings.clone();
        match action {
            // Set airspace option
            Action::Set { name, value } => {
                match name.as_str() {
                    "atz" => set.atz = get_airtype(&value).unwrap_or(AirType::Ctr),
                    "unlicensed" => set.unlicensed = get_airtype(&value),
                    "microlight" => set.microlight = get_airtype(&value),
                    "gliding" => set.gliding = get_airtype(&value),
                    "hirta_gvs" => set.hirta_gvs = get_airtype(&value),
                    "obstacle" => set.obstacle = value == "include",
                    "max_level" => set.max_level = value.parse::<u16>().unwrap(),
                    "radio" => set.radio = value == "yes",
                    "home" => set.home = if value == "None" { None } else { Some(value) },
                    "overlay" => {
                        set.overlay = match value.as_str() {
                            "fl105" => Overlay::FL105,
                            "fl65" => Overlay::FL65,
                            "atzdz" => Overlay::AtzDz,
                            _ => Overlay::None,
                        }
                    }
                    "format" => {
                        set.format = match value.as_str() {
                            "ratonly" => Format::RatOnly,
                            "competition" => Format::Competition,
                            _ => Format::OpenAir,
                        }
                    }
                    _ => (),
                };
            }
            // Include/exclude LOA
            Action::SetLoa { name, checked } => {
                if checked {
                    set.loa.replace(name);
                } else {
                    set.loa.remove(&name);
                }
            }
            // Include/exclude RAT
            Action::SetRat { name, checked } => {
                if checked {
                    set.rat.replace(name);
                } else {
                    set.rat.remove(&name);
                }
            }
            // Include/exclude wave box
            Action::SetWave { name, checked } => {
                if checked {
                    set.wave.replace(name);
                } else {
                    set.wave.remove(&name);
                }
            }
            // Clear all LOAs
            Action::ClearLoa => set.loa.clear(),
            // Clear all RATs
            Action::ClearRat => set.rat.clear(),
            // Clear all Wave boxes
            Action::ClearWave => set.wave.clear(),
        }
        Self { settings: set }.into()
    }
}

// Default mapping value to airspace type
fn get_airtype(value: &str) -> Option<AirType> {
    match value {
        "classd" => Some(AirType::ClassD),
        "classf" => Some(AirType::ClassF),
        "classg" => Some(AirType::ClassG),
        "ctr" => Some(AirType::Ctr),
        "danger" => Some(AirType::Danger),
        "restricted" => Some(AirType::Restricted),
        "gsec" => Some(AirType::Gliding),
        _ => None,
    }
}
