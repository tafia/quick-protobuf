//! Automatically generated mod.rs
pub mod person;

use self::person::{Address, City, Person, PersonPacked};
use quick_protobuf::{BytesReader, Writer};

#[test]
fn ignore_non_primitive_packed() {
    let person = Person {
        address: Some(Address {
            city: Some(City::LONDON),
        }),
        names: "Mr John Doe".split_whitespace().collect(),
    };

    let mut person_bytes = Vec::new();
    {
        let mut w = Writer::new(&mut person_bytes);
        w.write_message(&person).unwrap();
    }

    let mut reader = BytesReader::from_bytes(&person_bytes);
    let person_packed: PersonPacked = reader.read_message(&person_bytes).unwrap();

    let mut person_packed_bytes = Vec::new();
    {
        let mut w = Writer::new(&mut person_packed_bytes);
        w.write_message(&person_packed).unwrap();
    }

    let mut reader = BytesReader::from_bytes(&person_packed_bytes);
    let person_new: Person = reader.read_message(&person_packed_bytes).unwrap();

    assert_eq!(person, person_new);
}
