//! NDArray support for the LabVIEW array types. This requires 64 bit to
//! access internal array elements.

use super::memory::NumericArrayResizable;
use super::{LVArray, LVArrayHandle};
use crate::errors::Result;
use ndarray::{ArrayView, ArrayViewMut, Dim, Ix};

macro_rules! array_with_dim {
    ($dim:literal) => {
        impl<T> LVArray<$dim, T> {
            /// Get the dimensions in the NDArray format.
            fn ndarray_dim(&self) -> Dim<[Ix; $dim]> {
                let sizes: [Ix; $dim] = self.dim_sizes.into();
                Dim(sizes)
            }

            /// Get the LabVIEW array as an NDArray view.
            pub fn ndarray_view(&self) -> ArrayView<T, Dim<[Ix; $dim]>> {
                let dim_sizes = self.ndarray_dim();
                let data = self.data_as_slice();
                ArrayView::from_shape(dim_sizes, data).unwrap()
            }

            /// Get the LabVIEW array as an NDArray mutable view.
            pub fn ndarray_view_mut(&mut self) -> ArrayViewMut<T, Dim<[Ix; $dim]>> {
                let dim_sizes = self.ndarray_dim();
                let data = self.data_as_slice_mut();
                ArrayViewMut::from_shape(dim_sizes, data).unwrap()
            }
        }

        // Implement the copy methods.
        impl<'array, T: Copy + NumericArrayResizable + 'array> LVArrayHandle<'array, $dim, T> {
            /// Set the LabVIEW array from the ND Array.
            ///
            /// It will resize the array to match the dimensions if required.
            pub fn copy_from_ndarray(
                &'array mut self,
                array: impl Into<ArrayView<'array, T, Dim<[Ix; $dim]>>>,
            ) -> Result<()> {
                self.copy_from_ndarray_view(array.into())
            }

            fn copy_from_ndarray_view(
                &'array mut self,
                array: ArrayView<'array, T, Dim<[Ix; $dim]>>,
            ) -> Result<()> {
                // If the size isn't right either resize if available or error.
                if array.raw_dim() != self.ndarray_dim() {
                    #[cfg(feature = "link")]
                    {
                        self.resize_array(array.shape().try_into()?)?;
                    }
                    #[cfg(not(feature = "link"))]
                    {
                        return Err(LVInteropError::ArrayDimensionMismatch);
                    }
                }

                let lv_array = unsafe { self.as_ref_mut()? };
                for (output, input) in lv_array.data_as_slice_mut().iter_mut().zip(array.iter()) {
                    *output = *input
                }
                Ok(())
            }
        }
    };
}

// NDarray only supports 6 const dims.
array_with_dim!(1);
array_with_dim!(2);
array_with_dim!(3);
array_with_dim!(4);
array_with_dim!(5);
array_with_dim!(6);
