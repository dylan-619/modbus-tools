pub fn modbus_crc16(data: &[u8]) -> u16 {
    let mut crc = 0xFFFFu16;
    for byte in data {
        crc ^= *byte as u16;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc16() {
        // Example from design document: 01 03 00 64 00 02 CRC_LO CRC_HI
        // We know for 01 03 00 64 00 02, let's verify.
        let data = [0x01, 0x03, 0x00, 0x64, 0x00, 0x02];
        let crc = modbus_crc16(&data);
        // We can just check it doesn't crash and gives stable output
        assert_eq!(crc, 0xD485); // LO=85, HI=D4
    }
}
