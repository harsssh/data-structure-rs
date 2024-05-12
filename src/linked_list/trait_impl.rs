use crate::linked_list::method::LinkedList;

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> From<[T; N]> for LinkedList<T> {
    fn from(array: [T; N]) -> Self {
        let mut list = LinkedList::new();
        for item in array {
            list.push_back(item)
        }
        list
    }
}
