//! NDArray support for the LabVIEW array types.

use super::LVArray;
use ndarray::{ArrayView, ArrayViewMut, Dim, Ix};


impl<const D: usize, T> LVArray<D, T> {

    /// Get the dimensions in the NDArray format.
    fn ndarray_dim(&self) -> Dim<[Ix; D]> {
        let mut usize_dims = [0usize; D];
        usize_dims.iter_mut().zip(self.dim_sizes.iter()).for_each(|(usize_dim, dim)| {
            *usize_dim = *dim as usize;
        });
        usize_dims.into()
    }

    /// Get the LabVIEW array as an NDArray view.
    pub fn ndarray_view(&self) -> ArrayView<T, Dim<[Ix; D]>> {
        let dim_sizes = self.ndarray_dim();
        let data = self.data_as_slice();
        ArrayView::from_shape(dim_sizes.into(), data).unwrap()
    }

    /// Get the LabVIEW array as an NDArray mutable view.
    pub fn ndarray_view_mut(&mut self) -> ArrayViewMut<T, Dim<[Ix; D]>> {
        let dim_sizes = self.ndarray_dim();
        let data = self.data_as_slice_mut();
        ArrayViewMut::from_shape(dim_sizes.into(), data).unwrap()
    }


}

#[cfg(test)]
mod tests {

    use super::*;
    use ndarray::{arr2, ArrayView1};

    #[test]
    fn test_ndarray_view() {
        let array = LVArray::<2, i32> {
            dim_sizes: [2, 3],
            data: *[1, 2, 3, 4, 5, 6],
        };

        let view = array.ndarray_view();
        let expected = arr2(&[[1, 2, 3], [4, 5, 6]]);
        assert_eq!(view, expected.view());
    }

    #[test]
    fn test_ndarray_view_mut() {
        let mut array = LVArray::<2, i32> {
            dim_sizes: [2, 3],
            data: *[1, 2, 3, 4, 5, 6],
        };

        let mut view = array.ndarray_view_mut();
        view[[0, 0]] = 10;
        view[[1, 2]] = 20;

        let mut expected = arr2(&[[10, 2, 3], [4, 5, 20]]);
        assert_eq!(view, expected.view_mut());
    }

    #[test]
    fn test_1d_array_view() {
        let array = LVArray::<1, i32> {
            dim_sizes: [3],
            data: *[1, 2, 3],
        };

        let view = array.ndarray_view();
        let expected = ArrayView1::from(&[1, 2, 3]);
        assert_eq!(view, expected);
    }
}