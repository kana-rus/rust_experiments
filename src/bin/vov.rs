fn flatten<T>(vov: Vec<Option<Vec<T>>>) -> Vec<T> {
    let len = vov
        .iter()
        .filter_map(|ov| ov.as_ref().map(|v| v.len()))
        .sum();

    vov.into_iter().fold(Vec::with_capacity(len), |mut res, ov| {
        ov.map_or((), |mut v| res.append(&mut v));
        res
    })
}

fn main() {
    let vov = vec![
        Some(vec![3, 1, 4]),
        None,
        Some(vec![1, 4, 1, 5, 9]),
        Some(vec![2, 6]),
        None,
    ];

    println!("{:?}", flatten(vov))
}
