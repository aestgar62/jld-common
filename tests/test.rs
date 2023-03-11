#[cfg(test)]
mod tests {

    use jld_common::jld_common;

    #[test]
    fn test_jld_common() {
        let jld_common = jld_common::new();
        assert_eq!(jld_common, jld_common::default());
    }
}
