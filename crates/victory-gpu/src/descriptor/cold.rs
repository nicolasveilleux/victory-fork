//! Eight cold descriptor slots can be skipped only if every cache entry and record is zero.

pub(crate) fn cold_empty_run(
    copied: &[u64],
    index: usize,
    page: &[u8],
    at: usize,
    step: usize,
) -> bool {
    if !copied
        .get(index..)
        .is_some_and(|slots| slots.starts_with(&[0; 8]))
    {
        return false;
    }
    let Some(records) = page.get(at..) else {
        return false;
    };
    match step {
        16 => records.starts_with(&[0; 128]),
        32 => records.starts_with(&[0; 256]),
        _ => false,
    }
}

#[test]
fn cold_empty_runs_do_not_skip_past_cached_slots_or_clipped_bytes() {
    let mut cache = [0u64; 9];
    let mut page = [0u8; 257];
    assert!(cold_empty_run(&cache, 1, &page, 1, 32));
    assert!(!cold_empty_run(&cache, 2, &page, 1, 32));
    assert!(!cold_empty_run(&cache, 1, &page, 2, 32));
    cache[8] = 7;
    assert!(!cold_empty_run(&cache, 1, &page, 1, 32));
    assert!(cold_empty_run(&cache, 0, &page, 1, 16));
    page[128] = 1;
    assert!(!cold_empty_run(&cache, 0, &page, 1, 16));
}

#[test]
fn every_byte_and_every_cache_entry_can_veto_a_skip() {
    for step in [16, 32] {
        let mut page = vec![0; step * 8];
        let mut cache = [0; 8];
        assert!(cold_empty_run(&cache, 0, &page, 0, step));
        for byte in 0..page.len() {
            page[byte] = 1;
            assert!(!cold_empty_run(&cache, 0, &page, 0, step));
            page[byte] = 0;
        }
        for slot in 0..8 {
            cache[slot] = 1;
            assert!(!cold_empty_run(&cache, 0, &page, 0, step));
            cache[slot] = 0;
        }
        assert!(!cold_empty_run(&cache, usize::MAX, &page, 0, step));
        assert!(!cold_empty_run(&cache, 0, &page, usize::MAX, step));
        assert!(!cold_empty_run(&cache, 0, &page, 0, 64));
    }
}
