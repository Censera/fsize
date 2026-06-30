use fsize::format_size;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size_decimal() {
        assert_eq!(format_size(1500, None, false), "1.5 KB");
        assert_eq!(format_size(2048, None, false), "2.0 KB");
    }

    #[test]
    fn test_format_size_binary() {
        assert_eq!(format_size(1024, None, true), "1 KiB");
        assert_eq!(format_size(2048, None, true), "2 KiB");
    }
}
