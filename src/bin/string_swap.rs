// m <= n を仮定してよければ...
fn swap_chars_in_string(string: &mut String, m: usize, n: usize) {
    let (pos_m, char_m) = string.char_indices().nth(m).unwrap();
    let (pos_n, char_n) = string.char_indices().nth(n).unwrap();

    // let bytes = unsafe {string.as_bytes_mut()};
    // bytes.rep // [pos_n..(pos_n+char_n.len_utf8())] = ;

    string.replace_range(pos_n..pos_n+char_n.len_utf8(), &char_m.to_string());
    string.replace_range(pos_m..pos_m+char_m.len_utf8(), &char_n.to_string());
}

fn main() {
    let mut str1 = format!("fuga");
    swap_chars_in_string(&mut str1, 0, 2);
    assert_eq!(str1, "gufa");

    // 0 <= m =< n < str.len() を仮定する
    let mut str3 = format!("fuga");
    // str3.insert(idx, ch)
    // str3.remove(idx)
    let (m, n) = (0usize, 2usize);
    let (pos_m, char_m) = str3.char_indices().nth(m).unwrap();
    let (pos_n, char_n) = str3.char_indices().nth(n).unwrap();
    str3.remove(pos_n);
    str3.insert(pos_n, char_m);
    str3.remove(pos_m);
    str3.insert(pos_m, char_n);
}
