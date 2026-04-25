use std::str::FromStr;

use crate::utils::{Degree, Sizes, PATTERNS};
use dioxus::prelude::*;
use rand::seq::{IndexedRandom, SliceRandom};
use strum::IntoEnumIterator;

#[component]
pub fn Grid() -> Element {
    rsx! {
        div { id: "container",
            span { id: "container-header", "Pattern Randomizer" }
            PatternGrid {}
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
fn PatternGrid() -> Element {
    let mut belt_choice = use_context_provider(|| BeltChoice {
        choice: Signal::new(Degree::Kup1.to_string()),
    });

    let mut refresh = use_context_provider(|| Refresh {
        state: Signal::new(false),
    });

    let mut grid_size = use_signal(|| Sizes::One.to_string());
    let mut pattern_grid = use_signal(|| PATTERNS[..Sizes::One.num_patterns()].to_vec());

    use_effect(move || {
        let _ = refresh.state.read();

        // If changing belt
        let choice = belt_choice.choice.read();

        let chosen_degree =
            Degree::from_str(choice.as_str()).expect(&format!("failed to parse {:?}", choice));
        let num_patterns = chosen_degree.num_patterns();

        // Get available patterns and shuffle
        let mut rng = rand::rng();
        let mut available_patterns = PATTERNS[..num_patterns].to_vec();
        available_patterns.shuffle(&mut rng);

        // If changing grid size
        let grid_size_choice = grid_size.read();
        let chosen_size = Sizes::from_str(grid_size_choice.as_str())
            .expect(&format!("failed to parse {:?}", grid_size_choice));
        let num_required_patterns = chosen_size.num_patterns();

        // Fill with random patterns if needed to reach required num patterns:
        // 		E.g., 1-Kup only has 9 patterns and if grid size is 4x4
        // 		we need to fill with 7 more patterns to reach 16 total.
        while available_patterns.len() < num_required_patterns {
            let mut secondary_set = available_patterns.clone();
            secondary_set.shuffle(&mut rng);

            // This should be safe regardless of grid size.
            available_patterns.push(secondary_set.choose(&mut rng).unwrap());
        }

        pattern_grid.set(available_patterns[..num_required_patterns].to_vec());
    });

    rsx! {
        div { id: "dropdown-container",
            div { id: "select-belt-size",
                div { id: "select-belt-container",
                    // Selector for belt
                    label { r#for: "select-belt", "Grade:" }
                    select {
                        name: "select-belt",
                        id: "select-belt",
                        onchange: move |evt| {
                            belt_choice.choice.set(evt.value());
                        },
                        for d in Degree::iter() {
                            option { value: "{d.to_string()}", "{d.to_display()}" }
                        }
                    }
                }

                div { id: "select-size-container",
                    label { r#for: "select-grid", "Size:" }
                    select {
                        name: "select-grid",
                        id: "select-grid",
                        onchange: move |evt| {
                            grid_size.set(evt.value());
                        },
                        for size in Sizes::iter() {
                            option { value: "{size.to_string()}", "{size.to_string()}" }
                        }
                    }
                }
            }

            button {
                id: "refresh-btn",
                onclick: move |_| { refresh.state.toggle() },
                span { id: "refresh-logo", class: "material-symbols-outlined", "autorenew" }
            }

        }

        div { id: "pattern-grid-container",
            div {
                class: "pattern-grid size-{grid_size.read()}",
                id: "pattern-grid_id",
                for i in pattern_grid.read().iter() {
                    ButtonToggle { button_name: i.to_string() }
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

            if *button_toggle.read() {
                span {
                    id: "toggle-is-complete",
                    class: "material-symbols-outlined",
                    "check"
                }
            }
        }
    }
}
