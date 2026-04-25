use strum::{Display, EnumIter, EnumString};

// Could use an enum here, not sure if necessary.
pub static PATTERNS: &'static [&str; 24] = &[
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

#[derive(Debug, Clone, EnumIter, EnumString, Display)]
pub enum Degree {
    #[strum(to_string = "1-Kup")]
    Kup1,
    #[strum(to_string = "1-Dan")]
    Dan1,
    #[strum(to_string = "2-Dan")]
    Dan2,
    #[strum(to_string = "3-Dan")]
    Dan3,
    #[strum(to_string = "4-Dan")]
    Dan4,
    #[strum(to_string = "5-Dan")]
    Dan5,
    #[strum(to_string = "6-Dan")]
    Dan6,
}

impl Degree {
    pub fn to_display(&self) -> String {
        match self {
            Self::Kup1 => "1 Kup".into(),
            Self::Dan1 => "1 Dan".into(),
            Self::Dan2 => "2 Dan".into(),
            Self::Dan3 => "3 Dan".into(),
            Self::Dan4 => "4 Dan".into(),
            Self::Dan5 => "5 Dan".into(),
            Self::Dan6 => "6 Dan".into(),
        }
    }
}

impl Degree {
    pub fn num_patterns(&self) -> usize {
        match self {
            Self::Kup1 => 9,
            Self::Dan1 => 12,
            Self::Dan2 => 15,
            Self::Dan3 => 18,
            Self::Dan4 => 21,
            Self::Dan5 => 23,
            Self::Dan6 => 24,
        }
    }
}

/// NOTE - must have matching CSS for
/// proper formatting to be applied
#[derive(Debug, Clone, EnumIter, EnumString, Display)]
pub enum Sizes {
    #[strum(to_string = "1x1")]
    One,
    #[strum(to_string = "2x2")]
    Two,
    #[strum(to_string = "3x3")]
    Three,
    #[strum(to_string = "4x4")]
    Four,
    #[strum(to_string = "5x5")]
    Five,
}

impl Sizes {
    pub fn num_patterns(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2 * 2,
            Self::Three => 3 * 3,
            Self::Four => 4 * 4,
            Self::Five => 5 * 5,
        }
    }
}
