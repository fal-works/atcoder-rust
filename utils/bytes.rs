fn count_contiguous_bytes(bytes: &Vec<u8>, start_index: usize, end_index: usize) -> usize {
    let byte_to_count = bytes[start_index];
    let mut count = 1;

    for index in start_index + 1..end_index {
        if bytes[index] != byte_to_count {
            break;
        }

        count += 1;
    }

    count
}
