#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Computational Photography
//! 
//! This module includes photo processing algorithms
//!    # Inpainting
//!    # Denoising
//!    # HDR imaging
//! 
//! This section describes high dynamic range imaging algorithms namely tonemapping, exposure alignment,
//! camera calibration with multiple exposures and exposure fusion.
//! 
//!    # Contrast Preserving Decolorization
//! 
//! Useful links:
//! 
//! http://www.cse.cuhk.edu.hk/leojia/projects/color2gray/index.html
//! 
//!    # Seamless Cloning
//! 
//! Useful links:
//! 
//! https://www.learnopencv.com/seamless-cloning-using-opencv-python-cpp
//! 
//!    # Non-Photorealistic Rendering
//! 
//! Useful links:
//! 
//! http://www.inf.ufrgs.br/~eslgastal/DomainTransform
//! 
//! https://www.learnopencv.com/non-photorealistic-rendering-using-opencv-python-c/
//! 
//!    # C API
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::TonemapConst, super::Tonemap, super::TonemapDragoConst, super::TonemapDrago, super::TonemapReinhardConst, super::TonemapReinhard, super::TonemapMantiukConst, super::TonemapMantiuk, super::AlignExposuresConst, super::AlignExposures, super::AlignMTBConst, super::AlignMTB, super::CalibrateCRFConst, super::CalibrateCRF, super::CalibrateDebevecConst, super::CalibrateDebevec, super::CalibrateRobertsonConst, super::CalibrateRobertson, super::MergeExposuresConst, super::MergeExposures, super::MergeDebevecConst, super::MergeDebevec, super::MergeMertensConst, super::MergeMertens, super::MergeRobertsonConst, super::MergeRobertson };
}

// INPAINT_NS /usr/include/opencv2/photo.hpp:96
pub const INPAINT_NS: i32 = 0;
// INPAINT_TELEA /usr/include/opencv2/photo.hpp:97
pub const INPAINT_TELEA: i32 = 1;
// LDR_SIZE /usr/include/opencv2/photo.hpp:332
pub const LDR_SIZE: i32 = 256;
// MIXED_CLONE /usr/include/opencv2/photo.hpp:719
pub const MIXED_CLONE: i32 = 2;
// MONOCHROME_TRANSFER /usr/include/opencv2/photo.hpp:721
pub const MONOCHROME_TRANSFER: i32 = 3;
// NORMAL_CLONE /usr/include/opencv2/photo.hpp:716
pub const NORMAL_CLONE: i32 = 1;
// NORMCONV_FILTER /usr/include/opencv2/photo.hpp:800
pub const NORMCONV_FILTER: i32 = 2;
// RECURS_FILTER /usr/include/opencv2/photo.hpp:799
pub const RECURS_FILTER: i32 = 1;
/// ## C++ default parameters
/// * red_mul: 1.0f
/// * green_mul: 1.0f
/// * blue_mul: 1.0f
// colorChange(cv::InputArray, cv::InputArray, cv::OutputArray, float, float, float) /usr/include/opencv2/photo.hpp:755
#[inline]
pub fn color_change(src: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, red_mul: f32, green_mul: f32, blue_mul: f32) -> Result<()> {
	input_array_arg!(src);
	input_array_arg!(mask);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_colorChange_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_float_float(src.as_raw__InputArray(), mask.as_raw__InputArray(), dst.as_raw__OutputArray(), red_mul, green_mul, blue_mul, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * max_bits: 6
/// * exclude_range: 4
/// * cut: true
// createAlignMTB(int, int, bool) /usr/include/opencv2/photo.hpp:527
#[inline]
pub fn create_align_mtb(max_bits: i32, exclude_range: i32, cut: bool) -> Result<core::Ptr<dyn crate::photo::AlignMTB>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createAlignMTB_int_int_bool(max_bits, exclude_range, cut, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::photo::AlignMTB>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * samples: 70
/// * lambda: 10.0f
/// * random: false
// createCalibrateDebevec(int, float, bool) /usr/include/opencv2/photo.hpp:570
#[inline]
pub fn create_calibrate_debevec(samples: i32, lambda: f32, random: bool) -> Result<core::Ptr<dyn crate::photo::CalibrateDebevec>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createCalibrateDebevec_int_float_bool(samples, lambda, random, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::photo::CalibrateDebevec>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * max_iter: 30
/// * threshold: 0.01f
// createCalibrateRobertson(int, float) /usr/include/opencv2/photo.hpp:594
#[inline]
pub fn create_calibrate_robertson(max_iter: i32, threshold: f32) -> Result<core::Ptr<dyn crate::photo::CalibrateRobertson>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createCalibrateRobertson_int_float(max_iter, threshold, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::photo::CalibrateRobertson>::opencv_from_extern(ret) };
	Ok(ret)
}

// createMergeDebevec() /usr/include/opencv2/photo.hpp:628
#[inline]
pub fn create_merge_debevec() -> Result<core::Ptr<dyn crate::photo::MergeDebevec>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createMergeDebevec(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::photo::MergeDebevec>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * contrast_weight: 1.0f
/// * saturation_weight: 1.0f
/// * exposure_weight: 0.0f
// createMergeMertens(float, float, float) /usr/include/opencv2/photo.hpp:670
#[inline]
pub fn create_merge_mertens(contrast_weight: f32, saturation_weight: f32, exposure_weight: f32) -> Result<core::Ptr<dyn crate::photo::MergeMertens>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createMergeMertens_float_float_float(contrast_weight, saturation_weight, exposure_weight, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::photo::MergeMertens>::opencv_from_extern(ret) };
	Ok(ret)
}

// createMergeRobertson() /usr/include/opencv2/photo.hpp:687
#[inline]
pub fn create_merge_robertson() -> Result<core::Ptr<dyn crate::photo::MergeRobertson>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createMergeRobertson(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::photo::MergeRobertson>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * gamma: 1.0f
/// * saturation: 1.0f
/// * bias: 0.85f
// createTonemapDrago(float, float, float) /usr/include/opencv2/photo.hpp:387
#[inline]
pub fn create_tonemap_drago(gamma: f32, saturation: f32, bias: f32) -> Result<core::Ptr<dyn crate::photo::TonemapDrago>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createTonemapDrago_float_float_float(gamma, saturation, bias, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::photo::TonemapDrago>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * gamma: 1.0f
/// * scale: 0.7f
/// * saturation: 1.0f
// createTonemapMantiuk(float, float, float) /usr/include/opencv2/photo.hpp:446
#[inline]
pub fn create_tonemap_mantiuk(gamma: f32, scale: f32, saturation: f32) -> Result<core::Ptr<dyn crate::photo::TonemapMantiuk>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createTonemapMantiuk_float_float_float(gamma, scale, saturation, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::photo::TonemapMantiuk>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * gamma: 1.0f
/// * intensity: 0.0f
/// * light_adapt: 1.0f
/// * color_adapt: 0.0f
// createTonemapReinhard(float, float, float, float) /usr/include/opencv2/photo.hpp:420
#[inline]
pub fn create_tonemap_reinhard(gamma: f32, intensity: f32, light_adapt: f32, color_adapt: f32) -> Result<core::Ptr<dyn crate::photo::TonemapReinhard>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createTonemapReinhard_float_float_float_float(gamma, intensity, light_adapt, color_adapt, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::photo::TonemapReinhard>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * gamma: 1.0f
// createTonemap(float) /usr/include/opencv2/photo.hpp:356
#[inline]
pub fn create_tonemap(gamma: f32) -> Result<core::Ptr<dyn crate::photo::Tonemap>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createTonemap_float(gamma, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::photo::Tonemap>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * search_window: 21
/// * block_size: 7
/// * stream: Stream::Null()
// fastNlMeansDenoisingColored(cv::InputArray, cv::OutputArray, float, float, int, int, cv::cuda::Stream &) /usr/include/opencv2/photo/cuda.hpp:122
#[inline]
pub fn fast_nl_means_denoising_colored_cuda(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, h_luminance: f32, photo_render: f32, search_window: i32, block_size: i32, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_fastNlMeansDenoisingColored_const__InputArrayR_const__OutputArrayR_float_float_int_int_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), h_luminance, photo_render, search_window, block_size, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * search_window: 21
/// * block_size: 7
/// * stream: Stream::Null()
// fastNlMeansDenoising(cv::InputArray, cv::OutputArray, float, int, int, cv::cuda::Stream &) /usr/include/opencv2/photo/cuda.hpp:95
#[inline]
pub fn fast_nl_means_denoising_cuda(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, h: f32, search_window: i32, block_size: i32, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_float_int_int_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), h, search_window, block_size, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * search_window: 21
/// * block_size: 7
/// * border_mode: BORDER_DEFAULT
/// * stream: Stream::Null()
// nonLocalMeans(cv::InputArray, cv::OutputArray, float, int, int, int, cv::cuda::Stream &) /usr/include/opencv2/photo/cuda.hpp:67
#[inline]
pub fn non_local_means(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, h: f32, search_window: i32, block_size: i32, border_mode: i32, stream: &mut core::Stream) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_nonLocalMeans_const__InputArrayR_const__OutputArrayR_float_int_int_int_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), h, search_window, block_size, border_mode, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// decolor(cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/photo.hpp:704
#[inline]
pub fn decolor(src: &dyn core::ToInputArray, grayscale: &mut dyn core::ToOutputArray, color_boost: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(grayscale);
	output_array_arg!(color_boost);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_decolor_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src.as_raw__InputArray(), grayscale.as_raw__OutputArray(), color_boost.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * lambda: 1.0
/// * niters: 30
// denoise_TVL1(const std::vector<Mat> &, cv::Mat &, double, int) /usr/include/opencv2/photo.hpp:325
#[inline]
pub fn denoise_tvl1(observations: &core::Vector<core::Mat>, result: &mut core::Mat, lambda: f64, niters: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_denoise_TVL1_const_vector_Mat_R_MatR_double_int(observations.as_raw_VectorOfMat(), result.as_raw_mut_Mat(), lambda, niters, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * sigma_s: 10
/// * sigma_r: 0.15f
// detailEnhance(cv::InputArray, cv::OutputArray, float, float) /usr/include/opencv2/photo.hpp:822
#[inline]
pub fn detail_enhance(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, sigma_s: f32, sigma_r: f32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detailEnhance_const__InputArrayR_const__OutputArrayR_float_float(src.as_raw__InputArray(), dst.as_raw__OutputArray(), sigma_s, sigma_r, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * flags: 1
/// * sigma_s: 60
/// * sigma_r: 0.4f
// edgePreservingFilter(cv::InputArray, cv::OutputArray, int, float, float) /usr/include/opencv2/photo.hpp:812
#[inline]
pub fn edge_preserving_filter(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, flags: i32, sigma_s: f32, sigma_r: f32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_edgePreservingFilter_const__InputArrayR_const__OutputArrayR_int_float_float(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flags, sigma_s, sigma_r, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * h: 3
/// * h_color: 3
/// * template_window_size: 7
/// * search_window_size: 21
// fastNlMeansDenoisingColoredMulti(cv::InputArrayOfArrays, cv::OutputArray, int, int, float, float, int, int) /usr/include/opencv2/photo.hpp:283
#[inline]
pub fn fast_nl_means_denoising_colored_multi(src_imgs: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, img_to_denoise_index: i32, temporal_window_size: i32, h: f32, h_color: f32, template_window_size: i32, search_window_size: i32) -> Result<()> {
	input_array_arg!(src_imgs);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fastNlMeansDenoisingColoredMulti_const__InputArrayR_const__OutputArrayR_int_int_float_float_int_int(src_imgs.as_raw__InputArray(), dst.as_raw__OutputArray(), img_to_denoise_index, temporal_window_size, h, h_color, template_window_size, search_window_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * h: 3
/// * h_color: 3
/// * template_window_size: 7
/// * search_window_size: 21
// fastNlMeansDenoisingColored(cv::InputArray, cv::OutputArray, float, float, int, int) /usr/include/opencv2/photo.hpp:198
#[inline]
pub fn fast_nl_means_denoising_colored(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, h: f32, h_color: f32, template_window_size: i32, search_window_size: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fastNlMeansDenoisingColored_const__InputArrayR_const__OutputArrayR_float_float_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), h, h_color, template_window_size, search_window_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * template_window_size: 7
/// * search_window_size: 21
/// * norm_type: NORM_L2
// fastNlMeansDenoisingMulti(cv::InputArrayOfArrays, cv::OutputArray, int, int, const std::vector<float> &, int, int, int) /usr/include/opencv2/photo.hpp:254
#[inline]
pub fn fast_nl_means_denoising_multi_vec(src_imgs: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, img_to_denoise_index: i32, temporal_window_size: i32, h: &core::Vector<f32>, template_window_size: i32, search_window_size: i32, norm_type: i32) -> Result<()> {
	input_array_arg!(src_imgs);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fastNlMeansDenoisingMulti_const__InputArrayR_const__OutputArrayR_int_int_const_vector_float_R_int_int_int(src_imgs.as_raw__InputArray(), dst.as_raw__OutputArray(), img_to_denoise_index, temporal_window_size, h.as_raw_VectorOff32(), template_window_size, search_window_size, norm_type, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * h: 3
/// * template_window_size: 7
/// * search_window_size: 21
// fastNlMeansDenoisingMulti(cv::InputArrayOfArrays, cv::OutputArray, int, int, float, int, int) /usr/include/opencv2/photo.hpp:225
#[inline]
pub fn fast_nl_means_denoising_multi(src_imgs: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, img_to_denoise_index: i32, temporal_window_size: i32, h: f32, template_window_size: i32, search_window_size: i32) -> Result<()> {
	input_array_arg!(src_imgs);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fastNlMeansDenoisingMulti_const__InputArrayR_const__OutputArrayR_int_int_float_int_int(src_imgs.as_raw__InputArray(), dst.as_raw__OutputArray(), img_to_denoise_index, temporal_window_size, h, template_window_size, search_window_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * template_window_size: 7
/// * search_window_size: 21
/// * norm_type: NORM_L2
// fastNlMeansDenoising(cv::InputArray, cv::OutputArray, const std::vector<float> &, int, int, int) /usr/include/opencv2/photo.hpp:175
#[inline]
pub fn fast_nl_means_denoising_vec(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, h: &core::Vector<f32>, template_window_size: i32, search_window_size: i32, norm_type: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_const_vector_float_R_int_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), h.as_raw_VectorOff32(), template_window_size, search_window_size, norm_type, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * h: 3
/// * template_window_size: 7
/// * search_window_size: 21
// fastNlMeansDenoising(cv::InputArray, cv::OutputArray, float, int, int) /usr/include/opencv2/photo.hpp:148
#[inline]
pub fn fast_nl_means_denoising(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, h: f32, template_window_size: i32, search_window_size: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_float_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), h, template_window_size, search_window_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * alpha: 0.2f
/// * beta: 0.4f
// illuminationChange(cv::InputArray, cv::InputArray, cv::OutputArray, float, float) /usr/include/opencv2/photo.hpp:769
#[inline]
pub fn illumination_change(src: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, alpha: f32, beta: f32) -> Result<()> {
	input_array_arg!(src);
	input_array_arg!(mask);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_illuminationChange_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_float(src.as_raw__InputArray(), mask.as_raw__InputArray(), dst.as_raw__OutputArray(), alpha, beta, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// inpaint(cv::InputArray, cv::InputArray, cv::OutputArray, double, int) /usr/include/opencv2/photo.hpp:120
#[inline]
pub fn inpaint(src: &dyn core::ToInputArray, inpaint_mask: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, inpaint_radius: f64, flags: i32) -> Result<()> {
	input_array_arg!(src);
	input_array_arg!(inpaint_mask);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_inpaint_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(src.as_raw__InputArray(), inpaint_mask.as_raw__InputArray(), dst.as_raw__OutputArray(), inpaint_radius, flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * sigma_s: 60
/// * sigma_r: 0.07f
/// * shade_factor: 0.02f
// pencilSketch(cv::InputArray, cv::OutputArray, cv::OutputArray, float, float, float) /usr/include/opencv2/photo.hpp:837
#[inline]
pub fn pencil_sketch(src: &dyn core::ToInputArray, dst1: &mut dyn core::ToOutputArray, dst2: &mut dyn core::ToOutputArray, sigma_s: f32, sigma_r: f32, shade_factor: f32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst1);
	output_array_arg!(dst2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_pencilSketch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_float_float_float(src.as_raw__InputArray(), dst1.as_raw__OutputArray(), dst2.as_raw__OutputArray(), sigma_s, sigma_r, shade_factor, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// seamlessClone(cv::InputArray, cv::InputArray, cv::InputArray, cv::Point, cv::OutputArray, int) /usr/include/opencv2/photo.hpp:740
#[inline]
pub fn seamless_clone(src: &dyn core::ToInputArray, dst: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, p: core::Point, blend: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
	input_array_arg!(src);
	input_array_arg!(dst);
	input_array_arg!(mask);
	output_array_arg!(blend);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_seamlessClone_const__InputArrayR_const__InputArrayR_const__InputArrayR_Point_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__InputArray(), mask.as_raw__InputArray(), p.opencv_as_extern(), blend.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * sigma_s: 60
/// * sigma_r: 0.45f
// stylization(cv::InputArray, cv::OutputArray, float, float) /usr/include/opencv2/photo.hpp:849
#[inline]
pub fn stylization(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, sigma_s: f32, sigma_r: f32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stylization_const__InputArrayR_const__OutputArrayR_float_float(src.as_raw__InputArray(), dst.as_raw__OutputArray(), sigma_s, sigma_r, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * low_threshold: 30
/// * high_threshold: 45
/// * kernel_size: 3
// textureFlattening(cv::InputArray, cv::InputArray, cv::OutputArray, float, float, int) /usr/include/opencv2/photo.hpp:787
#[inline]
pub fn texture_flattening(src: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, low_threshold: f32, high_threshold: f32, kernel_size: i32) -> Result<()> {
	input_array_arg!(src);
	input_array_arg!(mask);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_textureFlattening_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_float_int(src.as_raw__InputArray(), mask.as_raw__InputArray(), dst.as_raw__OutputArray(), low_threshold, high_threshold, kernel_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// AlignExposures /usr/include/opencv2/photo.hpp:450
pub trait AlignExposuresConst: core::AlgorithmTraitConst {
	fn as_raw_AlignExposures(&self) -> *const c_void;

}

pub trait AlignExposures: core::AlgorithmTrait + crate::photo::AlignExposuresConst {
	fn as_raw_mut_AlignExposures(&mut self) -> *mut c_void;

	// process(cv::InputArrayOfArrays, std::vector<Mat> &, cv::InputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:461
	#[inline]
	fn process(&mut self, src: &dyn core::ToInputArray, dst: &mut core::Vector<core::Mat>, times: &dyn core::ToInputArray, response: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(times);
		input_array_arg!(response);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AlignExposures_process_const__InputArrayR_vector_Mat_R_const__InputArrayR_const__InputArrayR(self.as_raw_mut_AlignExposures(), src.as_raw__InputArray(), dst.as_raw_mut_VectorOfMat(), times.as_raw__InputArray(), response.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// AlignMTB /usr/include/opencv2/photo.hpp:474
pub trait AlignMTBConst: crate::photo::AlignExposuresConst {
	fn as_raw_AlignMTB(&self) -> *const c_void;

	// getMaxBits() /usr/include/opencv2/photo.hpp:509
	#[inline]
	fn get_max_bits(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AlignMTB_getMaxBits_const(self.as_raw_AlignMTB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getExcludeRange() /usr/include/opencv2/photo.hpp:512
	#[inline]
	fn get_exclude_range(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AlignMTB_getExcludeRange_const(self.as_raw_AlignMTB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getCut() /usr/include/opencv2/photo.hpp:515
	#[inline]
	fn get_cut(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AlignMTB_getCut_const(self.as_raw_AlignMTB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait AlignMTB: crate::photo::AlignExposures + crate::photo::AlignMTBConst {
	fn as_raw_mut_AlignMTB(&mut self) -> *mut c_void;

	// process(cv::InputArrayOfArrays, std::vector<Mat> &, cv::InputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:477
	#[inline]
	fn process_with_response(&mut self, src: &dyn core::ToInputArray, dst: &mut core::Vector<core::Mat>, times: &dyn core::ToInputArray, response: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(times);
		input_array_arg!(response);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AlignMTB_process_const__InputArrayR_vector_Mat_R_const__InputArrayR_const__InputArrayR(self.as_raw_mut_AlignMTB(), src.as_raw__InputArray(), dst.as_raw_mut_VectorOfMat(), times.as_raw__InputArray(), response.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// process(cv::InputArrayOfArrays, std::vector<Mat> &) /usr/include/opencv2/photo.hpp:485
	#[inline]
	fn process(&mut self, src: &dyn core::ToInputArray, dst: &mut core::Vector<core::Mat>) -> Result<()> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AlignMTB_process_const__InputArrayR_vector_Mat_R(self.as_raw_mut_AlignMTB(), src.as_raw__InputArray(), dst.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// calculateShift(cv::InputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:493
	#[inline]
	fn calculate_shift(&mut self, img0: &dyn core::ToInputArray, img1: &dyn core::ToInputArray) -> Result<core::Point> {
		input_array_arg!(img0);
		input_array_arg!(img1);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AlignMTB_calculateShift_const__InputArrayR_const__InputArrayR(self.as_raw_mut_AlignMTB(), img0.as_raw__InputArray(), img1.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// shiftMat(cv::InputArray, cv::OutputArray, const cv::Point) /usr/include/opencv2/photo.hpp:500
	#[inline]
	fn shift_mat(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, shift: core::Point) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AlignMTB_shiftMat_const__InputArrayR_const__OutputArrayR_const_Point(self.as_raw_mut_AlignMTB(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), shift.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// computeBitmaps(cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/photo.hpp:507
	#[inline]
	fn compute_bitmaps(&mut self, img: &dyn core::ToInputArray, tb: &mut dyn core::ToOutputArray, eb: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(img);
		output_array_arg!(tb);
		output_array_arg!(eb);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AlignMTB_computeBitmaps_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_AlignMTB(), img.as_raw__InputArray(), tb.as_raw__OutputArray(), eb.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setMaxBits(int) /usr/include/opencv2/photo.hpp:510
	#[inline]
	fn set_max_bits(&mut self, max_bits: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AlignMTB_setMaxBits_int(self.as_raw_mut_AlignMTB(), max_bits, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setExcludeRange(int) /usr/include/opencv2/photo.hpp:513
	#[inline]
	fn set_exclude_range(&mut self, exclude_range: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AlignMTB_setExcludeRange_int(self.as_raw_mut_AlignMTB(), exclude_range, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setCut(bool) /usr/include/opencv2/photo.hpp:516
	#[inline]
	fn set_cut(&mut self, value: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AlignMTB_setCut_bool(self.as_raw_mut_AlignMTB(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// CalibrateCRF /usr/include/opencv2/photo.hpp:531
pub trait CalibrateCRFConst: core::AlgorithmTraitConst {
	fn as_raw_CalibrateCRF(&self) -> *const c_void;

}

pub trait CalibrateCRF: core::AlgorithmTrait + crate::photo::CalibrateCRFConst {
	fn as_raw_mut_CalibrateCRF(&mut self) -> *mut c_void;

	// process(cv::InputArrayOfArrays, cv::OutputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:540
	#[inline]
	fn process(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, times: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(times);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CalibrateCRF_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR(self.as_raw_mut_CalibrateCRF(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), times.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// CalibrateDebevec /usr/include/opencv2/photo.hpp:549
pub trait CalibrateDebevecConst: crate::photo::CalibrateCRFConst {
	fn as_raw_CalibrateDebevec(&self) -> *const c_void;

	// getLambda() /usr/include/opencv2/photo.hpp:552
	#[inline]
	fn get_lambda(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CalibrateDebevec_getLambda_const(self.as_raw_CalibrateDebevec(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSamples() /usr/include/opencv2/photo.hpp:555
	#[inline]
	fn get_samples(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CalibrateDebevec_getSamples_const(self.as_raw_CalibrateDebevec(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getRandom() /usr/include/opencv2/photo.hpp:558
	#[inline]
	fn get_random(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CalibrateDebevec_getRandom_const(self.as_raw_CalibrateDebevec(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait CalibrateDebevec: crate::photo::CalibrateCRF + crate::photo::CalibrateDebevecConst {
	fn as_raw_mut_CalibrateDebevec(&mut self) -> *mut c_void;

	// setLambda(float) /usr/include/opencv2/photo.hpp:553
	#[inline]
	fn set_lambda(&mut self, lambda: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CalibrateDebevec_setLambda_float(self.as_raw_mut_CalibrateDebevec(), lambda, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setSamples(int) /usr/include/opencv2/photo.hpp:556
	#[inline]
	fn set_samples(&mut self, samples: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CalibrateDebevec_setSamples_int(self.as_raw_mut_CalibrateDebevec(), samples, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setRandom(bool) /usr/include/opencv2/photo.hpp:559
	#[inline]
	fn set_random(&mut self, random: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CalibrateDebevec_setRandom_bool(self.as_raw_mut_CalibrateDebevec(), random, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// CalibrateRobertson /usr/include/opencv2/photo.hpp:577
pub trait CalibrateRobertsonConst: crate::photo::CalibrateCRFConst {
	fn as_raw_CalibrateRobertson(&self) -> *const c_void;

	// getMaxIter() /usr/include/opencv2/photo.hpp:580
	#[inline]
	fn get_max_iter(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CalibrateRobertson_getMaxIter_const(self.as_raw_CalibrateRobertson(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getThreshold() /usr/include/opencv2/photo.hpp:583
	#[inline]
	fn get_threshold(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CalibrateRobertson_getThreshold_const(self.as_raw_CalibrateRobertson(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getRadiance() /usr/include/opencv2/photo.hpp:586
	#[inline]
	fn get_radiance(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CalibrateRobertson_getRadiance_const(self.as_raw_CalibrateRobertson(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait CalibrateRobertson: crate::photo::CalibrateCRF + crate::photo::CalibrateRobertsonConst {
	fn as_raw_mut_CalibrateRobertson(&mut self) -> *mut c_void;

	// setMaxIter(int) /usr/include/opencv2/photo.hpp:581
	#[inline]
	fn set_max_iter(&mut self, max_iter: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CalibrateRobertson_setMaxIter_int(self.as_raw_mut_CalibrateRobertson(), max_iter, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setThreshold(float) /usr/include/opencv2/photo.hpp:584
	#[inline]
	fn set_threshold(&mut self, threshold: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CalibrateRobertson_setThreshold_float(self.as_raw_mut_CalibrateRobertson(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// MergeDebevec /usr/include/opencv2/photo.hpp:618
pub trait MergeDebevecConst: crate::photo::MergeExposuresConst {
	fn as_raw_MergeDebevec(&self) -> *const c_void;

}

pub trait MergeDebevec: crate::photo::MergeDebevecConst + crate::photo::MergeExposures {
	fn as_raw_mut_MergeDebevec(&mut self) -> *mut c_void;

	// process(cv::InputArrayOfArrays, cv::OutputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:621
	#[inline]
	fn process_with_response(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, times: &dyn core::ToInputArray, response: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(times);
		input_array_arg!(response);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MergeDebevec_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_MergeDebevec(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), times.as_raw__InputArray(), response.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// process(cv::InputArrayOfArrays, cv::OutputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:623
	#[inline]
	fn process(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, times: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(times);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MergeDebevec_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR(self.as_raw_mut_MergeDebevec(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), times.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// MergeExposures /usr/include/opencv2/photo.hpp:598
pub trait MergeExposuresConst: core::AlgorithmTraitConst {
	fn as_raw_MergeExposures(&self) -> *const c_void;

}

pub trait MergeExposures: core::AlgorithmTrait + crate::photo::MergeExposuresConst {
	fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void;

	// process(cv::InputArrayOfArrays, cv::OutputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:609
	#[inline]
	fn process(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, times: &dyn core::ToInputArray, response: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(times);
		input_array_arg!(response);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MergeExposures_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_MergeExposures(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), times.as_raw__InputArray(), response.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// MergeMertens /usr/include/opencv2/photo.hpp:641
pub trait MergeMertensConst: crate::photo::MergeExposuresConst {
	fn as_raw_MergeMertens(&self) -> *const c_void;

	// getContrastWeight() /usr/include/opencv2/photo.hpp:653
	#[inline]
	fn get_contrast_weight(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MergeMertens_getContrastWeight_const(self.as_raw_MergeMertens(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSaturationWeight() /usr/include/opencv2/photo.hpp:656
	#[inline]
	fn get_saturation_weight(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MergeMertens_getSaturationWeight_const(self.as_raw_MergeMertens(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getExposureWeight() /usr/include/opencv2/photo.hpp:659
	#[inline]
	fn get_exposure_weight(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MergeMertens_getExposureWeight_const(self.as_raw_MergeMertens(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait MergeMertens: crate::photo::MergeExposures + crate::photo::MergeMertensConst {
	fn as_raw_mut_MergeMertens(&mut self) -> *mut c_void;

	// process(cv::InputArrayOfArrays, cv::OutputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:644
	#[inline]
	fn process_with_response(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, times: &dyn core::ToInputArray, response: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(times);
		input_array_arg!(response);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MergeMertens_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_MergeMertens(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), times.as_raw__InputArray(), response.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// process(cv::InputArrayOfArrays, cv::OutputArray) /usr/include/opencv2/photo.hpp:651
	#[inline]
	fn process(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MergeMertens_process_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_MergeMertens(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setContrastWeight(float) /usr/include/opencv2/photo.hpp:654
	#[inline]
	fn set_contrast_weight(&mut self, contrast_weiht: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MergeMertens_setContrastWeight_float(self.as_raw_mut_MergeMertens(), contrast_weiht, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setSaturationWeight(float) /usr/include/opencv2/photo.hpp:657
	#[inline]
	fn set_saturation_weight(&mut self, saturation_weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MergeMertens_setSaturationWeight_float(self.as_raw_mut_MergeMertens(), saturation_weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setExposureWeight(float) /usr/include/opencv2/photo.hpp:660
	#[inline]
	fn set_exposure_weight(&mut self, exposure_weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MergeMertens_setExposureWeight_float(self.as_raw_mut_MergeMertens(), exposure_weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// MergeRobertson /usr/include/opencv2/photo.hpp:677
pub trait MergeRobertsonConst: crate::photo::MergeExposuresConst {
	fn as_raw_MergeRobertson(&self) -> *const c_void;

}

pub trait MergeRobertson: crate::photo::MergeExposures + crate::photo::MergeRobertsonConst {
	fn as_raw_mut_MergeRobertson(&mut self) -> *mut c_void;

	// process(cv::InputArrayOfArrays, cv::OutputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:680
	#[inline]
	fn process_with_response(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, times: &dyn core::ToInputArray, response: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(times);
		input_array_arg!(response);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MergeRobertson_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_MergeRobertson(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), times.as_raw__InputArray(), response.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// process(cv::InputArrayOfArrays, cv::OutputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:682
	#[inline]
	fn process(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, times: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(times);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MergeRobertson_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR(self.as_raw_mut_MergeRobertson(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), times.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Tonemap /usr/include/opencv2/photo.hpp:336
pub trait TonemapConst: core::AlgorithmTraitConst {
	fn as_raw_Tonemap(&self) -> *const c_void;

	// getGamma() /usr/include/opencv2/photo.hpp:346
	#[inline]
	fn get_gamma(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Tonemap_getGamma_const(self.as_raw_Tonemap(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Tonemap: core::AlgorithmTrait + crate::photo::TonemapConst {
	fn as_raw_mut_Tonemap(&mut self) -> *mut c_void;

	// process(cv::InputArray, cv::OutputArray) /usr/include/opencv2/photo.hpp:344
	#[inline]
	fn process(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Tonemap_process_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Tonemap(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setGamma(float) /usr/include/opencv2/photo.hpp:347
	#[inline]
	fn set_gamma(&mut self, gamma: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Tonemap_setGamma_float(self.as_raw_mut_Tonemap(), gamma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// TonemapDrago /usr/include/opencv2/photo.hpp:368
pub trait TonemapDragoConst: crate::photo::TonemapConst {
	fn as_raw_TonemapDrago(&self) -> *const c_void;

	// getSaturation() /usr/include/opencv2/photo.hpp:372
	#[inline]
	fn get_saturation(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TonemapDrago_getSaturation_const(self.as_raw_TonemapDrago(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getBias() /usr/include/opencv2/photo.hpp:375
	#[inline]
	fn get_bias(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TonemapDrago_getBias_const(self.as_raw_TonemapDrago(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait TonemapDrago: crate::photo::Tonemap + crate::photo::TonemapDragoConst {
	fn as_raw_mut_TonemapDrago(&mut self) -> *mut c_void;

	// setSaturation(float) /usr/include/opencv2/photo.hpp:373
	#[inline]
	fn set_saturation(&mut self, saturation: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TonemapDrago_setSaturation_float(self.as_raw_mut_TonemapDrago(), saturation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setBias(float) /usr/include/opencv2/photo.hpp:376
	#[inline]
	fn set_bias(&mut self, bias: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TonemapDrago_setBias_float(self.as_raw_mut_TonemapDrago(), bias, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// TonemapMantiuk /usr/include/opencv2/photo.hpp:428
pub trait TonemapMantiukConst: crate::photo::TonemapConst {
	fn as_raw_TonemapMantiuk(&self) -> *const c_void;

	// getScale() /usr/include/opencv2/photo.hpp:431
	#[inline]
	fn get_scale(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TonemapMantiuk_getScale_const(self.as_raw_TonemapMantiuk(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSaturation() /usr/include/opencv2/photo.hpp:434
	#[inline]
	fn get_saturation(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TonemapMantiuk_getSaturation_const(self.as_raw_TonemapMantiuk(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait TonemapMantiuk: crate::photo::Tonemap + crate::photo::TonemapMantiukConst {
	fn as_raw_mut_TonemapMantiuk(&mut self) -> *mut c_void;

	// setScale(float) /usr/include/opencv2/photo.hpp:432
	#[inline]
	fn set_scale(&mut self, scale: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TonemapMantiuk_setScale_float(self.as_raw_mut_TonemapMantiuk(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setSaturation(float) /usr/include/opencv2/photo.hpp:435
	#[inline]
	fn set_saturation(&mut self, saturation: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TonemapMantiuk_setSaturation_float(self.as_raw_mut_TonemapMantiuk(), saturation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// TonemapReinhard /usr/include/opencv2/photo.hpp:397
pub trait TonemapReinhardConst: crate::photo::TonemapConst {
	fn as_raw_TonemapReinhard(&self) -> *const c_void;

	// getIntensity() /usr/include/opencv2/photo.hpp:400
	#[inline]
	fn get_intensity(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TonemapReinhard_getIntensity_const(self.as_raw_TonemapReinhard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getLightAdaptation() /usr/include/opencv2/photo.hpp:403
	#[inline]
	fn get_light_adaptation(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TonemapReinhard_getLightAdaptation_const(self.as_raw_TonemapReinhard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getColorAdaptation() /usr/include/opencv2/photo.hpp:406
	#[inline]
	fn get_color_adaptation(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TonemapReinhard_getColorAdaptation_const(self.as_raw_TonemapReinhard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait TonemapReinhard: crate::photo::Tonemap + crate::photo::TonemapReinhardConst {
	fn as_raw_mut_TonemapReinhard(&mut self) -> *mut c_void;

	// setIntensity(float) /usr/include/opencv2/photo.hpp:401
	#[inline]
	fn set_intensity(&mut self, intensity: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TonemapReinhard_setIntensity_float(self.as_raw_mut_TonemapReinhard(), intensity, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setLightAdaptation(float) /usr/include/opencv2/photo.hpp:404
	#[inline]
	fn set_light_adaptation(&mut self, light_adapt: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TonemapReinhard_setLightAdaptation_float(self.as_raw_mut_TonemapReinhard(), light_adapt, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setColorAdaptation(float) /usr/include/opencv2/photo.hpp:407
	#[inline]
	fn set_color_adaptation(&mut self, color_adapt: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TonemapReinhard_setColorAdaptation_float(self.as_raw_mut_TonemapReinhard(), color_adapt, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
