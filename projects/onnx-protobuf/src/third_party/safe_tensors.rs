use safetensors::{Dtype, SafeTensorError};
use safetensors::tensor::TensorView;

use crate::TensorProto;

impl<'a, 'b> TryFrom<&'a TensorProto> for TensorView<'b>
    where 'a: 'b
{
    type Error = SafeTensorError;

    fn try_from(tensor: &'a TensorProto) -> Result<Self, Self::Error> {
        let dtype = get_type(&tensor.data_type);
        let shape = get_shape(tensor.dims.as_slice());
        TensorView::new(dtype, shape, tensor.raw_data())
    }
}

#[inline(always)]
fn get_shape(tensor: &[i64]) -> Vec<usize> {
    tensor.iter().map(|x| *x as usize).collect()
}

#[inline(always)]
fn get_type(tensor: &Option<i32>) -> Dtype {
    match tensor {
        Some(1) => Dtype::F32,
        Some(n) => unimplemented!("Dtype {} not implemented", n),
        None => Dtype::F32,
    }
}