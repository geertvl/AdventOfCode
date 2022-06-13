use md5::{Md5, Digest};

fn main() {
    let mut counter = 0;
    let input = "ckczppom";
    let mut found = false;
    while !found {
        counter += 1;
        let mut hasher = Md5::new();

        hasher.update(format!("{}{}", input, counter));

        let result = hasher.finalize();
        let mut str = String::new();
        for r in result {
            let f = format!("{:0>2x}", r);
            str.push_str(&*f);
        }

        let first_five_chars = &str[..6];
        if first_five_chars == "000000" {
            found = true;
        }
    }

    println!("counter {}", counter);
}
