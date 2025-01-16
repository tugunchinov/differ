use crate::patch::Patch;

#[test]
fn test_apply_patch() {
    let bytes = "hello, world!".as_bytes();
    let serialized_patch = [
        1, 0, 0, 0, 7, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 95, 108, 111, 114,
    ];

    let patch = Patch::from_bytes(serialized_patch).expect("deserialize patch");

    let applied = crate::patch::apply(bytes, patch).expect("apply");
    let applied_str = std::str::from_utf8(&applied).expect("convert to utf8");

    assert_eq!(applied_str, "hello, _lord!");
}

#[test]
fn test_new_bytes_longer() {
    let bytes = "hello, world!".as_bytes();
    let serialized_patch = [
        3, 0, 0, 0, 7, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 109, 121, 32, 11, 0, 0, 0, 2, 0, 0, 0, 2,
        0, 0, 0, 111, 114, 13, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 100, 33,
    ];

    let patch = Patch::from_bytes(serialized_patch).expect("deserialize patch");

    let applied = crate::patch::apply(bytes, patch).expect("apply");
    let applied_str = std::str::from_utf8(&applied).expect("convert to utf8");

    assert_eq!(applied_str, "hello, my lord!");
}

#[test]
fn test_new_bytes_shorter() {
    let bytes = "hello, world!".as_bytes();
    let serialized_patch = [
        2, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 33, 5, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0,
    ];

    let patch = Patch::from_bytes(serialized_patch).expect("deserialize patch");

    let applied = crate::patch::apply(bytes, patch).expect("apply");
    let applied_str = std::str::from_utf8(&applied).expect("convert to utf8");

    assert_eq!(applied_str, "hell!");
}

#[test]
fn test_new_bytes_empty() {
    let bytes = "hello, world!".as_bytes();
    let serialized_patch = [1, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0];

    let patch = Patch::from_bytes(serialized_patch).expect("deserialize patch");

    let applied = crate::patch::apply(bytes, patch).expect("apply");
    let applied_str = std::str::from_utf8(&applied).expect("convert to utf8");

    assert_eq!(applied_str, "");
}

#[test]
fn test_old_bytes_empty() {
    let bytes = &[];
    let serialized_patch = [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 104, 101, 108, 108, 33,
    ];

    let patch = Patch::from_bytes(serialized_patch).expect("deserialize patch");

    let applied = crate::patch::apply(bytes, patch).expect("apply");
    let applied_str = std::str::from_utf8(&applied).expect("convert to utf8");

    assert_eq!(applied_str, "hell!");
}
