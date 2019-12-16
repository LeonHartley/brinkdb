pub trait IsJson {
    fn is_json(&self) -> bool {
        false
    }
}

impl IsJson for Vec<u8> {
    fn is_json(&self) -> bool {
        if let Some(c) = self.first() {
            if *c == b'{' {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
