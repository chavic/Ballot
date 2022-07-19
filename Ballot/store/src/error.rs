pub struct StoreError {}

impl From<std::io::Error> for StoreError {
    fn from(_: std::io::Error) -> Self {
        todo!("Impliment io error conversion")
    }
}
