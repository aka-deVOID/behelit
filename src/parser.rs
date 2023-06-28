trait Parser {
    fn header_parser(header: &[u8]);

    /// .
    fn parse(buffer: String) -> Self;
}
