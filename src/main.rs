use local_ip_address::local_ip;

fn main() {
    println!("Hello, {}", get_ip());
}

/// Get local IP address
fn get_ip() -> String {
    local_ip().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_ip_v4() {
        assert!(get_ip().contains("."));
        assert_eq!(4, get_ip().split(".").collect::<Vec<_>>().len());
    }
}
