pub(crate) fn longest_collatz() -> i32 {
    let mut length = 0;
    let mut longest_seq: i32 = 0;
    let seq_lengths: [i32; 1000000] = [0; 1000000];

    for i in 1..=1000000 {
        let mut local_len = 0;
        let initial: i32 = i;
        let mut curr = i;

        while curr != 1 {
            if seq_lengths[i] != 0 {
                local_len += seq_lengths[curr];
                break;
            } else {
                if curr % 2 == 0 {
                    curr = curr / 2;
                } else {
                    curr = curr * 3;
                    curr += 1;
                }
                local_len += 1;
            }
        }
        seq_lengths[initial] = local_len;
        if local_len > length {
            longest_seq = initial;
        }
    }
    return longest_seq;
}