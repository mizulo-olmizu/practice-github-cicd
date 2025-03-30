fn main() {
    println!("{}", even_or_odd(3));
    println!("Example v{}", env!("CARGO_PKG_VERSION"));
}

fn even_or_odd(n: i32) -> &'static str {
    if n % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_even_or_odd() {
        assert_eq!(even_or_odd(2), "Even");
        assert_eq!(even_or_odd(3), "Odd");
    }
}
