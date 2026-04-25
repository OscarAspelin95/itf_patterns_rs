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

#[derive(Debug, Clone)]
pub enum Degree {
    Kup1,
    Dan1,
    Dan2,
    Dan3,
    Dan4,
    Dan5,
    Dan6,
}

/// Currently loose coupling between string and num_patterns representation
impl Degree {
    fn as_str<'a>(&'a self) -> &'a str {
        match self {
            Self::Kup1 => "1-Kup",
            Self::Dan1 => "1-Dan",
            Self::Dan2 => "2-Dan",
            Self::Dan3 => "3-Dan",
            Self::Dan4 => "4-Dan",
            Self::Dan5 => "5-Dan",
            Self::Dan6 => "6-Dan",
        }
    }

    fn num_patterns(&self) -> usize {
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

#[derive(Debug, Clone)]
pub enum Sizes {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Sizes {
    fn as_str<'a>(&'a self) -> &'a str {
        match self {
            Self::One => "1x1",
            Self::Two => "2x2",
            Self::Three => "3x3",
            Self::Four => "4x4",
            Self::Five => "5x5",
        }
    }
}
