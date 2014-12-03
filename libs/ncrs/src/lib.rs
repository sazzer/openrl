pub fn ncrs() -> String {
    "Hello".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ncrs() {
        let v = ::ncrs();
        assert!(v == "Hello".to_string());
    }
}

