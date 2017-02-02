extern crate quick_protobuf;

mod codegen;

use std::borrow::Cow;

use codegen::data_types::FooMessage;
use quick_protobuf::{BytesReader, Writer};

fn main() {

    // Generate a message, somehow
    //
    // For the example we will leverage the `Default` derive of all messages
    let message = FooMessage {
        f_int32: Some(54), 
        f_string: Some(Cow::Borrowed("Hello world from example!")),
        f_bytes: Some(Cow::Borrowed(b"I see you!")),
        ..FooMessage::default()
    };

    // Write the message into our buffer, it could be a file, a ZipFile etc ...
    let mut out = Vec::new();
    {
        let mut writer = Writer::new(&mut out);
        writer.write_message(&message).expect("Cannot write message!");
    }
    println!("Message written successfully!");

    // Try to read it back and confirm that we still have the exact same message
    //
    // In general, if we had written the data to say, a file, or if someone else have written that
    // data, it would be more convenient to use a `Reader` which will feed an internal, owned, buffer
    // Here, on the contrary, we already hold the `out` buffer. Thus it is more efficient 
    // to directly use a `BytesWriter`. 
    let read_message = {
        let mut reader = BytesReader::from_bytes(&out);
        reader.read_message(&out, FooMessage::from_reader).expect("Cannot read message")
    };
    assert_eq!(message, read_message);

    println!("Message read back and everything matches!");
    println!("{:#?}", read_message);
}
