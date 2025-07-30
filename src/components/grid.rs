use dioxus::prelude::*;
use rand::seq::{IndexedRandom, SliceRandom};

const RANDOMIZE_SVG: Asset = asset!("/assets/randomize.svg");
const TTKD_LOGO: Asset = asset!("/assets/ttkd_logo.jpeg");

static PATTERNS: &'static [&str; 24] = &[
    // 1-Kup
    "Chon-Ji",
    "Dan-Gun",
    "Do-San",
    "Won-Hyo",
    "Yul-Gok",
    "Choong-Gun",
    "Toi-Gye",
    "Hwa-Rang",
    "Choong-Moo",
    // 1-Dan
    "Kwan-Gae",
    "Po-Eun",
    "Ge-Baek",
    // 2-Dan
    "Eui-Am",
    "Choong-Jang",
    "Juche",
    // 3-Dan
    "Sam-Il",
    "Yoo-Sin",
    "Choi-Yong",
    // 4-Dan
    "Yon-Gae",
    "Ul-Ji",
    "Moon-Moo",
    // 5-Dan
    "So-San",
    "Se-Jong",
    // 6-Dan
    "Tong-Il",
];

#[component]
pub fn Grid() -> Element {
    rsx! {
        div { id: "container",
            span { id: "container-header", "Pattern randomizer" }
            div { id: "logo-container",
                img { id: "itf-logo", src: TTKD_LOGO }
            }
            BingoGrid {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct BeltChoice {
    choice: Signal<String>,
}

#[derive(Debug, Clone)]
pub struct Refresh {
    state: Signal<bool>,
}

#[component]
fn BingoGrid() -> Element {
    let mut belt_choice = use_context_provider(|| BeltChoice {
        choice: Signal::new("1-Kup".to_string()),
    });

    let mut refresh = use_context_provider(|| Refresh {
        state: Signal::new(false),
    });

    let mut grid_size = use_signal(|| "3x3".to_string());

    let mut pattern_grid = use_signal(|| PATTERNS[..9].to_vec());

    use_effect(move || {
        let choice = belt_choice.choice.read();
        let _ = refresh.state.read();

        let s = choice.as_str();
        let mut rng = rand::rng();

        let num_patterns: usize;

        match s {
            "1-Kup" => {
                num_patterns = 9;
            }
            "1-Dan" => {
                num_patterns = 12;
            }
            "2-Dan" => {
                num_patterns = 15;
            }
            "3-Dan" => {
                num_patterns = 18;
            }
            "4-Dan" => {
                num_patterns = 21;
            }
            "5-Dan" => {
                num_patterns = 23;
            }
            "6-Dan" => {
                num_patterns = 24;
            }

            _ => {
                panic!("Invalid belt choice")
            }
        }

        // First, extract available patterns in order.
        let mut available_patterns = PATTERNS[..num_patterns].to_vec();

        // Randomly shuffle.
        available_patterns.shuffle(&mut rng);

        // Then we need to extract only 3x3 or 4x4 patterns
        let grid_size_choice = grid_size.read();
        let gs_as_str = grid_size_choice.as_str();

        match gs_as_str {
            // For 3x3 we are safe since every belt has at least 9 patterns available.
            "3x3" => {
                pattern_grid.set(available_patterns[..9].to_vec());
            }
            // For all other grid sizes, we need to backfill.
            "4x4" => {
                //
                if available_patterns.len() < 16 {
                    // Backfill until we have the desired amount of patterns.
                    let mut secondary_set = available_patterns.clone();
                    secondary_set.shuffle(&mut rng);

                    // This should be safe regardless of grid size.
                    while available_patterns.len() < 16 {
                        available_patterns.push(secondary_set.choose(&mut rng).unwrap());
                    }
                }

                pattern_grid.set(available_patterns[..16].to_vec());
            }
            "5x5" => {
                //
                if available_patterns.len() < 25 {
                    // Backfill until we have the desired amount of patterns.
                    let mut secondary_set = available_patterns.clone();
                    secondary_set.shuffle(&mut rng);

                    // This should be safe regardless of grid size.
                    while available_patterns.len() < 25 {
                        available_patterns.push(secondary_set.choose(&mut rng).unwrap());
                    }
                }

                pattern_grid.set(available_patterns[..25].to_vec());
            }

            _ => panic!(""),
        }

        dbg!("{}", choice);
    });

    rsx! {
        div { id: "dropdown-container",
            // Selector for belt
            label { r#for: "select-belt", "Grade:" }
            select {
                name: "select-belt",
                id: "select-belt",
                onchange: move |evt| {
                    belt_choice.choice.set(evt.value());
                },
                // List all options here.
                option { value: "1-Kup", "1-Kup" }
                option { value: "1-Dan", "I Dan" }
                option { value: "2-Dan", "II Dan" }
                option { value: "3-Dan", "III Dan" }
                option { value: "4-Dan", "IV Dan" }
                option { value: "5-Dan", "V Dan" }
                option { value: "6-Dan", "VI Dan" }
            }

            label { r#for: "select-grid", "Size:" }
            select {
                name: "select-grid",
                id: "select-grid",
                onchange: move |evt| {
                    grid_size.set(evt.value());
                },
                // List all options here.
                option { value: "3x3", "3x3" }
                option { value: "4x4", "4x4" }
                option { value: "5x5", "5x5" }

            }

            button {
                id: "refresh-btn",
                onclick: move |_| { refresh.state.toggle() },
                img { src: RANDOMIZE_SVG }
            }


        }

        div { id: "pattern-grid-container",
            div {
                class: "pattern-grid size-{grid_size.read()}",
                id: "pattern-grid_id",
                for i in pattern_grid.read().iter() {
                    ButtonToggle { button_name: i }
                }
            }


        }
    }
}

#[component]
fn ButtonToggle(button_name: String) -> Element {
    let mut button_toggle = use_signal(|| false);

    let belt_choice = use_context::<BeltChoice>();
    let refresh = use_context::<Refresh>();

    use_effect(move || {
        let _ = belt_choice.choice.read();
        let _ = refresh.state.read();

        button_toggle.set(false);
    });

    rsx! {

        button {
            class: "button-toggle {button_toggle.read()}",
            id: "pattern-grid-single",
            onclick: move |_| { button_toggle.toggle() },
            "{button_name}"
        }
    }
}
