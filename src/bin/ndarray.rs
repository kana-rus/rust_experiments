use ndarray::{arr2, ArrayView, ArrayBase, OwnedRepr, Dim};

trait ExtendWith0 {
    fn extend_with0(&mut self);
}
impl ExtendWith0 for ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>> {
    fn extend_with0(&mut self) {
        let shape = self.shape();
        let (m, n) = (shape[0], shape[1]);
        self.push_column(ArrayView::from(&vec![0; m])).ok();
        self.push_row(ArrayView::from(&vec![0; n+1])).ok();
    }
}

fn main() {
    let mut arr = arr2(&[
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ]);

    arr.extend_with0();
    assert_eq!(arr, arr2(&[
        [1, 2, 3, 0],
        [4, 5, 6, 0],
        [7, 8, 9, 0],
        [0, 0, 0, 0],
    ]));
}
