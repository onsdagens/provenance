fn main() {
    let a: u64;
    let b: u64;
    a = {
        let v = 0;
        &v as *const _ as u64
    };
    b = {
        let v = 0;
        &v as *const _ as u64
    };
    // here it makes sense that the pointers are not equal
    // (different provenance)
    // but aren't both a and b usize (not pointers?)

    if a != b {
        println!("neq");
    }
    println!("0x{a:x}==0x{b:x} : {}", a == b);
    println!("0x{a:x}==0x{b:x} : {}", a == b);

    /*    let f = a as u64;
    let g = b as u64;
    print_type_of(&f);
    print_type_of(&g);*/
    //println!("f == g: {}", f == g);

    let e = a - b; // (usize - usize = usize)
    // , also should evaluate to 0

    //println!("e :{} {}", e, e == 1);
    //let g = std::hint::black_box(0);
    let f = 4 / e;

    println!("{}", f);
    //let f = if e == 0 { 4 } else { 0 };
    // uncomment this also for some funny time.
    //let arr: [u8; 1] = [1];

    // println!("arr[0]: {}", arr[e]);
    // you can also do this:
    //let e = e + 1;
    //println!("arr[1]: {}", arr[e]);
    //println!("{}==0:{}", e, e == 0);
    //println!("{}==0:{}", e, e == 0);
    //println!("arr[0]: {}", arr[e]);
}

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>());
//}
