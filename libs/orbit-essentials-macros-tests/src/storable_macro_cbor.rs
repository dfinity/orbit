use ic_stable_structures::Storable;
use orbit_essentials::storable;

#[storable(serializer = "cbor")]
struct MyInitialStruct {
    id: u32,
    name: String,
}

#[storable(serializer = "cbor")]
struct MyUpdatedStruct {
    id: u32,
    name: String,
    #[serde(default)]
    active: bool,
}

#[storable(serializer = "cbor")]
#[derive(Debug, PartialEq, Eq)]
enum MyInitialEnum {
    A(u32),
    B(String),
}

#[storable(serializer = "cbor")]
#[derive(Debug, PartialEq, Eq)]
enum MyUpdatedEnum {
    A(u32),
    B(String),
    C(bool),
}

#[test]
fn validate_serialization() {
    let initial = MyInitialStruct {
        id: 1,
        name: "test".to_string(),
    };

    let serialized_initial = initial.to_bytes();
    let deserialized_to_initial = MyInitialStruct::from_bytes(serialized_initial);

    assert_eq!(deserialized_to_initial.id, initial.id);
    assert_eq!(deserialized_to_initial.name, initial.name);
}

#[test]
fn validate_serialization_is_forward_compatible() {
    let initial = MyInitialStruct {
        id: 1,
        name: "test".to_string(),
    };

    let serialized_initial = initial.to_bytes();
    let deserialized_to_updated = MyUpdatedStruct::from_bytes(serialized_initial);

    assert_eq!(deserialized_to_updated.id, initial.id);
    assert_eq!(deserialized_to_updated.name, initial.name);
    assert!(!deserialized_to_updated.active);
}

#[test]
fn validate_serialization_accepts_missing_fields() {
    let updated = MyUpdatedStruct {
        id: 1,
        name: "test".to_string(),
        active: true,
    };

    let serialized_updated = updated.to_bytes();
    let deserialized_to_initial = MyInitialStruct::from_bytes(serialized_updated);

    assert_eq!(deserialized_to_initial.id, updated.id);
    assert_eq!(deserialized_to_initial.name, updated.name);
}

#[test]
fn validate_serialization_is_forward_compatible_for_enum() {
    let initial = MyInitialEnum::A(1);

    let serialized_initial = initial.to_bytes();
    let deserialized_to_updated = MyUpdatedEnum::from_bytes(serialized_initial);

    assert_eq!(deserialized_to_updated, MyUpdatedEnum::A(1));
}

#[test]
#[should_panic]
fn validate_deserialization_should_fail_for_missing_variant() {
    let updated = MyUpdatedEnum::C(true);

    let serialized_updated = updated.to_bytes();
    let _ = MyInitialEnum::from_bytes(serialized_updated);
}
