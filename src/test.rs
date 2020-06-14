// import functionality form lib
mod lib;
// TODO write some tests
#[cfg(test)]
mod tests {
    #[test]
    fn hello() {
        assert_eq!(lib::http_get("www.notarealsite/foo/bar"), "");
    }
}
