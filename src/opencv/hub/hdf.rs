#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Hierarchical Data Format I/O routines
//! 
//! This module provides storage routines for Hierarchical Data Format objects.
//!    # Hierarchical Data Format version 5
//! 
//! Hierarchical Data Format version 5
//! --------------------------------------------------------
//! 
//! In order to use it, the hdf5 library has to be installed, which
//! means cmake should find it using `find_package(HDF5)` .
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::HDF5Const, super::HDF5 };
}

pub const HDF5_H5_GETCHUNKDIMS: i32 = 102;
pub const HDF5_H5_GETDIMS: i32 = 100;
pub const HDF5_H5_GETMAXDIMS: i32 = 101;
pub const HDF5_H5_NONE: i32 = -1;
pub const HDF5_H5_UNLIMITED: i32 = -1;
#[inline]
pub fn open(hdf5_filename: &str) -> Result<core::Ptr<dyn crate::hdf::HDF5>> {
	extern_container_arg!(hdf5_filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_hdf_open_const_StringR(hdf5_filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::hdf::HDF5>::opencv_from_extern(ret) };
	Ok(ret)
}

pub trait HDF5Const {
	fn as_raw_HDF5(&self) -> *const c_void;

	#[inline]
	fn hlexists(&self, label: &str) -> Result<bool> {
		extern_container_arg!(label);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_hlexists_const_const_StringR(self.as_raw_HDF5(), label.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn atexists(&self, atlabel: &str) -> Result<bool> {
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atexists_const_const_StringR(self.as_raw_HDF5(), atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn dscreate(&self, rows: i32, cols: i32, typ: i32, dslabel: &str) -> Result<()> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR(self.as_raw_HDF5(), rows, cols, typ, dslabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn dscreate_1(&self, rows: i32, cols: i32, typ: i32, dslabel: &str, compresslevel: i32) -> Result<()> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR_const_int(self.as_raw_HDF5(), rows, cols, typ, dslabel.opencv_as_extern(), compresslevel, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn dscreate_2(&self, rows: i32, cols: i32, typ: i32, dslabel: &str, compresslevel: i32, dims_chunks: &core::Vector<i32>) -> Result<()> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR_const_int_const_vector_int_R(self.as_raw_HDF5(), rows, cols, typ, dslabel.opencv_as_extern(), compresslevel, dims_chunks.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn dscreate_3(&self, rows: i32, cols: i32, typ: i32, dslabel: &str, compresslevel: i32, dims_chunks: &i32) -> Result<()> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR_const_int_const_intX(self.as_raw_HDF5(), rows, cols, typ, dslabel.opencv_as_extern(), compresslevel, dims_chunks, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn dscreate_4(&self, n_dims: i32, sizes: &i32, typ: i32, dslabel: &str) -> Result<()> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR(self.as_raw_HDF5(), n_dims, sizes, typ, dslabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn dscreate_5(&self, n_dims: i32, sizes: &i32, typ: i32, dslabel: &str, compresslevel: i32) -> Result<()> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR_const_int(self.as_raw_HDF5(), n_dims, sizes, typ, dslabel.opencv_as_extern(), compresslevel, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * compresslevel: HDF5::H5_NONE
	/// * dims_chunks: vector<int>()
	#[inline]
	fn dscreate_6(&self, sizes: &core::Vector<i32>, typ: i32, dslabel: &str, compresslevel: i32, dims_chunks: &core::Vector<i32>) -> Result<()> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_vector_int_R_const_int_const_StringR_const_int_const_vector_int_R(self.as_raw_HDF5(), sizes.as_raw_VectorOfi32(), typ, dslabel.opencv_as_extern(), compresslevel, dims_chunks.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn dscreate_7(&self, n_dims: i32, sizes: &i32, typ: i32, dslabel: &str, compresslevel: i32, dims_chunks: &i32) -> Result<()> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR_const_int_const_intX(self.as_raw_HDF5(), n_dims, sizes, typ, dslabel.opencv_as_extern(), compresslevel, dims_chunks, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * dims_flag: HDF5::H5_GETDIMS
	#[inline]
	fn dsgetsize(&self, dslabel: &str, dims_flag: i32) -> Result<core::Vector<i32>> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsgetsize_const_const_StringR_int(self.as_raw_HDF5(), dslabel.opencv_as_extern(), dims_flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn dsgettype(&self, dslabel: &str) -> Result<i32> {
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsgettype_const_const_StringR(self.as_raw_HDF5(), dslabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn dswrite(&self, array: &dyn core::ToInputArray, dslabel: &str) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn dswrite_1(&self, array: &dyn core::ToInputArray, dslabel: &str, dims_offset: &i32) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR_const_intX(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern(), dims_offset, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * dims_counts: vector<int>()
	#[inline]
	fn dswrite_2(&self, array: &dyn core::ToInputArray, dslabel: &str, dims_offset: &core::Vector<i32>, dims_counts: &core::Vector<i32>) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR_const_vector_int_R_const_vector_int_R(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern(), dims_offset.as_raw_VectorOfi32(), dims_counts.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn dswrite_3(&self, array: &dyn core::ToInputArray, dslabel: &str, dims_offset: &i32, dims_counts: &i32) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR_const_intX_const_intX(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern(), dims_offset, dims_counts, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn dsinsert(&self, array: &dyn core::ToInputArray, dslabel: &str) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn dsinsert_1(&self, array: &dyn core::ToInputArray, dslabel: &str, dims_offset: &i32) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR_const_intX(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern(), dims_offset, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * dims_counts: vector<int>()
	#[inline]
	fn dsinsert_2(&self, array: &dyn core::ToInputArray, dslabel: &str, dims_offset: &core::Vector<i32>, dims_counts: &core::Vector<i32>) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR_const_vector_int_R_const_vector_int_R(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern(), dims_offset.as_raw_VectorOfi32(), dims_counts.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn dsinsert_3(&self, array: &dyn core::ToInputArray, dslabel: &str, dims_offset: &i32, dims_counts: &i32) -> Result<()> {
		input_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR_const_intX_const_intX(self.as_raw_HDF5(), array.as_raw__InputArray(), dslabel.opencv_as_extern(), dims_offset, dims_counts, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn dsread(&self, array: &mut dyn core::ToOutputArray, dslabel: &str) -> Result<()> {
		output_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR(self.as_raw_HDF5(), array.as_raw__OutputArray(), dslabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn dsread_1(&self, array: &mut dyn core::ToOutputArray, dslabel: &str, dims_offset: &i32) -> Result<()> {
		output_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR_const_intX(self.as_raw_HDF5(), array.as_raw__OutputArray(), dslabel.opencv_as_extern(), dims_offset, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * dims_counts: vector<int>()
	#[inline]
	fn dsread_2(&self, array: &mut dyn core::ToOutputArray, dslabel: &str, dims_offset: &core::Vector<i32>, dims_counts: &core::Vector<i32>) -> Result<()> {
		output_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR_const_vector_int_R_const_vector_int_R(self.as_raw_HDF5(), array.as_raw__OutputArray(), dslabel.opencv_as_extern(), dims_offset.as_raw_VectorOfi32(), dims_counts.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn dsread_3(&self, array: &mut dyn core::ToOutputArray, dslabel: &str, dims_offset: &i32, dims_counts: &i32) -> Result<()> {
		output_array_arg!(array);
		extern_container_arg!(dslabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR_const_intX_const_intX(self.as_raw_HDF5(), array.as_raw__OutputArray(), dslabel.opencv_as_extern(), dims_offset, dims_counts, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * dims_flag: HDF5::H5_GETDIMS
	#[inline]
	fn kpgetsize(&self, kplabel: &str, dims_flag: i32) -> Result<i32> {
		extern_container_arg!(kplabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_kpgetsize_const_const_StringR_int(self.as_raw_HDF5(), kplabel.opencv_as_extern(), dims_flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * compresslevel: H5_NONE
	/// * chunks: H5_NONE
	#[inline]
	fn kpcreate(&self, size: i32, kplabel: &str, compresslevel: i32, chunks: i32) -> Result<()> {
		extern_container_arg!(kplabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_kpcreate_const_const_int_const_StringR_const_int_const_int(self.as_raw_HDF5(), size, kplabel.opencv_as_extern(), compresslevel, chunks, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * offset: H5_NONE
	/// * counts: H5_NONE
	#[inline]
	fn kpwrite(&self, keypoints: core::Vector<core::KeyPoint>, kplabel: &str, offset: i32, counts: i32) -> Result<()> {
		extern_container_arg!(kplabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_kpwrite_const_const_vector_KeyPoint__const_StringR_const_int_const_int(self.as_raw_HDF5(), keypoints.as_raw_VectorOfKeyPoint(), kplabel.opencv_as_extern(), offset, counts, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * offset: H5_NONE
	/// * counts: H5_NONE
	#[inline]
	fn kpinsert(&self, keypoints: core::Vector<core::KeyPoint>, kplabel: &str, offset: i32, counts: i32) -> Result<()> {
		extern_container_arg!(kplabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_kpinsert_const_const_vector_KeyPoint__const_StringR_const_int_const_int(self.as_raw_HDF5(), keypoints.as_raw_VectorOfKeyPoint(), kplabel.opencv_as_extern(), offset, counts, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * offset: H5_NONE
	/// * counts: H5_NONE
	#[inline]
	fn kpread(&self, keypoints: &mut core::Vector<core::KeyPoint>, kplabel: &str, offset: i32, counts: i32) -> Result<()> {
		extern_container_arg!(kplabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_kpread_const_vector_KeyPoint_R_const_StringR_const_int_const_int(self.as_raw_HDF5(), keypoints.as_raw_mut_VectorOfKeyPoint(), kplabel.opencv_as_extern(), offset, counts, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait HDF5: crate::hdf::HDF5Const {
	fn as_raw_mut_HDF5(&mut self) -> *mut c_void;

	#[inline]
	fn close(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_close(self.as_raw_mut_HDF5(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn grcreate(&mut self, grlabel: &str) -> Result<()> {
		extern_container_arg!(grlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_grcreate_const_StringR(self.as_raw_mut_HDF5(), grlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn atdelete(&mut self, atlabel: &str) -> Result<()> {
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atdelete_const_StringR(self.as_raw_mut_HDF5(), atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn atwrite(&mut self, value: i32, atlabel: &str) -> Result<()> {
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atwrite_const_int_const_StringR(self.as_raw_mut_HDF5(), value, atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn atread(&mut self, value: &mut i32, atlabel: &str) -> Result<()> {
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atread_intX_const_StringR(self.as_raw_mut_HDF5(), value, atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn atwrite_1(&mut self, value: f64, atlabel: &str) -> Result<()> {
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atwrite_const_double_const_StringR(self.as_raw_mut_HDF5(), value, atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn atread_1(&mut self, value: &mut f64, atlabel: &str) -> Result<()> {
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atread_doubleX_const_StringR(self.as_raw_mut_HDF5(), value, atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn atwrite_2(&mut self, value: &str, atlabel: &str) -> Result<()> {
		extern_container_arg!(value);
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atwrite_const_StringR_const_StringR(self.as_raw_mut_HDF5(), value.opencv_as_extern(), atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn atread_2(&mut self, value: &mut String, atlabel: &str) -> Result<()> {
		string_arg_output_send!(via value_via);
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atread_StringX_const_StringR(self.as_raw_mut_HDF5(), &mut value_via, atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		string_arg_output_receive!(value_via => value);
		Ok(ret)
	}
	
	#[inline]
	fn atwrite_3(&mut self, value: &dyn core::ToInputArray, atlabel: &str) -> Result<()> {
		input_array_arg!(value);
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atwrite_const__InputArrayR_const_StringR(self.as_raw_mut_HDF5(), value.as_raw__InputArray(), atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn atread_3(&mut self, value: &mut dyn core::ToOutputArray, atlabel: &str) -> Result<()> {
		output_array_arg!(value);
		extern_container_arg!(atlabel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_hdf_HDF5_atread_const__OutputArrayR_const_StringR(self.as_raw_mut_HDF5(), value.as_raw__OutputArray(), atlabel.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
