use std::collections::VecDeque;

pub fn base_to_base(n: &Vec<char>, base_src: usize, base_dst: usize) -> Vec<char> {
    let mut num = 0;
    for i in 0..n.len() {
        num = num * base_src + (n[i] as usize - '0' as usize);
    }

    let mut res = VecDeque::new();
    if num == 0 {
        res.push_front('0');
    }
    while num > 0 {
        res.push_front(((num % base_dst) as u8 + '0' as u8) as char);
        num /= base_dst;
    }
    return res.into_iter().collect();
}