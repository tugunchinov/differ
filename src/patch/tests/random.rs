use crate::patch::Patch;
use rand::RngCore;

const MAX_VEC_BYTES_SIZE: usize = 100_000;

#[test]
fn test_random() {
    for _ in 0..1000 {
        let old_bytes_len = 1 + rand::random::<usize>() % MAX_VEC_BYTES_SIZE;
        let new_bytes_len = 1 + rand::random::<usize>() % MAX_VEC_BYTES_SIZE;

        let mut old_bytes = vec![0u8; old_bytes_len];
        rand::thread_rng().fill_bytes(&mut old_bytes);

        let mut new_bytes = vec![0u8; new_bytes_len];
        rand::thread_rng().fill_bytes(&mut new_bytes);

        let patch = crate::patch::create(&old_bytes, &new_bytes).expect("create patch");
        let serialized_patch = patch.clone().into_bytes();
        let deserialized_patch = Patch::from_bytes(serialized_patch).expect("deserialize patch");

        assert_eq!(deserialized_patch, patch);

        let applied = crate::patch::apply(&old_bytes, patch).expect("apply patch");

        assert_eq!(applied, new_bytes);
    }
}
