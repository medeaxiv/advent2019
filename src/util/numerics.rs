pub fn min_max<T>(a: T, b: T) -> (T, T)
where
    T: PartialOrd,
{
    if b < a {
        (b, a)
    } else {
        (a, b)
    }
}
