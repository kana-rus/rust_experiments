fn main() {
    #[cfg(target_pointer_width="64")]
    #[cfg(target_pointer_width="32")]
    compile_error! {"unreachable"}
}