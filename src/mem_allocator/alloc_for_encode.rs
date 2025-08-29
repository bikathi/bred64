use crate::error::EncoderError;

pub trait AllocForEncode {
    /// Calculates the length of the output from the encoding process.
    /// # Arguments
    /// * input - A slice of bytes
    ///
    /// Essentially, we:
    ///
    /// 1. Take the length of the input array of bytes
    /// 2. Perform round-up division by 3 (as encoding works with 3 bytes at a time)
    /// 3. Multiply the result of step 2 by 4 as each group of 3 bytes produces 4 bytes
    ///
    /// By default, if `input_bytes.len()` < 3,the function always returns 4
    fn length_of_encode_output(input_bytes: &[u8]) -> Result<usize, EncoderError> {
        if input_bytes.len() < 3 {
            return Ok(4usize);
        }

        let groups_of_three = (input_bytes.len() + 2usize) / 3usize;
        let result = groups_of_three
            .checked_mul(4)
            .ok_or_else(|| TryInto::<u8>::try_into(1000u32).unwrap_err())?;

        Ok(result)
    }
}
