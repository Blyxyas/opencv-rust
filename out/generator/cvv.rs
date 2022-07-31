#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # GUI for Interactive Visual Debugging of Computer Vision Programs
//! 
//! Namespace for all functions is **cvv**, i.e. *cvv::showImage()*.
//! 
//! Compilation:
//! 
//! *   For development, i.e. for cvv GUI to show up, compile your code using cvv with
//!    *g++ -DCVVISUAL_DEBUGMODE*.
//! *   For release, i.e. cvv calls doing nothing, compile your code without above flag.
//! 
//! See cvv tutorial for a commented example application using cvv.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::CallMetaDataTraitConst, super::CallMetaDataTrait };
}

// debugDMatch(cv::InputArray, std::vector<cv::KeyPoint>, cv::InputArray, std::vector<cv::KeyPoint>, std::vector<cv::DMatch>, const cvv::impl::CallMetaData &, const char *, const char *, bool) /usr/include/opencv2/cvv/dmatch.hpp:24
#[inline]
pub fn debug_d_match(img1: &dyn core::ToInputArray, mut keypoints1: core::Vector<core::KeyPoint>, img2: &dyn core::ToInputArray, mut keypoints2: core::Vector<core::KeyPoint>, mut matches: core::Vector<core::DMatch>, data: &crate::cvv::CallMetaData, description: &str, view: &str, use_train_descriptor: bool) -> Result<()> {
	input_array_arg!(img1);
	input_array_arg!(img2);
	extern_container_arg!(description);
	extern_container_arg!(view);
	return_send!(via ocvrs_return);
	unsafe { sys::cvv_impl_debugDMatch_const__InputArrayR_vector_KeyPoint__const__InputArrayR_vector_KeyPoint__vector_DMatch__const_CallMetaDataR_const_charX_const_charX_bool(img1.as_raw__InputArray(), keypoints1.as_raw_mut_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_mut_VectorOfKeyPoint(), matches.as_raw_mut_VectorOfDMatch(), data.as_raw_CallMetaData(), description.opencv_as_extern(), view.opencv_as_extern(), use_train_descriptor, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// debugFilter(cv::InputArray, cv::InputArray, const cvv::impl::CallMetaData &, const char *, const char *) /usr/include/opencv2/cvv/filter.hpp:24
#[inline]
pub fn debug_filter(original: &dyn core::ToInputArray, result: &dyn core::ToInputArray, data: &crate::cvv::CallMetaData, description: &str, view: &str) -> Result<()> {
	input_array_arg!(original);
	input_array_arg!(result);
	extern_container_arg!(description);
	extern_container_arg!(view);
	return_send!(via ocvrs_return);
	unsafe { sys::cvv_impl_debugFilter_const__InputArrayR_const__InputArrayR_const_CallMetaDataR_const_charX_const_charX(original.as_raw__InputArray(), result.as_raw__InputArray(), data.as_raw_CallMetaData(), description.opencv_as_extern(), view.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// finalShow() /usr/include/opencv2/cvv/final_show.hpp:15
#[inline]
pub fn final_show() -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cvv_impl_finalShow(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// showImage(cv::InputArray, const cvv::impl::CallMetaData &, const char *, const char *) /usr/include/opencv2/cvv/show_image.hpp:24
#[inline]
pub fn show_image(img: &dyn core::ToInputArray, data: &crate::cvv::CallMetaData, description: &str, view: &str) -> Result<()> {
	input_array_arg!(img);
	extern_container_arg!(description);
	extern_container_arg!(view);
	return_send!(via ocvrs_return);
	unsafe { sys::cvv_impl_showImage_const__InputArrayR_const_CallMetaDataR_const_charX_const_charX(img.as_raw__InputArray(), data.as_raw_CallMetaData(), description.opencv_as_extern(), view.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// CallMetaData /usr/include/opencv2/cvv/call_meta_data.hpp:20
pub trait CallMetaDataTraitConst {
	fn as_raw_CallMetaData(&self) -> *const c_void;

	// file /usr/include/opencv2/cvv/call_meta_data.hpp:46
	#[inline]
	fn file(&self) -> String {
		let ret = unsafe { sys::cvv_impl_CallMetaData_getPropFile_const(self.as_raw_CallMetaData()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	// line /usr/include/opencv2/cvv/call_meta_data.hpp:47
	#[inline]
	fn line(&self) -> size_t {
		let ret = unsafe { sys::cvv_impl_CallMetaData_getPropLine_const(self.as_raw_CallMetaData()) };
		ret
	}
	
	// function /usr/include/opencv2/cvv/call_meta_data.hpp:48
	#[inline]
	fn function(&self) -> String {
		let ret = unsafe { sys::cvv_impl_CallMetaData_getPropFunction_const(self.as_raw_CallMetaData()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	// isKnown /usr/include/opencv2/cvv/call_meta_data.hpp:53
	#[inline]
	fn is_known(&self) -> bool {
		let ret = unsafe { sys::cvv_impl_CallMetaData_getPropIsKnown_const(self.as_raw_CallMetaData()) };
		ret
	}
	
}

pub trait CallMetaDataTrait: crate::cvv::CallMetaDataTraitConst {
	fn as_raw_mut_CallMetaData(&mut self) -> *mut c_void;

	// operator bool() /usr/include/opencv2/cvv/call_meta_data.hpp:40
	#[inline]
	fn to_bool(&mut self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cvv_impl_CallMetaData_operator_bool(self.as_raw_mut_CallMetaData(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// CallMetaData /usr/include/opencv2/cvv/call_meta_data.hpp:20
pub struct CallMetaData {
	ptr: *mut c_void
}

opencv_type_boxed! { CallMetaData }

impl Drop for CallMetaData {
	fn drop(&mut self) {
		extern "C" { fn cv_CallMetaData_delete(instance: *mut c_void); }
		unsafe { cv_CallMetaData_delete(self.as_raw_mut_CallMetaData()) };
	}
}

unsafe impl Send for CallMetaData {}

impl crate::cvv::CallMetaDataTraitConst for CallMetaData {
	#[inline] fn as_raw_CallMetaData(&self) -> *const c_void { self.as_raw() }
}

impl crate::cvv::CallMetaDataTrait for CallMetaData {
	#[inline] fn as_raw_mut_CallMetaData(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CallMetaData {
	// CallMetaData() /usr/include/opencv2/cvv/call_meta_data.hpp:26
	#[inline]
	pub fn default() -> Result<crate::cvv::CallMetaData> {
		return_send!(via ocvrs_return);
		unsafe { sys::cvv_impl_CallMetaData_CallMetaData(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::cvv::CallMetaData::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// CallMetaData(const char *, size_t, const char *) /usr/include/opencv2/cvv/call_meta_data.hpp:36
	#[inline]
	pub fn new(file: &str, line: size_t, function: &str) -> Result<crate::cvv::CallMetaData> {
		extern_container_arg!(file);
		extern_container_arg!(function);
		return_send!(via ocvrs_return);
		unsafe { sys::cvv_impl_CallMetaData_CallMetaData_const_charX_size_t_const_charX(file.opencv_as_extern(), line, function.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::cvv::CallMetaData::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
