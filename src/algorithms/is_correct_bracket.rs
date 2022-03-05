pub fn is_correct_bracket(s: &Vec<char>) -> bool {
    let mut cnt = 0;
    for &c in s {
        match c {
            '(' => cnt += 1,
            ')' => cnt -= 1,
            _ => return false,
        }

        if cnt < 0 {
            return false;
        }
    }

    cnt == 0
}