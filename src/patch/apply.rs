use crate::patch::Patch;
use std::io::ErrorKind;

pub fn apply<B: AsRef<[u8]>>(bytes: B, patch: Patch) -> std::io::Result<Vec<u8>> {
    let mut result = bytes.as_ref().to_vec();

    let mut shift: i64 = 0;
    for diff in patch.diffs {
        let current_offset = (diff.get_offset() as i64 + shift) as usize;
        let len_old = diff.get_len_old() as usize;
        let len_new = diff.get_len_new() as usize;

        if len_new <= len_old {
            if current_offset + len_old > result.len() {
                return Err(std::io::Error::new(ErrorKind::InvalidData, "broken patch"));
            }

            for (i, byte) in result[current_offset..current_offset + len_new]
                .iter_mut()
                .enumerate()
            {
                *byte = diff[i];
            }

            result.drain(current_offset + len_new..current_offset + len_old);
        } else {
            result.splice(current_offset..current_offset + len_old, diff);
        }

        shift += len_new as i64 - len_old as i64;
    }

    Ok(result)
}
