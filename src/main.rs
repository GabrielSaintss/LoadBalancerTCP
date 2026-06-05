fn main() {
    let a = 10;
    let b = 20;
    let X = 1;
    let result = add(a, b);
    println!("Hello, world!");
    println!("{} + {} = {}", a, b, result);
}

fn add(x: i32, y: i32) -> i32 { x + y }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 999);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }
}
