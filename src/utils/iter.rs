/// Check that the iterator only contains at most one distinct value.
///
/// # Parameters
///
/// * `iterator` - Any Iterator
///
/// # Return
///
/// `true` if the iterator contain zero or one distinct value. `false`
/// otherwise.
pub fn all_same<T: PartialEq>(iterator: &mut impl Iterator<Item = T>) -> bool {
    if let Some(first) = iterator.next() {
        iterator.all(|x| x == first)
    } else {
        true
    }
}
