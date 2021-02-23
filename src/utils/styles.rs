pub struct Styles {
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub bold: String,
    pub reset: String,
}

fn ctc(code: u8) -> String {
    return format!("\x1b[{}m", code);
}

impl Styles {
    pub fn new() -> Styles {
        Styles {
            red: ctc(31),
            green: ctc(32),
            yellow: ctc(33),
            blue: ctc(34),
            bold: ctc(1),
            reset: ctc(0),
        }
    }
}
