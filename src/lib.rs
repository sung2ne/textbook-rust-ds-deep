impl<T: Ord> BinarySearchTree<T> {
    pub fn min(&self) -> Option<&T> {
        let mut current = self.root.as_deref()?;
        while let Some(left) = current.left.as_deref() {
            current = left;
        }
        Some(&current.value)
    }

    pub fn max(&self) -> Option<&T> {
        let mut current = self.root.as_deref()?;
        while let Some(right) = current.right.as_deref() {
            current = right;
        }
        Some(&current.value)
    }
}