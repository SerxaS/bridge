#[cfg(test)]
mod tests {
    use crate::middleware::provider;

    #[test]
    fn block_number() {
        let _ = provider::main();
    }
}
