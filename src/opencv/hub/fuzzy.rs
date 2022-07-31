#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Image processing based on fuzzy mathematics
//! 
//! Namespace for all functions is `ft`. The module brings implementation of the last image processing algorithms based on fuzzy mathematics. Method are named based on the pattern `FT`_degree_dimension`_`method.
//!    # Math with F0-transform support
//! 
//! Fuzzy transform (![inline formula](https://latex.codecogs.com/png.latex?F%5E0)-transform) of the 0th degree transforms whole image to a matrix of its components. These components are used in latter computation where each of them represents average color of certain subarea.
//! 
//!    # Math with F1-transform support
//! 
//! Fuzzy transform (![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transform) of the 1th degree transforms whole image to a matrix of its components. Each component is polynomial of the 1th degree carrying information about average color and average gradient of certain subarea.
//! 
//!    # Fuzzy image processing
//! 
//! Image proceesing based on fuzzy mathematics namely F-transform.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use {  };
}

pub const ITERATIVE: i32 = 3;
pub const LINEAR: i32 = 1;
pub const MULTI_STEP: i32 = 2;
pub const ONE_STEP: i32 = 1;
pub const SINUS: i32 = 2;
#[inline]
pub fn ft02_d_fl_process(matrix: &dyn core::ToInputArray, radius: i32, output: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(matrix);
	output_array_arg!(output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT02D_FL_process_const__InputArrayR_const_int_const__OutputArrayR(matrix.as_raw__InputArray(), radius, output.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn ft02_d_fl_process_float(matrix: &dyn core::ToInputArray, radius: i32, output: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(matrix);
	output_array_arg!(output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT02D_FL_process_float_const__InputArrayR_const_int_const__OutputArrayR(matrix.as_raw__InputArray(), radius, output.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * mask: noArray()
#[inline]
pub fn ft02_d_components(matrix: &dyn core::ToInputArray, kernel: &dyn core::ToInputArray, components: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(components);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT02D_components_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), components.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn ft02_d_inverse_ft(components: &dyn core::ToInputArray, kernel: &dyn core::ToInputArray, output: &mut dyn core::ToOutputArray, width: i32, height: i32) -> Result<()> {
	input_array_arg!(components);
	input_array_arg!(kernel);
	output_array_arg!(output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT02D_inverseFT_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int(components.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray(), width, height, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn ft02_d_iteration(matrix: &dyn core::ToInputArray, kernel: &dyn core::ToInputArray, output: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray, mask_output: &mut dyn core::ToOutputArray, first_stop: bool) -> Result<i32> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(output);
	input_array_arg!(mask);
	output_array_arg!(mask_output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT02D_iteration_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__OutputArrayR_bool(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray(), mask.as_raw__InputArray(), mask_output.as_raw__OutputArray(), first_stop, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * mask: noArray()
#[inline]
pub fn ft02_d_process(matrix: &dyn core::ToInputArray, kernel: &dyn core::ToInputArray, output: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(output);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT02D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn ft12_d_components(matrix: &dyn core::ToInputArray, kernel: &dyn core::ToInputArray, components: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(components);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT12D_components_const__InputArrayR_const__InputArrayR_const__OutputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), components.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn ft12_d_create_polynom_matrix_horizontal(radius: i32, matrix: &mut dyn core::ToOutputArray, chn: i32) -> Result<()> {
	output_array_arg!(matrix);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT12D_createPolynomMatrixHorizontal_int_const__OutputArrayR_const_int(radius, matrix.as_raw__OutputArray(), chn, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn ft12_d_create_polynom_matrix_vertical(radius: i32, matrix: &mut dyn core::ToOutputArray, chn: i32) -> Result<()> {
	output_array_arg!(matrix);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT12D_createPolynomMatrixVertical_int_const__OutputArrayR_const_int(radius, matrix.as_raw__OutputArray(), chn, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn ft12_d_inverse_ft(components: &dyn core::ToInputArray, kernel: &dyn core::ToInputArray, output: &mut dyn core::ToOutputArray, width: i32, height: i32) -> Result<()> {
	input_array_arg!(components);
	input_array_arg!(kernel);
	output_array_arg!(output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT12D_inverseFT_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int(components.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray(), width, height, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * mask: noArray()
#[inline]
pub fn ft12_d_polynomial(matrix: &dyn core::ToInputArray, kernel: &dyn core::ToInputArray, c00: &mut dyn core::ToOutputArray, c10: &mut dyn core::ToOutputArray, c01: &mut dyn core::ToOutputArray, components: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(c00);
	output_array_arg!(c10);
	output_array_arg!(c01);
	output_array_arg!(components);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT12D_polynomial_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), c00.as_raw__OutputArray(), c10.as_raw__OutputArray(), c01.as_raw__OutputArray(), components.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * mask: noArray()
#[inline]
pub fn ft12_d_process(matrix: &dyn core::ToInputArray, kernel: &dyn core::ToInputArray, output: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(output);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT12D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn create_kernel1(a: &dyn core::ToInputArray, b: &dyn core::ToInputArray, kernel: &mut dyn core::ToOutputArray, chn: i32) -> Result<()> {
	input_array_arg!(a);
	input_array_arg!(b);
	output_array_arg!(kernel);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_createKernel_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_int(a.as_raw__InputArray(), b.as_raw__InputArray(), kernel.as_raw__OutputArray(), chn, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn create_kernel(function: i32, radius: i32, kernel: &mut dyn core::ToOutputArray, chn: i32) -> Result<()> {
	output_array_arg!(kernel);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_createKernel_int_int_const__OutputArrayR_const_int(function, radius, kernel.as_raw__OutputArray(), chn, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn filter(image: &dyn core::ToInputArray, kernel: &dyn core::ToInputArray, output: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(image);
	input_array_arg!(kernel);
	output_array_arg!(output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR(image.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn inpaint(image: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, output: &mut dyn core::ToOutputArray, radius: i32, function: i32, algorithm: i32) -> Result<()> {
	input_array_arg!(image);
	input_array_arg!(mask);
	output_array_arg!(output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_inpaint_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int(image.as_raw__InputArray(), mask.as_raw__InputArray(), output.as_raw__OutputArray(), radius, function, algorithm, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}
