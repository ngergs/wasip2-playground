#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
// has to be fixed in wit-bindgen
#![allow(unsafe_op_in_unsafe_fn)]

wit_bindgen::generate!({
    path:"../math.wit",
    world: "math",
});

struct GuestImpl;

impl Guest for GuestImpl {
    fn sum(args: Arguments) -> i32 {
        args.x + args.y
    }

    fn minus(args: Arguments) -> i32 {
        args.x - args.y
    }

    fn mul(args: Arguments) -> i32 {
        args.x * args.y
    }

    fn div(args: Arguments) -> Result<f32, String> {
        if args.y == 0 {
            return Err("division by zero".into());
        };
        Ok((args.x as f32) / (args.y as f32))
    }
}

export!(GuestImpl);
