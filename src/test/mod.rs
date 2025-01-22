// boiler add fn
#[allow(dead_code)]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// boiler testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boiler_test(){
        assert_eq!(add(1, 3), 4)
    }
}