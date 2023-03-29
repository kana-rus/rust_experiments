// macro_rules! key {
//     ($f:literal $l:literal $c:literal) => {
//         stringify!($f:$l:$c)
//     };
//     () => {
//         {
//             let (_f, _l, _c) = (file!(), line!(), column!());
//             key!(_f _l _c)
//         }
//     };
// }

fn main() {
    let _file:   &'static str = file!();
    let _line:   u32          = line!();
    let _column: u32          = column!();

    // let _key = key!();
}

fn __expanded__() {
    let _file:   &'static str = "src/bin/debuginfo.rs";
    let _line:   u32          = 3u32;
    let _column: u32          = 33u32;
}
