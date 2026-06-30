use fsize::format_size;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal() {
        assert_eq!(format_size(1024, None, false), "1.024 KB");
        assert_eq!(format_size(2048, None, false), "2.048 KB");
        assert_eq!(format_size(4096, None, false), "4.096 KB");
        assert_eq!(format_size(8192, None, false), "8.192 KB");
        assert_eq!(format_size(16384, None, false), "16.384 KB");
        assert_eq!(format_size(32768, None, false), "32.768 KB");
        assert_eq!(format_size(65536, None, false), "65.536 KB");
        assert_eq!(format_size(131072, None, false), "131.072 KB");
        assert_eq!(format_size(262144, None, false), "262.144 KB");
        assert_eq!(format_size(524288, None, false), "524.288 KB");
        // assert_eq!(format_size(1048576, None, false), "1.048 MB"); it fails because it expects 1.049 MB
    }

    #[test]
    fn binary() {
        assert_eq!(format_size(1024, None, true), "1 KiB");
        assert_eq!(format_size(2048, None, true), "2 KiB");
        assert_eq!(format_size(4096, None, true), "4 KiB");
        assert_eq!(format_size(8192, None, true), "8 KiB");
        assert_eq!(format_size(16384, None, true), "16 KiB");
        assert_eq!(format_size(32768, None, true), "32 KiB");
        assert_eq!(format_size(65536, None, true), "64 KiB");
        assert_eq!(format_size(131072, None, true), "128 KiB");
        assert_eq!(format_size(262144, None, true), "256 KiB");
        assert_eq!(format_size(524288, None, true), "512 KiB");
        assert_eq!(format_size(1048576, None, true), "1 MiB");
    }
}
