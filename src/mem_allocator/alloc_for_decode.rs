use crate::error::EncoderError;

pub trait AllocForDecode {
    /// Performs the reverse of the logic for `length_of_encode_output` function with a twist.
    /// Essentially, we:
    ///
    /// 1. Take the length of the input array of bytes
    /// 2. Perform round-down division by 4 (as decoding reduces 4 bytes to the original 3 bytes)
    /// 3. Multiply the result of step 2 by 3
    /// 4. For each appearance of the character `=` found in the input byte array, subtract 1 from
    /// the result of step 3 above
    ///
    /// By default, if `input_bytes.len()` < 4,the function always returns 3
    fn length_of_decode_output(input_bytes: &[u8]) -> Result<usize, EncoderError> {
        if input_bytes.len() < 4 {
            return Ok(4usize);
        }

        let groups_of_4 = input_bytes.len() / 4usize;
        let mut multiple_groups = groups_of_4 * 3usize;

        for i in input_bytes.iter() {
            if i.to_owned() == b'=' {
                multiple_groups -= 1
            }
        }
        Ok(multiple_groups)
    }
}
