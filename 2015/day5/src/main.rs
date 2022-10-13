fn main() {
    println!("Hello, world!");
}

fn contains_three_vowels()

#[allow(dead_code)]
fn is_nice_string(str: &str) -> bool {
    const VOWELS:[char;5] = ['a', 'e', 'i', 'o', 'u'];
    let s: String = str.to_owned();
    let mut chars: Vec<char> = s[..]
        .chars()
        .collect();
    chars
        .sort_by(|a, b| b.cmp(a));
    println!("test={:?}", chars);

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_vowels() {
        assert!(is_nice_string("aei"));
        assert!(is_nice_string("xazegov"));
        assert!(is_nice_string("aeiouaeiouaeiou"));
    }
}
