use crate::diff::Diff;
use crate::patch::Patch;

const ZERO_U64_LE_BYTES: &[u8] = &(0u64.to_le_bytes());

pub fn create<BO: AsRef<[u8]>, BN: AsRef<[u8]>>(
    old_bytes: BO,
    new_bytes: BN,
) -> std::io::Result<Patch> {
    let old_bytes = old_bytes.as_ref();
    let new_bytes = new_bytes.as_ref();

    let mut diffs = Vec::with_capacity(256);

    let mut i = 0;
    let len_old = old_bytes.len();
    let len_new = new_bytes.len();

    while i < len_old && i < len_new {
        if old_bytes[i] != new_bytes[i] {
            let diff_begin = i;
            while i < len_old && i < len_new && old_bytes[i] != new_bytes[i] {
                i += 1;
            }
            let diff_end = i;

            let offset_bytes = (diff_begin as u64).to_le_bytes();
            let diff_len_bytes = ((diff_end - diff_begin) as u64).to_le_bytes();
            let data = &new_bytes[diff_begin..diff_end];

            let diff =
                Diff::from_bytes([&offset_bytes, &diff_len_bytes, &diff_len_bytes, data].concat())?
                    .0;

            diffs.push(diff);
        } else {
            i += 1;
        }
    }

    // new file is bigger
    if i < len_new {
        let offset_bytes = (i as u64).to_le_bytes();
        let diff_len_bytes = ((len_new - i) as u64).to_le_bytes();
        let data = &new_bytes[i..len_new];

        let diff =
            Diff::from_bytes([&offset_bytes, ZERO_U64_LE_BYTES, &diff_len_bytes, data].concat())?.0;

        diffs.push(diff);
    }

    // new file is shorter
    if i < len_old {
        let offset_bytes = (i as u64).to_le_bytes();
        let diff_len_bytes = ((len_old - i) as u64).to_le_bytes();

        let diff =
            Diff::from_bytes([&offset_bytes, &diff_len_bytes, ZERO_U64_LE_BYTES].concat())?.0;

        diffs.push(diff);
    }

    Ok(Patch { diffs })
}
