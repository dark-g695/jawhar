pub struct Execve<'a> {
    pub args: &'a str,
}

impl<'a> Execve<'a> {
    pub const fn new(args: &'a str) -> Self {
        Self { args }
    }
}
