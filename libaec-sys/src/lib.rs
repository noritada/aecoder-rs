#![allow(non_camel_case_types, unused)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    struct Stream(aec_stream);

    impl Stream {
        fn new(bits_per_sample: u32, block_size: u32, rsi: u32, flags: u32) -> Self {
            let mut raw: aec_stream = unsafe { std::mem::zeroed() };
            raw.bits_per_sample = bits_per_sample;
            raw.block_size = block_size;
            raw.rsi = rsi;
            raw.flags = flags;
            Self(raw)
        }

        fn total_in(&self) -> usize {
            self.0.total_in
        }

        fn total_out(&self) -> usize {
            self.0.total_out
        }

        fn encode(&mut self, input: &[u8], output: &mut [u8]) -> Result<(), &'static str> {
            self.0.next_in = input.as_ptr();
            self.0.avail_in = input.len();
            self.0.next_out = output.as_mut_ptr();
            self.0.avail_out = output.len();

            let result = unsafe { aec_encode_init(&mut self.0) };
            if result as u32 != AEC_OK {
                return Err("aec_encode_init() failed");
            }

            let result = unsafe { aec_encode(&mut self.0, AEC_FLUSH as i32) };
            if result as u32 != AEC_OK {
                return Err("aec_encode() failed");
            }

            let result = unsafe { aec_encode_end(&mut self.0) };
            if result as u32 != AEC_OK {
                return Err("aec_encode_end() failed"); // FIXME: support incomplete processing
            }

            Ok(())
        }

        fn decode(&mut self, input: &[u8], output: &mut [u8]) -> Result<(), &'static str> {
            self.0.next_in = input.as_ptr();
            self.0.avail_in = input.len();
            self.0.next_out = output.as_mut_ptr();
            self.0.avail_out = output.len();

            let result = unsafe { aec_decode_init(&mut self.0) };
            if result as u32 != AEC_OK {
                return Err("aec_decode_init() failed");
            }

            let result = unsafe { aec_decode(&mut self.0, AEC_FLUSH as i32) };
            if result as u32 != AEC_OK {
                return Err("aec_decode() failed");
            }

            let result = unsafe { aec_decode_end(&mut self.0) };
            if result as u32 != AEC_OK {
                return Err("aec_decode_end() failed"); // FIXME: support incomplete processing
            }

            Ok(())
        }
    }

    #[test]
    fn aec_encoding_and_decoding_works() {
        let input = [
            [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0],
            [3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0],
            [7, 0, 0, 0, 8, 0, 0, 0, 9, 0, 0, 0, 10, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [8, 8, 8, 8, 12, 12, 12, 12, 4, 4, 4, 4, 15, 15, 15, 15],
        ];
        let input = input.as_flattened();

        let bits_per_sample = 4;
        let block_size = 16;
        let rsi = 128;
        let flags = AEC_DATA_MSB;

        let mut stream = Stream::new(bits_per_sample, block_size, rsi, flags);
        let mut encoded = vec![0; 100];
        let result = stream.encode(&input, &mut encoded);
        assert!(result.is_ok());

        let encoded = &encoded[..stream.total_out()];
        let expected_encoded = [
            0x1f, 0x63, 0x23, 0xc3, 0xc1, 0xe0, 0x7a, 0x1e, 0x1e, 0x1e, 0x0f, 0x80, 0x80, 0x01,
            0xf1, 0x11, 0x19, 0x99, 0x88, 0x88, 0x9f, 0xff, 0xe0,
        ];
        assert_eq!(encoded, expected_encoded);

        let mut stream = Stream::new(bits_per_sample, block_size, rsi, flags);
        let mut decoded = vec![0; 100];
        let result = stream.decode(&encoded, &mut decoded);
        assert!(result.is_ok());

        assert_eq!(stream.total_out(), decoded.len());
        assert_eq!(&decoded[..stream.total_out()], &input[..stream.total_out()]);
    }
}
