pub struct OnNewLine<T>(pub T);

impl<T> std::fmt::Display for OnNewLine<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        write!(f, "{}", self.0)
    }
}

impl<T> std::fmt::Debug for OnNewLine<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        write!(f, "{:?}", self.0)
    }
}
