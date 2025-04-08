pub trait SerializeBuffer {
    fn to_buffer(&self) -> Vec<u8>;
}

pub trait DeserializeBuffer {
    type Slice;
    fn from_buffer(val: Self::Slice) -> Self;
}
