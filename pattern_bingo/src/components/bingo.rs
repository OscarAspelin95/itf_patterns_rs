use dioxus::prelude::*;
use rand::seq::SliceRandom;

const RANDOMIZE_SVG: Asset = asset!("/assets/randomize.svg");
const TTKD_LOGO: Asset = asset!("/assets/ttkd_logo.jpeg");

// TODO - add reset/reload button. Keep belt choice but randomize patterns and reset button toggles
// TODO - add option for 3x3 grid, 4x4 grid or all patterns.

#[component]
pub fn Bingo() -> Element {
    rsx! {
        div { id: "container",
            span { id: "container-header", "Mönster-bingo" }
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

#[derive(Debug, Clone)]
pub struct GridSize {
    current_size: Signal<String>,
    options: Signal<Vec<String>>,
}

#[component]
fn BingoGrid() -> Element {
    // Tmp solution
    // 1 Kup.

    let mut bingo_grid_1kup: Vec<String> = vec![
        "Chon-Ji".to_string(),
        "Dan-Gun".to_string(),
        "Do-San".to_string(),
        "Won-Hyo".to_string(),
        "Yul-Gok".to_string(),
        "Choong-Gun".to_string(),
        "Toi-Gye".to_string(),
        "Hwa-Rang".to_string(),
        "Choong-Moo".to_string(),
    ];

    // 1 Dan.
    let mut bingo_grid_1dan: Vec<String> = [
        bingo_grid_1kup.clone(),
        vec![
            "Kwan-Gae".to_string(),
            "Po-Eun".to_string(),
            "Ge-Baek".to_string(),
        ],
    ]
    .concat();

    // 2 Dan.
    let mut bingo_grid_2dan: Vec<String> = [
        bingo_grid_1dan.clone(),
        vec![
            "Eui-Am".to_string(),
            "Choong-Jang".to_string(),
            "Juche".to_string(),
        ],
    ]
    .concat();

    // 3 Dan.
    let mut bingo_grid_3dan: Vec<String> = [
        bingo_grid_2dan.clone(),
        vec![
            "Sam-Il".to_string(),
            "Yoo-Sin".to_string(),
            "Choi-Yong".to_string(),
        ],
    ]
    .concat();

    // 4 Dan.
    let mut bingo_grid_4dan: Vec<String> = [
        bingo_grid_3dan.clone(),
        vec![
            "Yon-Gae".to_string(),
            "Ul-Ji".to_string(),
            "Moon-Moo".to_string(),
        ],
    ]
    .concat();

    let mut bingo_grid_5dan: Vec<String> = [
        bingo_grid_4dan.clone(),
        vec!["So-San".to_string(), "Se-Jong".to_string()],
    ]
    .concat();

    let mut belt_choice = use_context_provider(|| BeltChoice {
        choice: Signal::new("1-Kup".to_string()),
    });

    let mut refresh = use_context_provider(|| Refresh {
        state: Signal::new(false),
    });

    let mut grid_size = use_signal(|| "3x3".to_string());

    let mut bingo_grid = use_signal(|| bingo_grid_1kup.clone());

    use_effect(move || {
        let choice = belt_choice.choice.read();
        let _ = refresh.state.read();

        let s = choice.as_str();
        let mut rng = rand::rng();

        match s {
            "1-Kup" => {
                grid_size.set("3x3".to_string());
                bingo_grid_1kup.shuffle(&mut rng);
                bingo_grid.set(bingo_grid_1kup.clone());
            }
            "1-Dan" => {
                grid_size.set("3x3".to_string());
                bingo_grid_1dan.shuffle(&mut rng);
                bingo_grid.set(bingo_grid_1dan[..9].to_vec());
            }
            "2-Dan" => {
                grid_size.set("3x3".to_string());
                bingo_grid_2dan.shuffle(&mut rng);
                bingo_grid.set(bingo_grid_2dan[..9].to_vec());
            }
            "3-Dan" => {
                grid_size.set("4x4".to_string());
                bingo_grid_3dan.shuffle(&mut rng);
                bingo_grid.set(bingo_grid_3dan[..16].to_vec());
            }
            "4-Dan" => {
                grid_size.set("4x4".to_string());
                bingo_grid_4dan.shuffle(&mut rng);
                bingo_grid.set(bingo_grid_4dan[..16].to_vec());
            }
            "5-Dan" => {
                grid_size.set("4x4".to_string());
                bingo_grid_5dan.shuffle(&mut rng);
                bingo_grid.set(bingo_grid_5dan[..16].to_vec());
            }

            _ => {
                panic!("Invalid belt choice")
            }
        }

        dbg!("{}", choice);
    });

    rsx! {
        div { id: "dropdown-container",
            // Selector for belt
            label { r#for: "select-belt", "Välj bälte:" }
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
            }

            button {
                id: "refresh-btn",
                onclick: move |_| { refresh.state.toggle() },
                img { src: RANDOMIZE_SVG }
            }


        }

        div { id: "bingo-grid-container",
            div {
                class: "bingo-grid size-{grid_size.read()}",
                id: "bingo-grid_id",
                for i in bingo_grid.read().iter() {
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
            id: "bingo-grid-single",
            onclick: move |_| { button_toggle.toggle() },
            "{button_name}"
        }
    }
}
