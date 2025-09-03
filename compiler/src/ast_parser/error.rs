
type LocationError = (u32, u32); // (line, column)

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub error: String,
    pub location: LocationError
}
