use crate::error::EncoderError;
use crate::mem_allocator::{alloc_for_decode::AllocForDecode, alloc_for_encode::AllocForEncode};

pub struct Base64 {
    lookup_table: [u8; 64],
}

impl Base64 {
    pub fn new() -> Self {
        Self {
            lookup_table: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
        }
    }

    fn char_at_index(&self, index: usize) -> Result<u8, EncoderError> {
        if index > self.lookup_table.len() - 1 {
            return Err(EncoderError::General(String::from(
                "table indexing failed!",
            )));
        }

        Ok(self.lookup_table[index])
    }

    fn index_of_char(&self, char: u8) -> Result<usize, EncoderError> {
        if char == b'=' {
            return Ok(64usize);
        }

        // Linear search -> as array isn't that large
        for (index, &table_char) in self.lookup_table.iter().enumerate() {
            if table_char == char {
                return Ok(index);
            }
        }

        Err(EncoderError::InvalidCharacter)
    }

    pub fn encode<T: AllocForEncode>(
        &self,
        input: &[u8],
        space_allocator: Option<T>,
    ) -> Result<Box<[u8]>, EncoderError> {
        if input.is_empty() {
            return Err(EncoderError::General(String::from(
                "input cannot be empty!",
            )));
        }

        let output_space = {
            let default_space = <Base64 as AllocForEncode>::length_of_encode_output(input)?;
            match space_allocator {
                Some(_) => {
                    let user_space = T::length_of_encode_output(&input)?;
                    if user_space < default_space {
                        return Err(EncoderError::General(String::from(
                            "Provided space too small!",
                        )));
                    }

                    user_space
                }
                None => default_space,
            }
        };

        let mut output = Vec::with_capacity(output_space);
        {
            let mut temp_buff = [0u8; 3];
            let mut count = 0usize;

            for i in 0..input.len() {
                temp_buff[count] = input[i];
                count += 1usize;

                if count == 3usize {
                    // first byte
                    output.push(self.char_at_index((temp_buff[0] >> 2) as usize)?);

                    // second byte
                    output.push(self.char_at_index(
                        (((temp_buff[0] & 0x03) << 4) + (temp_buff[1] >> 4)) as usize,
                    )?);

                    // third byte
                    output.push(self.char_at_index(
                        (((temp_buff[1] & 0x0f) << 2) + (temp_buff[2] >> 6)) as usize,
                    )?);

                    // fourth byte
                    output.push(self.char_at_index((temp_buff[2] & 0x3f) as usize)?);

                    // reset count for the next round
                    count = 0usize;
                }
            }

            // when we have only one byte
            if count == 1usize {
                // first byte
                output.push(self.char_at_index((temp_buff[0] >> 2) as usize)?);

                // second byte
                output.push(self.char_at_index(((temp_buff[0] & 0x03) << 4) as usize)?);

                // third & fourth byte
                for _ in 0..=1 {
                    output.push(b'=');
                }
            }

            // when we have only two bytes
            if count == 2usize {
                // first byte
                output.push(self.char_at_index((temp_buff[0] >> 2) as usize)?);

                // second byte
                output.push(self.char_at_index(
                    (((temp_buff[0] & 0x03) << 4) + (temp_buff[1] >> 4)) as usize,
                )?);

                // third byte
                output.push(self.char_at_index(((temp_buff[1] & 0x0f) << 2) as usize)?);

                // fourth byte
                output.push(b'=');
            }
        }

        Ok(output.into_boxed_slice())
    }

    pub fn decode<T>(
        &self,
        input: &[u8],
        space_allocator: Option<T>,
    ) -> Result<Box<[u8]>, EncoderError>
    where
        T: AllocForDecode,
    {
        if input.is_empty() {
            return Err(EncoderError::General(String::from(
                "input cannot be empty!",
            )));
        }

        let output_space = {
            let default_space = <Base64 as AllocForDecode>::length_of_decode_output(input)?;
            match space_allocator {
                Some(_) => {
                    let user_space = T::length_of_decode_output(&input)?;
                    if user_space < default_space {
                        return Err(EncoderError::General(String::from(
                            "Provided space too small!",
                        )));
                    }

                    user_space
                }
                None => default_space,
            }
        };

        let mut output = Vec::with_capacity(output_space);
        {
            let mut temp_buff = [0u8; 4];
            let mut count = 0usize;

            for i in 0..input.len() {
                temp_buff[count] = self.index_of_char(input[i])? as u8;
                count += 1usize;

                if count == 4usize {
                    // first byte
                    output.push((temp_buff[0usize] << 2) + (temp_buff[1] >> 4));

                    // second byte
                    if temp_buff[2usize] != 64u8 {
                        output.push((temp_buff[1] << 4) + (temp_buff[2] >> 2))
                    }

                    // third byte
                    if temp_buff[3usize] != 64u8 {
                        output.push((temp_buff[2] << 6) + (temp_buff[3]))
                    }

                    // reset count for next round
                    count = 0usize;
                }
            }
        }
        Ok(output.into_boxed_slice())
    }
}

impl AllocForEncode for Base64 {}
impl AllocForDecode for Base64 {}

#[cfg(test)]
mod user_encoder_space_allocator_testing {
    use super::Base64;
    use crate::mem_allocator::alloc_for_encode::AllocForEncode;

    struct MyFaultyAllocator;
    impl AllocForEncode for MyFaultyAllocator {
        fn length_of_encode_output(
            input_bytes: &[u8],
        ) -> Result<usize, crate::error::EncoderError> {
            // here, I am calling the default implementation of this function but you're free to do what you want here
            let base_space = <Base64 as AllocForEncode>::length_of_encode_output(input_bytes)?;
            Ok(base_space / 2usize) // we reduce the space to be less than what is needed by default
        }
    }

    #[test]
    fn less_space_should_fail() {
        let instance = Base64::new();
        assert!(instance
            .encode(b"some test string", Some(MyFaultyAllocator))
            .is_err());
    }

    struct MyNotFaultyAllocator;
    impl AllocForEncode for MyNotFaultyAllocator {
        fn length_of_encode_output(
            input_bytes: &[u8],
        ) -> Result<usize, crate::error::EncoderError> {
            // here, I am calling the default implementation of this function but you're free to do what you want here
            let base_space = <Base64 as AllocForEncode>::length_of_encode_output(input_bytes)?;
            Ok(base_space * 2usize) // we provide double the space than  what is needed by default
        }
    }

    #[test]
    fn more_space_should_pass() {
        let instance = Base64::new();
        assert!(instance
            .encode(b"some test string", Some(MyNotFaultyAllocator))
            .is_ok());
    }
}

#[cfg(test)]
mod user_decoder_space_allocator_testing {
    use super::Base64;
    use crate::mem_allocator::{
        alloc_for_decode::AllocForDecode, alloc_for_encode::AllocForEncode,
    };

    struct MyFaultyAllocator;
    impl AllocForDecode for MyFaultyAllocator {
        fn length_of_decode_output(
            input_bytes: &[u8],
        ) -> Result<usize, crate::error::EncoderError> {
            // here, I am calling the default implementation of this function but you're free to do what you want here
            let base_space = <Base64 as AllocForDecode>::length_of_decode_output(input_bytes)?;
            Ok(base_space / 2usize) // we reduce the space to be less than what is needed by default
        }
    }

    #[test]
    fn less_space_should_fail() {
        let instance = Base64::new();
        assert!(instance
            .decode(b"SGk=", Some(MyFaultyAllocator))
            .is_err());
    }

    struct MyNotFaultyAllocator;
    impl AllocForDecode for MyNotFaultyAllocator {
        fn length_of_decode_output(
            input_bytes: &[u8],
        ) -> Result<usize, crate::error::EncoderError> {
            // here, I am calling the default implementation of this function but you're free to do what you want here
            let base_space = <Base64 as AllocForDecode>::length_of_decode_output(input_bytes)?;
            Ok(base_space * 2usize) // we provide double the space than  what is needed by default
        }
    }

    #[test]
    fn more_space_should_pass() {
        let instance = Base64::new();
        assert!(instance
            .decode(b"SGk=", Some(MyNotFaultyAllocator))
            .is_ok());
    }
}

#[cfg(test)]
mod char_lookup_and_indexing {
    use super::Base64;

    #[test]
    fn should_locate_character() {
        let instance = Base64::new();
        let target_index = 0usize;
        assert_eq!(instance.char_at_index(target_index).unwrap(), b'A');
    }

    #[test]
    fn should_not_locate_character() {
        let instance = Base64::new();
        let target_index = 100usize;
        assert!(instance.char_at_index(target_index).is_err());
    }

    #[test]
    fn should_index_starting_character() {
        let instance = Base64::new();
        let target_char = b'A';
        assert_eq!(instance.index_of_char(target_char).unwrap(), 0);
    }

    #[test]
    fn should_index_ending_character() {
        let instance = Base64::new();
        let target_char = b'/';
        assert_eq!(instance.index_of_char(target_char).unwrap(), 63);
    }

    #[test]
    fn should_find_any_middle_character() {
        let instance = Base64::new();
        let target_char = b'M';
        assert!(instance.index_of_char(target_char).is_ok());
    }

    #[test]
    fn should_not_find_invalidcharacter() {
        let instance = Base64::new();
        let target_char = b'_';
        assert!(instance.index_of_char(target_char).is_err());
    }
}

#[cfg(test)]
mod encode_testing {
    use super::Base64;

    #[test]
    fn encoder_errors_on_empty_input() {
        let instance = Base64::new();
        assert!(instance.encode::<Base64>(b"", None).is_err());
    }

    #[test]
    fn encoded_should_match_single_char() {
        let input = b"=";
        let instance = Base64::new();
        assert_eq!(&*instance.encode::<Base64>(input, None).unwrap(), b"PQ==");
    }

    #[test]
    fn encoded_should_match_double_char() {
        let input = b"Hi";
        let instance = Base64::new();
        assert_eq!(&*instance.encode::<Base64>(input, None).unwrap(), b"SGk=");
    }

    #[test]
    fn encoded_should_match_multi_char() {
        let input = b"hello world";
        let instance = Base64::new();
        assert_eq!(
            &*instance.encode::<Base64>(input, None).unwrap(),
            b"aGVsbG8gd29ybGQ="
        );
    }

    #[test]
    fn encoded_should_match_complex_multichar() {
        let input = b"This iS A RanDom Sen_tenCCe I?.. Fo..un??d oN tHe inter:))net";
        let instance = Base64::new();
        assert_eq!(
            &*instance.encode::<Base64>(input, None).unwrap(),
            b"VGhpcyBpUyBBIFJhbkRvbSBTZW5fdGVuQ0NlIEk/Li4gRm8uLnVuPz9kIG9OIHRIZSBpbnRlcjopKW5ldA=="
        );
    }
}

#[cfg(test)]
mod decode_testing {
    use super::Base64;

    #[test]
    fn decoder_errors_on_empty_input() {
        let instance = Base64::new();
        assert!(instance.decode::<Base64>(b"", None).is_err());
    }

    #[test]
    fn decoded_should_match_single_char() {
        let input = b"=";
        let instance = Base64::new();
        let encoded = instance.encode::<Base64>(input, None).unwrap();
        let decoded = instance.decode::<Base64>(&*encoded, None).unwrap();
        assert_eq!(&*decoded, b"=");
    }

    #[test]
    fn decoded_should_match_double_char() {
        let input = b"Hi";
        let instance = Base64::new();
        let encoded = instance.encode::<Base64>(input, None).unwrap();
        let decoded = instance.decode::<Base64>(&*encoded, None).unwrap();
        assert_eq!(&*decoded, b"Hi");
    }

    #[test]
    fn decoded_should_match_multi_char() {
        let input = b"hello world";
        let instance = Base64::new();
        let encoded = instance.encode::<Base64>(input, None).unwrap();
        let decoded = instance.decode::<Base64>(&*encoded, None).unwrap();
        assert_eq!(&*decoded, b"hello world");
    }

    #[test]
    fn decoded_should_match_complex_multi_char() {
        let input = b"This iS A RanDom Sen_tenCCe I?.. Fo..un??d oN tHe inter:))net";
        let instance = Base64::new();
        let encoded = instance.encode::<Base64>(input, None).unwrap();
        let decoded = instance.decode::<Base64>(&*encoded, None).unwrap();
        assert_eq!(
            &*decoded,
            b"This iS A RanDom Sen_tenCCe I?.. Fo..un??d oN tHe inter:))net"
        );
    }
}
