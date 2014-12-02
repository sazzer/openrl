pub fn openrl() -> String {
    "Hello".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_openrl() {
        let v = ::openrl();
        assert!(v == "Hello".to_string());
    }
}
