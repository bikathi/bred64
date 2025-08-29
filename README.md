# bred64 (Base64 Rust Encoder & Decoder)

A simple, highly-optimized base64 library for Rust. Bred64 keeps it simple by using only one dependency
and a sizeable amount of bit shifting to achive the spec of the encoding standard.

# Enconding

For compatibility purporses, the encoding function takes in a byte array/slice, `[u8]` and a custom space
allocator.
The:

1. **byte array** is so that you can plug in the byte sequence of anything, a file, a picture, a video
   and the encoder will do the encoding. It therefore means that it's your duty to figure out how to get the byte
   sequence of whatever you want to encode and the library will encode it for you.
2. **custom space allocator** allows you to declare space for the output of the encoder. You don't usually need
   to do this as the library has a simple algorithm to determine the space needed for the encoding output (hence the none
   in the example below).

## Example on encoding

The example below shows how to use the encoding function. Notice that the porcess may fail so it's up to you
to handle the resultant `Result<Box<u8>, EncoderError>` return type properly.

Also notice the `None` param passed in. You'll most likely be doing this unless you want to pass your own function
that will be called to allocate a custom space for the encoding output.

```rust
let input = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    let encoded = Base64::new().encode::<Base64>(input, None).unwrap();
    assert_eq!(&*encoded, b"TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdCwgc2VkIGRvIGVpdXNtb2QgdGVtcG9yIGluY2lkaWR1bnQgdXQgbGFib3JlIGV0IGRvbG9yZSBtYWduYSBhbGlxdWEuIFV0IGVuaW0gYWQgbWluaW0gdmVuaWFtLCBxdWlzIG5vc3RydWQgZXhlcmNpdGF0aW9uIHVsbGFtY28gbGFib3JpcyBuaXNpIHV0IGFsaXF1aXAgZXggZWEgY29tbW9kbyBjb25zZXF1YXQuIER1aXMgYXV0ZSBpcnVyZSBkb2xvciBpbiByZXByZWhlbmRlcml0IGluIHZvbHVwdGF0ZSB2ZWxpdCBlc3NlIGNpbGx1bSBkb2xvcmUgZXUgZnVnaWF0IG51bGxhIHBhcmlhdHVyLiBFeGNlcHRldXIgc2ludCBvY2NhZWNhdCBjdXBpZGF0YXQgbm9uIHByb2lkZW50LCBzdW50IGluIGN1bHBhIHF1aSBvZmZpY2lhIGRlc2VydW50IG1vbGxpdCBhbmltIGlkIGVzdCBsYWJvcnVtLg==");
```

# Decoding

The decoding function works in a similar way to the encoding function, in that you pass a sequence of bytes representing the Base64 text and
get back a sequence of bytes representing the original input as a `Result<Box<u8>, EncoderError>`.

You can also optionally choose to provide a function that will tell the encoder how much space to allocate for the output of the decoder, but
in most cases you won't need to do this, hence the `None` param passed into the `decode(...)` function in the example below:

```rust
#[test]
fn decode_base64() {
    let input = b"TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdCwgc2VkIGRvIGVpdXNtb2QgdGVtcG9yIGluY2lkaWR1bnQgdXQgbGFib3JlIGV0IGRvbG9yZSBtYWduYSBhbGlxdWEuIFV0IGVuaW0gYWQgbWluaW0gdmVuaWFtLCBxdWlzIG5vc3RydWQgZXhlcmNpdGF0aW9uIHVsbGFtY28gbGFib3JpcyBuaXNpIHV0IGFsaXF1aXAgZXggZWEgY29tbW9kbyBjb25zZXF1YXQuIER1aXMgYXV0ZSBpcnVyZSBkb2xvciBpbiByZXByZWhlbmRlcml0IGluIHZvbHVwdGF0ZSB2ZWxpdCBlc3NlIGNpbGx1bSBkb2xvcmUgZXUgZnVnaWF0IG51bGxhIHBhcmlhdHVyLiBFeGNlcHRldXIgc2ludCBvY2NhZWNhdCBjdXBpZGF0YXQgbm9uIHByb2lkZW50LCBzdW50IGluIGN1bHBhIHF1aSBvZmZpY2lhIGRlc2VydW50IG1vbGxpdCBhbmltIGlkIGVzdCBsYWJvcnVtLg==";
    let decoded = Base64::new().decode::<Base64>(input, None).unwrap();
    assert_eq!(&*decoded, b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");
}
```

# Custom Space Allocation

As mentioned in both example sections above, you can pass a custom implementation of the logic for allocating space for the output (bytes) of the encoder
and the decoder. To do this, implement the `AllocForEncode` or `AllocForDecode` on a unit struct and pass it to the `encode(..)` or `(decode)` function
respectively.

You however may not need to do this as the internal logic of either processes has an algorithm it uses to ensure it allocates the correct, and just
enough space.
As a running example, here is how we would declare a custom allocator to pass to the encode function:

```rust
struct MyCustomSpaceAllocator;

impl AllocForEncode for MyCustomSpaceAllocator {
    fn length_of_encode_output(input_bytes: &[u8]) -> Result<usize, crate::error::EncoderError> {
        // here, I am calling the default implementation of this function but you're free to do what you want here
        <Base64 as AllocForEncode>::length_of_encode_output(input_bytes)
    }
}

#[test]
fn encode_paragraph_with_custom_space_allocator() {
    let input = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    let encoded = Base64::new()
        .encode(input, Some(MyCustomSpaceAllocator))
        .unwrap();
    assert_eq!(&*encoded, b"TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdCwgc2VkIGRvIGVpdXNtb2QgdGVtcG9yIGluY2lkaWR1bnQgdXQgbGFib3JlIGV0IGRvbG9yZSBtYWduYSBhbGlxdWEuIFV0IGVuaW0gYWQgbWluaW0gdmVuaWFtLCBxdWlzIG5vc3RydWQgZXhlcmNpdGF0aW9uIHVsbGFtY28gbGFib3JpcyBuaXNpIHV0IGFsaXF1aXAgZXggZWEgY29tbW9kbyBjb25zZXF1YXQuIER1aXMgYXV0ZSBpcnVyZSBkb2xvciBpbiByZXByZWhlbmRlcml0IGluIHZvbHVwdGF0ZSB2ZWxpdCBlc3NlIGNpbGx1bSBkb2xvcmUgZXUgZnVnaWF0IG51bGxhIHBhcmlhdHVyLiBFeGNlcHRldXIgc2ludCBvY2NhZWNhdCBjdXBpZGF0YXQgbm9uIHByb2lkZW50LCBzdW50IGluIGN1bHBhIHF1aSBvZmZpY2lhIGRlc2VydW50IG1vbGxpdCBhbmltIGlkIGVzdCBsYWJvcnVtLg==");
}
```

# Special Thanks To
The [Zig documentation](https://pedropark99.github.io/zig-book/Chapters/01-base64.html) for offering a good starter point o learn how this works.

# License
This project is licensed under the MIT license and you are free to use it in any of your projects without credit.
