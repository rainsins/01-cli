fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let adds = 23;
    println!("add is {}", add(adds, 33));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
