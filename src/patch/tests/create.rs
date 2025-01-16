use crate::patch::Patch;

#[test]
fn test_create_patch() {
    let old_bytes = "hello, world!".as_bytes();
    let new_bytes = "hello, _lord!".as_bytes();

    let patch = crate::patch::create(old_bytes, new_bytes).expect("create patch");
    let serialized_patch = patch.clone().into_bytes();
    let deserialized_patch = Patch::from_bytes(serialized_patch).expect("deserialize patch");

    assert_eq!(deserialized_patch, patch);
}

#[test]
fn test_new_bytes_longer() {
    let old_bytes = "hello, world!".as_bytes();
    let new_bytes = "hello, my lord!".as_bytes();

    let patch = crate::patch::create(old_bytes, new_bytes).expect("create patch");
    let serialized_patch = patch.clone().into_bytes();
    let deserialized_patch = Patch::from_bytes(serialized_patch).expect("deserialize patch");

    assert_eq!(deserialized_patch, patch);
}

#[test]
fn test_new_bytes_shorter() {
    let old_bytes = "hello, world!".as_bytes();
    let new_bytes = "hell!".as_bytes();

    let patch = crate::patch::create(old_bytes, new_bytes).expect("create patch");
    let serialized_patch = patch.clone().into_bytes();
    let deserialized_patch = Patch::from_bytes(serialized_patch).expect("deserialize patch");

    assert_eq!(deserialized_patch, patch);
}

#[test]
fn test_new_bytes_empty() {
    let old_bytes = "hello, world!".as_bytes();
    let new_bytes = &[];

    let patch = crate::patch::create(old_bytes, new_bytes).expect("create patch");
    let serialized_patch = patch.clone().into_bytes();
    let deserialized_patch = Patch::from_bytes(serialized_patch).expect("deserialize patch");

    assert_eq!(deserialized_patch, patch);
}

#[test]
fn test_old_bytes_empty() {
    let old_bytes = &[];
    let new_bytes = "hell!".as_bytes();

    let patch = crate::patch::create(old_bytes, new_bytes).expect("create patch");
    let serialized_patch = patch.clone().into_bytes();
    let deserialized_patch = Patch::from_bytes(serialized_patch).expect("deserialize patch");

    assert_eq!(deserialized_patch, patch);
}
