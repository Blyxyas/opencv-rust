#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Stereo Correspondance Algorithms
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::QuasiDenseStereoConst, super::QuasiDenseStereo };
}

// CV_CS_CENSUS /usr/include/opencv2/stereo/descriptor.hpp:14
pub const CV_CS_CENSUS: i32 = 2;
// CV_DENSE_CENSUS /usr/include/opencv2/stereo/descriptor.hpp:13
pub const CV_DENSE_CENSUS: i32 = 0;
// CV_MEAN_VARIATION /usr/include/opencv2/stereo/descriptor.hpp:15
pub const CV_MEAN_VARIATION: i32 = 5;
// CV_MODIFIED_CENSUS_TRANSFORM /usr/include/opencv2/stereo/descriptor.hpp:14
pub const CV_MODIFIED_CENSUS_TRANSFORM: i32 = 4;
// CV_MODIFIED_CS_CENSUS /usr/include/opencv2/stereo/descriptor.hpp:14
pub const CV_MODIFIED_CS_CENSUS: i32 = 3;
// CV_QUADRATIC_INTERPOLATION /usr/include/opencv2/stereo.hpp:118
pub const CV_QUADRATIC_INTERPOLATION: i32 = 0;
// CV_SIMETRICV_INTERPOLATION /usr/include/opencv2/stereo.hpp:118
pub const CV_SIMETRICV_INTERPOLATION: i32 = 1;
// CV_SPARSE_CENSUS /usr/include/opencv2/stereo/descriptor.hpp:13
pub const CV_SPARSE_CENSUS: i32 = 1;
// CV_SPECKLE_REMOVAL_ALGORITHM /usr/include/opencv2/stereo.hpp:114
pub const CV_SPECKLE_REMOVAL_ALGORITHM: i32 = 0;
// CV_SPECKLE_REMOVAL_AVG_ALGORITHM /usr/include/opencv2/stereo.hpp:114
pub const CV_SPECKLE_REMOVAL_AVG_ALGORITHM: i32 = 1;
// CV_STAR_KERNEL /usr/include/opencv2/stereo/descriptor.hpp:15
pub const CV_STAR_KERNEL: i32 = 6;
// censusTransform(const cv::Mat &, const cv::Mat &, int, cv::Mat &, cv::Mat &, const int) /usr/include/opencv2/stereo/descriptor.hpp:22
#[inline]
pub fn census_transform(image1: &core::Mat, image2: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, dist2: &mut core::Mat, typ: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_censusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(image1.as_raw_Mat(), image2.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), dist2.as_raw_mut_Mat(), typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// censusTransform(const cv::Mat &, int, cv::Mat &, const int) /usr/include/opencv2/stereo/descriptor.hpp:24
#[inline]
pub fn census_transform_1(image1: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, typ: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_censusTransform_const_MatR_int_MatR_const_int(image1.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * t: 0
/// * integral_image1: Mat()
/// * integral_image2: Mat()
// modifiedCensusTransform(const cv::Mat &, const cv::Mat &, int, cv::Mat &, cv::Mat &, const int, int, const cv::Mat &, const cv::Mat &) /usr/include/opencv2/stereo/descriptor.hpp:29
#[inline]
pub fn modified_census_transform(img1: &core::Mat, img2: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, dist2: &mut core::Mat, typ: i32, t: i32, integral_image1: &core::Mat, integral_image2: &core::Mat) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_modifiedCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int_int_const_MatR_const_MatR(img1.as_raw_Mat(), img2.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), dist2.as_raw_mut_Mat(), typ, t, integral_image1.as_raw_Mat(), integral_image2.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * t: 0
/// * integral_image: Mat()
// modifiedCensusTransform(const cv::Mat &, int, cv::Mat &, const int, int, const cv::Mat &) /usr/include/opencv2/stereo/descriptor.hpp:31
#[inline]
pub fn modified_census_transform_1(img1: &core::Mat, kernel_size: i32, dist: &mut core::Mat, typ: i32, t: i32, integral_image: &core::Mat) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_modifiedCensusTransform_const_MatR_int_MatR_const_int_int_const_MatR(img1.as_raw_Mat(), kernel_size, dist.as_raw_mut_Mat(), typ, t, integral_image.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// starCensusTransform(const cv::Mat &, const cv::Mat &, int, cv::Mat &, cv::Mat &) /usr/include/opencv2/stereo/descriptor.hpp:39
#[inline]
pub fn star_census_transform(img1: &core::Mat, img2: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, dist2: &mut core::Mat) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_starCensusTransform_const_MatR_const_MatR_int_MatR_MatR(img1.as_raw_Mat(), img2.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), dist2.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// starCensusTransform(const cv::Mat &, int, cv::Mat &) /usr/include/opencv2/stereo/descriptor.hpp:41
#[inline]
pub fn star_census_transform_1(img1: &core::Mat, kernel_size: i32, dist: &mut core::Mat) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_starCensusTransform_const_MatR_int_MatR(img1.as_raw_Mat(), kernel_size, dist.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// symetricCensusTransform(const cv::Mat &, const cv::Mat &, int, cv::Mat &, cv::Mat &, const int) /usr/include/opencv2/stereo/descriptor.hpp:35
#[inline]
pub fn symetric_census_transform(img1: &core::Mat, img2: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, dist2: &mut core::Mat, typ: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_symetricCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(img1.as_raw_Mat(), img2.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), dist2.as_raw_mut_Mat(), typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// symetricCensusTransform(const cv::Mat &, int, cv::Mat &, const int) /usr/include/opencv2/stereo/descriptor.hpp:37
#[inline]
pub fn symetric_census_transform_1(img1: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, typ: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_symetricCensusTransform_const_MatR_int_MatR_const_int(img1.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// MatchQuasiDense /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:27
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MatchQuasiDense {
	pub p0: core::Point2i,
	pub p1: core::Point2i,
	pub corr: f32,
}

opencv_type_simple! { crate::stereo::MatchQuasiDense }

impl MatchQuasiDense {
	// MatchQuasiDense() /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:33
	#[inline]
	pub fn default() -> Result<crate::stereo::MatchQuasiDense> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_MatchQuasiDense_MatchQuasiDense(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// PropagationParameters /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:40
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PropagationParameters {
	pub corr_win_size_x: i32,
	pub corr_win_size_y: i32,
	pub border_x: i32,
	pub border_y: i32,
	pub correlation_threshold: f32,
	pub textrure_threshold: f32,
	pub neighborhood_size: i32,
	pub disparity_gradient: i32,
	pub lk_template_size: i32,
	pub lk_pyr_lvl: i32,
	pub lk_term_param1: i32,
	pub lk_term_param2: f32,
	pub gft_quality_thres: f32,
	pub gft_min_seperation_dist: i32,
	pub gft_max_num_features: i32,
}

opencv_type_simple! { crate::stereo::PropagationParameters }

impl PropagationParameters {
}

// QuasiDenseStereo /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:95
pub trait QuasiDenseStereoConst {
	fn as_raw_QuasiDenseStereo(&self) -> *const c_void;

	// Param /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:190
	#[inline]
	fn param(&self) -> crate::stereo::PropagationParameters {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_getPropParam_const(self.as_raw_QuasiDenseStereo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
}

pub trait QuasiDenseStereo: crate::stereo::QuasiDenseStereoConst {
	fn as_raw_mut_QuasiDenseStereo(&mut self) -> *mut c_void;

	// Param /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:190
	#[inline]
	fn set_param(&mut self, val: crate::stereo::PropagationParameters) {
		let ret = unsafe { sys::cv_stereo_QuasiDenseStereo_setPropParam_PropagationParameters(self.as_raw_mut_QuasiDenseStereo(), val.opencv_as_extern()) };
		ret
	}
	
	// loadParameters(cv::String) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:118
	#[inline]
	fn load_parameters(&mut self, filepath: &str) -> Result<i32> {
		extern_container_arg!(mut filepath);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_loadParameters_String(self.as_raw_mut_QuasiDenseStereo(), filepath.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// saveParameters(cv::String) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:129
	#[inline]
	fn save_parameters(&mut self, filepath: &str) -> Result<i32> {
		extern_container_arg!(mut filepath);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_saveParameters_String(self.as_raw_mut_QuasiDenseStereo(), filepath.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSparseMatches(std::vector<MatchQuasiDense> &) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:138
	#[inline]
	fn get_sparse_matches(&mut self, s_matches: &mut core::Vector<crate::stereo::MatchQuasiDense>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_getSparseMatches_vector_MatchQuasiDense_R(self.as_raw_mut_QuasiDenseStereo(), s_matches.as_raw_mut_VectorOfMatchQuasiDense(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getDenseMatches(std::vector<MatchQuasiDense> &) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:147
	#[inline]
	fn get_dense_matches(&mut self, dense_matches: &mut core::Vector<crate::stereo::MatchQuasiDense>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_getDenseMatches_vector_MatchQuasiDense_R(self.as_raw_mut_QuasiDenseStereo(), dense_matches.as_raw_mut_VectorOfMatchQuasiDense(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// process(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:163
	#[inline]
	fn process(&mut self, img_left: &core::Mat, img_right: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_process_const_MatR_const_MatR(self.as_raw_mut_QuasiDenseStereo(), img_left.as_raw_Mat(), img_right.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getMatch(const int, const int) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:174
	#[inline]
	fn get_match(&mut self, x: i32, y: i32) -> Result<core::Point2f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_getMatch_const_int_const_int(self.as_raw_mut_QuasiDenseStereo(), x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getDisparity() /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:184
	#[inline]
	fn get_disparity(&mut self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_getDisparity(self.as_raw_mut_QuasiDenseStereo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl dyn QuasiDenseStereo + '_ {
	/// ## C++ default parameters
	/// * param_filepath: cv::String()
	// create(cv::Size, cv::String) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:187
	#[inline]
	pub fn create(mono_img_size: core::Size, param_filepath: &str) -> Result<core::Ptr<dyn crate::stereo::QuasiDenseStereo>> {
		extern_container_arg!(mut param_filepath);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_create_Size_String(mono_img_size.opencv_as_extern(), param_filepath.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::stereo::QuasiDenseStereo>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}