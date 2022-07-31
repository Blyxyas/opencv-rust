#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Custom Calibration Pattern for 3D reconstruction
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::CustomPatternTraitConst, super::CustomPatternTrait, super::RandomPatternCornerFinderTraitConst, super::RandomPatternCornerFinderTrait, super::RandomPatternGeneratorTraitConst, super::RandomPatternGeneratorTrait, super::MultiCameraCalibration_edgeTraitConst, super::MultiCameraCalibration_edgeTrait, super::MultiCameraCalibration_vertexTraitConst, super::MultiCameraCalibration_vertexTrait, super::MultiCameraCalibrationTraitConst, super::MultiCameraCalibrationTrait };
}

// CALIB_FIX_CENTER /usr/include/opencv2/ccalib/omnidir.hpp:65
pub const CALIB_FIX_CENTER: i32 = 256;
// CALIB_FIX_GAMMA /usr/include/opencv2/ccalib/omnidir.hpp:64
pub const CALIB_FIX_GAMMA: i32 = 128;
// CALIB_FIX_K1 /usr/include/opencv2/ccalib/omnidir.hpp:59
pub const CALIB_FIX_K1: i32 = 4;
// CALIB_FIX_K2 /usr/include/opencv2/ccalib/omnidir.hpp:60
pub const CALIB_FIX_K2: i32 = 8;
// CALIB_FIX_P1 /usr/include/opencv2/ccalib/omnidir.hpp:61
pub const CALIB_FIX_P1: i32 = 16;
// CALIB_FIX_P2 /usr/include/opencv2/ccalib/omnidir.hpp:62
pub const CALIB_FIX_P2: i32 = 32;
// CALIB_FIX_SKEW /usr/include/opencv2/ccalib/omnidir.hpp:58
pub const CALIB_FIX_SKEW: i32 = 2;
// CALIB_FIX_XI /usr/include/opencv2/ccalib/omnidir.hpp:63
pub const CALIB_FIX_XI: i32 = 64;
// CALIB_USE_GUESS /usr/include/opencv2/ccalib/omnidir.hpp:57
pub const CALIB_USE_GUESS: i32 = 1;
// HEAD /usr/include/opencv2/ccalib/multicalib.hpp:55
pub const HEAD: i32 = -1;
// INVALID /usr/include/opencv2/ccalib/multicalib.hpp:56
pub const INVALID: i32 = -2;
// OMNIDIRECTIONAL /usr/include/opencv2/ccalib/multicalib.hpp:77
pub const MultiCameraCalibration_OMNIDIRECTIONAL: i32 = 1;
// PINHOLE /usr/include/opencv2/ccalib/multicalib.hpp:76
pub const MultiCameraCalibration_PINHOLE: i32 = 0;
// RECTIFY_CYLINDRICAL /usr/include/opencv2/ccalib/omnidir.hpp:70
pub const RECTIFY_CYLINDRICAL: i32 = 2;
// RECTIFY_LONGLATI /usr/include/opencv2/ccalib/omnidir.hpp:71
pub const RECTIFY_LONGLATI: i32 = 3;
// RECTIFY_PERSPECTIVE /usr/include/opencv2/ccalib/omnidir.hpp:69
pub const RECTIFY_PERSPECTIVE: i32 = 1;
// RECTIFY_STEREOGRAPHIC /usr/include/opencv2/ccalib/omnidir.hpp:72
pub const RECTIFY_STEREOGRAPHIC: i32 = 4;
// XYZ /usr/include/opencv2/ccalib/omnidir.hpp:77
pub const XYZ: i32 = 2;
// XYZRGB /usr/include/opencv2/ccalib/omnidir.hpp:76
pub const XYZRGB: i32 = 1;
/// ## C++ default parameters
/// * idx: noArray()
// calibrate(cv::InputArrayOfArrays, cv::InputArrayOfArrays, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, int, cv::TermCriteria, cv::OutputArray) /usr/include/opencv2/ccalib/omnidir.hpp:176
#[inline]
pub fn calibrate(object_points: &dyn core::ToInputArray, image_points: &dyn core::ToInputArray, size: core::Size, k: &mut dyn core::ToInputOutputArray, xi: &mut dyn core::ToInputOutputArray, d: &mut dyn core::ToInputOutputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, flags: i32, criteria: core::TermCriteria, idx: &mut dyn core::ToOutputArray) -> Result<f64> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_output_array_arg!(k);
	input_output_array_arg!(xi);
	input_output_array_arg!(d);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	output_array_arg!(idx);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), size.opencv_as_extern(), k.as_raw__InputOutputArray(), xi.as_raw__InputOutputArray(), d.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, criteria.opencv_as_extern(), idx.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// initUndistortRectifyMap(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, const cv::Size &, int, cv::OutputArray, cv::OutputArray, int) /usr/include/opencv2/ccalib/omnidir.hpp:141
#[inline]
pub fn init_undistort_rectify_map(k: &dyn core::ToInputArray, d: &dyn core::ToInputArray, xi: &dyn core::ToInputArray, r: &dyn core::ToInputArray, p: &dyn core::ToInputArray, size: core::Size, m1type: i32, map1: &mut dyn core::ToOutputArray, map2: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
	input_array_arg!(k);
	input_array_arg!(d);
	input_array_arg!(xi);
	input_array_arg!(r);
	input_array_arg!(p);
	output_array_arg!(map1);
	output_array_arg!(map2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_initUndistortRectifyMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_int_const__OutputArrayR_const__OutputArrayR_int(k.as_raw__InputArray(), d.as_raw__InputArray(), xi.as_raw__InputArray(), r.as_raw__InputArray(), p.as_raw__InputArray(), &size, m1type, map1.as_raw__OutputArray(), map2.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * jacobian: noArray()
// projectPoints(cv::InputArray, cv::OutputArray, const cv::Affine3d &, cv::InputArray, double, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ccalib/omnidir.hpp:107
#[inline]
pub fn project_points_1(object_points: &dyn core::ToInputArray, image_points: &mut dyn core::ToOutputArray, affine: core::Affine3d, k: &dyn core::ToInputArray, xi: f64, d: &dyn core::ToInputArray, jacobian: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(object_points);
	output_array_arg!(image_points);
	input_array_arg!(k);
	input_array_arg!(d);
	output_array_arg!(jacobian);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__OutputArray(), &affine, k.as_raw__InputArray(), xi, d.as_raw__InputArray(), jacobian.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * jacobian: noArray()
// projectPoints(cv::InputArray, cv::OutputArray, cv::InputArray, cv::InputArray, cv::InputArray, double, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ccalib/omnidir.hpp:103
#[inline]
pub fn project_points(object_points: &dyn core::ToInputArray, image_points: &mut dyn core::ToOutputArray, rvec: &dyn core::ToInputArray, tvec: &dyn core::ToInputArray, k: &dyn core::ToInputArray, xi: f64, d: &dyn core::ToInputArray, jacobian: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(object_points);
	output_array_arg!(image_points);
	input_array_arg!(rvec);
	input_array_arg!(tvec);
	input_array_arg!(k);
	input_array_arg!(d);
	output_array_arg!(jacobian);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__OutputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), k.as_raw__InputArray(), xi, d.as_raw__InputArray(), jacobian.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * idx: noArray()
// stereoCalibrate(cv::InputOutputArrayOfArrays, cv::InputOutputArrayOfArrays, cv::InputOutputArrayOfArrays, const cv::Size &, const cv::Size &, cv::InputOutputArray, cv::InputOutputArray, cv::InputOutputArray, cv::InputOutputArray, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, int, cv::TermCriteria, cv::OutputArray) /usr/include/opencv2/ccalib/omnidir.hpp:207
#[inline]
pub fn stereo_calibrate(object_points: &mut dyn core::ToInputOutputArray, image_points1: &mut dyn core::ToInputOutputArray, image_points2: &mut dyn core::ToInputOutputArray, image_size1: core::Size, image_size2: core::Size, k1: &mut dyn core::ToInputOutputArray, xi1: &mut dyn core::ToInputOutputArray, d1: &mut dyn core::ToInputOutputArray, k2: &mut dyn core::ToInputOutputArray, xi2: &mut dyn core::ToInputOutputArray, d2: &mut dyn core::ToInputOutputArray, rvec: &mut dyn core::ToOutputArray, tvec: &mut dyn core::ToOutputArray, rvecs_l: &mut dyn core::ToOutputArray, tvecs_l: &mut dyn core::ToOutputArray, flags: i32, criteria: core::TermCriteria, idx: &mut dyn core::ToOutputArray) -> Result<f64> {
	input_output_array_arg!(object_points);
	input_output_array_arg!(image_points1);
	input_output_array_arg!(image_points2);
	input_output_array_arg!(k1);
	input_output_array_arg!(xi1);
	input_output_array_arg!(d1);
	input_output_array_arg!(k2);
	input_output_array_arg!(xi2);
	input_output_array_arg!(d2);
	output_array_arg!(rvec);
	output_array_arg!(tvec);
	output_array_arg!(rvecs_l);
	output_array_arg!(tvecs_l);
	output_array_arg!(idx);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_stereoCalibrate_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_SizeR_const_SizeR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria_const__OutputArrayR(object_points.as_raw__InputOutputArray(), image_points1.as_raw__InputOutputArray(), image_points2.as_raw__InputOutputArray(), &image_size1, &image_size2, k1.as_raw__InputOutputArray(), xi1.as_raw__InputOutputArray(), d1.as_raw__InputOutputArray(), k2.as_raw__InputOutputArray(), xi2.as_raw__InputOutputArray(), d2.as_raw__InputOutputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), rvecs_l.as_raw__OutputArray(), tvecs_l.as_raw__OutputArray(), flags, criteria.opencv_as_extern(), idx.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * new_size: Size()
/// * knew: cv::noArray()
/// * point_cloud: cv::noArray()
/// * point_type: XYZRGB
// stereoReconstruct(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, int, int, int, cv::OutputArray, cv::OutputArray, cv::OutputArray, const cv::Size &, cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ccalib/omnidir.hpp:243
#[inline]
pub fn stereo_reconstruct(image1: &dyn core::ToInputArray, image2: &dyn core::ToInputArray, k1: &dyn core::ToInputArray, d1: &dyn core::ToInputArray, xi1: &dyn core::ToInputArray, k2: &dyn core::ToInputArray, d2: &dyn core::ToInputArray, xi2: &dyn core::ToInputArray, r: &dyn core::ToInputArray, t: &dyn core::ToInputArray, flag: i32, num_disparities: i32, sad_window_size: i32, disparity: &mut dyn core::ToOutputArray, image1_rec: &mut dyn core::ToOutputArray, image2_rec: &mut dyn core::ToOutputArray, new_size: core::Size, knew: &dyn core::ToInputArray, point_cloud: &mut dyn core::ToOutputArray, point_type: i32) -> Result<()> {
	input_array_arg!(image1);
	input_array_arg!(image2);
	input_array_arg!(k1);
	input_array_arg!(d1);
	input_array_arg!(xi1);
	input_array_arg!(k2);
	input_array_arg!(d2);
	input_array_arg!(xi2);
	input_array_arg!(r);
	input_array_arg!(t);
	output_array_arg!(disparity);
	output_array_arg!(image1_rec);
	output_array_arg!(image2_rec);
	input_array_arg!(knew);
	output_array_arg!(point_cloud);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_stereoReconstruct_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_int_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_int(image1.as_raw__InputArray(), image2.as_raw__InputArray(), k1.as_raw__InputArray(), d1.as_raw__InputArray(), xi1.as_raw__InputArray(), k2.as_raw__InputArray(), d2.as_raw__InputArray(), xi2.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), flag, num_disparities, sad_window_size, disparity.as_raw__OutputArray(), image1_rec.as_raw__OutputArray(), image2_rec.as_raw__OutputArray(), &new_size, knew.as_raw__InputArray(), point_cloud.as_raw__OutputArray(), point_type, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// stereoRectify(cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ccalib/omnidir.hpp:218
#[inline]
pub fn stereo_rectify(r: &dyn core::ToInputArray, t: &dyn core::ToInputArray, r1: &mut dyn core::ToOutputArray, r2: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(r);
	input_array_arg!(t);
	output_array_arg!(r1);
	output_array_arg!(r2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_stereoRectify_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(r.as_raw__InputArray(), t.as_raw__InputArray(), r1.as_raw__OutputArray(), r2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * knew: cv::noArray()
/// * new_size: Size()
/// * r: Mat::eye(3,3,CV_64F)
// undistortImage(cv::InputArray, cv::OutputArray, cv::InputArray, cv::InputArray, cv::InputArray, int, cv::InputArray, const cv::Size &, cv::InputArray) /usr/include/opencv2/ccalib/omnidir.hpp:156
#[inline]
pub fn undistort_image(distorted: &dyn core::ToInputArray, undistorted: &mut dyn core::ToOutputArray, k: &dyn core::ToInputArray, d: &dyn core::ToInputArray, xi: &dyn core::ToInputArray, flags: i32, knew: &dyn core::ToInputArray, new_size: core::Size, r: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(distorted);
	output_array_arg!(undistorted);
	input_array_arg!(k);
	input_array_arg!(d);
	input_array_arg!(xi);
	input_array_arg!(knew);
	input_array_arg!(r);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_const__InputArrayR_const_SizeR_const__InputArrayR(distorted.as_raw__InputArray(), undistorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), xi.as_raw__InputArray(), flags, knew.as_raw__InputArray(), &new_size, r.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// undistortPoints(cv::InputArray, cv::OutputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/ccalib/omnidir.hpp:122
#[inline]
pub fn undistort_points(distorted: &dyn core::ToInputArray, undistorted: &mut dyn core::ToOutputArray, k: &dyn core::ToInputArray, d: &dyn core::ToInputArray, xi: &dyn core::ToInputArray, r: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(distorted);
	output_array_arg!(undistorted);
	input_array_arg!(k);
	input_array_arg!(d);
	input_array_arg!(xi);
	input_array_arg!(r);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(distorted.as_raw__InputArray(), undistorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), xi.as_raw__InputArray(), r.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// CustomPattern /usr/include/opencv2/ccalib.hpp:60
pub trait CustomPatternTraitConst: core::AlgorithmTraitConst {
	fn as_raw_CustomPattern(&self) -> *const c_void;

}

pub trait CustomPatternTrait: core::AlgorithmTrait + crate::ccalib::CustomPatternTraitConst {
	fn as_raw_mut_CustomPattern(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * output: noArray()
	// create(cv::InputArray, const cv::Size2f, cv::OutputArray) /usr/include/opencv2/ccalib.hpp:66
	#[inline]
	fn create(&mut self, pattern: &dyn core::ToInputArray, board_size: core::Size2f, output: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(pattern);
		output_array_arg!(output);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_create_const__InputArrayR_const_Size2f_const__OutputArrayR(self.as_raw_mut_CustomPattern(), pattern.as_raw__InputArray(), board_size.opencv_as_extern(), output.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * ratio: 0.7
	/// * proj_error: 8.0
	/// * refine_position: false
	/// * out: noArray()
	/// * h: noArray()
	/// * pattern_corners: noArray()
	// findPattern(cv::InputArray, cv::OutputArray, cv::OutputArray, const double, const double, const bool, cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ccalib.hpp:68
	#[inline]
	fn find_pattern(&mut self, image: &dyn core::ToInputArray, matched_features: &mut dyn core::ToOutputArray, pattern_points: &mut dyn core::ToOutputArray, ratio: f64, proj_error: f64, refine_position: bool, out: &mut dyn core::ToOutputArray, h: &mut dyn core::ToOutputArray, pattern_corners: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(matched_features);
		output_array_arg!(pattern_points);
		output_array_arg!(out);
		output_array_arg!(h);
		output_array_arg!(pattern_corners);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_findPattern_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const_double_const_double_const_bool_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_CustomPattern(), image.as_raw__InputArray(), matched_features.as_raw__OutputArray(), pattern_points.as_raw__OutputArray(), ratio, proj_error, refine_position, out.as_raw__OutputArray(), h.as_raw__OutputArray(), pattern_corners.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// isInitialized() /usr/include/opencv2/ccalib.hpp:72
	#[inline]
	fn is_initialized(&mut self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_isInitialized(self.as_raw_mut_CustomPattern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getPatternPoints(std::vector<KeyPoint> &) /usr/include/opencv2/ccalib.hpp:74
	#[inline]
	fn get_pattern_points(&mut self, original_points: &mut core::Vector<core::KeyPoint>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_getPatternPoints_vector_KeyPoint_R(self.as_raw_mut_CustomPattern(), original_points.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getPixelSize() /usr/include/opencv2/ccalib.hpp:78
	#[inline]
	fn get_pixel_size(&mut self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_getPixelSize(self.as_raw_mut_CustomPattern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setFeatureDetector(Ptr<cv::FeatureDetector>) /usr/include/opencv2/ccalib.hpp:83
	#[inline]
	fn set_feature_detector(&mut self, mut feature_detector: core::Ptr<crate::features2d::Feature2D>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_setFeatureDetector_Ptr_Feature2D_(self.as_raw_mut_CustomPattern(), feature_detector.as_raw_mut_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setDescriptorExtractor(Ptr<cv::DescriptorExtractor>) /usr/include/opencv2/ccalib.hpp:84
	#[inline]
	fn set_descriptor_extractor(&mut self, mut extractor: core::Ptr<crate::features2d::Feature2D>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_setDescriptorExtractor_Ptr_Feature2D_(self.as_raw_mut_CustomPattern(), extractor.as_raw_mut_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setDescriptorMatcher(Ptr<cv::DescriptorMatcher>) /usr/include/opencv2/ccalib.hpp:85
	#[inline]
	fn set_descriptor_matcher(&mut self, mut matcher: core::Ptr<dyn crate::features2d::DescriptorMatcher>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_setDescriptorMatcher_Ptr_DescriptorMatcher_(self.as_raw_mut_CustomPattern(), matcher.as_raw_mut_PtrOfDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getFeatureDetector() /usr/include/opencv2/ccalib.hpp:87
	#[inline]
	fn get_feature_detector(&mut self) -> Result<core::Ptr<crate::features2d::Feature2D>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_getFeatureDetector(self.as_raw_mut_CustomPattern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::Feature2D>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getDescriptorExtractor() /usr/include/opencv2/ccalib.hpp:88
	#[inline]
	fn get_descriptor_extractor(&mut self) -> Result<core::Ptr<crate::features2d::Feature2D>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_getDescriptorExtractor(self.as_raw_mut_CustomPattern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::Feature2D>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getDescriptorMatcher() /usr/include/opencv2/ccalib.hpp:89
	#[inline]
	fn get_descriptor_matcher(&mut self) -> Result<core::Ptr<dyn crate::features2d::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_getDescriptorMatcher(self.as_raw_mut_CustomPattern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::features2d::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
	// calibrate(cv::InputArrayOfArrays, cv::InputArrayOfArrays, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, int, cv::TermCriteria) /usr/include/opencv2/ccalib.hpp:91
	#[inline]
	fn calibrate(&mut self, object_points: &dyn core::ToInputArray, image_points: &dyn core::ToInputArray, image_size: core::Size, camera_matrix: &mut dyn core::ToInputOutputArray, dist_coeffs: &mut dyn core::ToInputOutputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(self.as_raw_mut_CustomPattern(), object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), image_size.opencv_as_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, criteria.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * use_extrinsic_guess: false
	/// * flags: SOLVEPNP_ITERATIVE
	// findRt(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool, int) /usr/include/opencv2/ccalib.hpp:99
	#[inline]
	fn find_rt(&mut self, object_points: &dyn core::ToInputArray, image_points: &dyn core::ToInputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvec: &mut dyn core::ToInputOutputArray, tvec: &mut dyn core::ToInputOutputArray, use_extrinsic_guess: bool, flags: i32) -> Result<bool> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_findRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool_int(self.as_raw_mut_CustomPattern(), object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), use_extrinsic_guess, flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * use_extrinsic_guess: false
	/// * flags: SOLVEPNP_ITERATIVE
	// findRt(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool, int) /usr/include/opencv2/ccalib.hpp:101
	#[inline]
	fn find_rt_1(&mut self, image: &dyn core::ToInputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvec: &mut dyn core::ToInputOutputArray, tvec: &mut dyn core::ToInputOutputArray, use_extrinsic_guess: bool, flags: i32) -> Result<bool> {
		input_array_arg!(image);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_findRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool_int(self.as_raw_mut_CustomPattern(), image.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), use_extrinsic_guess, flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * use_extrinsic_guess: false
	/// * iterations_count: 100
	/// * reprojection_error: 8.0
	/// * min_inliers_count: 100
	/// * inliers: noArray()
	/// * flags: SOLVEPNP_ITERATIVE
	// findRtRANSAC(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool, int, float, int, cv::OutputArray, int) /usr/include/opencv2/ccalib.hpp:108
	#[inline]
	fn find_rt_ransac(&mut self, object_points: &dyn core::ToInputArray, image_points: &dyn core::ToInputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvec: &mut dyn core::ToInputOutputArray, tvec: &mut dyn core::ToInputOutputArray, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, min_inliers_count: i32, inliers: &mut dyn core::ToOutputArray, flags: i32) -> Result<bool> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		output_array_arg!(inliers);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool_int_float_int_const__OutputArrayR_int(self.as_raw_mut_CustomPattern(), object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), use_extrinsic_guess, iterations_count, reprojection_error, min_inliers_count, inliers.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * use_extrinsic_guess: false
	/// * iterations_count: 100
	/// * reprojection_error: 8.0
	/// * min_inliers_count: 100
	/// * inliers: noArray()
	/// * flags: SOLVEPNP_ITERATIVE
	// findRtRANSAC(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool, int, float, int, cv::OutputArray, int) /usr/include/opencv2/ccalib.hpp:111
	#[inline]
	fn find_rt_ransac_1(&mut self, image: &dyn core::ToInputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvec: &mut dyn core::ToInputOutputArray, tvec: &mut dyn core::ToInputOutputArray, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, min_inliers_count: i32, inliers: &mut dyn core::ToOutputArray, flags: i32) -> Result<bool> {
		input_array_arg!(image);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		output_array_arg!(inliers);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool_int_float_int_const__OutputArrayR_int(self.as_raw_mut_CustomPattern(), image.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), use_extrinsic_guess, iterations_count, reprojection_error, min_inliers_count, inliers.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * axis_length: 3
	/// * axis_width: 2
	// drawOrientation(cv::InputOutputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, double, int) /usr/include/opencv2/ccalib.hpp:118
	#[inline]
	fn draw_orientation(&mut self, image: &mut dyn core::ToInputOutputArray, tvec: &dyn core::ToInputArray, rvec: &dyn core::ToInputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, axis_length: f64, axis_width: i32) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(tvec);
		input_array_arg!(rvec);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_drawOrientation_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_int(self.as_raw_mut_CustomPattern(), image.as_raw__InputOutputArray(), tvec.as_raw__InputArray(), rvec.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), axis_length, axis_width, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// CustomPattern /usr/include/opencv2/ccalib.hpp:60
pub struct CustomPattern {
	ptr: *mut c_void
}

opencv_type_boxed! { CustomPattern }

impl Drop for CustomPattern {
	fn drop(&mut self) {
		extern "C" { fn cv_CustomPattern_delete(instance: *mut c_void); }
		unsafe { cv_CustomPattern_delete(self.as_raw_mut_CustomPattern()) };
	}
}

unsafe impl Send for CustomPattern {}

impl core::AlgorithmTraitConst for CustomPattern {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for CustomPattern {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ccalib::CustomPatternTraitConst for CustomPattern {
	#[inline] fn as_raw_CustomPattern(&self) -> *const c_void { self.as_raw() }
}

impl crate::ccalib::CustomPatternTrait for CustomPattern {
	#[inline] fn as_raw_mut_CustomPattern(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CustomPattern {
	// CustomPattern() /usr/include/opencv2/ccalib.hpp:63
	#[inline]
	pub fn default() -> Result<crate::ccalib::CustomPattern> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_CustomPattern(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::ccalib::CustomPattern::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { CustomPattern, core::Algorithm, cv_CustomPattern_to_Algorithm }

// MultiCameraCalibration /usr/include/opencv2/ccalib/multicalib.hpp:72
pub trait MultiCameraCalibrationTraitConst {
	fn as_raw_MultiCameraCalibration(&self) -> *const c_void;

}

pub trait MultiCameraCalibrationTrait: crate::ccalib::MultiCameraCalibrationTraitConst {
	fn as_raw_mut_MultiCameraCalibration(&mut self) -> *mut c_void;

	// loadImages() /usr/include/opencv2/ccalib/multicalib.hpp:141
	#[inline]
	fn load_images(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_loadImages(self.as_raw_mut_MultiCameraCalibration(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// initialize() /usr/include/opencv2/ccalib/multicalib.hpp:145
	#[inline]
	fn initialize(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_initialize(self.as_raw_mut_MultiCameraCalibration(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// optimizeExtrinsics() /usr/include/opencv2/ccalib/multicalib.hpp:149
	#[inline]
	fn optimize_extrinsics(&mut self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_optimizeExtrinsics(self.as_raw_mut_MultiCameraCalibration(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// run() /usr/include/opencv2/ccalib/multicalib.hpp:153
	#[inline]
	fn run(&mut self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_run(self.as_raw_mut_MultiCameraCalibration(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// writeParameters(const std::string &) /usr/include/opencv2/ccalib/multicalib.hpp:157
	#[inline]
	fn write_parameters(&mut self, filename: &str) -> Result<()> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_writeParameters_const_stringR(self.as_raw_mut_MultiCameraCalibration(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// MultiCameraCalibration /usr/include/opencv2/ccalib/multicalib.hpp:72
pub struct MultiCameraCalibration {
	ptr: *mut c_void
}

opencv_type_boxed! { MultiCameraCalibration }

impl Drop for MultiCameraCalibration {
	fn drop(&mut self) {
		extern "C" { fn cv_MultiCameraCalibration_delete(instance: *mut c_void); }
		unsafe { cv_MultiCameraCalibration_delete(self.as_raw_mut_MultiCameraCalibration()) };
	}
}

unsafe impl Send for MultiCameraCalibration {}

impl crate::ccalib::MultiCameraCalibrationTraitConst for MultiCameraCalibration {
	#[inline] fn as_raw_MultiCameraCalibration(&self) -> *const c_void { self.as_raw() }
}

impl crate::ccalib::MultiCameraCalibrationTrait for MultiCameraCalibration {
	#[inline] fn as_raw_mut_MultiCameraCalibration(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MultiCameraCalibration {
	/// ## C++ default parameters
	/// * verbose: 0
	/// * show_extration: 0
	/// * n_mini_matches: 20
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,200,1e-7)
	/// * detector: AKAZE::create(AKAZE::DESCRIPTOR_MLDB,0,3,0.006f)
	/// * descriptor: AKAZE::create(AKAZE::DESCRIPTOR_MLDB,0,3,0.006f)
	/// * matcher: DescriptorMatcher::create("BruteForce-L1")
	// MultiCameraCalibration(int, int, const std::string &, float, float, int, int, int, int, cv::TermCriteria, Ptr<cv::FeatureDetector>, Ptr<cv::DescriptorExtractor>, Ptr<cv::DescriptorMatcher>) /usr/include/opencv2/ccalib/multicalib.hpp:132
	#[inline]
	pub fn new(camera_type: i32, n_cameras: i32, file_name: &str, pattern_width: f32, pattern_height: f32, verbose: i32, show_extration: i32, n_mini_matches: i32, flags: i32, criteria: core::TermCriteria, mut detector: core::Ptr<crate::features2d::Feature2D>, mut descriptor: core::Ptr<crate::features2d::Feature2D>, mut matcher: core::Ptr<dyn crate::features2d::DescriptorMatcher>) -> Result<crate::ccalib::MultiCameraCalibration> {
		extern_container_arg!(file_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_MultiCameraCalibration_int_int_const_stringR_float_float_int_int_int_int_TermCriteria_Ptr_Feature2D__Ptr_Feature2D__Ptr_DescriptorMatcher_(camera_type, n_cameras, file_name.opencv_as_extern(), pattern_width, pattern_height, verbose, show_extration, n_mini_matches, flags, criteria.opencv_as_extern(), detector.as_raw_mut_PtrOfFeature2D(), descriptor.as_raw_mut_PtrOfFeature2D(), matcher.as_raw_mut_PtrOfDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::ccalib::MultiCameraCalibration::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// edge /usr/include/opencv2/ccalib/multicalib.hpp:82
pub trait MultiCameraCalibration_edgeTraitConst {
	fn as_raw_MultiCameraCalibration_edge(&self) -> *const c_void;

	// cameraVertex /usr/include/opencv2/ccalib/multicalib.hpp:84
	#[inline]
	fn camera_vertex(&self) -> i32 {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_getPropCameraVertex_const(self.as_raw_MultiCameraCalibration_edge()) };
		ret
	}
	
	// photoVertex /usr/include/opencv2/ccalib/multicalib.hpp:85
	#[inline]
	fn photo_vertex(&self) -> i32 {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_getPropPhotoVertex_const(self.as_raw_MultiCameraCalibration_edge()) };
		ret
	}
	
	// photoIndex /usr/include/opencv2/ccalib/multicalib.hpp:86
	#[inline]
	fn photo_index(&self) -> i32 {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_getPropPhotoIndex_const(self.as_raw_MultiCameraCalibration_edge()) };
		ret
	}
	
	// transform /usr/include/opencv2/ccalib/multicalib.hpp:87
	#[inline]
	fn transform(&self) -> core::Mat {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_getPropTransform_const(self.as_raw_MultiCameraCalibration_edge()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait MultiCameraCalibration_edgeTrait: crate::ccalib::MultiCameraCalibration_edgeTraitConst {
	fn as_raw_mut_MultiCameraCalibration_edge(&mut self) -> *mut c_void;

	// cameraVertex /usr/include/opencv2/ccalib/multicalib.hpp:84
	#[inline]
	fn set_camera_vertex(&mut self, val: i32) {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_setPropCameraVertex_int(self.as_raw_mut_MultiCameraCalibration_edge(), val) };
		ret
	}
	
	// photoVertex /usr/include/opencv2/ccalib/multicalib.hpp:85
	#[inline]
	fn set_photo_vertex(&mut self, val: i32) {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_setPropPhotoVertex_int(self.as_raw_mut_MultiCameraCalibration_edge(), val) };
		ret
	}
	
	// photoIndex /usr/include/opencv2/ccalib/multicalib.hpp:86
	#[inline]
	fn set_photo_index(&mut self, val: i32) {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_setPropPhotoIndex_int(self.as_raw_mut_MultiCameraCalibration_edge(), val) };
		ret
	}
	
	// transform /usr/include/opencv2/ccalib/multicalib.hpp:87
	#[inline]
	fn set_transform(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_setPropTransform_Mat(self.as_raw_mut_MultiCameraCalibration_edge(), val.as_raw_mut_Mat()) };
		ret
	}
	
}

// edge /usr/include/opencv2/ccalib/multicalib.hpp:82
pub struct MultiCameraCalibration_edge {
	ptr: *mut c_void
}

opencv_type_boxed! { MultiCameraCalibration_edge }

impl Drop for MultiCameraCalibration_edge {
	fn drop(&mut self) {
		extern "C" { fn cv_MultiCameraCalibration_edge_delete(instance: *mut c_void); }
		unsafe { cv_MultiCameraCalibration_edge_delete(self.as_raw_mut_MultiCameraCalibration_edge()) };
	}
}

unsafe impl Send for MultiCameraCalibration_edge {}

impl crate::ccalib::MultiCameraCalibration_edgeTraitConst for MultiCameraCalibration_edge {
	#[inline] fn as_raw_MultiCameraCalibration_edge(&self) -> *const c_void { self.as_raw() }
}

impl crate::ccalib::MultiCameraCalibration_edgeTrait for MultiCameraCalibration_edge {
	#[inline] fn as_raw_mut_MultiCameraCalibration_edge(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MultiCameraCalibration_edge {
	// edge(int, int, int, cv::Mat) /usr/include/opencv2/ccalib/multicalib.hpp:89
	#[inline]
	pub fn new(cv: i32, pv: i32, pi: i32, mut trans: core::Mat) -> Result<crate::ccalib::MultiCameraCalibration_edge> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_edge_int_int_int_Mat(cv, pv, pi, trans.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::ccalib::MultiCameraCalibration_edge::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// vertex /usr/include/opencv2/ccalib/multicalib.hpp:98
pub trait MultiCameraCalibration_vertexTraitConst {
	fn as_raw_MultiCameraCalibration_vertex(&self) -> *const c_void;

	// pose /usr/include/opencv2/ccalib/multicalib.hpp:100
	#[inline]
	fn pose(&self) -> core::Mat {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_getPropPose_const(self.as_raw_MultiCameraCalibration_vertex()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// timestamp /usr/include/opencv2/ccalib/multicalib.hpp:103
	#[inline]
	fn timestamp(&self) -> i32 {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_getPropTimestamp_const(self.as_raw_MultiCameraCalibration_vertex()) };
		ret
	}
	
}

pub trait MultiCameraCalibration_vertexTrait: crate::ccalib::MultiCameraCalibration_vertexTraitConst {
	fn as_raw_mut_MultiCameraCalibration_vertex(&mut self) -> *mut c_void;

	// pose /usr/include/opencv2/ccalib/multicalib.hpp:100
	#[inline]
	fn set_pose(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_setPropPose_Mat(self.as_raw_mut_MultiCameraCalibration_vertex(), val.as_raw_mut_Mat()) };
		ret
	}
	
	// timestamp /usr/include/opencv2/ccalib/multicalib.hpp:103
	#[inline]
	fn set_timestamp(&mut self, val: i32) {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_setPropTimestamp_int(self.as_raw_mut_MultiCameraCalibration_vertex(), val) };
		ret
	}
	
}

// vertex /usr/include/opencv2/ccalib/multicalib.hpp:98
pub struct MultiCameraCalibration_vertex {
	ptr: *mut c_void
}

opencv_type_boxed! { MultiCameraCalibration_vertex }

impl Drop for MultiCameraCalibration_vertex {
	fn drop(&mut self) {
		extern "C" { fn cv_MultiCameraCalibration_vertex_delete(instance: *mut c_void); }
		unsafe { cv_MultiCameraCalibration_vertex_delete(self.as_raw_mut_MultiCameraCalibration_vertex()) };
	}
}

unsafe impl Send for MultiCameraCalibration_vertex {}

impl crate::ccalib::MultiCameraCalibration_vertexTraitConst for MultiCameraCalibration_vertex {
	#[inline] fn as_raw_MultiCameraCalibration_vertex(&self) -> *const c_void { self.as_raw() }
}

impl crate::ccalib::MultiCameraCalibration_vertexTrait for MultiCameraCalibration_vertex {
	#[inline] fn as_raw_mut_MultiCameraCalibration_vertex(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MultiCameraCalibration_vertex {
	// vertex(cv::Mat, int) /usr/include/opencv2/ccalib/multicalib.hpp:105
	#[inline]
	pub fn new(mut po: core::Mat, ts: i32) -> Result<crate::ccalib::MultiCameraCalibration_vertex> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_vertex_Mat_int(po.as_raw_mut_Mat(), ts, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::ccalib::MultiCameraCalibration_vertex::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// vertex() /usr/include/opencv2/ccalib/multicalib.hpp:111
	#[inline]
	pub fn default() -> Result<crate::ccalib::MultiCameraCalibration_vertex> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_vertex(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::ccalib::MultiCameraCalibration_vertex::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// RandomPatternCornerFinder /usr/include/opencv2/ccalib/randpattern.hpp:65
pub trait RandomPatternCornerFinderTraitConst {
	fn as_raw_RandomPatternCornerFinder(&self) -> *const c_void;

}

pub trait RandomPatternCornerFinderTrait: crate::ccalib::RandomPatternCornerFinderTraitConst {
	fn as_raw_mut_RandomPatternCornerFinder(&mut self) -> *mut c_void;

	// loadPattern(const cv::Mat &) /usr/include/opencv2/ccalib/randpattern.hpp:89
	#[inline]
	fn load_pattern(&mut self, pattern_image: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternCornerFinder_loadPattern_const_MatR(self.as_raw_mut_RandomPatternCornerFinder(), pattern_image.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// loadPattern(const cv::Mat &, const std::vector<cv::KeyPoint> &, const cv::Mat &) /usr/include/opencv2/ccalib/randpattern.hpp:96
	#[inline]
	fn load_pattern_1(&mut self, pattern_image: &core::Mat, pattern_key_points: &core::Vector<core::KeyPoint>, pattern_descriptors: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternCornerFinder_loadPattern_const_MatR_const_vector_KeyPoint_R_const_MatR(self.as_raw_mut_RandomPatternCornerFinder(), pattern_image.as_raw_Mat(), pattern_key_points.as_raw_VectorOfKeyPoint(), pattern_descriptors.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// computeObjectImagePoints(std::vector<cv::Mat>) /usr/include/opencv2/ccalib/randpattern.hpp:105
	#[inline]
	fn compute_object_image_points(&mut self, mut input_images: core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternCornerFinder_computeObjectImagePoints_vector_Mat_(self.as_raw_mut_RandomPatternCornerFinder(), input_images.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// computeObjectImagePointsForSingle(cv::Mat) /usr/include/opencv2/ccalib/randpattern.hpp:114
	#[inline]
	fn compute_object_image_points_for_single(&mut self, mut input_image: core::Mat) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternCornerFinder_computeObjectImagePointsForSingle_Mat(self.as_raw_mut_RandomPatternCornerFinder(), input_image.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getObjectPoints() /usr/include/opencv2/ccalib/randpattern.hpp:118
	#[inline]
	fn get_object_points(&mut self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternCornerFinder_getObjectPoints(self.as_raw_mut_RandomPatternCornerFinder(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getImagePoints() /usr/include/opencv2/ccalib/randpattern.hpp:122
	#[inline]
	fn get_image_points(&mut self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternCornerFinder_getImagePoints(self.as_raw_mut_RandomPatternCornerFinder(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// RandomPatternCornerFinder /usr/include/opencv2/ccalib/randpattern.hpp:65
pub struct RandomPatternCornerFinder {
	ptr: *mut c_void
}

opencv_type_boxed! { RandomPatternCornerFinder }

impl Drop for RandomPatternCornerFinder {
	fn drop(&mut self) {
		extern "C" { fn cv_RandomPatternCornerFinder_delete(instance: *mut c_void); }
		unsafe { cv_RandomPatternCornerFinder_delete(self.as_raw_mut_RandomPatternCornerFinder()) };
	}
}

unsafe impl Send for RandomPatternCornerFinder {}

impl crate::ccalib::RandomPatternCornerFinderTraitConst for RandomPatternCornerFinder {
	#[inline] fn as_raw_RandomPatternCornerFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::ccalib::RandomPatternCornerFinderTrait for RandomPatternCornerFinder {
	#[inline] fn as_raw_mut_RandomPatternCornerFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RandomPatternCornerFinder {
	/// ## C++ default parameters
	/// * nmini_match: 20
	/// * depth: CV_32F
	/// * verbose: 0
	/// * show_extraction: 0
	/// * detector: AKAZE::create(AKAZE::DESCRIPTOR_MLDB,0,3,0.005f)
	/// * descriptor: AKAZE::create(AKAZE::DESCRIPTOR_MLDB,0,3,0.005f)
	/// * matcher: DescriptorMatcher::create("BruteForce-L1")
	// RandomPatternCornerFinder(float, float, int, int, int, int, Ptr<cv::FeatureDetector>, Ptr<cv::DescriptorExtractor>, Ptr<cv::DescriptorMatcher>) /usr/include/opencv2/ccalib/randpattern.hpp:80
	#[inline]
	pub fn new(pattern_width: f32, pattern_height: f32, nmini_match: i32, depth: i32, verbose: i32, show_extraction: i32, mut detector: core::Ptr<crate::features2d::Feature2D>, mut descriptor: core::Ptr<crate::features2d::Feature2D>, mut matcher: core::Ptr<dyn crate::features2d::DescriptorMatcher>) -> Result<crate::ccalib::RandomPatternCornerFinder> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternCornerFinder_RandomPatternCornerFinder_float_float_int_int_int_int_Ptr_Feature2D__Ptr_Feature2D__Ptr_DescriptorMatcher_(pattern_width, pattern_height, nmini_match, depth, verbose, show_extraction, detector.as_raw_mut_PtrOfFeature2D(), descriptor.as_raw_mut_PtrOfFeature2D(), matcher.as_raw_mut_PtrOfDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::ccalib::RandomPatternCornerFinder::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// RandomPatternGenerator /usr/include/opencv2/ccalib/randpattern.hpp:160
pub trait RandomPatternGeneratorTraitConst {
	fn as_raw_RandomPatternGenerator(&self) -> *const c_void;

}

pub trait RandomPatternGeneratorTrait: crate::ccalib::RandomPatternGeneratorTraitConst {
	fn as_raw_mut_RandomPatternGenerator(&mut self) -> *mut c_void;

	// generatePattern() /usr/include/opencv2/ccalib/randpattern.hpp:172
	#[inline]
	fn generate_pattern(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternGenerator_generatePattern(self.as_raw_mut_RandomPatternGenerator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getPattern() /usr/include/opencv2/ccalib/randpattern.hpp:175
	#[inline]
	fn get_pattern(&mut self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternGenerator_getPattern(self.as_raw_mut_RandomPatternGenerator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// RandomPatternGenerator /usr/include/opencv2/ccalib/randpattern.hpp:160
pub struct RandomPatternGenerator {
	ptr: *mut c_void
}

opencv_type_boxed! { RandomPatternGenerator }

impl Drop for RandomPatternGenerator {
	fn drop(&mut self) {
		extern "C" { fn cv_RandomPatternGenerator_delete(instance: *mut c_void); }
		unsafe { cv_RandomPatternGenerator_delete(self.as_raw_mut_RandomPatternGenerator()) };
	}
}

unsafe impl Send for RandomPatternGenerator {}

impl crate::ccalib::RandomPatternGeneratorTraitConst for RandomPatternGenerator {
	#[inline] fn as_raw_RandomPatternGenerator(&self) -> *const c_void { self.as_raw() }
}

impl crate::ccalib::RandomPatternGeneratorTrait for RandomPatternGenerator {
	#[inline] fn as_raw_mut_RandomPatternGenerator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RandomPatternGenerator {
	// RandomPatternGenerator(int, int) /usr/include/opencv2/ccalib/randpattern.hpp:168
	#[inline]
	pub fn new(image_width: i32, image_height: i32) -> Result<crate::ccalib::RandomPatternGenerator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternGenerator_RandomPatternGenerator_int_int(image_width, image_height, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::ccalib::RandomPatternGenerator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
