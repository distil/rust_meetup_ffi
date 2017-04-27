#[link(name = "calling_c")]
extern {
	fn squared(x: i32) -> i32;
}

fn main() {
    let squared = unsafe {
        squared(5)
    };
	println!("5 * 5 = {}", squared);
}
