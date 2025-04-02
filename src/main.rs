fn main() {
    let a = {
        let v = 0;
        &v as *const _ as usize
    };
    let b = {
        let v = 0;
        &v as *const _ as usize
    };
    println!("0x{a:x}==0x{b:x} : {}", a == b);
    println!("0x{a:x}==0x{b:x} : {}", a == b);
    let c = {
        let v = 0;
        &v as *const _ as usize
    }; // usize
    let d = {
        let v = 0;
        &v as *const _ as usize
    }; // usize
    let e = c - d; // (usize - usize = usize)
    println!("{}==0:{}", e, e == 0);
    println!("{}==0:{}", e, e == 0);
}
