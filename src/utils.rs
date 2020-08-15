pub trait ToUsize {
    fn to_usize(&self) -> usize;
}

impl ToUsize for u8 {
    fn to_usize(&self) -> usize {
        *self as usize
    }
}

impl ToUsize for u16 {
    fn to_usize(&self) -> usize {
        *self as usize
    }
}

impl ToUsize for u32 {
    fn to_usize(&self) -> usize {
        *self as usize
    }
}

impl ToUsize for u64 {
    fn to_usize(&self) -> usize {
        *self as usize
    }
}

impl ToUsize for usize {
    fn to_usize(&self) -> usize {
        *self
    }
}
