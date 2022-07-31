#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Extended Image Processing
//!    # Structured forests for fast edge detection
//! 
//! This module contains implementations of modern structured edge detection algorithms,
//! i.e. algorithms which somehow takes into account pixel affinities in natural images.
//! 
//!    # EdgeBoxes
//! 
//!    # Filters
//! 
//!    # Superpixels
//! 
//!    # Image segmentation
//! 
//!    # Fast line detector
//! 
//!    # EdgeDrawing
//! 
//! EDGE DRAWING LIBRARY FOR GEOMETRIC FEATURE EXTRACTION AND VALIDATION
//! 
//! Edge Drawing (ED) algorithm is an proactive approach on edge detection problem. In contrast to many other existing edge detection algorithms which follow a subtractive
//! approach (i.e. after applying gradient filters onto an image eliminating pixels w.r.t. several rules, e.g. non-maximal suppression and hysteresis in Canny), ED algorithm
//! works via an additive strategy, i.e. it picks edge pixels one by one, hence the name Edge Drawing. Then we process those random shaped edge segments to extract higher level
//! edge features, i.e. lines, circles, ellipses, etc. The popular method of extraction edge pixels from the thresholded gradient magnitudes is non-maximal supression that tests
//! every pixel whether it has the maximum gradient response along its gradient direction and eliminates if it does not. However, this method does not check status of the
//! neighboring pixels, and therefore might result low quality (in terms of edge continuity, smoothness, thinness, localization) edge segments. Instead of non-maximal supression,
//! ED points a set of edge pixels and join them by maximizing the total gradient response of edge segments. Therefore it can extract high quality edge segments without need for
//! an additional hysteresis step.
//! 
//!    # Fourier descriptors
//! 
//!    # Binary morphology on run-length encoded image
//! 
//!    These functions support morphological operations on binary images. In order to be fast and space efficient binary images are encoded with a run-length representation.
//!    This representation groups continuous horizontal sequences of "on" pixels together in a "run". A run is charactarized by the column position of the first pixel in the run, the column
//!    position of the last pixel in the run and the row position. This representation is very compact for binary images which contain large continuous areas of "on" and "off" pixels. A checkerboard
//!    pattern would be a good example. The representation is not so suitable for binary images created from random noise images or other images where little correlation between neighboring pixels
//!    exists.
//! 
//!    The morphological operations supported here are very similar to the operations supported in the imgproc module. In general they are fast. However on several occasions they are slower than the functions
//!    from imgproc. The structuring elements of cv::MORPH_RECT and cv::MORPH_CROSS have very good support from the imgproc module. Also small structuring elements are very fast in imgproc (presumably
//!    due to opencl support). Therefore the functions from this module are recommended for larger structuring elements (cv::MORPH_ELLIPSE or self defined structuring elements). A sample application
//!    (run_length_morphology_demo) is supplied which allows to compare the speed of some morphological operations for the functions using run-length encoding and the imgproc functions for a given image.
//! 
//!    Run length encoded images are stored in standard opencv images. Images have a single column of cv::Point3i elements. The number of rows is the number of run + 1. The first row contains
//!    the size of the original (not encoded) image.  For the runs the following mapping is used (x: column begin, y: column end (last column), z: row).
//! 
//!    The size of the original image is required for compatibility with the imgproc functions when the boundary handling requires that pixel outside the image boundary are
//!    "on".
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::DTFilterConst, super::DTFilter, super::GuidedFilterConst, super::GuidedFilter, super::AdaptiveManifoldFilterConst, super::AdaptiveManifoldFilter, super::FastBilateralSolverFilterConst, super::FastBilateralSolverFilter, super::FastGlobalSmootherFilterConst, super::FastGlobalSmootherFilter, super::DisparityFilterConst, super::DisparityFilter, super::DisparityWLSFilterConst, super::DisparityWLSFilter, super::SparseMatchInterpolatorConst, super::SparseMatchInterpolator, super::EdgeAwareInterpolatorConst, super::EdgeAwareInterpolator, super::RICInterpolatorConst, super::RICInterpolator, super::RFFeatureGetterConst, super::RFFeatureGetter, super::StructuredEdgeDetectionConst, super::StructuredEdgeDetection, super::EdgeBoxesConst, super::EdgeBoxes, super::EdgeDrawingConst, super::EdgeDrawing, super::ScanSegmentConst, super::ScanSegment, super::SuperpixelSEEDSConst, super::SuperpixelSEEDS, super::GraphSegmentationConst, super::GraphSegmentation, super::SelectiveSearchSegmentationStrategyConst, super::SelectiveSearchSegmentationStrategy, super::SelectiveSearchSegmentationStrategyColorConst, super::SelectiveSearchSegmentationStrategyColor, super::SelectiveSearchSegmentationStrategySizeConst, super::SelectiveSearchSegmentationStrategySize, super::SelectiveSearchSegmentationStrategyTextureConst, super::SelectiveSearchSegmentationStrategyTexture, super::SelectiveSearchSegmentationStrategyFillConst, super::SelectiveSearchSegmentationStrategyFill, super::SelectiveSearchSegmentationStrategyMultipleConst, super::SelectiveSearchSegmentationStrategyMultiple, super::SelectiveSearchSegmentationConst, super::SelectiveSearchSegmentation, super::SuperpixelSLICConst, super::SuperpixelSLIC, super::SuperpixelLSCConst, super::SuperpixelLSC, super::FastLineDetectorConst, super::FastLineDetector, super::ContourFittingTraitConst, super::ContourFittingTrait, super::RidgeDetectionFilterConst, super::RidgeDetectionFilter };
}

// AM_FILTER /usr/include/opencv2/ximgproc/edge_filter.hpp:58
pub const AM_FILTER: i32 = 4;
// ARO_0_45 /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:66
pub const ARO_0_45: i32 = 0;
// ARO_315_0 /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:69
pub const ARO_315_0: i32 = 3;
// ARO_315_135 /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:72
pub const ARO_315_135: i32 = 6;
// ARO_315_45 /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:70
pub const ARO_315_45: i32 = 4;
// ARO_45_135 /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:71
pub const ARO_45_135: i32 = 5;
// ARO_45_90 /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:67
pub const ARO_45_90: i32 = 1;
// ARO_90_135 /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:68
pub const ARO_90_135: i32 = 2;
// ARO_CTR_HOR /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:73
pub const ARO_CTR_HOR: i32 = 7;
// ARO_CTR_VER /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:75
pub const ARO_CTR_VER: i32 = 8;
// BINARIZATION_NIBLACK /usr/include/opencv2/ximgproc.hpp:135
pub const BINARIZATION_NIBLACK: i32 = 0;
// BINARIZATION_NICK /usr/include/opencv2/ximgproc.hpp:138
pub const BINARIZATION_NICK: i32 = 3;
// BINARIZATION_SAUVOLA /usr/include/opencv2/ximgproc.hpp:136
pub const BINARIZATION_SAUVOLA: i32 = 1;
// BINARIZATION_WOLF /usr/include/opencv2/ximgproc.hpp:137
pub const BINARIZATION_WOLF: i32 = 2;
// DTF_IC /usr/include/opencv2/ximgproc/edge_filter.hpp:54
pub const DTF_IC: i32 = 1;
// DTF_NC /usr/include/opencv2/ximgproc/edge_filter.hpp:53
pub const DTF_NC: i32 = 0;
// DTF_RF /usr/include/opencv2/ximgproc/edge_filter.hpp:55
pub const DTF_RF: i32 = 2;
// FHT_ADD /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:94
pub const FHT_ADD: i32 = 2;
// FHT_AVE /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:96
pub const FHT_AVE: i32 = 3;
// FHT_MAX /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:92
pub const FHT_MAX: i32 = 1;
// FHT_MIN /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:90
pub const FHT_MIN: i32 = 0;
// GUIDED_FILTER /usr/include/opencv2/ximgproc/edge_filter.hpp:57
pub const GUIDED_FILTER: i32 = 3;
// HDO_DESKEW /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:108
pub const HDO_DESKEW: i32 = 1;
// HDO_RAW /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:107
pub const HDO_RAW: i32 = 0;
// MSLIC /usr/include/opencv2/ximgproc/slic.hpp:64
pub const MSLIC: i32 = 102;
// RO_IGNORE_BORDERS /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:118
pub const RO_IGNORE_BORDERS: i32 = 1;
// RO_STRICT /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:117
pub const RO_STRICT: i32 = 0;
// SLIC /usr/include/opencv2/ximgproc/slic.hpp:64
pub const SLIC: i32 = 100;
// SLICO /usr/include/opencv2/ximgproc/slic.hpp:64
pub const SLICO: i32 = 101;
// THINNING_GUOHALL /usr/include/opencv2/ximgproc.hpp:128
pub const THINNING_GUOHALL: i32 = 1;
// THINNING_ZHANGSUEN /usr/include/opencv2/ximgproc.hpp:127
pub const THINNING_ZHANGSUEN: i32 = 0;
// WMF_COS /usr/include/opencv2/ximgproc/weighted_median_filter.hpp:69
pub const WMF_COS: i32 = 8;
// WMF_EXP /usr/include/opencv2/ximgproc/weighted_median_filter.hpp:66
pub const WMF_EXP: i32 = 1;
// WMF_IV1 /usr/include/opencv2/ximgproc/weighted_median_filter.hpp:67
pub const WMF_IV1: i32 = 2;
// WMF_IV2 /usr/include/opencv2/ximgproc/weighted_median_filter.hpp:68
pub const WMF_IV2: i32 = 4;
// WMF_JAC /usr/include/opencv2/ximgproc/weighted_median_filter.hpp:70
pub const WMF_JAC: i32 = 16;
// WMF_OFF /usr/include/opencv2/ximgproc/weighted_median_filter.hpp:71
pub const WMF_OFF: i32 = 32;
// AngleRangeOption /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:64
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AngleRangeOption {
	ARO_0_45 = 0,
	ARO_45_90 = 1,
	ARO_90_135 = 2,
	ARO_315_0 = 3,
	ARO_315_45 = 4,
	ARO_45_135 = 5,
	ARO_315_135 = 6,
	ARO_CTR_HOR = 7,
	ARO_CTR_VER = 8,
}

opencv_type_enum! { crate::ximgproc::AngleRangeOption }

// EdgeAwareFiltersList /usr/include/opencv2/ximgproc/edge_filter.hpp:51
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EdgeAwareFiltersList {
	DTF_NC = 0,
	DTF_IC = 1,
	DTF_RF = 2,
	GUIDED_FILTER = 3,
	AM_FILTER = 4,
}

opencv_type_enum! { crate::ximgproc::EdgeAwareFiltersList }

// GradientOperator /usr/include/opencv2/ximgproc/edge_drawing.hpp:25
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EdgeDrawing_GradientOperator {
	PREWITT = 0,
	SOBEL = 1,
	SCHARR = 2,
	LSD = 3,
}

opencv_type_enum! { crate::ximgproc::EdgeDrawing_GradientOperator }

// HoughDeskewOption /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:105
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HoughDeskewOption {
	HDO_RAW = 0,
	HDO_DESKEW = 1,
}

opencv_type_enum! { crate::ximgproc::HoughDeskewOption }

// HoughOp /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:88
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HoughOp {
	FHT_MIN = 0,
	FHT_MAX = 1,
	FHT_ADD = 2,
	FHT_AVE = 3,
}

opencv_type_enum! { crate::ximgproc::HoughOp }

// LocalBinarizationMethods /usr/include/opencv2/ximgproc.hpp:134
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LocalBinarizationMethods {
	BINARIZATION_NIBLACK = 0,
	BINARIZATION_SAUVOLA = 1,
	BINARIZATION_WOLF = 2,
	BINARIZATION_NICK = 3,
}

opencv_type_enum! { crate::ximgproc::LocalBinarizationMethods }

// RulesOption /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:119
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RulesOption {
	RO_STRICT = 0,
	RO_IGNORE_BORDERS = 1,
}

opencv_type_enum! { crate::ximgproc::RulesOption }

// SLICType /usr/include/opencv2/ximgproc/slic.hpp:64
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SLICType {
	SLIC = 100,
	SLICO = 101,
	MSLIC = 102,
}

opencv_type_enum! { crate::ximgproc::SLICType }

// ThinningTypes /usr/include/opencv2/ximgproc.hpp:126
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ThinningTypes {
	THINNING_ZHANGSUEN = 0,
	THINNING_GUOHALL = 1,
}

opencv_type_enum! { crate::ximgproc::ThinningTypes }

// WMFWeightType /usr/include/opencv2/ximgproc/weighted_median_filter.hpp:64
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WMFWeightType {
	WMF_EXP = 1,
	WMF_IV1 = 2,
	WMF_IV2 = 4,
	WMF_COS = 8,
	WMF_JAC = 16,
	WMF_OFF = 32,
}

opencv_type_enum! { crate::ximgproc::WMFWeightType }

/// ## C++ default parameters
/// * contrast: 1
/// * shortrange: 3
/// * longrange: 9
// BrightEdges(cv::Mat &, cv::Mat &, int, int, int) /usr/include/opencv2/ximgproc/brightedges.hpp:48
#[inline]
pub fn bright_edges(_original: &mut core::Mat, _edgeview: &mut core::Mat, contrast: i32, shortrange: i32, longrange: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_BrightEdges_MatR_MatR_int_int_int(_original.as_raw_mut_Mat(), _edgeview.as_raw_mut_Mat(), contrast, shortrange, longrange, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * angle_range: ARO_315_135
/// * op: FHT_ADD
/// * make_skew: HDO_DESKEW
// FastHoughTransform(cv::InputArray, cv::OutputArray, int, int, int, int) /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:133
#[inline]
pub fn fast_hough_transform(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, dst_mat_depth: i32, angle_range: i32, op: i32, make_skew: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_FastHoughTransform_const__InputArrayR_const__OutputArrayR_int_int_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), dst_mat_depth, angle_range, op, make_skew, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// GradientDericheX(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/ximgproc/deriche_filter.hpp:72
#[inline]
pub fn gradient_deriche_x(op: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, alpha: f64, omega: f64) -> Result<()> {
	input_array_arg!(op);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_GradientDericheX_const__InputArrayR_const__OutputArrayR_double_double(op.as_raw__InputArray(), dst.as_raw__OutputArray(), alpha, omega, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// GradientDericheY(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/ximgproc/deriche_filter.hpp:60
#[inline]
pub fn gradient_deriche_y(op: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, alpha: f64, omega: f64) -> Result<()> {
	input_array_arg!(op);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_GradientDericheY_const__InputArrayR_const__OutputArrayR_double_double(op.as_raw__InputArray(), dst.as_raw__OutputArray(), alpha, omega, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// GradientPaillouX(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/ximgproc/paillou_filter.hpp:62
#[inline]
pub fn gradient_paillou_x(op: &dyn core::ToInputArray, _dst: &mut dyn core::ToOutputArray, alpha: f64, omega: f64) -> Result<()> {
	input_array_arg!(op);
	output_array_arg!(_dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_GradientPaillouX_const__InputArrayR_const__OutputArrayR_double_double(op.as_raw__InputArray(), _dst.as_raw__OutputArray(), alpha, omega, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// GradientPaillouY(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/ximgproc/paillou_filter.hpp:61
#[inline]
pub fn gradient_paillou_y(op: &dyn core::ToInputArray, _dst: &mut dyn core::ToOutputArray, alpha: f64, omega: f64) -> Result<()> {
	input_array_arg!(op);
	output_array_arg!(_dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_GradientPaillouY_const__InputArrayR_const__OutputArrayR_double_double(op.as_raw__InputArray(), _dst.as_raw__OutputArray(), alpha, omega, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * angle_range: ARO_315_135
/// * make_skew: HDO_DESKEW
/// * rules: RO_IGNORE_BORDERS
// HoughPoint2Line(const cv::Point &, cv::InputArray, int, int, int) /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:155
#[inline]
pub fn hough_point2_line(hough_point: core::Point, src_img_info: &dyn core::ToInputArray, angle_range: i32, make_skew: i32, rules: i32) -> Result<core::Vec4i> {
	input_array_arg!(src_img_info);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_HoughPoint2Line_const_PointR_const__InputArrayR_int_int_int(&hough_point, src_img_info.as_raw__InputArray(), angle_range, make_skew, rules, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// PeiLinNormalization(cv::InputArray) /usr/include/opencv2/ximgproc/peilin.hpp:26
#[inline]
pub fn pei_lin_normalization(i: &dyn core::ToInputArray) -> Result<core::Matx23d> {
	input_array_arg!(i);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_PeiLinNormalization_const__InputArrayR(i.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// PeiLinNormalization(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/peilin.hpp:28
#[inline]
pub fn pei_lin_normalization_1(i: &dyn core::ToInputArray, t: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(i);
	output_array_arg!(t);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_PeiLinNormalization_const__InputArrayR_const__OutputArrayR(i.as_raw__InputArray(), t.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * theta: 1
/// * start_angle: 0
/// * end_angle: 180
/// * crop: false
/// * norm: false
// RadonTransform(cv::InputArray, cv::OutputArray, double, double, double, bool, bool) /usr/include/opencv2/ximgproc/radon_transform.hpp:31
#[inline]
pub fn radon_transform(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, theta: f64, start_angle: f64, end_angle: f64, crop: bool, norm: bool) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_RadonTransform_const__InputArrayR_const__OutputArrayR_double_double_double_bool_bool(src.as_raw__InputArray(), dst.as_raw__OutputArray(), theta, start_angle, end_angle, crop, norm, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * adjust_outliers: false
// amFilter(cv::InputArray, cv::InputArray, cv::OutputArray, double, double, bool) /usr/include/opencv2/ximgproc/edge_filter.hpp:284
#[inline]
pub fn am_filter(joint: &dyn core::ToInputArray, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, sigma_s: f64, sigma_r: f64, adjust_outliers: bool) -> Result<()> {
	input_array_arg!(joint);
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_amFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_bool(joint.as_raw__InputArray(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), sigma_s, sigma_r, adjust_outliers, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// anisotropicDiffusion(cv::InputArray, cv::OutputArray, float, float, int) /usr/include/opencv2/ximgproc.hpp:211
#[inline]
pub fn anisotropic_diffusion(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, alpha: f32, k: f32, niters: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_anisotropicDiffusion_const__InputArrayR_const__OutputArrayR_float_float_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), alpha, k, niters, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * fr: 3
/// * num_iter: 1
/// * sigma_alpha: -1.
/// * sigma_avg: -1.
// bilateralTextureFilter(cv::InputArray, cv::OutputArray, int, int, double, double) /usr/include/opencv2/ximgproc/edge_filter.hpp:339
#[inline]
pub fn bilateral_texture_filter(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, fr: i32, num_iter: i32, sigma_alpha: f64, sigma_avg: f64) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_bilateralTextureFilter_const__InputArrayR_const__OutputArrayR_int_int_double_double(src.as_raw__InputArray(), dst.as_raw__OutputArray(), fr, num_iter, sigma_alpha, sigma_avg, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// colorMatchTemplate(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/color_match.hpp:62
#[inline]
pub fn color_match_template(img: &dyn core::ToInputArray, templ: &dyn core::ToInputArray, result: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(img);
	input_array_arg!(templ);
	output_array_arg!(result);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_colorMatchTemplate_const__InputArrayR_const__InputArrayR_const__OutputArrayR(img.as_raw__InputArray(), templ.as_raw__InputArray(), result.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * thresh: 24
// computeBadPixelPercent(cv::InputArray, cv::InputArray, cv::Rect, int) /usr/include/opencv2/ximgproc/disparity_filter.hpp:193
#[inline]
pub fn compute_bad_pixel_percent(gt: &dyn core::ToInputArray, src: &dyn core::ToInputArray, roi: core::Rect, thresh: i32) -> Result<f64> {
	input_array_arg!(gt);
	input_array_arg!(src);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_computeBadPixelPercent_const__InputArrayR_const__InputArrayR_Rect_int(gt.as_raw__InputArray(), src.as_raw__InputArray(), roi.opencv_as_extern(), thresh, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// computeMSE(cv::InputArray, cv::InputArray, cv::Rect) /usr/include/opencv2/ximgproc/disparity_filter.hpp:177
#[inline]
pub fn compute_mse(gt: &dyn core::ToInputArray, src: &dyn core::ToInputArray, roi: core::Rect) -> Result<f64> {
	input_array_arg!(gt);
	input_array_arg!(src);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_computeMSE_const__InputArrayR_const__InputArrayR_Rect(gt.as_raw__InputArray(), src.as_raw__InputArray(), roi.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// contourSampling(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:106
#[inline]
pub fn contour_sampling(src: &dyn core::ToInputArray, out: &mut dyn core::ToOutputArray, nb_elt: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(out);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_contourSampling_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), out.as_raw__OutputArray(), nb_elt, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// covarianceEstimation(cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/ximgproc/estimated_covariance.hpp:77
#[inline]
pub fn covariance_estimation(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, window_rows: i32, window_cols: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_covarianceEstimation_const__InputArrayR_const__OutputArrayR_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), window_rows, window_cols, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * adjust_outliers: false
// createAMFilter(double, double, bool) /usr/include/opencv2/ximgproc/edge_filter.hpp:262
#[inline]
pub fn create_am_filter(sigma_s: f64, sigma_r: f64, adjust_outliers: bool) -> Result<core::Ptr<dyn crate::ximgproc::AdaptiveManifoldFilter>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createAMFilter_double_double_bool(sigma_s, sigma_r, adjust_outliers, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::AdaptiveManifoldFilter>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * ctr: 1024
/// * fd: 16
// createContourFitting(int, int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:114
#[inline]
pub fn create_contour_fitting(ctr: i32, fd: i32) -> Result<core::Ptr<crate::ximgproc::ContourFitting>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createContourFitting_int_int(ctr, fd, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::ximgproc::ContourFitting>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * mode: DTF_NC
/// * num_iters: 3
// createDTFilter(cv::InputArray, double, double, int, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:102
#[inline]
pub fn create_dt_filter(guide: &dyn core::ToInputArray, sigma_spatial: f64, sigma_color: f64, mode: i32, num_iters: i32) -> Result<core::Ptr<dyn crate::ximgproc::DTFilter>> {
	input_array_arg!(guide);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createDTFilter_const__InputArrayR_double_double_int_int(guide.as_raw__InputArray(), sigma_spatial, sigma_color, mode, num_iters, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::DTFilter>::opencv_from_extern(ret) };
	Ok(ret)
}

// createDisparityWLSFilterGeneric(bool) /usr/include/opencv2/ximgproc/disparity_filter.hpp:149
#[inline]
pub fn create_disparity_wls_filter_generic(use_confidence: bool) -> Result<core::Ptr<dyn crate::ximgproc::DisparityWLSFilter>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createDisparityWLSFilterGeneric_bool(use_confidence, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::DisparityWLSFilter>::opencv_from_extern(ret) };
	Ok(ret)
}

// createDisparityWLSFilter(Ptr<cv::StereoMatcher>) /usr/include/opencv2/ximgproc/disparity_filter.hpp:131
#[inline]
pub fn create_disparity_wls_filter(mut matcher_left: core::Ptr<dyn crate::calib3d::StereoMatcher>) -> Result<core::Ptr<dyn crate::ximgproc::DisparityWLSFilter>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createDisparityWLSFilter_Ptr_StereoMatcher_(matcher_left.as_raw_mut_PtrOfStereoMatcher(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::DisparityWLSFilter>::opencv_from_extern(ret) };
	Ok(ret)
}

// createEdgeAwareInterpolator() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:138
#[inline]
pub fn create_edge_aware_interpolator() -> Result<core::Ptr<dyn crate::ximgproc::EdgeAwareInterpolator>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createEdgeAwareInterpolator(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::EdgeAwareInterpolator>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * alpha: 0.65f
/// * beta: 0.75f
/// * eta: 1
/// * min_score: 0.01f
/// * max_boxes: 10000
/// * edge_min_mag: 0.1f
/// * edge_merge_thr: 0.5f
/// * cluster_min_mag: 0.5f
/// * max_aspect_ratio: 3
/// * min_box_area: 1000
/// * gamma: 2
/// * kappa: 1.5f
// createEdgeBoxes(float, float, float, float, int, float, float, float, float, float, float, float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:183
#[inline]
pub fn create_edge_boxes(alpha: f32, beta: f32, eta: f32, min_score: f32, max_boxes: i32, edge_min_mag: f32, edge_merge_thr: f32, cluster_min_mag: f32, max_aspect_ratio: f32, min_box_area: f32, gamma: f32, kappa: f32) -> Result<core::Ptr<dyn crate::ximgproc::EdgeBoxes>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createEdgeBoxes_float_float_float_float_int_float_float_float_float_float_float_float(alpha, beta, eta, min_score, max_boxes, edge_min_mag, edge_merge_thr, cluster_min_mag, max_aspect_ratio, min_box_area, gamma, kappa, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::EdgeBoxes>::opencv_from_extern(ret) };
	Ok(ret)
}

// createEdgeDrawing() /usr/include/opencv2/ximgproc/edge_drawing.hpp:126
#[inline]
pub fn create_edge_drawing() -> Result<core::Ptr<dyn crate::ximgproc::EdgeDrawing>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createEdgeDrawing(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::EdgeDrawing>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * lambda: 128.0
/// * num_iter: 25
/// * max_tol: 1e-5
// createFastBilateralSolverFilter(cv::InputArray, double, double, double, double, int, double) /usr/include/opencv2/ximgproc/edge_filter.hpp:417
#[inline]
pub fn create_fast_bilateral_solver_filter(guide: &dyn core::ToInputArray, sigma_spatial: f64, sigma_luma: f64, sigma_chroma: f64, lambda: f64, num_iter: i32, max_tol: f64) -> Result<core::Ptr<dyn crate::ximgproc::FastBilateralSolverFilter>> {
	input_array_arg!(guide);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createFastBilateralSolverFilter_const__InputArrayR_double_double_double_double_int_double(guide.as_raw__InputArray(), sigma_spatial, sigma_luma, sigma_chroma, lambda, num_iter, max_tol, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::FastBilateralSolverFilter>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * lambda_attenuation: 0.25
/// * num_iter: 3
// createFastGlobalSmootherFilter(cv::InputArray, double, double, double, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:489
#[inline]
pub fn create_fast_global_smoother_filter(guide: &dyn core::ToInputArray, lambda: f64, sigma_color: f64, lambda_attenuation: f64, num_iter: i32) -> Result<core::Ptr<dyn crate::ximgproc::FastGlobalSmootherFilter>> {
	input_array_arg!(guide);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createFastGlobalSmootherFilter_const__InputArrayR_double_double_double_int(guide.as_raw__InputArray(), lambda, sigma_color, lambda_attenuation, num_iter, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::FastGlobalSmootherFilter>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * length_threshold: 10
/// * distance_threshold: 1.414213562f
/// * canny_th1: 50.0
/// * canny_th2: 50.0
/// * canny_aperture_size: 3
/// * do_merge: false
// createFastLineDetector(int, float, double, double, int, bool) /usr/include/opencv2/ximgproc/fast_line_detector.hpp:71
#[inline]
pub fn create_fast_line_detector(length_threshold: i32, distance_threshold: f32, canny_th1: f64, canny_th2: f64, canny_aperture_size: i32, do_merge: bool) -> Result<core::Ptr<dyn crate::ximgproc::FastLineDetector>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createFastLineDetector_int_float_double_double_int_bool(length_threshold, distance_threshold, canny_th1, canny_th2, canny_aperture_size, do_merge, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::FastLineDetector>::opencv_from_extern(ret) };
	Ok(ret)
}

// createGuidedFilter(cv::InputArray, int, double) /usr/include/opencv2/ximgproc/edge_filter.hpp:158
#[inline]
pub fn create_guided_filter(guide: &dyn core::ToInputArray, radius: i32, eps: f64) -> Result<core::Ptr<dyn crate::ximgproc::GuidedFilter>> {
	input_array_arg!(guide);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createGuidedFilter_const__InputArrayR_int_double(guide.as_raw__InputArray(), radius, eps, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::GuidedFilter>::opencv_from_extern(ret) };
	Ok(ret)
}

// createQuaternionImage(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/color_match.hpp:22
#[inline]
pub fn create_quaternion_image(img: &dyn core::ToInputArray, qimg: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(img);
	output_array_arg!(qimg);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createQuaternionImage_const__InputArrayR_const__OutputArrayR(img.as_raw__InputArray(), qimg.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// createRFFeatureGetter() /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:91
#[inline]
pub fn create_rf_feature_getter() -> Result<core::Ptr<dyn crate::ximgproc::RFFeatureGetter>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createRFFeatureGetter(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::RFFeatureGetter>::opencv_from_extern(ret) };
	Ok(ret)
}

// createRICInterpolator() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:265
#[inline]
pub fn create_ric_interpolator() -> Result<core::Ptr<dyn crate::ximgproc::RICInterpolator>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createRICInterpolator(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::RICInterpolator>::opencv_from_extern(ret) };
	Ok(ret)
}

// createRightMatcher(Ptr<cv::StereoMatcher>) /usr/include/opencv2/ximgproc/disparity_filter.hpp:139
#[inline]
pub fn create_right_matcher(mut matcher_left: core::Ptr<dyn crate::calib3d::StereoMatcher>) -> Result<core::Ptr<dyn crate::calib3d::StereoMatcher>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createRightMatcher_Ptr_StereoMatcher_(matcher_left.as_raw_mut_PtrOfStereoMatcher(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::calib3d::StereoMatcher>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * slices: 8
/// * merge_small: true
// createScanSegment(int, int, int, int, bool) /usr/include/opencv2/ximgproc/scansegment.hpp:80
#[inline]
pub fn create_scan_segment(image_width: i32, image_height: i32, num_superpixels: i32, slices: i32, merge_small: bool) -> Result<core::Ptr<dyn crate::ximgproc::ScanSegment>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createScanSegment_int_int_int_int_bool(image_width, image_height, num_superpixels, slices, merge_small, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::ScanSegment>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * how_to_get_features: Ptr<RFFeatureGetter>()
// createStructuredEdgeDetection(const cv::String &, Ptr<const cv::ximgproc::RFFeatureGetter>) /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:140
#[inline]
pub fn create_structured_edge_detection(model: &str, how_to_get_features: core::Ptr<dyn crate::ximgproc::RFFeatureGetter>) -> Result<core::Ptr<dyn crate::ximgproc::StructuredEdgeDetection>> {
	extern_container_arg!(model);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createStructuredEdgeDetection_const_StringR_Ptr_const_RFFeatureGetter_(model.opencv_as_extern(), how_to_get_features.as_raw_PtrOfRFFeatureGetter(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::StructuredEdgeDetection>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * region_size: 10
/// * ratio: 0.075f
// createSuperpixelLSC(cv::InputArray, int, float) /usr/include/opencv2/ximgproc/lsc.hpp:150
#[inline]
pub fn create_superpixel_lsc(image: &dyn core::ToInputArray, region_size: i32, ratio: f32) -> Result<core::Ptr<dyn crate::ximgproc::SuperpixelLSC>> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createSuperpixelLSC_const__InputArrayR_int_float(image.as_raw__InputArray(), region_size, ratio, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::SuperpixelLSC>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * prior: 2
/// * histogram_bins: 5
/// * double_step: false
// createSuperpixelSEEDS(int, int, int, int, int, int, int, bool) /usr/include/opencv2/ximgproc/seeds.hpp:173
#[inline]
pub fn create_superpixel_seeds(image_width: i32, image_height: i32, image_channels: i32, num_superpixels: i32, num_levels: i32, prior: i32, histogram_bins: i32, double_step: bool) -> Result<core::Ptr<dyn crate::ximgproc::SuperpixelSEEDS>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createSuperpixelSEEDS_int_int_int_int_int_int_int_bool(image_width, image_height, image_channels, num_superpixels, num_levels, prior, histogram_bins, double_step, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::SuperpixelSEEDS>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * algorithm: SLICO
/// * region_size: 10
/// * ruler: 10.0f
// createSuperpixelSLIC(cv::InputArray, int, int, float) /usr/include/opencv2/ximgproc/slic.hpp:160
#[inline]
pub fn create_superpixel_slic(image: &dyn core::ToInputArray, algorithm: i32, region_size: i32, ruler: f32) -> Result<core::Ptr<dyn crate::ximgproc::SuperpixelSLIC>> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_createSuperpixelSLIC_const__InputArrayR_int_int_float(image.as_raw__InputArray(), algorithm, region_size, ruler, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::SuperpixelSLIC>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * mode: DTF_NC
/// * num_iters: 3
// dtFilter(cv::InputArray, cv::InputArray, cv::OutputArray, double, double, int, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:121
#[inline]
pub fn dt_filter(guide: &dyn core::ToInputArray, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, sigma_spatial: f64, sigma_color: f64, mode: i32, num_iters: i32) -> Result<()> {
	input_array_arg!(guide);
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_dtFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_int_int(guide.as_raw__InputArray(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), sigma_spatial, sigma_color, mode, num_iters, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// edgePreservingFilter(cv::InputArray, cv::OutputArray, int, double) /usr/include/opencv2/ximgproc/edgepreserving_filter.hpp:27
#[inline]
pub fn edge_preserving_filter(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, d: i32, threshold: f64) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_edgePreservingFilter_const__InputArrayR_const__OutputArrayR_int_double(src.as_raw__InputArray(), dst.as_raw__OutputArray(), d, threshold, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * sigma_spatial: 8
/// * sigma_luma: 8
/// * sigma_chroma: 8
/// * lambda: 128.0
/// * num_iter: 25
/// * max_tol: 1e-5
// fastBilateralSolverFilter(cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray, double, double, double, double, int, double) /usr/include/opencv2/ximgproc/edge_filter.hpp:448
#[inline]
pub fn fast_bilateral_solver_filter(guide: &dyn core::ToInputArray, src: &dyn core::ToInputArray, confidence: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, sigma_spatial: f64, sigma_luma: f64, sigma_chroma: f64, lambda: f64, num_iter: i32, max_tol: f64) -> Result<()> {
	input_array_arg!(guide);
	input_array_arg!(src);
	input_array_arg!(confidence);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_fastBilateralSolverFilter_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_double_double_int_double(guide.as_raw__InputArray(), src.as_raw__InputArray(), confidence.as_raw__InputArray(), dst.as_raw__OutputArray(), sigma_spatial, sigma_luma, sigma_chroma, lambda, num_iter, max_tol, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * lambda_attenuation: 0.25
/// * num_iter: 3
// fastGlobalSmootherFilter(cv::InputArray, cv::InputArray, cv::OutputArray, double, double, double, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:509
#[inline]
pub fn fast_global_smoother_filter(guide: &dyn core::ToInputArray, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, lambda: f64, sigma_color: f64, lambda_attenuation: f64, num_iter: i32) -> Result<()> {
	input_array_arg!(guide);
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_fastGlobalSmootherFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_double_int(guide.as_raw__InputArray(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), lambda, sigma_color, lambda_attenuation, num_iter, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * nb_elt: -1
/// * nb_fd: -1
// fourierDescriptor(cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:87
#[inline]
pub fn fourier_descriptor(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, nb_elt: i32, nb_fd: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_fourierDescriptor_const__InputArrayR_const__OutputArrayR_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), nb_elt, nb_fd, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * scale: 1.0
// getDisparityVis(cv::InputArray, cv::OutputArray, double) /usr/include/opencv2/ximgproc/disparity_filter.hpp:204
#[inline]
pub fn get_disparity_vis(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, scale: f64) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_getDisparityVis_const__InputArrayR_const__OutputArrayR_double(src.as_raw__InputArray(), dst.as_raw__OutputArray(), scale, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * d_depth: -1
// guidedFilter(cv::InputArray, cv::InputArray, cv::OutputArray, int, double, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:180
#[inline]
pub fn guided_filter(guide: &dyn core::ToInputArray, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, radius: i32, eps: f64, d_depth: i32) -> Result<()> {
	input_array_arg!(guide);
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_guidedFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_int(guide.as_raw__InputArray(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), radius, eps, d_depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * border_type: BORDER_DEFAULT
// jointBilateralFilter(cv::InputArray, cv::InputArray, cv::OutputArray, int, double, double, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:317
#[inline]
pub fn joint_bilateral_filter(joint: &dyn core::ToInputArray, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, d: i32, sigma_color: f64, sigma_space: f64, border_type: i32) -> Result<()> {
	input_array_arg!(joint);
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_jointBilateralFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_double_int(joint.as_raw__InputArray(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), d, sigma_color, sigma_space, border_type, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * lambda: 0.02
/// * kappa: 2.0
// l0Smooth(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/ximgproc/edge_filter.hpp:523
#[inline]
pub fn l0_smooth(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, lambda: f64, kappa: f64) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_l0Smooth_const__InputArrayR_const__OutputArrayR_double_double(src.as_raw__InputArray(), dst.as_raw__OutputArray(), lambda, kappa, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * binarization_method: BINARIZATION_NIBLACK
/// * r: 128
// niBlackThreshold(cv::InputArray, cv::OutputArray, double, int, int, double, int, double) /usr/include/opencv2/ximgproc.hpp:176
#[inline]
pub fn ni_black_threshold(_src: &dyn core::ToInputArray, _dst: &mut dyn core::ToOutputArray, max_value: f64, typ: i32, block_size: i32, k: f64, binarization_method: i32, r: f64) -> Result<()> {
	input_array_arg!(_src);
	output_array_arg!(_dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_niBlackThreshold_const__InputArrayR_const__OutputArrayR_double_int_int_double_int_double(_src.as_raw__InputArray(), _dst.as_raw__OutputArray(), max_value, typ, block_size, k, binarization_method, r, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// qconj(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/color_match.hpp:30
#[inline]
pub fn qconj(qimg: &dyn core::ToInputArray, qcimg: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(qimg);
	output_array_arg!(qcimg);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_qconj_const__InputArrayR_const__OutputArrayR(qimg.as_raw__InputArray(), qcimg.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// qdft(cv::InputArray, cv::OutputArray, int, bool) /usr/include/opencv2/ximgproc/color_match.hpp:54
#[inline]
pub fn qdft(img: &dyn core::ToInputArray, qimg: &mut dyn core::ToOutputArray, flags: i32, side_left: bool) -> Result<()> {
	input_array_arg!(img);
	output_array_arg!(qimg);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_qdft_const__InputArrayR_const__OutputArrayR_int_bool(img.as_raw__InputArray(), qimg.as_raw__OutputArray(), flags, side_left, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// qmultiply(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/color_match.hpp:45
#[inline]
pub fn qmultiply(src1: &dyn core::ToInputArray, src2: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(src1);
	input_array_arg!(src2);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_qmultiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// qunitary(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/color_match.hpp:37
#[inline]
pub fn qunitary(qimg: &dyn core::ToInputArray, qnimg: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(qimg);
	output_array_arg!(qnimg);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_qunitary_const__InputArrayR_const__OutputArrayR(qimg.as_raw__InputArray(), qnimg.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// readGT(cv::String, cv::OutputArray) /usr/include/opencv2/ximgproc/disparity_filter.hpp:164
#[inline]
pub fn read_gt(src_path: &str, dst: &mut dyn core::ToOutputArray) -> Result<i32> {
	extern_container_arg!(mut src_path);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_readGT_String_const__OutputArrayR(src_path.opencv_as_extern_mut(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * size: Size(0,0)
// createRLEImage(const std::vector<cv::Point3i> &, cv::OutputArray, cv::Size) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:97
#[inline]
pub fn create_rle_image(runs: &core::Vector<core::Point3i>, res: &mut dyn core::ToOutputArray, size: core::Size) -> Result<()> {
	output_array_arg!(res);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_rl_createRLEImage_const_vector_Point3i_R_const__OutputArrayR_Size(runs.as_raw_VectorOfPoint3i(), res.as_raw__OutputArray(), size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * anchor: Point(0,0)
// dilate(cv::InputArray, cv::OutputArray, cv::InputArray, cv::Point) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:42
#[inline]
pub fn dilate(rl_src: &dyn core::ToInputArray, rl_dest: &mut dyn core::ToOutputArray, rl_kernel: &dyn core::ToInputArray, anchor: core::Point) -> Result<()> {
	input_array_arg!(rl_src);
	output_array_arg!(rl_dest);
	input_array_arg!(rl_kernel);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_rl_dilate_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Point(rl_src.as_raw__InputArray(), rl_dest.as_raw__OutputArray(), rl_kernel.as_raw__InputArray(), anchor.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * b_boundary_on: true
/// * anchor: Point(0,0)
// erode(cv::InputArray, cv::OutputArray, cv::InputArray, bool, cv::Point) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:57
#[inline]
pub fn erode(rl_src: &dyn core::ToInputArray, rl_dest: &mut dyn core::ToOutputArray, rl_kernel: &dyn core::ToInputArray, b_boundary_on: bool, anchor: core::Point) -> Result<()> {
	input_array_arg!(rl_src);
	output_array_arg!(rl_dest);
	input_array_arg!(rl_kernel);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_rl_erode_const__InputArrayR_const__OutputArrayR_const__InputArrayR_bool_Point(rl_src.as_raw__InputArray(), rl_dest.as_raw__OutputArray(), rl_kernel.as_raw__InputArray(), b_boundary_on, anchor.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getStructuringElement(int, cv::Size) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:68
#[inline]
pub fn get_structuring_element(shape: i32, ksize: core::Size) -> Result<core::Mat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_rl_getStructuringElement_int_Size(shape, ksize.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

// isRLMorphologyPossible(cv::InputArray) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:87
#[inline]
pub fn is_rl_morphology_possible(rl_structuring_element: &dyn core::ToInputArray) -> Result<bool> {
	input_array_arg!(rl_structuring_element);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_rl_isRLMorphologyPossible_const__InputArrayR(rl_structuring_element.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * b_boundary_on_for_erosion: true
/// * anchor: Point(0,0)
// morphologyEx(cv::InputArray, cv::OutputArray, int, cv::InputArray, bool, cv::Point) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:113
#[inline]
pub fn morphology_ex(rl_src: &dyn core::ToInputArray, rl_dest: &mut dyn core::ToOutputArray, op: i32, rl_kernel: &dyn core::ToInputArray, b_boundary_on_for_erosion: bool, anchor: core::Point) -> Result<()> {
	input_array_arg!(rl_src);
	output_array_arg!(rl_dest);
	input_array_arg!(rl_kernel);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_rl_morphologyEx_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_bool_Point(rl_src.as_raw__InputArray(), rl_dest.as_raw__OutputArray(), op, rl_kernel.as_raw__InputArray(), b_boundary_on_for_erosion, anchor.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// paint(cv::InputOutputArray, cv::InputArray, const cv::Scalar &) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:79
#[inline]
pub fn paint(image: &mut dyn core::ToInputOutputArray, rl_src: &dyn core::ToInputArray, value: core::Scalar) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(rl_src);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_rl_paint_const__InputOutputArrayR_const__InputArrayR_const_ScalarR(image.as_raw__InputOutputArray(), rl_src.as_raw__InputArray(), &value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// threshold(cv::InputArray, cv::OutputArray, double, int) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:28
#[inline]
pub fn threshold(src: &dyn core::ToInputArray, rl_dest: &mut dyn core::ToOutputArray, thresh: f64, typ: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(rl_dest);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_rl_threshold_const__InputArrayR_const__OutputArrayR_double_int(src.as_raw__InputArray(), rl_dest.as_raw__OutputArray(), thresh, typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * d: -1
/// * sigma_color: 25
/// * sigma_space: 3
/// * num_of_iter: 4
/// * border_type: BORDER_DEFAULT
// rollingGuidanceFilter(cv::InputArray, cv::OutputArray, int, double, double, int, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:373
#[inline]
pub fn rolling_guidance_filter(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, d: i32, sigma_color: f64, sigma_space: f64, num_of_iter: i32, border_type: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_rollingGuidanceFilter_const__InputArrayR_const__OutputArrayR_int_double_double_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), d, sigma_color, sigma_space, num_of_iter, border_type, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * sigma: 0.5
/// * k: 300
/// * min_size: 100
// createGraphSegmentation(double, float, int) /usr/include/opencv2/ximgproc/segmentation.hpp:69
#[inline]
pub fn create_graph_segmentation(sigma: f64, k: f32, min_size: i32) -> Result<core::Ptr<dyn crate::ximgproc::GraphSegmentation>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_segmentation_createGraphSegmentation_double_float_int(sigma, k, min_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::GraphSegmentation>::opencv_from_extern(ret) };
	Ok(ret)
}

// createSelectiveSearchSegmentation() /usr/include/opencv2/ximgproc/segmentation.hpp:244
#[inline]
pub fn create_selective_search_segmentation() -> Result<core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentation>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentation(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentation>::opencv_from_extern(ret) };
	Ok(ret)
}

// createSelectiveSearchSegmentationStrategyColor() /usr/include/opencv2/ximgproc/segmentation.hpp:104
#[inline]
pub fn create_selective_search_segmentation_strategy_color() -> Result<core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyColor>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyColor(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyColor>::opencv_from_extern(ret) };
	Ok(ret)
}

// createSelectiveSearchSegmentationStrategyFill() /usr/include/opencv2/ximgproc/segmentation.hpp:131
#[inline]
pub fn create_selective_search_segmentation_strategy_fill() -> Result<core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyFill>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyFill(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyFill>::opencv_from_extern(ret) };
	Ok(ret)
}

// createSelectiveSearchSegmentationStrategyMultiple() /usr/include/opencv2/ximgproc/segmentation.hpp:149
#[inline]
pub fn create_selective_search_segmentation_strategy_multiple() -> Result<core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>::opencv_from_extern(ret) };
	Ok(ret)
}

// createSelectiveSearchSegmentationStrategyMultiple(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>) /usr/include/opencv2/ximgproc/segmentation.hpp:154
#[inline]
pub fn create_selective_search_segmentation_strategy_multiple_1(mut s1: core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>) -> Result<core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_Ptr_SelectiveSearchSegmentationStrategy_(s1.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>::opencv_from_extern(ret) };
	Ok(ret)
}

// createSelectiveSearchSegmentationStrategyMultiple(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>) /usr/include/opencv2/ximgproc/segmentation.hpp:160
#[inline]
pub fn create_selective_search_segmentation_strategy_multiple_2(mut s1: core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>, mut s2: core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>) -> Result<core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy_(s1.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), s2.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>::opencv_from_extern(ret) };
	Ok(ret)
}

// createSelectiveSearchSegmentationStrategyMultiple(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>) /usr/include/opencv2/ximgproc/segmentation.hpp:168
#[inline]
pub fn create_selective_search_segmentation_strategy_multiple_3(mut s1: core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>, mut s2: core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>, mut s3: core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>) -> Result<core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy_(s1.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), s2.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), s3.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>::opencv_from_extern(ret) };
	Ok(ret)
}

// createSelectiveSearchSegmentationStrategyMultiple(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>) /usr/include/opencv2/ximgproc/segmentation.hpp:176
#[inline]
pub fn create_selective_search_segmentation_strategy_multiple_4(mut s1: core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>, mut s2: core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>, mut s3: core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>, mut s4: core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>) -> Result<core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy_(s1.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), s2.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), s3.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), s4.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>::opencv_from_extern(ret) };
	Ok(ret)
}

// createSelectiveSearchSegmentationStrategySize() /usr/include/opencv2/ximgproc/segmentation.hpp:113
#[inline]
pub fn create_selective_search_segmentation_strategy_size() -> Result<core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategySize>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategySize(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategySize>::opencv_from_extern(ret) };
	Ok(ret)
}

// createSelectiveSearchSegmentationStrategyTexture() /usr/include/opencv2/ximgproc/segmentation.hpp:122
#[inline]
pub fn create_selective_search_segmentation_strategy_texture() -> Result<core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyTexture>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyTexture(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyTexture>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * thinning_type: THINNING_ZHANGSUEN
// thinning(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ximgproc.hpp:189
#[inline]
pub fn thinning(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, thinning_type: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_thinning_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), thinning_type, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * fd_contour: true
// transformFD(cv::InputArray, cv::InputArray, cv::OutputArray, bool) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:97
#[inline]
pub fn transform_fd(src: &dyn core::ToInputArray, t: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, fd_contour: bool) -> Result<()> {
	input_array_arg!(src);
	input_array_arg!(t);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_transformFD_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(src.as_raw__InputArray(), t.as_raw__InputArray(), dst.as_raw__OutputArray(), fd_contour, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * sigma: 25.5
/// * weight_type: WMF_EXP
/// * mask: noArray()
// weightedMedianFilter(cv::InputArray, cv::InputArray, cv::OutputArray, int, double, int, cv::InputArray) /usr/include/opencv2/ximgproc/weighted_median_filter.hpp:90
#[inline]
pub fn weighted_median_filter(joint: &dyn core::ToInputArray, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, r: i32, sigma: f64, weight_type: i32, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(joint);
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ximgproc_weightedMedianFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_int_const__InputArrayR(joint.as_raw__InputArray(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), r, sigma, weight_type, mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// AdaptiveManifoldFilter /usr/include/opencv2/ximgproc/edge_filter.hpp:203
pub trait AdaptiveManifoldFilterConst: core::AlgorithmTraitConst {
	fn as_raw_AdaptiveManifoldFilter(&self) -> *const c_void;

	// getSigmaS() /usr/include/opencv2/ximgproc/edge_filter.hpp:221
	#[inline]
	fn get_sigma_s(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_getSigmaS_const(self.as_raw_AdaptiveManifoldFilter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSigmaR() /usr/include/opencv2/ximgproc/edge_filter.hpp:225
	#[inline]
	fn get_sigma_r(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_getSigmaR_const(self.as_raw_AdaptiveManifoldFilter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getTreeHeight() /usr/include/opencv2/ximgproc/edge_filter.hpp:229
	#[inline]
	fn get_tree_height(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_getTreeHeight_const(self.as_raw_AdaptiveManifoldFilter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getPCAIterations() /usr/include/opencv2/ximgproc/edge_filter.hpp:233
	#[inline]
	fn get_pca_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_getPCAIterations_const(self.as_raw_AdaptiveManifoldFilter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getAdjustOutliers() /usr/include/opencv2/ximgproc/edge_filter.hpp:237
	#[inline]
	fn get_adjust_outliers(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_getAdjustOutliers_const(self.as_raw_AdaptiveManifoldFilter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getUseRNG() /usr/include/opencv2/ximgproc/edge_filter.hpp:241
	#[inline]
	fn get_use_rng(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_getUseRNG_const(self.as_raw_AdaptiveManifoldFilter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait AdaptiveManifoldFilter: core::AlgorithmTrait + crate::ximgproc::AdaptiveManifoldFilterConst {
	fn as_raw_mut_AdaptiveManifoldFilter(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * joint: noArray()
	// filter(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/ximgproc/edge_filter.hpp:214
	#[inline]
	fn filter(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, joint: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(joint);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_filter_const__InputArrayR_const__OutputArrayR_const__InputArrayR(self.as_raw_mut_AdaptiveManifoldFilter(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), joint.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// collectGarbage() /usr/include/opencv2/ximgproc/edge_filter.hpp:216
	#[inline]
	fn collect_garbage(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_collectGarbage(self.as_raw_mut_AdaptiveManifoldFilter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setSigmaS(double) /usr/include/opencv2/ximgproc/edge_filter.hpp:223
	#[inline]
	fn set_sigma_s(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_setSigmaS_double(self.as_raw_mut_AdaptiveManifoldFilter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setSigmaR(double) /usr/include/opencv2/ximgproc/edge_filter.hpp:227
	#[inline]
	fn set_sigma_r(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_setSigmaR_double(self.as_raw_mut_AdaptiveManifoldFilter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setTreeHeight(int) /usr/include/opencv2/ximgproc/edge_filter.hpp:231
	#[inline]
	fn set_tree_height(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_setTreeHeight_int(self.as_raw_mut_AdaptiveManifoldFilter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setPCAIterations(int) /usr/include/opencv2/ximgproc/edge_filter.hpp:235
	#[inline]
	fn set_pca_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_setPCAIterations_int(self.as_raw_mut_AdaptiveManifoldFilter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setAdjustOutliers(bool) /usr/include/opencv2/ximgproc/edge_filter.hpp:239
	#[inline]
	fn set_adjust_outliers(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_setAdjustOutliers_bool(self.as_raw_mut_AdaptiveManifoldFilter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setUseRNG(bool) /usr/include/opencv2/ximgproc/edge_filter.hpp:243
	#[inline]
	fn set_use_rng(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_setUseRNG_bool(self.as_raw_mut_AdaptiveManifoldFilter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn AdaptiveManifoldFilter + '_ {
	// create() /usr/include/opencv2/ximgproc/edge_filter.hpp:218
	#[inline]
	pub fn create() -> Result<core::Ptr<dyn crate::ximgproc::AdaptiveManifoldFilter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_AdaptiveManifoldFilter_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::ximgproc::AdaptiveManifoldFilter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
// ContourFitting /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:20
pub trait ContourFittingTraitConst: core::AlgorithmTraitConst {
	fn as_raw_ContourFitting(&self) -> *const c_void;

}

pub trait ContourFittingTrait: core::AlgorithmTrait + crate::ximgproc::ContourFittingTraitConst {
	fn as_raw_mut_ContourFitting(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * dist: 0
	/// * fd_contour: false
	// estimateTransformation(cv::InputArray, cv::InputArray, cv::OutputArray, double *, bool) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:47
	#[inline]
	fn estimate_transformation(&mut self, src: &dyn core::ToInputArray, dst: &dyn core::ToInputArray, alpha_phi_st: &mut dyn core::ToOutputArray, dist: &mut f64, fd_contour: bool) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(dst);
		output_array_arg!(alpha_phi_st);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_doubleX_bool(self.as_raw_mut_ContourFitting(), src.as_raw__InputArray(), dst.as_raw__InputArray(), alpha_phi_st.as_raw__OutputArray(), dist, fd_contour, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * fd_contour: false
	// estimateTransformation(cv::InputArray, cv::InputArray, cv::OutputArray, double &, bool) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:56
	#[inline]
	fn estimate_transformation_1(&mut self, src: &dyn core::ToInputArray, dst: &dyn core::ToInputArray, alpha_phi_st: &mut dyn core::ToOutputArray, dist: &mut f64, fd_contour: bool) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(dst);
		output_array_arg!(alpha_phi_st);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_doubleR_bool(self.as_raw_mut_ContourFitting(), src.as_raw__InputArray(), dst.as_raw__InputArray(), alpha_phi_st.as_raw__OutputArray(), dist, fd_contour, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setCtrSize(int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:61
	#[inline]
	fn set_ctr_size(&mut self, n: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_ContourFitting_setCtrSize_int(self.as_raw_mut_ContourFitting(), n, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setFDSize(int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:66
	#[inline]
	fn set_fd_size(&mut self, n: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_ContourFitting_setFDSize_int(self.as_raw_mut_ContourFitting(), n, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getCtrSize() /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:70
	#[inline]
	fn get_ctr_size(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_ContourFitting_getCtrSize(self.as_raw_mut_ContourFitting(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getFDSize() /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:74
	#[inline]
	fn get_fd_size(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_ContourFitting_getFDSize(self.as_raw_mut_ContourFitting(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// ContourFitting /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:20
pub struct ContourFitting {
	ptr: *mut c_void
}

opencv_type_boxed! { ContourFitting }

impl Drop for ContourFitting {
	fn drop(&mut self) {
		extern "C" { fn cv_ContourFitting_delete(instance: *mut c_void); }
		unsafe { cv_ContourFitting_delete(self.as_raw_mut_ContourFitting()) };
	}
}

unsafe impl Send for ContourFitting {}

impl core::AlgorithmTraitConst for ContourFitting {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ContourFitting {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::ContourFittingTraitConst for ContourFitting {
	#[inline] fn as_raw_ContourFitting(&self) -> *const c_void { self.as_raw() }
}

impl crate::ximgproc::ContourFittingTrait for ContourFitting {
	#[inline] fn as_raw_mut_ContourFitting(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ContourFitting {
	/// ## C++ default parameters
	/// * ctr: 1024
	/// * fd: 16
	// ContourFitting(int, int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:38
	#[inline]
	pub fn new(ctr: i32, fd: i32) -> Result<crate::ximgproc::ContourFitting> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_ContourFitting_ContourFitting_int_int(ctr, fd, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::ximgproc::ContourFitting::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ContourFitting, core::Algorithm, cv_ContourFitting_to_Algorithm }

// DTFilter /usr/include/opencv2/ximgproc/edge_filter.hpp:66
pub trait DTFilterConst: core::AlgorithmTraitConst {
	fn as_raw_DTFilter(&self) -> *const c_void;

}

pub trait DTFilter: core::AlgorithmTrait + crate::ximgproc::DTFilterConst {
	fn as_raw_mut_DTFilter(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * d_depth: -1
	// filter(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:79
	#[inline]
	fn filter(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, d_depth: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_DTFilter_filter_const__InputArrayR_const__OutputArrayR_int(self.as_raw_mut_DTFilter(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), d_depth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// DisparityFilter /usr/include/opencv2/ximgproc/disparity_filter.hpp:52
pub trait DisparityFilterConst: core::AlgorithmTraitConst {
	fn as_raw_DisparityFilter(&self) -> *const c_void;

}

pub trait DisparityFilter: core::AlgorithmTrait + crate::ximgproc::DisparityFilterConst {
	fn as_raw_mut_DisparityFilter(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * disparity_map_right: Mat()
	/// * roi: Rect()
	/// * right_view: Mat()
	// filter(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray, cv::Rect, cv::InputArray) /usr/include/opencv2/ximgproc/disparity_filter.hpp:75
	#[inline]
	fn filter(&mut self, disparity_map_left: &dyn core::ToInputArray, left_view: &dyn core::ToInputArray, filtered_disparity_map: &mut dyn core::ToOutputArray, disparity_map_right: &dyn core::ToInputArray, roi: core::Rect, right_view: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(disparity_map_left);
		input_array_arg!(left_view);
		output_array_arg!(filtered_disparity_map);
		input_array_arg!(disparity_map_right);
		input_array_arg!(right_view);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_DisparityFilter_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Rect_const__InputArrayR(self.as_raw_mut_DisparityFilter(), disparity_map_left.as_raw__InputArray(), left_view.as_raw__InputArray(), filtered_disparity_map.as_raw__OutputArray(), disparity_map_right.as_raw__InputArray(), roi.opencv_as_extern(), right_view.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// DisparityWLSFilter /usr/include/opencv2/ximgproc/disparity_filter.hpp:82
pub trait DisparityWLSFilterConst: crate::ximgproc::DisparityFilterConst {
	fn as_raw_DisparityWLSFilter(&self) -> *const c_void;

}

pub trait DisparityWLSFilter: crate::ximgproc::DisparityFilter + crate::ximgproc::DisparityWLSFilterConst {
	fn as_raw_mut_DisparityWLSFilter(&mut self) -> *mut c_void;

	// getLambda() /usr/include/opencv2/ximgproc/disparity_filter.hpp:90
	#[inline]
	fn get_lambda(&mut self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_getLambda(self.as_raw_mut_DisparityWLSFilter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setLambda(double) /usr/include/opencv2/ximgproc/disparity_filter.hpp:92
	#[inline]
	fn set_lambda(&mut self, _lambda: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_setLambda_double(self.as_raw_mut_DisparityWLSFilter(), _lambda, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSigmaColor() /usr/include/opencv2/ximgproc/disparity_filter.hpp:97
	#[inline]
	fn get_sigma_color(&mut self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_getSigmaColor(self.as_raw_mut_DisparityWLSFilter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setSigmaColor(double) /usr/include/opencv2/ximgproc/disparity_filter.hpp:99
	#[inline]
	fn set_sigma_color(&mut self, _sigma_color: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_setSigmaColor_double(self.as_raw_mut_DisparityWLSFilter(), _sigma_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getLRCthresh() /usr/include/opencv2/ximgproc/disparity_filter.hpp:106
	#[inline]
	fn get_lr_cthresh(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_getLRCthresh(self.as_raw_mut_DisparityWLSFilter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setLRCthresh(int) /usr/include/opencv2/ximgproc/disparity_filter.hpp:108
	#[inline]
	fn set_lr_cthresh(&mut self, _lrc_thresh: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_setLRCthresh_int(self.as_raw_mut_DisparityWLSFilter(), _lrc_thresh, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getDepthDiscontinuityRadius() /usr/include/opencv2/ximgproc/disparity_filter.hpp:112
	#[inline]
	fn get_depth_discontinuity_radius(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_getDepthDiscontinuityRadius(self.as_raw_mut_DisparityWLSFilter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setDepthDiscontinuityRadius(int) /usr/include/opencv2/ximgproc/disparity_filter.hpp:114
	#[inline]
	fn set_depth_discontinuity_radius(&mut self, _disc_radius: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_setDepthDiscontinuityRadius_int(self.as_raw_mut_DisparityWLSFilter(), _disc_radius, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getConfidenceMap() /usr/include/opencv2/ximgproc/disparity_filter.hpp:119
	#[inline]
	fn get_confidence_map(&mut self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_getConfidenceMap(self.as_raw_mut_DisparityWLSFilter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getROI() /usr/include/opencv2/ximgproc/disparity_filter.hpp:122
	#[inline]
	fn get_roi(&mut self) -> Result<core::Rect> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_DisparityWLSFilter_getROI(self.as_raw_mut_DisparityWLSFilter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// EdgeAwareInterpolator /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:77
pub trait EdgeAwareInterpolatorConst: crate::ximgproc::SparseMatchInterpolatorConst {
	fn as_raw_EdgeAwareInterpolator(&self) -> *const c_void;

}

pub trait EdgeAwareInterpolator: crate::ximgproc::EdgeAwareInterpolatorConst + crate::ximgproc::SparseMatchInterpolator {
	fn as_raw_mut_EdgeAwareInterpolator(&mut self) -> *mut c_void;

	// setCostMap(const cv::Mat &) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:88
	#[inline]
	fn set_cost_map(&mut self, _cost_map: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_setCostMap_const_MatR(self.as_raw_mut_EdgeAwareInterpolator(), _cost_map.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setK(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:96
	#[inline]
	fn set_k(&mut self, _k: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_setK_int(self.as_raw_mut_EdgeAwareInterpolator(), _k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getK() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:98
	#[inline]
	fn get_k(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_getK(self.as_raw_mut_EdgeAwareInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setSigma(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:104
	#[inline]
	fn set_sigma(&mut self, _sigma: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_setSigma_float(self.as_raw_mut_EdgeAwareInterpolator(), _sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSigma() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:106
	#[inline]
	fn get_sigma(&mut self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_getSigma(self.as_raw_mut_EdgeAwareInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setLambda(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:111
	#[inline]
	fn set_lambda(&mut self, _lambda: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_setLambda_float(self.as_raw_mut_EdgeAwareInterpolator(), _lambda, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getLambda() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:113
	#[inline]
	fn get_lambda(&mut self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_getLambda(self.as_raw_mut_EdgeAwareInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setUsePostProcessing(bool) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:118
	#[inline]
	fn set_use_post_processing(&mut self, _use_post_proc: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_setUsePostProcessing_bool(self.as_raw_mut_EdgeAwareInterpolator(), _use_post_proc, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getUsePostProcessing() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:120
	#[inline]
	fn get_use_post_processing(&mut self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_getUsePostProcessing(self.as_raw_mut_EdgeAwareInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setFGSLambda(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:124
	#[inline]
	fn set_fgs_lambda(&mut self, _lambda: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_setFGSLambda_float(self.as_raw_mut_EdgeAwareInterpolator(), _lambda, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getFGSLambda() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:126
	#[inline]
	fn get_fgs_lambda(&mut self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_getFGSLambda(self.as_raw_mut_EdgeAwareInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setFGSSigma(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:129
	#[inline]
	fn set_fgs_sigma(&mut self, _sigma: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_setFGSSigma_float(self.as_raw_mut_EdgeAwareInterpolator(), _sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getFGSSigma() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:131
	#[inline]
	fn get_fgs_sigma(&mut self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeAwareInterpolator_getFGSSigma(self.as_raw_mut_EdgeAwareInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// EdgeBoxes /usr/include/opencv2/ximgproc/edgeboxes.hpp:67
pub trait EdgeBoxesConst: core::AlgorithmTraitConst {
	fn as_raw_EdgeBoxes(&self) -> *const c_void;

	// getAlpha() /usr/include/opencv2/ximgproc/edgeboxes.hpp:83
	#[inline]
	fn get_alpha(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_getAlpha_const(self.as_raw_EdgeBoxes(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getBeta() /usr/include/opencv2/ximgproc/edgeboxes.hpp:90
	#[inline]
	fn get_beta(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_getBeta_const(self.as_raw_EdgeBoxes(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getEta() /usr/include/opencv2/ximgproc/edgeboxes.hpp:97
	#[inline]
	fn get_eta(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_getEta_const(self.as_raw_EdgeBoxes(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getMinScore() /usr/include/opencv2/ximgproc/edgeboxes.hpp:104
	#[inline]
	fn get_min_score(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_getMinScore_const(self.as_raw_EdgeBoxes(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getMaxBoxes() /usr/include/opencv2/ximgproc/edgeboxes.hpp:111
	#[inline]
	fn get_max_boxes(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_getMaxBoxes_const(self.as_raw_EdgeBoxes(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getEdgeMinMag() /usr/include/opencv2/ximgproc/edgeboxes.hpp:118
	#[inline]
	fn get_edge_min_mag(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_getEdgeMinMag_const(self.as_raw_EdgeBoxes(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getEdgeMergeThr() /usr/include/opencv2/ximgproc/edgeboxes.hpp:125
	#[inline]
	fn get_edge_merge_thr(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_getEdgeMergeThr_const(self.as_raw_EdgeBoxes(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getClusterMinMag() /usr/include/opencv2/ximgproc/edgeboxes.hpp:132
	#[inline]
	fn get_cluster_min_mag(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_getClusterMinMag_const(self.as_raw_EdgeBoxes(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getMaxAspectRatio() /usr/include/opencv2/ximgproc/edgeboxes.hpp:139
	#[inline]
	fn get_max_aspect_ratio(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_getMaxAspectRatio_const(self.as_raw_EdgeBoxes(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getMinBoxArea() /usr/include/opencv2/ximgproc/edgeboxes.hpp:146
	#[inline]
	fn get_min_box_area(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_getMinBoxArea_const(self.as_raw_EdgeBoxes(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getGamma() /usr/include/opencv2/ximgproc/edgeboxes.hpp:153
	#[inline]
	fn get_gamma(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_getGamma_const(self.as_raw_EdgeBoxes(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getKappa() /usr/include/opencv2/ximgproc/edgeboxes.hpp:160
	#[inline]
	fn get_kappa(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_getKappa_const(self.as_raw_EdgeBoxes(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait EdgeBoxes: core::AlgorithmTrait + crate::ximgproc::EdgeBoxesConst {
	fn as_raw_mut_EdgeBoxes(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * scores: noArray()
	// getBoundingBoxes(cv::InputArray, cv::InputArray, std::vector<Rect> &, cv::OutputArray) /usr/include/opencv2/ximgproc/edgeboxes.hpp:79
	#[inline]
	fn get_bounding_boxes(&mut self, edge_map: &dyn core::ToInputArray, orientation_map: &dyn core::ToInputArray, boxes: &mut core::Vector<core::Rect>, scores: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(edge_map);
		input_array_arg!(orientation_map);
		output_array_arg!(scores);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_getBoundingBoxes_const__InputArrayR_const__InputArrayR_vector_Rect_R_const__OutputArrayR(self.as_raw_mut_EdgeBoxes(), edge_map.as_raw__InputArray(), orientation_map.as_raw__InputArray(), boxes.as_raw_mut_VectorOfRect(), scores.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setAlpha(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:86
	#[inline]
	fn set_alpha(&mut self, value: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_setAlpha_float(self.as_raw_mut_EdgeBoxes(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setBeta(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:93
	#[inline]
	fn set_beta(&mut self, value: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_setBeta_float(self.as_raw_mut_EdgeBoxes(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setEta(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:100
	#[inline]
	fn set_eta(&mut self, value: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_setEta_float(self.as_raw_mut_EdgeBoxes(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setMinScore(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:107
	#[inline]
	fn set_min_score(&mut self, value: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_setMinScore_float(self.as_raw_mut_EdgeBoxes(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setMaxBoxes(int) /usr/include/opencv2/ximgproc/edgeboxes.hpp:114
	#[inline]
	fn set_max_boxes(&mut self, value: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_setMaxBoxes_int(self.as_raw_mut_EdgeBoxes(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setEdgeMinMag(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:121
	#[inline]
	fn set_edge_min_mag(&mut self, value: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_setEdgeMinMag_float(self.as_raw_mut_EdgeBoxes(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setEdgeMergeThr(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:128
	#[inline]
	fn set_edge_merge_thr(&mut self, value: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_setEdgeMergeThr_float(self.as_raw_mut_EdgeBoxes(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setClusterMinMag(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:135
	#[inline]
	fn set_cluster_min_mag(&mut self, value: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_setClusterMinMag_float(self.as_raw_mut_EdgeBoxes(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setMaxAspectRatio(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:142
	#[inline]
	fn set_max_aspect_ratio(&mut self, value: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_setMaxAspectRatio_float(self.as_raw_mut_EdgeBoxes(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setMinBoxArea(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:149
	#[inline]
	fn set_min_box_area(&mut self, value: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_setMinBoxArea_float(self.as_raw_mut_EdgeBoxes(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setGamma(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:156
	#[inline]
	fn set_gamma(&mut self, value: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_setGamma_float(self.as_raw_mut_EdgeBoxes(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setKappa(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:163
	#[inline]
	fn set_kappa(&mut self, value: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeBoxes_setKappa_float(self.as_raw_mut_EdgeBoxes(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// EdgeDrawing /usr/include/opencv2/ximgproc/edge_drawing.hpp:21
pub trait EdgeDrawingConst: core::AlgorithmTraitConst {
	fn as_raw_EdgeDrawing(&self) -> *const c_void;

	// params /usr/include/opencv2/ximgproc/edge_drawing.hpp:113
	#[inline]
	fn params(&self) -> crate::ximgproc::EdgeDrawing_Params {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeDrawing_getPropParams_const(self.as_raw_EdgeDrawing(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// getSegmentIndicesOfLines() /usr/include/opencv2/ximgproc/edge_drawing.hpp:97
	#[inline]
	fn get_segment_indices_of_lines(&self) -> Result<core::Vector<i32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeDrawing_getSegmentIndicesOfLines_const(self.as_raw_EdgeDrawing(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait EdgeDrawing: core::AlgorithmTrait + crate::ximgproc::EdgeDrawingConst {
	fn as_raw_mut_EdgeDrawing(&mut self) -> *mut c_void;

	// params /usr/include/opencv2/ximgproc/edge_drawing.hpp:113
	#[inline]
	fn set_params(&mut self, val: crate::ximgproc::EdgeDrawing_Params) {
		let ret = unsafe { sys::cv_ximgproc_EdgeDrawing_setPropParams_Params(self.as_raw_mut_EdgeDrawing(), val.opencv_as_extern()) };
		ret
	}
	
	// detectEdges(cv::InputArray) /usr/include/opencv2/ximgproc/edge_drawing.hpp:77
	#[inline]
	fn detect_edges(&mut self, src: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeDrawing_detectEdges_const__InputArrayR(self.as_raw_mut_EdgeDrawing(), src.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getEdgeImage(cv::OutputArray) /usr/include/opencv2/ximgproc/edge_drawing.hpp:83
	#[inline]
	fn get_edge_image(&mut self, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeDrawing_getEdgeImage_const__OutputArrayR(self.as_raw_mut_EdgeDrawing(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getGradientImage(cv::OutputArray) /usr/include/opencv2/ximgproc/edge_drawing.hpp:89
	#[inline]
	fn get_gradient_image(&mut self, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeDrawing_getGradientImage_const__OutputArrayR(self.as_raw_mut_EdgeDrawing(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSegments() /usr/include/opencv2/ximgproc/edge_drawing.hpp:93
	#[inline]
	fn get_segments(&mut self) -> Result<core::Vector<core::Vector<core::Point>>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeDrawing_getSegments(self.as_raw_mut_EdgeDrawing(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Vector<core::Point>>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// detectLines(cv::OutputArray) /usr/include/opencv2/ximgproc/edge_drawing.hpp:104
	#[inline]
	fn detect_lines(&mut self, lines: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(lines);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeDrawing_detectLines_const__OutputArrayR(self.as_raw_mut_EdgeDrawing(), lines.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// detectEllipses(cv::OutputArray) /usr/include/opencv2/ximgproc/edge_drawing.hpp:111
	#[inline]
	fn detect_ellipses(&mut self, ellipses: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(ellipses);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeDrawing_detectEllipses_const__OutputArrayR(self.as_raw_mut_EdgeDrawing(), ellipses.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setParams(const EdgeDrawing::Params &) /usr/include/opencv2/ximgproc/edge_drawing.hpp:120
	#[inline]
	fn set_params_1(&mut self, parameters: crate::ximgproc::EdgeDrawing_Params) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeDrawing_setParams_const_ParamsR(self.as_raw_mut_EdgeDrawing(), &parameters, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Params /usr/include/opencv2/ximgproc/edge_drawing.hpp:33
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EdgeDrawing_Params {
	pub p_fmode: bool,
	pub edge_detection_operator: i32,
	pub gradient_threshold_value: i32,
	pub anchor_threshold_value: i32,
	pub scan_interval: i32,
	pub min_path_length: i32,
	pub sigma: f32,
	pub sum_flag: bool,
	pub nfa_validation: bool,
	pub min_line_length: i32,
	pub max_distance_between_two_lines: f64,
	pub line_fit_error_threshold: f64,
	pub max_error_threshold: f64,
}

opencv_type_simple! { crate::ximgproc::EdgeDrawing_Params }

impl EdgeDrawing_Params {
	// write(cv::FileStorage &) /usr/include/opencv2/ximgproc/edge_drawing.hpp:70
	#[inline]
	pub fn write(self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeDrawing_Params_write_const_FileStorageR(self.opencv_as_extern(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// Params() /usr/include/opencv2/ximgproc/edge_drawing.hpp:35
	#[inline]
	pub fn default() -> Result<crate::ximgproc::EdgeDrawing_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeDrawing_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/ximgproc/edge_drawing.hpp:69
	#[inline]
	pub fn read(self, fn_: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_EdgeDrawing_Params_read_const_FileNodeR(self.opencv_as_extern(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// FastBilateralSolverFilter /usr/include/opencv2/ximgproc/edge_filter.hpp:382
pub trait FastBilateralSolverFilterConst: core::AlgorithmTraitConst {
	fn as_raw_FastBilateralSolverFilter(&self) -> *const c_void;

}

pub trait FastBilateralSolverFilter: core::AlgorithmTrait + crate::ximgproc::FastBilateralSolverFilterConst {
	fn as_raw_mut_FastBilateralSolverFilter(&mut self) -> *mut c_void;

	// filter(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/edge_filter.hpp:395
	#[inline]
	fn filter(&mut self, src: &dyn core::ToInputArray, confidence: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(confidence);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_FastBilateralSolverFilter_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_FastBilateralSolverFilter(), src.as_raw__InputArray(), confidence.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// FastGlobalSmootherFilter /usr/include/opencv2/ximgproc/edge_filter.hpp:457
pub trait FastGlobalSmootherFilterConst: core::AlgorithmTraitConst {
	fn as_raw_FastGlobalSmootherFilter(&self) -> *const c_void;

}

pub trait FastGlobalSmootherFilter: core::AlgorithmTrait + crate::ximgproc::FastGlobalSmootherFilterConst {
	fn as_raw_mut_FastGlobalSmootherFilter(&mut self) -> *mut c_void;

	// filter(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/edge_filter.hpp:466
	#[inline]
	fn filter(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_FastGlobalSmootherFilter_filter_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_FastGlobalSmootherFilter(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// FastLineDetector /usr/include/opencv2/ximgproc/fast_line_detector.hpp:24
pub trait FastLineDetectorConst: core::AlgorithmTraitConst {
	fn as_raw_FastLineDetector(&self) -> *const c_void;

}

pub trait FastLineDetector: core::AlgorithmTrait + crate::ximgproc::FastLineDetectorConst {
	fn as_raw_mut_FastLineDetector(&mut self) -> *mut c_void;

	// detect(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/fast_line_detector.hpp:44
	#[inline]
	fn detect(&mut self, image: &dyn core::ToInputArray, lines: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(lines);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_FastLineDetector_detect_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_FastLineDetector(), image.as_raw__InputArray(), lines.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * draw_arrow: false
	/// * linecolor: Scalar(0,0,255)
	/// * linethickness: 1
	// drawSegments(cv::InputOutputArray, cv::InputArray, bool, cv::Scalar, int) /usr/include/opencv2/ximgproc/fast_line_detector.hpp:54
	#[inline]
	fn draw_segments(&mut self, image: &mut dyn core::ToInputOutputArray, lines: &dyn core::ToInputArray, draw_arrow: bool, linecolor: core::Scalar, linethickness: i32) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(lines);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_FastLineDetector_drawSegments_const__InputOutputArrayR_const__InputArrayR_bool_Scalar_int(self.as_raw_mut_FastLineDetector(), image.as_raw__InputOutputArray(), lines.as_raw__InputArray(), draw_arrow, linecolor.opencv_as_extern(), linethickness, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// GuidedFilter /usr/include/opencv2/ximgproc/edge_filter.hpp:130
pub trait GuidedFilterConst: core::AlgorithmTraitConst {
	fn as_raw_GuidedFilter(&self) -> *const c_void;

}

pub trait GuidedFilter: core::AlgorithmTrait + crate::ximgproc::GuidedFilterConst {
	fn as_raw_mut_GuidedFilter(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * d_depth: -1
	// filter(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:143
	#[inline]
	fn filter(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, d_depth: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_GuidedFilter_filter_const__InputArrayR_const__OutputArrayR_int(self.as_raw_mut_GuidedFilter(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), d_depth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// RFFeatureGetter /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:65
pub trait RFFeatureGetterConst: core::AlgorithmTraitConst {
	fn as_raw_RFFeatureGetter(&self) -> *const c_void;

	// getFeatures(const cv::Mat &, cv::Mat &, const int, const int, const int, const int, const int) /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:83
	#[inline]
	fn get_features(&self, src: &core::Mat, features: &mut core::Mat, gnrm_rad: i32, gsmth_rad: i32, shrink: i32, out_num: i32, grad_num: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RFFeatureGetter_getFeatures_const_const_MatR_MatR_const_int_const_int_const_int_const_int_const_int(self.as_raw_RFFeatureGetter(), src.as_raw_Mat(), features.as_raw_mut_Mat(), gnrm_rad, gsmth_rad, shrink, out_num, grad_num, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait RFFeatureGetter: core::AlgorithmTrait + crate::ximgproc::RFFeatureGetterConst {
	fn as_raw_mut_RFFeatureGetter(&mut self) -> *mut c_void;

}

// RICInterpolator /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:146
pub trait RICInterpolatorConst: crate::ximgproc::SparseMatchInterpolatorConst {
	fn as_raw_RICInterpolator(&self) -> *const c_void;

	// getK() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:157
	#[inline]
	fn get_k(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_getK_const(self.as_raw_RICInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSuperpixelSize() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:174
	#[inline]
	fn get_superpixel_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_getSuperpixelSize_const(self.as_raw_RICInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSuperpixelNNCnt() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:182
	#[inline]
	fn get_superpixel_nn_cnt(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_getSuperpixelNNCnt_const(self.as_raw_RICInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSuperpixelRuler() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:190
	#[inline]
	fn get_superpixel_ruler(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_getSuperpixelRuler_const(self.as_raw_RICInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSuperpixelMode() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:201
	#[inline]
	fn get_superpixel_mode(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_getSuperpixelMode_const(self.as_raw_RICInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getAlpha() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:208
	#[inline]
	fn get_alpha(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_getAlpha_const(self.as_raw_RICInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getModelIter() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:215
	#[inline]
	fn get_model_iter(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_getModelIter_const(self.as_raw_RICInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getRefineModels() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:222
	#[inline]
	fn get_refine_models(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_getRefineModels_const(self.as_raw_RICInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getMaxFlow() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:230
	#[inline]
	fn get_max_flow(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_getMaxFlow_const(self.as_raw_RICInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getUseVariationalRefinement() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:237
	#[inline]
	fn get_use_variational_refinement(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_getUseVariationalRefinement_const(self.as_raw_RICInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getUseGlobalSmootherFilter() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:244
	#[inline]
	fn get_use_global_smoother_filter(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_getUseGlobalSmootherFilter_const(self.as_raw_RICInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getFGSLambda() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:251
	#[inline]
	fn get_fgs_lambda(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_getFGSLambda_const(self.as_raw_RICInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getFGSSigma() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:258
	#[inline]
	fn get_fgs_sigma(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_getFGSSigma_const(self.as_raw_RICInterpolator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait RICInterpolator: crate::ximgproc::RICInterpolatorConst + crate::ximgproc::SparseMatchInterpolator {
	fn as_raw_mut_RICInterpolator(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * k: 32
	// setK(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:153
	#[inline]
	fn set_k(&mut self, k: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_setK_int(self.as_raw_mut_RICInterpolator(), k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setCostMap(const cv::Mat &) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:166
	#[inline]
	fn set_cost_map(&mut self, cost_map: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_setCostMap_const_MatR(self.as_raw_mut_RICInterpolator(), cost_map.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * sp_size: 15
	// setSuperpixelSize(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:170
	#[inline]
	fn set_superpixel_size(&mut self, sp_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_setSuperpixelSize_int(self.as_raw_mut_RICInterpolator(), sp_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * sp_nn: 150
	// setSuperpixelNNCnt(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:178
	#[inline]
	fn set_superpixel_nn_cnt(&mut self, sp_nn: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_setSuperpixelNNCnt_int(self.as_raw_mut_RICInterpolator(), sp_nn, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * ruler: 15.f
	// setSuperpixelRuler(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:186
	#[inline]
	fn set_superpixel_ruler(&mut self, ruler: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_setSuperpixelRuler_float(self.as_raw_mut_RICInterpolator(), ruler, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * mode: 100
	// setSuperpixelMode(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:197
	#[inline]
	fn set_superpixel_mode(&mut self, mode: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_setSuperpixelMode_int(self.as_raw_mut_RICInterpolator(), mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * alpha: 0.7f
	// setAlpha(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:204
	#[inline]
	fn set_alpha(&mut self, alpha: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_setAlpha_float(self.as_raw_mut_RICInterpolator(), alpha, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * model_iter: 4
	// setModelIter(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:211
	#[inline]
	fn set_model_iter(&mut self, model_iter: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_setModelIter_int(self.as_raw_mut_RICInterpolator(), model_iter, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * refine_modles: true
	// setRefineModels(bool) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:218
	#[inline]
	fn set_refine_models(&mut self, refine_modles: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_setRefineModels_bool(self.as_raw_mut_RICInterpolator(), refine_modles, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * max_flow: 250.f
	// setMaxFlow(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:226
	#[inline]
	fn set_max_flow(&mut self, max_flow: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_setMaxFlow_float(self.as_raw_mut_RICInterpolator(), max_flow, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * use_variational_refinement: false
	// setUseVariationalRefinement(bool) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:233
	#[inline]
	fn set_use_variational_refinement(&mut self, use_variational_refinement: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_setUseVariationalRefinement_bool(self.as_raw_mut_RICInterpolator(), use_variational_refinement, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * use_fgs: true
	// setUseGlobalSmootherFilter(bool) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:240
	#[inline]
	fn set_use_global_smoother_filter(&mut self, use_fgs: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_setUseGlobalSmootherFilter_bool(self.as_raw_mut_RICInterpolator(), use_fgs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * lambda: 500.f
	// setFGSLambda(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:247
	#[inline]
	fn set_fgs_lambda(&mut self, lambda: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_setFGSLambda_float(self.as_raw_mut_RICInterpolator(), lambda, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * sigma: 1.5f
	// setFGSSigma(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:254
	#[inline]
	fn set_fgs_sigma(&mut self, sigma: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RICInterpolator_setFGSSigma_float(self.as_raw_mut_RICInterpolator(), sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// RidgeDetectionFilter /usr/include/opencv2/ximgproc/ridgefilter.hpp:27
pub trait RidgeDetectionFilterConst: core::AlgorithmTraitConst {
	fn as_raw_RidgeDetectionFilter(&self) -> *const c_void;

}

pub trait RidgeDetectionFilter: core::AlgorithmTrait + crate::ximgproc::RidgeDetectionFilterConst {
	fn as_raw_mut_RidgeDetectionFilter(&mut self) -> *mut c_void;

	// getRidgeFilteredImage(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/ridgefilter.hpp:48
	#[inline]
	fn get_ridge_filtered_image(&mut self, _img: &dyn core::ToInputArray, out: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(_img);
		output_array_arg!(out);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RidgeDetectionFilter_getRidgeFilteredImage_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_RidgeDetectionFilter(), _img.as_raw__InputArray(), out.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn RidgeDetectionFilter + '_ {
	/// ## C++ default parameters
	/// * ddepth: CV_32FC1
	/// * dx: 1
	/// * dy: 1
	/// * ksize: 3
	/// * out_dtype: CV_8UC1
	/// * scale: 1
	/// * delta: 0
	/// * border_type: BORDER_DEFAULT
	// create(int, int, int, int, int, double, double, int) /usr/include/opencv2/ximgproc/ridgefilter.hpp:42
	#[inline]
	pub fn create(ddepth: i32, dx: i32, dy: i32, ksize: i32, out_dtype: i32, scale: f64, delta: f64, border_type: i32) -> Result<core::Ptr<dyn crate::ximgproc::RidgeDetectionFilter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_RidgeDetectionFilter_create_int_int_int_int_int_double_double_int(ddepth, dx, dy, ksize, out_dtype, scale, delta, border_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::ximgproc::RidgeDetectionFilter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
// ScanSegment /usr/include/opencv2/ximgproc/scansegment.hpp:23
pub trait ScanSegmentConst: core::AlgorithmTraitConst {
	fn as_raw_ScanSegment(&self) -> *const c_void;

}

pub trait ScanSegment: core::AlgorithmTrait + crate::ximgproc::ScanSegmentConst {
	fn as_raw_mut_ScanSegment(&mut self) -> *mut c_void;

	// getNumberOfSuperpixels() /usr/include/opencv2/ximgproc/scansegment.hpp:32
	#[inline]
	fn get_number_of_superpixels(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_ScanSegment_getNumberOfSuperpixels(self.as_raw_mut_ScanSegment(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// iterate(cv::InputArray) /usr/include/opencv2/ximgproc/scansegment.hpp:43
	#[inline]
	fn iterate(&mut self, img: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_ScanSegment_iterate_const__InputArrayR(self.as_raw_mut_ScanSegment(), img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getLabels(cv::OutputArray) /usr/include/opencv2/ximgproc/scansegment.hpp:52
	#[inline]
	fn get_labels(&mut self, labels_out: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(labels_out);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_ScanSegment_getLabels_const__OutputArrayR(self.as_raw_mut_ScanSegment(), labels_out.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * thick_line: false
	// getLabelContourMask(cv::OutputArray, bool) /usr/include/opencv2/ximgproc/scansegment.hpp:61
	#[inline]
	fn get_label_contour_mask(&mut self, image: &mut dyn core::ToOutputArray, thick_line: bool) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_ScanSegment_getLabelContourMask_const__OutputArrayR_bool(self.as_raw_mut_ScanSegment(), image.as_raw__OutputArray(), thick_line, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// SparseMatchInterpolator /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:52
pub trait SparseMatchInterpolatorConst: core::AlgorithmTraitConst {
	fn as_raw_SparseMatchInterpolator(&self) -> *const c_void;

}

pub trait SparseMatchInterpolator: core::AlgorithmTrait + crate::ximgproc::SparseMatchInterpolatorConst {
	fn as_raw_mut_SparseMatchInterpolator(&mut self) -> *mut c_void;

	// interpolate(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:69
	#[inline]
	fn interpolate(&mut self, from_image: &dyn core::ToInputArray, from_points: &dyn core::ToInputArray, to_image: &dyn core::ToInputArray, to_points: &dyn core::ToInputArray, dense_flow: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(from_image);
		input_array_arg!(from_points);
		input_array_arg!(to_image);
		input_array_arg!(to_points);
		output_array_arg!(dense_flow);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_SparseMatchInterpolator_interpolate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_SparseMatchInterpolator(), from_image.as_raw__InputArray(), from_points.as_raw__InputArray(), to_image.as_raw__InputArray(), to_points.as_raw__InputArray(), dense_flow.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// StructuredEdgeDetection /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:97
pub trait StructuredEdgeDetectionConst: core::AlgorithmTraitConst {
	fn as_raw_StructuredEdgeDetection(&self) -> *const c_void;

	// detectEdges(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:109
	#[inline]
	fn detect_edges(&self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_StructuredEdgeDetection_detectEdges_const_const__InputArrayR_const__OutputArrayR(self.as_raw_StructuredEdgeDetection(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// computeOrientation(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:116
	#[inline]
	fn compute_orientation(&self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_StructuredEdgeDetection_computeOrientation_const_const__InputArrayR_const__OutputArrayR(self.as_raw_StructuredEdgeDetection(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * r: 2
	/// * s: 0
	/// * m: 1
	/// * is_parallel: true
	// edgesNms(cv::InputArray, cv::InputArray, cv::OutputArray, int, int, float, bool) /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:129
	#[inline]
	fn edges_nms(&self, edge_image: &dyn core::ToInputArray, orientation_image: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, r: i32, s: i32, m: f32, is_parallel: bool) -> Result<()> {
		input_array_arg!(edge_image);
		input_array_arg!(orientation_image);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_StructuredEdgeDetection_edgesNms_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_float_bool(self.as_raw_StructuredEdgeDetection(), edge_image.as_raw__InputArray(), orientation_image.as_raw__InputArray(), dst.as_raw__OutputArray(), r, s, m, is_parallel, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait StructuredEdgeDetection: core::AlgorithmTrait + crate::ximgproc::StructuredEdgeDetectionConst {
	fn as_raw_mut_StructuredEdgeDetection(&mut self) -> *mut c_void;

}

// SuperpixelLSC /usr/include/opencv2/ximgproc/lsc.hpp:71
pub trait SuperpixelLSCConst: core::AlgorithmTraitConst {
	fn as_raw_SuperpixelLSC(&self) -> *const c_void;

	// getNumberOfSuperpixels() /usr/include/opencv2/ximgproc/lsc.hpp:78
	#[inline]
	fn get_number_of_superpixels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_SuperpixelLSC_getNumberOfSuperpixels_const(self.as_raw_SuperpixelLSC(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getLabels(cv::OutputArray) /usr/include/opencv2/ximgproc/lsc.hpp:106
	#[inline]
	fn get_labels(&self, labels_out: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(labels_out);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_SuperpixelLSC_getLabels_const_const__OutputArrayR(self.as_raw_SuperpixelLSC(), labels_out.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * thick_line: true
	// getLabelContourMask(cv::OutputArray, bool) /usr/include/opencv2/ximgproc/lsc.hpp:118
	#[inline]
	fn get_label_contour_mask(&self, image: &mut dyn core::ToOutputArray, thick_line: bool) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_SuperpixelLSC_getLabelContourMask_const_const__OutputArrayR_bool(self.as_raw_SuperpixelLSC(), image.as_raw__OutputArray(), thick_line, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait SuperpixelLSC: core::AlgorithmTrait + crate::ximgproc::SuperpixelLSCConst {
	fn as_raw_mut_SuperpixelLSC(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * num_iterations: 10
	// iterate(int) /usr/include/opencv2/ximgproc/lsc.hpp:94
	#[inline]
	fn iterate(&mut self, num_iterations: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_SuperpixelLSC_iterate_int(self.as_raw_mut_SuperpixelLSC(), num_iterations, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * min_element_size: 25
	// enforceLabelConnectivity(int) /usr/include/opencv2/ximgproc/lsc.hpp:129
	#[inline]
	fn enforce_label_connectivity(&mut self, min_element_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_SuperpixelLSC_enforceLabelConnectivity_int(self.as_raw_mut_SuperpixelLSC(), min_element_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// SuperpixelSEEDS /usr/include/opencv2/ximgproc/seeds.hpp:66
pub trait SuperpixelSEEDSConst: core::AlgorithmTraitConst {
	fn as_raw_SuperpixelSEEDS(&self) -> *const c_void;

}

pub trait SuperpixelSEEDS: core::AlgorithmTrait + crate::ximgproc::SuperpixelSEEDSConst {
	fn as_raw_mut_SuperpixelSEEDS(&mut self) -> *mut c_void;

	// getNumberOfSuperpixels() /usr/include/opencv2/ximgproc/seeds.hpp:75
	#[inline]
	fn get_number_of_superpixels(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_SuperpixelSEEDS_getNumberOfSuperpixels(self.as_raw_mut_SuperpixelSEEDS(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * num_iterations: 4
	// iterate(cv::InputArray, int) /usr/include/opencv2/ximgproc/seeds.hpp:99
	#[inline]
	fn iterate(&mut self, img: &dyn core::ToInputArray, num_iterations: i32) -> Result<()> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_SuperpixelSEEDS_iterate_const__InputArrayR_int(self.as_raw_mut_SuperpixelSEEDS(), img.as_raw__InputArray(), num_iterations, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getLabels(cv::OutputArray) /usr/include/opencv2/ximgproc/seeds.hpp:111
	#[inline]
	fn get_labels(&mut self, labels_out: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(labels_out);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_SuperpixelSEEDS_getLabels_const__OutputArrayR(self.as_raw_mut_SuperpixelSEEDS(), labels_out.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * thick_line: false
	// getLabelContourMask(cv::OutputArray, bool) /usr/include/opencv2/ximgproc/seeds.hpp:139
	#[inline]
	fn get_label_contour_mask(&mut self, image: &mut dyn core::ToOutputArray, thick_line: bool) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_SuperpixelSEEDS_getLabelContourMask_const__OutputArrayR_bool(self.as_raw_mut_SuperpixelSEEDS(), image.as_raw__OutputArray(), thick_line, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// SuperpixelSLIC /usr/include/opencv2/ximgproc/slic.hpp:78
pub trait SuperpixelSLICConst: core::AlgorithmTraitConst {
	fn as_raw_SuperpixelSLIC(&self) -> *const c_void;

	// getNumberOfSuperpixels() /usr/include/opencv2/ximgproc/slic.hpp:85
	#[inline]
	fn get_number_of_superpixels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_SuperpixelSLIC_getNumberOfSuperpixels_const(self.as_raw_SuperpixelSLIC(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getLabels(cv::OutputArray) /usr/include/opencv2/ximgproc/slic.hpp:113
	#[inline]
	fn get_labels(&self, labels_out: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(labels_out);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_SuperpixelSLIC_getLabels_const_const__OutputArrayR(self.as_raw_SuperpixelSLIC(), labels_out.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * thick_line: true
	// getLabelContourMask(cv::OutputArray, bool) /usr/include/opencv2/ximgproc/slic.hpp:125
	#[inline]
	fn get_label_contour_mask(&self, image: &mut dyn core::ToOutputArray, thick_line: bool) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_SuperpixelSLIC_getLabelContourMask_const_const__OutputArrayR_bool(self.as_raw_SuperpixelSLIC(), image.as_raw__OutputArray(), thick_line, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait SuperpixelSLIC: core::AlgorithmTrait + crate::ximgproc::SuperpixelSLICConst {
	fn as_raw_mut_SuperpixelSLIC(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * num_iterations: 10
	// iterate(int) /usr/include/opencv2/ximgproc/slic.hpp:101
	#[inline]
	fn iterate(&mut self, num_iterations: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_SuperpixelSLIC_iterate_int(self.as_raw_mut_SuperpixelSLIC(), num_iterations, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * min_element_size: 25
	// enforceLabelConnectivity(int) /usr/include/opencv2/ximgproc/slic.hpp:136
	#[inline]
	fn enforce_label_connectivity(&mut self, min_element_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_SuperpixelSLIC_enforceLabelConnectivity_int(self.as_raw_mut_SuperpixelSLIC(), min_element_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// GraphSegmentation /usr/include/opencv2/ximgproc/segmentation.hpp:46
pub trait GraphSegmentationConst: core::AlgorithmTraitConst {
	fn as_raw_GraphSegmentation(&self) -> *const c_void;

}

pub trait GraphSegmentation: core::AlgorithmTrait + crate::ximgproc::GraphSegmentationConst {
	fn as_raw_mut_GraphSegmentation(&mut self) -> *mut c_void;

	// processImage(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/segmentation.hpp:52
	#[inline]
	fn process_image(&mut self, src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_GraphSegmentation_processImage_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_GraphSegmentation(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setSigma(double) /usr/include/opencv2/ximgproc/segmentation.hpp:54
	#[inline]
	fn set_sigma(&mut self, sigma: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_GraphSegmentation_setSigma_double(self.as_raw_mut_GraphSegmentation(), sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSigma() /usr/include/opencv2/ximgproc/segmentation.hpp:55
	#[inline]
	fn get_sigma(&mut self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_GraphSegmentation_getSigma(self.as_raw_mut_GraphSegmentation(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setK(float) /usr/include/opencv2/ximgproc/segmentation.hpp:57
	#[inline]
	fn set_k(&mut self, k: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_GraphSegmentation_setK_float(self.as_raw_mut_GraphSegmentation(), k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getK() /usr/include/opencv2/ximgproc/segmentation.hpp:58
	#[inline]
	fn get_k(&mut self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_GraphSegmentation_getK(self.as_raw_mut_GraphSegmentation(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setMinSize(int) /usr/include/opencv2/ximgproc/segmentation.hpp:60
	#[inline]
	fn set_min_size(&mut self, min_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_GraphSegmentation_setMinSize_int(self.as_raw_mut_GraphSegmentation(), min_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getMinSize() /usr/include/opencv2/ximgproc/segmentation.hpp:61
	#[inline]
	fn get_min_size(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_GraphSegmentation_getMinSize(self.as_raw_mut_GraphSegmentation(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// SelectiveSearchSegmentation /usr/include/opencv2/ximgproc/segmentation.hpp:181
pub trait SelectiveSearchSegmentationConst: core::AlgorithmTraitConst {
	fn as_raw_SelectiveSearchSegmentation(&self) -> *const c_void;

}

pub trait SelectiveSearchSegmentation: core::AlgorithmTrait + crate::ximgproc::SelectiveSearchSegmentationConst {
	fn as_raw_mut_SelectiveSearchSegmentation(&mut self) -> *mut c_void;

	// setBaseImage(cv::InputArray) /usr/include/opencv2/ximgproc/segmentation.hpp:187
	#[inline]
	fn set_base_image(&mut self, img: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_setBaseImage_const__InputArrayR(self.as_raw_mut_SelectiveSearchSegmentation(), img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * k: 200
	/// * sigma: 0.8f
	// switchToSingleStrategy(int, float) /usr/include/opencv2/ximgproc/segmentation.hpp:193
	#[inline]
	fn switch_to_single_strategy(&mut self, k: i32, sigma: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSingleStrategy_int_float(self.as_raw_mut_SelectiveSearchSegmentation(), k, sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * base_k: 150
	/// * inc_k: 150
	/// * sigma: 0.8f
	// switchToSelectiveSearchFast(int, int, float) /usr/include/opencv2/ximgproc/segmentation.hpp:200
	#[inline]
	fn switch_to_selective_search_fast(&mut self, base_k: i32, inc_k: i32, sigma: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchFast_int_int_float(self.as_raw_mut_SelectiveSearchSegmentation(), base_k, inc_k, sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * base_k: 150
	/// * inc_k: 150
	/// * sigma: 0.8f
	// switchToSelectiveSearchQuality(int, int, float) /usr/include/opencv2/ximgproc/segmentation.hpp:207
	#[inline]
	fn switch_to_selective_search_quality(&mut self, base_k: i32, inc_k: i32, sigma: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchQuality_int_int_float(self.as_raw_mut_SelectiveSearchSegmentation(), base_k, inc_k, sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// addImage(cv::InputArray) /usr/include/opencv2/ximgproc/segmentation.hpp:212
	#[inline]
	fn add_image(&mut self, img: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_addImage_const__InputArrayR(self.as_raw_mut_SelectiveSearchSegmentation(), img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// clearImages() /usr/include/opencv2/ximgproc/segmentation.hpp:216
	#[inline]
	fn clear_images(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearImages(self.as_raw_mut_SelectiveSearchSegmentation(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// addGraphSegmentation(Ptr<cv::ximgproc::segmentation::GraphSegmentation>) /usr/include/opencv2/ximgproc/segmentation.hpp:221
	#[inline]
	fn add_graph_segmentation(&mut self, mut g: core::Ptr<dyn crate::ximgproc::GraphSegmentation>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_addGraphSegmentation_Ptr_GraphSegmentation_(self.as_raw_mut_SelectiveSearchSegmentation(), g.as_raw_mut_PtrOfGraphSegmentation(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// clearGraphSegmentations() /usr/include/opencv2/ximgproc/segmentation.hpp:225
	#[inline]
	fn clear_graph_segmentations(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearGraphSegmentations(self.as_raw_mut_SelectiveSearchSegmentation(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// addStrategy(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>) /usr/include/opencv2/ximgproc/segmentation.hpp:230
	#[inline]
	fn add_strategy(&mut self, mut s: core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_addStrategy_Ptr_SelectiveSearchSegmentationStrategy_(self.as_raw_mut_SelectiveSearchSegmentation(), s.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// clearStrategies() /usr/include/opencv2/ximgproc/segmentation.hpp:234
	#[inline]
	fn clear_strategies(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearStrategies(self.as_raw_mut_SelectiveSearchSegmentation(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// process(std::vector<Rect> &) /usr/include/opencv2/ximgproc/segmentation.hpp:239
	#[inline]
	fn process(&mut self, rects: &mut core::Vector<core::Rect>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentation_process_vector_Rect_R(self.as_raw_mut_SelectiveSearchSegmentation(), rects.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// SelectiveSearchSegmentationStrategy /usr/include/opencv2/ximgproc/segmentation.hpp:74
pub trait SelectiveSearchSegmentationStrategyConst: core::AlgorithmTraitConst {
	fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void;

}

pub trait SelectiveSearchSegmentationStrategy: core::AlgorithmTrait + crate::ximgproc::SelectiveSearchSegmentationStrategyConst {
	fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * image_id: -1
	// setImage(cv::InputArray, cv::InputArray, cv::InputArray, int) /usr/include/opencv2/ximgproc/segmentation.hpp:82
	#[inline]
	fn set_image(&mut self, img: &dyn core::ToInputArray, regions: &dyn core::ToInputArray, sizes: &dyn core::ToInputArray, image_id: i32) -> Result<()> {
		input_array_arg!(img);
		input_array_arg!(regions);
		input_array_arg!(sizes);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_setImage_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(self.as_raw_mut_SelectiveSearchSegmentationStrategy(), img.as_raw__InputArray(), regions.as_raw__InputArray(), sizes.as_raw__InputArray(), image_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// get(int, int) /usr/include/opencv2/ximgproc/segmentation.hpp:88
	#[inline]
	fn get(&mut self, r1: i32, r2: i32) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_get_int_int(self.as_raw_mut_SelectiveSearchSegmentationStrategy(), r1, r2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// merge(int, int) /usr/include/opencv2/ximgproc/segmentation.hpp:94
	#[inline]
	fn merge(&mut self, r1: i32, r2: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_merge_int_int(self.as_raw_mut_SelectiveSearchSegmentationStrategy(), r1, r2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// SelectiveSearchSegmentationStrategyColor /usr/include/opencv2/ximgproc/segmentation.hpp:100
pub trait SelectiveSearchSegmentationStrategyColorConst: crate::ximgproc::SelectiveSearchSegmentationStrategyConst {
	fn as_raw_SelectiveSearchSegmentationStrategyColor(&self) -> *const c_void;

}

pub trait SelectiveSearchSegmentationStrategyColor: crate::ximgproc::SelectiveSearchSegmentationStrategy + crate::ximgproc::SelectiveSearchSegmentationStrategyColorConst {
	fn as_raw_mut_SelectiveSearchSegmentationStrategyColor(&mut self) -> *mut c_void;

}

// SelectiveSearchSegmentationStrategyFill /usr/include/opencv2/ximgproc/segmentation.hpp:127
pub trait SelectiveSearchSegmentationStrategyFillConst: crate::ximgproc::SelectiveSearchSegmentationStrategyConst {
	fn as_raw_SelectiveSearchSegmentationStrategyFill(&self) -> *const c_void;

}

pub trait SelectiveSearchSegmentationStrategyFill: crate::ximgproc::SelectiveSearchSegmentationStrategy + crate::ximgproc::SelectiveSearchSegmentationStrategyFillConst {
	fn as_raw_mut_SelectiveSearchSegmentationStrategyFill(&mut self) -> *mut c_void;

}

// SelectiveSearchSegmentationStrategyMultiple /usr/include/opencv2/ximgproc/segmentation.hpp:135
pub trait SelectiveSearchSegmentationStrategyMultipleConst: crate::ximgproc::SelectiveSearchSegmentationStrategyConst {
	fn as_raw_SelectiveSearchSegmentationStrategyMultiple(&self) -> *const c_void;

}

pub trait SelectiveSearchSegmentationStrategyMultiple: crate::ximgproc::SelectiveSearchSegmentationStrategy + crate::ximgproc::SelectiveSearchSegmentationStrategyMultipleConst {
	fn as_raw_mut_SelectiveSearchSegmentationStrategyMultiple(&mut self) -> *mut c_void;

	// addStrategy(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, float) /usr/include/opencv2/ximgproc/segmentation.hpp:142
	#[inline]
	fn add_strategy(&mut self, mut g: core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>, weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_addStrategy_Ptr_SelectiveSearchSegmentationStrategy__float(self.as_raw_mut_SelectiveSearchSegmentationStrategyMultiple(), g.as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(), weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// clearStrategies() /usr/include/opencv2/ximgproc/segmentation.hpp:145
	#[inline]
	fn clear_strategies(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_clearStrategies(self.as_raw_mut_SelectiveSearchSegmentationStrategyMultiple(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// SelectiveSearchSegmentationStrategySize /usr/include/opencv2/ximgproc/segmentation.hpp:109
pub trait SelectiveSearchSegmentationStrategySizeConst: crate::ximgproc::SelectiveSearchSegmentationStrategyConst {
	fn as_raw_SelectiveSearchSegmentationStrategySize(&self) -> *const c_void;

}

pub trait SelectiveSearchSegmentationStrategySize: crate::ximgproc::SelectiveSearchSegmentationStrategy + crate::ximgproc::SelectiveSearchSegmentationStrategySizeConst {
	fn as_raw_mut_SelectiveSearchSegmentationStrategySize(&mut self) -> *mut c_void;

}

// SelectiveSearchSegmentationStrategyTexture /usr/include/opencv2/ximgproc/segmentation.hpp:118
pub trait SelectiveSearchSegmentationStrategyTextureConst: crate::ximgproc::SelectiveSearchSegmentationStrategyConst {
	fn as_raw_SelectiveSearchSegmentationStrategyTexture(&self) -> *const c_void;

}

pub trait SelectiveSearchSegmentationStrategyTexture: crate::ximgproc::SelectiveSearchSegmentationStrategy + crate::ximgproc::SelectiveSearchSegmentationStrategyTextureConst {
	fn as_raw_mut_SelectiveSearchSegmentationStrategyTexture(&mut self) -> *mut c_void;

}
