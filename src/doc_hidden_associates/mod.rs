pub trait Fang {
    fn bite(&self, message: &str) -> String;

    #[doc(hidden)]
    fn bite_boxed_(&self, message: &str) -> Box<String> {
        Box::new(self.bite(message))
    }
}
