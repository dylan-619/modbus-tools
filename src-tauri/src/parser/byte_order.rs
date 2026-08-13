use crate::models::ByteLayout;

pub fn reorder_bytes_16(bytes: [u8; 2], layout: ByteLayout) -> [u8; 2] {
    let [a, b] = bytes;
    match layout {
        ByteLayout::Ab => [a, b],
        ByteLayout::Ba => [b, a],
        _ => [a, b], // Fallback for invalid config
    }
}

pub fn reorder_bytes_32(bytes: [u8; 4], layout: ByteLayout) -> [u8; 4] {
    let [a, b, c, d] = bytes;
    match layout {
        ByteLayout::Abcd => [a, b, c, d],
        ByteLayout::Cdab => [c, d, a, b],
        ByteLayout::Badc => [b, a, d, c],
        ByteLayout::Dcba => [d, c, b, a],
        _ => [a, b, c, d], // Fallback
    }
}

pub fn reorder_bytes_64(bytes: [u8; 8], layout: ByteLayout) -> [u8; 8] {
    let [a, b, c, d, e, f, g, h] = bytes;
    match layout {
        ByteLayout::Abcdefgh => [a, b, c, d, e, f, g, h],
        ByteLayout::Ghefcdab => [g, h, e, f, c, d, a, b],
        ByteLayout::Badcfehg => [b, a, d, c, f, e, h, g],
        ByteLayout::Hgfedcba => [h, g, f, e, d, c, b, a],
        _ => [a, b, c, d, e, f, g, h], // Fallback
    }
}
