fn main() {
    let mut x = 5;
    { // Create a new scope to limit borrow
        let y = &mut x;
        *y += 1;
    }
    { // Create another new scope to limit borrow
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}
