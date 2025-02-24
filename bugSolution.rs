fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using raw pointers, use safe methods like this:
    v[0] = 10;
    println!("First element: {}", v[0]);
    //Alternative approach using iterators if you need to modify elements based on some condition
    v.iter_mut().for_each(|x| {
        *x *= 2;
    });
    println!("Vector after modifying using iterators {:?}", v);
} 