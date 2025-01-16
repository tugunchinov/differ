mod apply;
mod create;

#[cfg(test)]
mod tests;

pub use apply::apply;
pub use create::create;

use crate::diff::Diff;
use std::io::Read;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Patch {
    // NOTE: keep it sorted by offset
    diffs: Vec<Diff>,
}

impl Patch {
    pub fn into_bytes(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.diffs.len() * size_of::<Diff>());

        result.extend((self.diffs.len() as u32).to_le_bytes());

        for diff in self.diffs {
            result.extend(diff.into_bytes());
        }

        result
    }

    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> std::io::Result<Self> {
        let mut bytes = bytes.as_ref();

        let mut diffs_cnt_bytes = [0u8; 4];
        bytes.read_exact(&mut diffs_cnt_bytes)?;
        let diffs_cnt = u32::from_le_bytes(diffs_cnt_bytes);

        let mut diffs = Vec::with_capacity(size_of::<Diff>() * diffs_cnt as usize);
        let mut total_bytes_read = 0;

        for _ in 0..diffs_cnt {
            let (diff, bytes_read) = Diff::from_bytes(&bytes[total_bytes_read..])?;
            diffs.push(diff);
            total_bytes_read += bytes_read;
        }

        Ok(Self { diffs })
    }
}

#[cfg(test)]
mod serde_tests {
    use crate::diff::Diff;
    use crate::patch::Patch;

    #[test]
    fn test_patch_serialize_deserialize() {
        let diff_bytes = [42, 0, 0, 0, 133, 0, 0, 0, 4, 0, 0, 0, 1, 2, 3, 4];

        let mut patch = Patch { diffs: Vec::new() };

        for _ in 0..3 {
            patch
                .diffs
                .push(Diff::from_bytes(&diff_bytes).expect("diff from bytes").0);
        }

        let serialized_patch = patch.clone().into_bytes();
        let deserialized_patch = Patch::from_bytes(serialized_patch).expect("deserializing patch");

        assert_eq!(patch, deserialized_patch);
    }
}
