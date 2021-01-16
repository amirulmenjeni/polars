use crate::chunked_array::builder::get_list_builder;
use crate::chunked_array::kernels::concat::concat;
#[cfg(feature = "object")]
use crate::chunked_array::object::builder::ObjectChunkedBuilder;
use crate::prelude::*;
use arrow::array::{Array, ArrayRef};
#[cfg(feature = "object")]
use std::any::Any;
#[cfg(feature = "object")]
use std::fmt::Debug;

pub trait ChunkOps {
    /// Aggregate to contiguous memory.
    fn rechunk(&self) -> Result<Self>
    where
        Self: std::marker::Sized;
}

#[inline]
fn mimic_chunks<T>(arr: &ArrayRef, chunk_lengths: &[usize], name: &str) -> ChunkedArray<T>
where
    T: PolarsDataType,
    ChunkedArray<T>: ChunkOps,
{
    let mut chunks = Vec::with_capacity(chunk_lengths.len());
    let mut offset = 0;
    for chunk_length in chunk_lengths {
        chunks.push(arr.slice(offset, *chunk_length));
        offset += *chunk_length
    }
    ChunkedArray::new_from_chunks(name, chunks)
}

impl<T> ChunkOps for ChunkedArray<T>
where
    T: PolarsNumericType,
{
    fn rechunk(&self) -> Result<Self> {
        if self.chunks().len() == 1 {
            Ok(self.clone())
        } else {
            let chunks = vec![concat(&self.chunks)?];
            Ok(ChunkedArray::new_from_chunks(self.name(), chunks))
        }
    }
}

impl ChunkOps for BooleanChunked {
    fn rechunk(&self) -> Result<Self> {
        if self.chunks().len() == 1 {
            Ok(self.clone())
        } else {
            let chunks = vec![concat(&self.chunks)?];
            Ok(ChunkedArray::new_from_chunks(self.name(), chunks))
        }
    }
}

impl ChunkOps for Utf8Chunked {
    fn rechunk(&self) -> Result<Self> {
        if self.chunks().len() == 1 {
            Ok(self.clone())
        } else {
            let chunks = vec![concat(&self.chunks)?];
            Ok(ChunkedArray::new_from_chunks(self.name(), chunks))
        }
    }
}

impl ChunkOps for CategoricalChunked {
    fn rechunk(&self) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        self.cast::<UInt32Type>()?.rechunk()?.cast()
    }
}

impl ChunkOps for ListChunked {
    fn rechunk(&self) -> Result<Self> {
        if self.chunks.len() == 1 {
            Ok(self.clone())
        } else {
            let mut builder = get_list_builder(&self.dtype(), self.len(), self.name());
            for v in self {
                builder.append_opt_series(v.as_ref())
            }
            Ok(builder.finish())
        }
    }
}

#[cfg(feature = "object")]
impl<T> ChunkOps for ObjectChunked<T>
where
    T: Any + Debug + Clone + Send + Sync + Default,
{
    fn rechunk(&self) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        if self.chunks.len() == 1 {
            Ok(self.clone())
        } else {
            let mut builder = ObjectChunkedBuilder::new(self.name(), self.len());
            let chunks = self.downcast_chunks();

            // todo! use iterators once implemented
            // no_null path
            if self.null_count() == 0 {
                for idx in 0..self.len() {
                    let (chunk_idx, idx) = self.index_to_chunked_index(idx);
                    let arr = unsafe { &**chunks.get_unchecked(chunk_idx) };
                    builder.append_value(arr.value(idx).clone())
                }
            } else {
                for idx in 0..self.len() {
                    let (chunk_idx, idx) = self.index_to_chunked_index(idx);
                    let arr = unsafe { &**chunks.get_unchecked(chunk_idx) };
                    if arr.is_valid(idx) {
                        builder.append_value(arr.value(idx).clone())
                    } else {
                        builder.append_null()
                    }
                }
            }
            Ok(builder.finish())
        }
    }
}