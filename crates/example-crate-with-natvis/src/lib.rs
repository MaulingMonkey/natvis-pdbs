bitflags::bitflags! {
    pub struct Flags : u8 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }
}
