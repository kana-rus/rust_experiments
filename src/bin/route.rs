#![allow(incomplete_features, unused, non_camel_case_types, non_snake_case)]
#![feature(adt_const_params, generic_const_exprs)]

struct route<const R: &'static str>;

const fn is_valid_handler_route(r: &'static str) -> bool {
    // for example
    r.is_empty()
}

trait IsValidHandlerRoute<const YES: bool> {}
impl<const R: &'static str> IsValidHandlerRoute<{is_valid_handler_route(R)}> for route<R> {}

impl<const R: &'static str> route<R>
where
Self: IsValidHandlerRoute<true>
{
    fn GET(self, _: impl Fn()) -> Self {self}
    fn POST(self, _: impl Fn()) -> Self {self}
}


fn handler() {
    println!("I am handler!")
}

fn main() {
    // let _ = route::<"">
    //     .GET(handler)
    //     .POST(handler);
    // 
    // let _ = route::<"ohkami">
    //     .GET(handler)
    //     .POST(handler);

    // let _ = route<"">::;
}
