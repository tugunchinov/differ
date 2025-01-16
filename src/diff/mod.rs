use std::io::Read;
use std::ops::Index;

/// A part of a patch
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Diff {
    /// position in the old file
    offset: u64,

    /// number of replaced bytes
    len_old: u64,

    /// number of replacing bytes
    len_new: u64,

    /// replacing bytes
    // NB: data.len() == len_new
    data: Vec<u8>,
}

impl Diff {
    pub fn into_bytes(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(size_of::<Diff>() + self.data.len());

        result.extend(self.offset.to_le_bytes());
        result.extend(self.len_old.to_le_bytes());
        result.extend(self.len_new.to_le_bytes());
        result.extend(self.data);

        result
    }

    /// returns the diff and the number of bytes read
    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> std::io::Result<(Self, usize)> {
        let mut bytes = bytes.as_ref();
        let mut bytes_read = 0;

        let mut offset_bytes = [0u8; 8];
        let mut length_old_bytes = [0u8; 8];
        let mut length_new_bytes = [0u8; 8];

        bytes.read_exact(&mut offset_bytes)?;
        bytes_read += offset_bytes.len();

        bytes.read_exact(&mut length_old_bytes)?;
        bytes_read += length_old_bytes.len();

        bytes.read_exact(&mut length_new_bytes)?;
        bytes_read += length_new_bytes.len();

        let offset = u64::from_le_bytes(offset_bytes);
        let len_old = u64::from_le_bytes(length_old_bytes);
        let len_new = u64::from_le_bytes(length_new_bytes);

        let mut data = vec![0u8; len_new as usize];
        bytes.as_ref().read_exact(&mut data)?;
        bytes_read += data.len();

        Ok((
            Self {
                offset,
                len_old,
                len_new,
                data,
            },
            bytes_read,
        ))
    }

    pub fn get_offset(&self) -> u64 {
        self.offset
    }

    pub fn get_len_old(&self) -> u64 {
        self.len_old
    }

    pub fn get_len_new(&self) -> u64 {
        self.len_new
    }
}

impl IntoIterator for Diff {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl Index<usize> for Diff {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[cfg(test)]
mod serde_tests {
    use crate::diff::Diff;

    #[test]
    fn test_diff_serialize_deserialize() {
        let diff = Diff {
            offset: 42,
            len_old: 133,
            len_new: 4,
            data: vec![1, 2, 3, 4],
        };

        let serialized_diff = diff.clone().into_bytes();

        println!("{serialized_diff:?}");

        let (deserialized_diff, _) = Diff::from_bytes(serialized_diff).expect("deserializing diff");

        assert_eq!(diff, deserialized_diff);
    }
}
