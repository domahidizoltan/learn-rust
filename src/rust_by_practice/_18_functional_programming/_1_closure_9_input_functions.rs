
/* Implement `call_me` to make it work */
fn call_me(f: fn()) {
    f();
}

fn function() {
    println!("I'm a function!");
}

pub fn main() {
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
