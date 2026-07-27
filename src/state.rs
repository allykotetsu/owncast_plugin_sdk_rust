pub trait State {
    fn new() -> Self;
}

impl State for () {
    fn new() -> Self {
        ()
    }
}