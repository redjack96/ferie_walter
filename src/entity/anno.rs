use strum::EnumIter;

#[derive(Default, Debug, PartialEq, Eq, EnumIter)]
pub enum Anno {
    #[default]
    Anno2025,
    Anno2026,
    Anno2027,
    Anno2028,
}

impl Anno {
    pub fn to_string_pretty(&self) -> String {
        match self {
            Anno::Anno2025 => "2025",
            Anno::Anno2026 => "2026",
            Anno::Anno2027 => "2027",
            Anno::Anno2028 => "2028",
        }.to_string()
    }


    pub fn to_i32(&self) -> i32 {
        match self {
            Anno::Anno2025 => 2025,
            Anno::Anno2026 => 2026,
            Anno::Anno2027 => 2027,
            Anno::Anno2028 => 2028,
        }
    }
}