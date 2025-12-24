pub fn pad_buffer(buf: &[u8], len: usize) -> Vec<u8> {
    let mut v = buf.to_vec();
    v.resize(len, 0);
    v
}