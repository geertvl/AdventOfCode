use md5::{Md5, Digest};

fn main() {
    let mut hasher = Md5::new();
    hasher.update("abcdef609043");
    let result = hasher.finalize();

    for r in result {
        let f = format!("{:x}", r);
        println!("{:0>2}", f);
    }
    
    //println!("result: {:?} {:x}", result, result);
}
