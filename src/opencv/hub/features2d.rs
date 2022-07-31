#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # 2D Features Framework
//!    # Feature Detection and Description
//!    # Descriptor Matchers
//! 
//! Matchers of keypoint descriptors in OpenCV have wrappers with a common interface that enables you to
//! easily switch between different algorithms solving the same problem. This section is devoted to
//! matching descriptors that are represented as vectors in a multidimensional space. All objects that
//! implement vector descriptor matchers inherit the DescriptorMatcher interface.
//! 
//!    # Drawing Function of Keypoints and Matches
//!    # Object Categorization
//! 
//! This section describes approaches based on local 2D features and used to categorize objects.
//! 
//!    # Hardware Acceleration Layer
//!        # Interface
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::KeyPointsFilterTraitConst, super::KeyPointsFilterTrait, super::Feature2DTraitConst, super::Feature2DTrait, super::AffineFeatureConst, super::AffineFeature, super::SIFTTraitConst, super::SIFTTrait, super::BRISKTraitConst, super::BRISKTrait, super::ORBConst, super::ORB, super::MSERConst, super::MSER, super::FastFeatureDetectorConst, super::FastFeatureDetector, super::AgastFeatureDetectorConst, super::AgastFeatureDetector, super::GFTTDetectorConst, super::GFTTDetector, super::SimpleBlobDetectorTraitConst, super::SimpleBlobDetectorTrait, super::KAZEConst, super::KAZE, super::AKAZEConst, super::AKAZE, super::DescriptorMatcherConst, super::DescriptorMatcher, super::BFMatcherTraitConst, super::BFMatcherTrait, super::FlannBasedMatcherTraitConst, super::FlannBasedMatcherTrait, super::BOWTrainerConst, super::BOWTrainer, super::BOWKMeansTrainerTraitConst, super::BOWKMeansTrainerTrait, super::BOWImgDescriptorExtractorTraitConst, super::BOWImgDescriptorExtractorTrait };
}

pub const AgastFeatureDetector_NONMAX_SUPPRESSION: i32 = 10001;
pub const AgastFeatureDetector_THRESHOLD: i32 = 10000;
pub const DrawMatchesFlags_DEFAULT: i32 = 0;
pub const DrawMatchesFlags_DRAW_OVER_OUTIMG: i32 = 1;
pub const DrawMatchesFlags_DRAW_RICH_KEYPOINTS: i32 = 4;
pub const DrawMatchesFlags_NOT_DRAW_SINGLE_POINTS: i32 = 2;
pub const FastFeatureDetector_FAST_N: i32 = 10002;
pub const FastFeatureDetector_NONMAX_SUPPRESSION: i32 = 10001;
pub const FastFeatureDetector_THRESHOLD: i32 = 10000;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AKAZE_DescriptorType {
	DESCRIPTOR_KAZE_UPRIGHT = 2,
	DESCRIPTOR_KAZE = 3,
	DESCRIPTOR_MLDB_UPRIGHT = 4,
	DESCRIPTOR_MLDB = 5,
}

opencv_type_enum! { crate::features2d::AKAZE_DescriptorType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AgastFeatureDetector_DetectorType {
	AGAST_5_8 = 0,
	AGAST_7_12d = 1,
	AGAST_7_12s = 2,
	OAST_9_16 = 3,
}

opencv_type_enum! { crate::features2d::AgastFeatureDetector_DetectorType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DescriptorMatcher_MatcherType {
	FLANNBASED = 1,
	BRUTEFORCE = 2,
	BRUTEFORCE_L1 = 3,
	BRUTEFORCE_HAMMING = 4,
	BRUTEFORCE_HAMMINGLUT = 5,
	BRUTEFORCE_SL2 = 6,
}

opencv_type_enum! { crate::features2d::DescriptorMatcher_MatcherType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DrawMatchesFlags {
	DEFAULT = 0,
	DRAW_OVER_OUTIMG = 1,
	NOT_DRAW_SINGLE_POINTS = 2,
	DRAW_RICH_KEYPOINTS = 4,
}

opencv_type_enum! { crate::features2d::DrawMatchesFlags }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FastFeatureDetector_DetectorType {
	TYPE_5_8 = 0,
	TYPE_7_12 = 1,
	TYPE_9_16 = 2,
}

opencv_type_enum! { crate::features2d::FastFeatureDetector_DetectorType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KAZE_DiffusivityType {
	DIFF_PM_G1 = 0,
	DIFF_PM_G2 = 1,
	DIFF_WEICKERT = 2,
	DIFF_CHARBONNIER = 3,
}

opencv_type_enum! { crate::features2d::KAZE_DiffusivityType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ORB_ScoreType {
	HARRIS_SCORE = 0,
	FAST_SCORE = 1,
}

opencv_type_enum! { crate::features2d::ORB_ScoreType }

pub type AffineDescriptorExtractor = dyn crate::features2d::AffineFeature;
pub type AffineFeatureDetector = dyn crate::features2d::AffineFeature;
pub type DescriptorExtractor = crate::features2d::Feature2D;
pub type FeatureDetector = crate::features2d::Feature2D;
pub type SiftDescriptorExtractor = crate::features2d::SIFT;
pub type SiftFeatureDetector = crate::features2d::SIFT;
/// ## C++ default parameters
/// * nonmax_suppression: true
#[inline]
pub fn AGAST(image: &dyn core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_AGAST_const__InputArrayR_vector_KeyPoint_R_int_bool(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn AGAST_with_type(image: &dyn core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool, typ: crate::features2d::AgastFeatureDetector_DetectorType) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_AGAST_const__InputArrayR_vector_KeyPoint_R_int_bool_DetectorType(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression, typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * nonmax_suppression: true
#[inline]
pub fn FAST(image: &dyn core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_FAST_const__InputArrayR_vector_KeyPoint_R_int_bool(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn FAST_with_type(image: &dyn core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool, typ: crate::features2d::FastFeatureDetector_DetectorType) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_FAST_const__InputArrayR_vector_KeyPoint_R_int_bool_DetectorType(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression, typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn compute_recall_precision_curve(matches1to2: &core::Vector<core::Vector<core::DMatch>>, correct_matches1to2_mask: &core::Vector<core::Vector<u8>>, recall_precision_curve: &mut core::Vector<core::Point2f>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_computeRecallPrecisionCurve_const_vector_vector_DMatch__R_const_vector_vector_unsigned_char__R_vector_Point2f_R(matches1to2.as_raw_VectorOfVectorOfDMatch(), correct_matches1to2_mask.as_raw_VectorOfVectorOfu8(), recall_precision_curve.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * color: Scalar::all(-1)
/// * flags: DrawMatchesFlags::DEFAULT
#[inline]
pub fn draw_keypoints(image: &dyn core::ToInputArray, keypoints: &core::Vector<core::KeyPoint>, out_image: &mut dyn core::ToInputOutputArray, color: core::Scalar, flags: crate::features2d::DrawMatchesFlags) -> Result<()> {
	input_array_arg!(image);
	input_output_array_arg!(out_image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_drawKeypoints_const__InputArrayR_const_vector_KeyPoint_R_const__InputOutputArrayR_const_ScalarR_DrawMatchesFlags(image.as_raw__InputArray(), keypoints.as_raw_VectorOfKeyPoint(), out_image.as_raw__InputOutputArray(), &color, flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * match_color: Scalar::all(-1)
/// * single_point_color: Scalar::all(-1)
/// * matches_mask: std::vector<char>()
/// * flags: DrawMatchesFlags::DEFAULT
#[inline]
pub fn draw_matches(img1: &dyn core::ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &dyn core::ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut dyn core::ToInputOutputArray, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &core::Vector<i8>, flags: crate::features2d::DrawMatchesFlags) -> Result<()> {
	input_array_arg!(img1);
	input_array_arg!(img2);
	input_output_array_arg!(out_img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_drawMatches_const__InputArrayR_const_vector_KeyPoint_R_const__InputArrayR_const_vector_KeyPoint_R_const_vector_DMatch_R_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vector_char_R_DrawMatchesFlags(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), out_img.as_raw__InputOutputArray(), &match_color, &single_point_color, matches_mask.as_raw_VectorOfi8(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * match_color: Scalar::all(-1)
/// * single_point_color: Scalar::all(-1)
/// * matches_mask: std::vector<char>()
/// * flags: DrawMatchesFlags::DEFAULT
#[inline]
pub fn draw_matches_1(img1: &dyn core::ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &dyn core::ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut dyn core::ToInputOutputArray, matches_thickness: i32, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &core::Vector<i8>, flags: crate::features2d::DrawMatchesFlags) -> Result<()> {
	input_array_arg!(img1);
	input_array_arg!(img2);
	input_output_array_arg!(out_img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_drawMatches_const__InputArrayR_const_vector_KeyPoint_R_const__InputArrayR_const_vector_KeyPoint_R_const_vector_DMatch_R_const__InputOutputArrayR_const_int_const_ScalarR_const_ScalarR_const_vector_char_R_DrawMatchesFlags(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), out_img.as_raw__InputOutputArray(), matches_thickness, &match_color, &single_point_color, matches_mask.as_raw_VectorOfi8(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * match_color: Scalar::all(-1)
/// * single_point_color: Scalar::all(-1)
/// * matches_mask: std::vector<std::vector<char>>()
/// * flags: DrawMatchesFlags::DEFAULT
#[inline]
pub fn draw_matches_knn(img1: &dyn core::ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &dyn core::ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::Vector<core::DMatch>>, out_img: &mut dyn core::ToInputOutputArray, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &core::Vector<core::Vector<i8>>, flags: crate::features2d::DrawMatchesFlags) -> Result<()> {
	input_array_arg!(img1);
	input_array_arg!(img2);
	input_output_array_arg!(out_img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_drawMatches_const__InputArrayR_const_vector_KeyPoint_R_const__InputArrayR_const_vector_KeyPoint_R_const_vector_vector_DMatch__R_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vector_vector_char__R_DrawMatchesFlags(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfVectorOfDMatch(), out_img.as_raw__InputOutputArray(), &match_color, &single_point_color, matches_mask.as_raw_VectorOfVectorOfi8(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * fdetector: Ptr<FeatureDetector>()
#[inline]
pub fn evaluate_feature_detector(img1: &core::Mat, img2: &core::Mat, h1to2: &core::Mat, keypoints1: &mut core::Vector<core::KeyPoint>, keypoints2: &mut core::Vector<core::KeyPoint>, repeatability: &mut f32, corresp_count: &mut i32, fdetector: &core::Ptr<crate::features2d::Feature2D>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_evaluateFeatureDetector_const_MatR_const_MatR_const_MatR_vector_KeyPoint_X_vector_KeyPoint_X_floatR_intR_const_Ptr_Feature2D_R(img1.as_raw_Mat(), img2.as_raw_Mat(), h1to2.as_raw_Mat(), keypoints1.as_raw_mut_VectorOfKeyPoint(), keypoints2.as_raw_mut_VectorOfKeyPoint(), repeatability, corresp_count, fdetector.as_raw_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn get_nearest_point(recall_precision_curve: &core::Vector<core::Point2f>, l_precision: f32) -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getNearestPoint_const_vector_Point2f_R_float(recall_precision_curve.as_raw_VectorOfPoint2f(), l_precision, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn get_recall(recall_precision_curve: &core::Vector<core::Point2f>, l_precision: f32) -> Result<f32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getRecall_const_vector_Point2f_R_float(recall_precision_curve.as_raw_VectorOfPoint2f(), l_precision, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

pub trait AKAZEConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_AKAZE(&self) -> *const c_void;

	#[inline]
	fn get_descriptor_type(&self) -> Result<crate::features2d::AKAZE_DescriptorType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_getDescriptorType_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_descriptor_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_getDescriptorSize_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_descriptor_channels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_getDescriptorChannels_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_getThreshold_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_n_octaves(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_getNOctaves_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_n_octave_layers(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_getNOctaveLayers_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_diffusivity(&self) -> Result<crate::features2d::KAZE_DiffusivityType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_getDiffusivity_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_getDefaultName_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait AKAZE: crate::features2d::AKAZEConst + crate::features2d::Feature2DTrait {
	fn as_raw_mut_AKAZE(&mut self) -> *mut c_void;

	#[inline]
	fn set_descriptor_type(&mut self, dtype: crate::features2d::AKAZE_DescriptorType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_setDescriptorType_DescriptorType(self.as_raw_mut_AKAZE(), dtype, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_descriptor_size(&mut self, dsize: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_setDescriptorSize_int(self.as_raw_mut_AKAZE(), dsize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_descriptor_channels(&mut self, dch: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_setDescriptorChannels_int(self.as_raw_mut_AKAZE(), dch, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_threshold(&mut self, threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_setThreshold_double(self.as_raw_mut_AKAZE(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_n_octaves(&mut self, octaves: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_setNOctaves_int(self.as_raw_mut_AKAZE(), octaves, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_n_octave_layers(&mut self, octave_layers: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_setNOctaveLayers_int(self.as_raw_mut_AKAZE(), octave_layers, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_diffusivity(&mut self, diff: crate::features2d::KAZE_DiffusivityType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_setDiffusivity_DiffusivityType(self.as_raw_mut_AKAZE(), diff, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn AKAZE + '_ {
	/// ## C++ default parameters
	/// * descriptor_type: AKAZE::DESCRIPTOR_MLDB
	/// * descriptor_size: 0
	/// * descriptor_channels: 3
	/// * threshold: 0.001f
	/// * n_octaves: 4
	/// * n_octave_layers: 4
	/// * diffusivity: KAZE::DIFF_PM_G2
	#[inline]
	pub fn create(descriptor_type: crate::features2d::AKAZE_DescriptorType, descriptor_size: i32, descriptor_channels: i32, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: crate::features2d::KAZE_DiffusivityType) -> Result<core::Ptr<dyn crate::features2d::AKAZE>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_create_DescriptorType_int_int_float_int_int_DiffusivityType(descriptor_type, descriptor_size, descriptor_channels, threshold, n_octaves, n_octave_layers, diffusivity, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::features2d::AKAZE>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait AffineFeatureConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_AffineFeature(&self) -> *const c_void;

	#[inline]
	fn get_view_params(&self, tilts: &mut core::Vector<f32>, rolls: &mut core::Vector<f32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AffineFeature_getViewParams_const_vector_float_R_vector_float_R(self.as_raw_AffineFeature(), tilts.as_raw_mut_VectorOff32(), rolls.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AffineFeature_getDefaultName_const(self.as_raw_AffineFeature(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait AffineFeature: crate::features2d::AffineFeatureConst + crate::features2d::Feature2DTrait {
	fn as_raw_mut_AffineFeature(&mut self) -> *mut c_void;

	#[inline]
	fn set_view_params(&mut self, tilts: &core::Vector<f32>, rolls: &core::Vector<f32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AffineFeature_setViewParams_const_vector_float_R_const_vector_float_R(self.as_raw_mut_AffineFeature(), tilts.as_raw_VectorOff32(), rolls.as_raw_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn AffineFeature + '_ {
	/// ## C++ default parameters
	/// * max_tilt: 5
	/// * min_tilt: 0
	/// * tilt_step: 1.4142135623730951f
	/// * rotate_step_base: 72
	#[inline]
	pub fn create(backend: &core::Ptr<crate::features2d::Feature2D>, max_tilt: i32, min_tilt: i32, tilt_step: f32, rotate_step_base: f32) -> Result<core::Ptr<dyn crate::features2d::AffineFeature>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AffineFeature_create_const_Ptr_Feature2D_R_int_int_float_float(backend.as_raw_PtrOfFeature2D(), max_tilt, min_tilt, tilt_step, rotate_step_base, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::features2d::AffineFeature>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait AgastFeatureDetectorConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_AgastFeatureDetector(&self) -> *const c_void;

	#[inline]
	fn get_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AgastFeatureDetector_getThreshold_const(self.as_raw_AgastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_nonmax_suppression(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AgastFeatureDetector_getNonmaxSuppression_const(self.as_raw_AgastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_type(&self) -> Result<crate::features2d::AgastFeatureDetector_DetectorType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AgastFeatureDetector_getType_const(self.as_raw_AgastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AgastFeatureDetector_getDefaultName_const(self.as_raw_AgastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait AgastFeatureDetector: crate::features2d::AgastFeatureDetectorConst + crate::features2d::Feature2DTrait {
	fn as_raw_mut_AgastFeatureDetector(&mut self) -> *mut c_void;

	#[inline]
	fn set_threshold(&mut self, threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AgastFeatureDetector_setThreshold_int(self.as_raw_mut_AgastFeatureDetector(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_nonmax_suppression(&mut self, f: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AgastFeatureDetector_setNonmaxSuppression_bool(self.as_raw_mut_AgastFeatureDetector(), f, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_type(&mut self, typ: crate::features2d::AgastFeatureDetector_DetectorType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AgastFeatureDetector_setType_DetectorType(self.as_raw_mut_AgastFeatureDetector(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn AgastFeatureDetector + '_ {
	/// ## C++ default parameters
	/// * threshold: 10
	/// * nonmax_suppression: true
	/// * typ: AgastFeatureDetector::OAST_9_16
	#[inline]
	pub fn create(threshold: i32, nonmax_suppression: bool, typ: crate::features2d::AgastFeatureDetector_DetectorType) -> Result<core::Ptr<dyn crate::features2d::AgastFeatureDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AgastFeatureDetector_create_int_bool_DetectorType(threshold, nonmax_suppression, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::features2d::AgastFeatureDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait BFMatcherTraitConst: crate::features2d::DescriptorMatcherConst {
	fn as_raw_BFMatcher(&self) -> *const c_void;

	#[inline]
	fn is_mask_supported(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BFMatcher_isMaskSupported_const(self.as_raw_BFMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * empty_train_data: false
	#[inline]
	#[must_use]
	fn clone(&self, empty_train_data: bool) -> Result<core::Ptr<dyn crate::features2d::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BFMatcher_clone_const_bool(self.as_raw_BFMatcher(), empty_train_data, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::features2d::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait BFMatcherTrait: crate::features2d::BFMatcherTraitConst + crate::features2d::DescriptorMatcher {
	fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void;

}

pub struct BFMatcher {
	ptr: *mut c_void
}

opencv_type_boxed! { BFMatcher }

impl Drop for BFMatcher {
	fn drop(&mut self) {
		extern "C" { fn cv_BFMatcher_delete(instance: *mut c_void); }
		unsafe { cv_BFMatcher_delete(self.as_raw_mut_BFMatcher()) };
	}
}

unsafe impl Send for BFMatcher {}

impl core::AlgorithmTraitConst for BFMatcher {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BFMatcher {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::DescriptorMatcherConst for BFMatcher {
	#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::DescriptorMatcher for BFMatcher {
	#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::BFMatcherTraitConst for BFMatcher {
	#[inline] fn as_raw_BFMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::BFMatcherTrait for BFMatcher {
	#[inline] fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BFMatcher {
	/// ## C++ default parameters
	/// * norm_type: NORM_L2
	/// * cross_check: false
	#[inline]
	pub fn new(norm_type: i32, cross_check: bool) -> Result<crate::features2d::BFMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BFMatcher_BFMatcher_int_bool(norm_type, cross_check, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features2d::BFMatcher::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * norm_type: NORM_L2
	/// * cross_check: false
	#[inline]
	pub fn create(norm_type: i32, cross_check: bool) -> Result<core::Ptr<crate::features2d::BFMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BFMatcher_create_int_bool(norm_type, cross_check, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::BFMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { BFMatcher, core::Algorithm, cv_BFMatcher_to_Algorithm }

pub trait BOWImgDescriptorExtractorTraitConst {
	fn as_raw_BOWImgDescriptorExtractor(&self) -> *const c_void;

	#[inline]
	fn get_vocabulary(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_getVocabulary_const(self.as_raw_BOWImgDescriptorExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn descriptor_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_descriptorSize_const(self.as_raw_BOWImgDescriptorExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn descriptor_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_descriptorType_const(self.as_raw_BOWImgDescriptorExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait BOWImgDescriptorExtractorTrait: crate::features2d::BOWImgDescriptorExtractorTraitConst {
	fn as_raw_mut_BOWImgDescriptorExtractor(&mut self) -> *mut c_void;

	#[inline]
	fn set_vocabulary(&mut self, vocabulary: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_setVocabulary_const_MatR(self.as_raw_mut_BOWImgDescriptorExtractor(), vocabulary.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * point_idxs_of_clusters: 0
	/// * descriptors: 0
	#[inline]
	fn compute_desc(&mut self, image: &dyn core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, img_descriptor: &mut dyn core::ToOutputArray, point_idxs_of_clusters: &mut core::Vector<core::Vector<i32>>, descriptors: &mut core::Mat) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(img_descriptor);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_vector_KeyPoint_R_const__OutputArrayR_vector_vector_int__X_MatX(self.as_raw_mut_BOWImgDescriptorExtractor(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), img_descriptor.as_raw__OutputArray(), point_idxs_of_clusters.as_raw_mut_VectorOfVectorOfi32(), descriptors.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * point_idxs_of_clusters: 0
	#[inline]
	fn compute(&mut self, keypoint_descriptors: &dyn core::ToInputArray, img_descriptor: &mut dyn core::ToOutputArray, point_idxs_of_clusters: &mut core::Vector<core::Vector<i32>>) -> Result<()> {
		input_array_arg!(keypoint_descriptors);
		output_array_arg!(img_descriptor);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_const__OutputArrayR_vector_vector_int__X(self.as_raw_mut_BOWImgDescriptorExtractor(), keypoint_descriptors.as_raw__InputArray(), img_descriptor.as_raw__OutputArray(), point_idxs_of_clusters.as_raw_mut_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn compute2(&mut self, image: &core::Mat, keypoints: &mut core::Vector<core::KeyPoint>, img_descriptor: &mut core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_compute2_const_MatR_vector_KeyPoint_R_MatR(self.as_raw_mut_BOWImgDescriptorExtractor(), image.as_raw_Mat(), keypoints.as_raw_mut_VectorOfKeyPoint(), img_descriptor.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct BOWImgDescriptorExtractor {
	ptr: *mut c_void
}

opencv_type_boxed! { BOWImgDescriptorExtractor }

impl Drop for BOWImgDescriptorExtractor {
	fn drop(&mut self) {
		extern "C" { fn cv_BOWImgDescriptorExtractor_delete(instance: *mut c_void); }
		unsafe { cv_BOWImgDescriptorExtractor_delete(self.as_raw_mut_BOWImgDescriptorExtractor()) };
	}
}

unsafe impl Send for BOWImgDescriptorExtractor {}

impl crate::features2d::BOWImgDescriptorExtractorTraitConst for BOWImgDescriptorExtractor {
	#[inline] fn as_raw_BOWImgDescriptorExtractor(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::BOWImgDescriptorExtractorTrait for BOWImgDescriptorExtractor {
	#[inline] fn as_raw_mut_BOWImgDescriptorExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BOWImgDescriptorExtractor {
	#[inline]
	pub fn new(dextractor: &core::Ptr<crate::features2d::Feature2D>, dmatcher: &core::Ptr<dyn crate::features2d::DescriptorMatcher>) -> Result<crate::features2d::BOWImgDescriptorExtractor> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_Ptr_Feature2D_R_const_Ptr_DescriptorMatcher_R(dextractor.as_raw_PtrOfFeature2D(), dmatcher.as_raw_PtrOfDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features2d::BOWImgDescriptorExtractor::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new_1(dmatcher: &core::Ptr<dyn crate::features2d::DescriptorMatcher>) -> Result<crate::features2d::BOWImgDescriptorExtractor> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_Ptr_DescriptorMatcher_R(dmatcher.as_raw_PtrOfDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features2d::BOWImgDescriptorExtractor::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait BOWKMeansTrainerTraitConst: crate::features2d::BOWTrainerConst {
	fn as_raw_BOWKMeansTrainer(&self) -> *const c_void;

	#[inline]
	fn cluster(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWKMeansTrainer_cluster_const(self.as_raw_BOWKMeansTrainer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn cluster_1(&self, descriptors: &core::Mat) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWKMeansTrainer_cluster_const_const_MatR(self.as_raw_BOWKMeansTrainer(), descriptors.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait BOWKMeansTrainerTrait: crate::features2d::BOWKMeansTrainerTraitConst + crate::features2d::BOWTrainer {
	fn as_raw_mut_BOWKMeansTrainer(&mut self) -> *mut c_void;

}

pub struct BOWKMeansTrainer {
	ptr: *mut c_void
}

opencv_type_boxed! { BOWKMeansTrainer }

impl Drop for BOWKMeansTrainer {
	fn drop(&mut self) {
		extern "C" { fn cv_BOWKMeansTrainer_delete(instance: *mut c_void); }
		unsafe { cv_BOWKMeansTrainer_delete(self.as_raw_mut_BOWKMeansTrainer()) };
	}
}

unsafe impl Send for BOWKMeansTrainer {}

impl crate::features2d::BOWTrainerConst for BOWKMeansTrainer {
	#[inline] fn as_raw_BOWTrainer(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::BOWTrainer for BOWKMeansTrainer {
	#[inline] fn as_raw_mut_BOWTrainer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::BOWKMeansTrainerTraitConst for BOWKMeansTrainer {
	#[inline] fn as_raw_BOWKMeansTrainer(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::BOWKMeansTrainerTrait for BOWKMeansTrainer {
	#[inline] fn as_raw_mut_BOWKMeansTrainer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BOWKMeansTrainer {
	/// ## C++ default parameters
	/// * termcrit: TermCriteria()
	/// * attempts: 3
	/// * flags: KMEANS_PP_CENTERS
	#[inline]
	pub fn new(cluster_count: i32, termcrit: core::TermCriteria, attempts: i32, flags: i32) -> Result<crate::features2d::BOWKMeansTrainer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWKMeansTrainer_BOWKMeansTrainer_int_const_TermCriteriaR_int_int(cluster_count, &termcrit, attempts, flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features2d::BOWKMeansTrainer::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait BOWTrainerConst {
	fn as_raw_BOWTrainer(&self) -> *const c_void;

	#[inline]
	fn get_descriptors(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWTrainer_getDescriptors_const(self.as_raw_BOWTrainer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn descriptors_count(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWTrainer_descriptorsCount_const(self.as_raw_BOWTrainer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn cluster(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWTrainer_cluster_const(self.as_raw_BOWTrainer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn cluster_with_descriptors(&self, descriptors: &core::Mat) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWTrainer_cluster_const_const_MatR(self.as_raw_BOWTrainer(), descriptors.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait BOWTrainer: crate::features2d::BOWTrainerConst {
	fn as_raw_mut_BOWTrainer(&mut self) -> *mut c_void;

	#[inline]
	fn add(&mut self, descriptors: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWTrainer_add_const_MatR(self.as_raw_mut_BOWTrainer(), descriptors.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWTrainer_clear(self.as_raw_mut_BOWTrainer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait BRISKTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_BRISK(&self) -> *const c_void;

	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_getDefaultName_const(self.as_raw_BRISK(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_getThreshold_const(self.as_raw_BRISK(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_octaves(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_getOctaves_const(self.as_raw_BRISK(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait BRISKTrait: crate::features2d::BRISKTraitConst + crate::features2d::Feature2DTrait {
	fn as_raw_mut_BRISK(&mut self) -> *mut c_void;

	#[inline]
	fn set_threshold(&mut self, threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_setThreshold_int(self.as_raw_mut_BRISK(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_octaves(&mut self, octaves: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_setOctaves_int(self.as_raw_mut_BRISK(), octaves, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct BRISK {
	ptr: *mut c_void
}

opencv_type_boxed! { BRISK }

impl Drop for BRISK {
	fn drop(&mut self) {
		extern "C" { fn cv_BRISK_delete(instance: *mut c_void); }
		unsafe { cv_BRISK_delete(self.as_raw_mut_BRISK()) };
	}
}

unsafe impl Send for BRISK {}

impl core::AlgorithmTraitConst for BRISK {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BRISK {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for BRISK {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for BRISK {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::BRISKTraitConst for BRISK {
	#[inline] fn as_raw_BRISK(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::BRISKTrait for BRISK {
	#[inline] fn as_raw_mut_BRISK(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BRISK {
	/// ## C++ default parameters
	/// * thresh: 30
	/// * octaves: 3
	/// * pattern_scale: 1.0f
	#[inline]
	pub fn create(thresh: i32, octaves: i32, pattern_scale: f32) -> Result<core::Ptr<crate::features2d::BRISK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_create_int_int_float(thresh, octaves, pattern_scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::BRISK>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * d_max: 5.85f
	/// * d_min: 8.2f
	/// * index_change: std::vector<int>()
	#[inline]
	pub fn create_with_pattern(radius_list: &core::Vector<f32>, number_list: &core::Vector<i32>, d_max: f32, d_min: f32, index_change: &core::Vector<i32>) -> Result<core::Ptr<crate::features2d::BRISK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_create_const_vector_float_R_const_vector_int_R_float_float_const_vector_int_R(radius_list.as_raw_VectorOff32(), number_list.as_raw_VectorOfi32(), d_max, d_min, index_change.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::BRISK>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * d_max: 5.85f
	/// * d_min: 8.2f
	/// * index_change: std::vector<int>()
	#[inline]
	pub fn create_with_pattern_threshold_octaves(thresh: i32, octaves: i32, radius_list: &core::Vector<f32>, number_list: &core::Vector<i32>, d_max: f32, d_min: f32, index_change: &core::Vector<i32>) -> Result<core::Ptr<crate::features2d::BRISK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_create_int_int_const_vector_float_R_const_vector_int_R_float_float_const_vector_int_R(thresh, octaves, radius_list.as_raw_VectorOff32(), number_list.as_raw_VectorOfi32(), d_max, d_min, index_change.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::BRISK>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { BRISK, core::Algorithm, cv_BRISK_to_Algorithm }

boxed_cast_base! { BRISK, crate::features2d::Feature2D, cv_BRISK_to_Feature2D }

pub trait DescriptorMatcherConst: core::AlgorithmTraitConst {
	fn as_raw_DescriptorMatcher(&self) -> *const c_void;

	#[inline]
	fn get_train_descriptors(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_getTrainDescriptors_const(self.as_raw_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_empty_const(self.as_raw_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn is_mask_supported(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_isMaskSupported_const(self.as_raw_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * mask: noArray()
	#[inline]
	fn train_match(&self, query_descriptors: &dyn core::ToInputArray, train_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector<core::DMatch>, mask: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_match_const_const__InputArrayR_const__InputArrayR_vector_DMatch_R_const__InputArrayR(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * compact_result: false
	#[inline]
	fn knn_train_match(&self, query_descriptors: &dyn core::ToInputArray, train_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32, mask: &dyn core::ToInputArray, compact_result: bool) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_knnMatch_const_const__InputArrayR_const__InputArrayR_vector_vector_DMatch__R_int_const__InputArrayR_bool(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, mask.as_raw__InputArray(), compact_result, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * compact_result: false
	#[inline]
	fn radius_train_match(&self, query_descriptors: &dyn core::ToInputArray, train_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32, mask: &dyn core::ToInputArray, compact_result: bool) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_radiusMatch_const_const__InputArrayR_const__InputArrayR_vector_vector_DMatch__R_float_const__InputArrayR_bool(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, mask.as_raw__InputArray(), compact_result, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn write(&self, file_name: &str) -> Result<()> {
		extern_container_arg!(file_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_write_const_const_StringR(self.as_raw_DescriptorMatcher(), file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn write_1(&self, unnamed: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_write_const_FileStorageR(self.as_raw_DescriptorMatcher(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * empty_train_data: false
	#[inline]
	#[must_use]
	fn clone(&self, empty_train_data: bool) -> Result<core::Ptr<dyn crate::features2d::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_clone_const_bool(self.as_raw_DescriptorMatcher(), empty_train_data, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::features2d::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * name: String()
	#[inline]
	fn write_2(&self, fs: &core::Ptr<core::FileStorage>, name: &str) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_write_const_const_Ptr_FileStorage_R_const_StringR(self.as_raw_DescriptorMatcher(), fs.as_raw_PtrOfFileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait DescriptorMatcher: core::AlgorithmTrait + crate::features2d::DescriptorMatcherConst {
	fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void;

	#[inline]
	fn add(&mut self, descriptors: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_add_const__InputArrayR(self.as_raw_mut_DescriptorMatcher(), descriptors.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_clear(self.as_raw_mut_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn train(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_train(self.as_raw_mut_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * masks: noArray()
	#[inline]
	fn match_(&mut self, query_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector<core::DMatch>, masks: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(masks);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_match_const__InputArrayR_vector_DMatch_R_const__InputArrayR(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), masks.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * masks: noArray()
	/// * compact_result: false
	#[inline]
	fn knn_match(&mut self, query_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32, masks: &dyn core::ToInputArray, compact_result: bool) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(masks);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_knnMatch_const__InputArrayR_vector_vector_DMatch__R_int_const__InputArrayR_bool(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, masks.as_raw__InputArray(), compact_result, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * masks: noArray()
	/// * compact_result: false
	#[inline]
	fn radius_match(&mut self, query_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32, masks: &dyn core::ToInputArray, compact_result: bool) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(masks);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_radiusMatch_const__InputArrayR_vector_vector_DMatch__R_float_const__InputArrayR_bool(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, masks.as_raw__InputArray(), compact_result, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn read(&mut self, file_name: &str) -> Result<()> {
		extern_container_arg!(file_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_read_const_StringR(self.as_raw_mut_DescriptorMatcher(), file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn read_1(&mut self, unnamed: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_read_const_FileNodeR(self.as_raw_mut_DescriptorMatcher(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn DescriptorMatcher + '_ {
	#[inline]
	pub fn create(descriptor_matcher_type: &str) -> Result<core::Ptr<dyn crate::features2d::DescriptorMatcher>> {
		extern_container_arg!(descriptor_matcher_type);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_create_const_StringR(descriptor_matcher_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::features2d::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create_with_matcher_type(matcher_type: crate::features2d::DescriptorMatcher_MatcherType) -> Result<core::Ptr<dyn crate::features2d::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_create_const_MatcherTypeR(&matcher_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::features2d::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait FastFeatureDetectorConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_FastFeatureDetector(&self) -> *const c_void;

	#[inline]
	fn get_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_getThreshold_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_nonmax_suppression(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_getNonmaxSuppression_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_type(&self) -> Result<crate::features2d::FastFeatureDetector_DetectorType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_getType_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_getDefaultName_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait FastFeatureDetector: crate::features2d::FastFeatureDetectorConst + crate::features2d::Feature2DTrait {
	fn as_raw_mut_FastFeatureDetector(&mut self) -> *mut c_void;

	#[inline]
	fn set_threshold(&mut self, threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_setThreshold_int(self.as_raw_mut_FastFeatureDetector(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_nonmax_suppression(&mut self, f: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_setNonmaxSuppression_bool(self.as_raw_mut_FastFeatureDetector(), f, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_type(&mut self, typ: crate::features2d::FastFeatureDetector_DetectorType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_setType_DetectorType(self.as_raw_mut_FastFeatureDetector(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn FastFeatureDetector + '_ {
	/// ## C++ default parameters
	/// * threshold: 10
	/// * nonmax_suppression: true
	/// * typ: FastFeatureDetector::TYPE_9_16
	#[inline]
	pub fn create(threshold: i32, nonmax_suppression: bool, typ: crate::features2d::FastFeatureDetector_DetectorType) -> Result<core::Ptr<dyn crate::features2d::FastFeatureDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_create_int_bool_DetectorType(threshold, nonmax_suppression, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::features2d::FastFeatureDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait Feature2DTraitConst: core::AlgorithmTraitConst {
	fn as_raw_Feature2D(&self) -> *const c_void;

	#[inline]
	fn descriptor_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_descriptorSize_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn descriptor_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_descriptorType_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn default_norm(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_defaultNorm_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn write(&self, file_name: &str) -> Result<()> {
		extern_container_arg!(file_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_write_const_const_StringR(self.as_raw_Feature2D(), file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn write_1(&self, unnamed: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_write_const_FileStorageR(self.as_raw_Feature2D(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_empty_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_getDefaultName_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * name: String()
	#[inline]
	fn write_2(&self, fs: &core::Ptr<core::FileStorage>, name: &str) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_write_const_const_Ptr_FileStorage_R_const_StringR(self.as_raw_Feature2D(), fs.as_raw_PtrOfFileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Feature2DTrait: core::AlgorithmTrait + crate::features2d::Feature2DTraitConst {
	fn as_raw_mut_Feature2D(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * mask: noArray()
	#[inline]
	fn detect(&mut self, image: &dyn core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, mask: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_detect_const__InputArrayR_vector_KeyPoint_R_const__InputArrayR(self.as_raw_mut_Feature2D(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * masks: noArray()
	#[inline]
	fn detect_multiple(&mut self, images: &dyn core::ToInputArray, keypoints: &mut core::Vector<core::Vector<core::KeyPoint>>, masks: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(images);
		input_array_arg!(masks);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_detect_const__InputArrayR_vector_vector_KeyPoint__R_const__InputArrayR(self.as_raw_mut_Feature2D(), images.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfVectorOfKeyPoint(), masks.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn compute(&mut self, image: &dyn core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_compute_const__InputArrayR_vector_KeyPoint_R_const__OutputArrayR(self.as_raw_mut_Feature2D(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn compute_multiple(&mut self, images: &dyn core::ToInputArray, keypoints: &mut core::Vector<core::Vector<core::KeyPoint>>, descriptors: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(images);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_compute_const__InputArrayR_vector_vector_KeyPoint__R_const__OutputArrayR(self.as_raw_mut_Feature2D(), images.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfVectorOfKeyPoint(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * use_provided_keypoints: false
	#[inline]
	fn detect_and_compute(&mut self, image: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut dyn core::ToOutputArray, use_provided_keypoints: bool) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vector_KeyPoint_R_const__OutputArrayR_bool(self.as_raw_mut_Feature2D(), image.as_raw__InputArray(), mask.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw__OutputArray(), use_provided_keypoints, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn read(&mut self, file_name: &str) -> Result<()> {
		extern_container_arg!(file_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_read_const_StringR(self.as_raw_mut_Feature2D(), file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn read_1(&mut self, unnamed: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_read_const_FileNodeR(self.as_raw_mut_Feature2D(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct Feature2D {
	ptr: *mut c_void
}

opencv_type_boxed! { Feature2D }

impl Drop for Feature2D {
	fn drop(&mut self) {
		extern "C" { fn cv_Feature2D_delete(instance: *mut c_void); }
		unsafe { cv_Feature2D_delete(self.as_raw_mut_Feature2D()) };
	}
}

unsafe impl Send for Feature2D {}

impl core::AlgorithmTraitConst for Feature2D {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for Feature2D {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for Feature2D {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for Feature2D {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Feature2D {
}

boxed_cast_descendant! { Feature2D, crate::features2d::BRISK, cv_Feature2D_to_BRISK }

boxed_cast_descendant! { Feature2D, crate::features2d::SIFT, cv_Feature2D_to_SIFT }

boxed_cast_descendant! { Feature2D, crate::features2d::SimpleBlobDetector, cv_Feature2D_to_SimpleBlobDetector }

boxed_cast_base! { Feature2D, core::Algorithm, cv_Feature2D_to_Algorithm }

pub trait FlannBasedMatcherTraitConst: crate::features2d::DescriptorMatcherConst {
	fn as_raw_FlannBasedMatcher(&self) -> *const c_void;

	#[inline]
	fn write(&self, unnamed: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_write_const_FileStorageR(self.as_raw_FlannBasedMatcher(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn is_mask_supported(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_isMaskSupported_const(self.as_raw_FlannBasedMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * empty_train_data: false
	#[inline]
	#[must_use]
	fn clone(&self, empty_train_data: bool) -> Result<core::Ptr<dyn crate::features2d::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_clone_const_bool(self.as_raw_FlannBasedMatcher(), empty_train_data, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::features2d::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait FlannBasedMatcherTrait: crate::features2d::DescriptorMatcher + crate::features2d::FlannBasedMatcherTraitConst {
	fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void;

	#[inline]
	fn add(&mut self, descriptors: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_add_const__InputArrayR(self.as_raw_mut_FlannBasedMatcher(), descriptors.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_clear(self.as_raw_mut_FlannBasedMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn read(&mut self, unnamed: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_read_const_FileNodeR(self.as_raw_mut_FlannBasedMatcher(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn train(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_train(self.as_raw_mut_FlannBasedMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct FlannBasedMatcher {
	ptr: *mut c_void
}

opencv_type_boxed! { FlannBasedMatcher }

impl Drop for FlannBasedMatcher {
	fn drop(&mut self) {
		extern "C" { fn cv_FlannBasedMatcher_delete(instance: *mut c_void); }
		unsafe { cv_FlannBasedMatcher_delete(self.as_raw_mut_FlannBasedMatcher()) };
	}
}

unsafe impl Send for FlannBasedMatcher {}

impl core::AlgorithmTraitConst for FlannBasedMatcher {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for FlannBasedMatcher {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::DescriptorMatcherConst for FlannBasedMatcher {
	#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::DescriptorMatcher for FlannBasedMatcher {
	#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::FlannBasedMatcherTraitConst for FlannBasedMatcher {
	#[inline] fn as_raw_FlannBasedMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::FlannBasedMatcherTrait for FlannBasedMatcher {
	#[inline] fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FlannBasedMatcher {
	/// ## C++ default parameters
	/// * index_params: makePtr<flann::KDTreeIndexParams>()
	/// * search_params: makePtr<flann::SearchParams>()
	#[inline]
	pub fn new(index_params: &core::Ptr<crate::flann::IndexParams>, search_params: &core::Ptr<crate::flann::SearchParams>) -> Result<crate::features2d::FlannBasedMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_FlannBasedMatcher_const_Ptr_IndexParams_R_const_Ptr_SearchParams_R(index_params.as_raw_PtrOfIndexParams(), search_params.as_raw_PtrOfSearchParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features2d::FlannBasedMatcher::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::features2d::FlannBasedMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::FlannBasedMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { FlannBasedMatcher, core::Algorithm, cv_FlannBasedMatcher_to_Algorithm }

pub trait GFTTDetectorConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_GFTTDetector(&self) -> *const c_void;

	#[inline]
	fn get_max_features(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getMaxFeatures_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_quality_level(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getQualityLevel_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_min_distance(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getMinDistance_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_block_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getBlockSize_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_harris_detector(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getHarrisDetector_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_k(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getK_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getDefaultName_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait GFTTDetector: crate::features2d::Feature2DTrait + crate::features2d::GFTTDetectorConst {
	fn as_raw_mut_GFTTDetector(&mut self) -> *mut c_void;

	#[inline]
	fn set_max_features(&mut self, max_features: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setMaxFeatures_int(self.as_raw_mut_GFTTDetector(), max_features, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_quality_level(&mut self, qlevel: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setQualityLevel_double(self.as_raw_mut_GFTTDetector(), qlevel, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_min_distance(&mut self, min_distance: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setMinDistance_double(self.as_raw_mut_GFTTDetector(), min_distance, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_block_size(&mut self, block_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setBlockSize_int(self.as_raw_mut_GFTTDetector(), block_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_harris_detector(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setHarrisDetector_bool(self.as_raw_mut_GFTTDetector(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_k(&mut self, k: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setK_double(self.as_raw_mut_GFTTDetector(), k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn GFTTDetector + '_ {
	/// ## C++ default parameters
	/// * max_corners: 1000
	/// * quality_level: 0.01
	/// * min_distance: 1
	/// * block_size: 3
	/// * use_harris_detector: false
	/// * k: 0.04
	#[inline]
	pub fn create(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, use_harris_detector: bool, k: f64) -> Result<core::Ptr<dyn crate::features2d::GFTTDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_create_int_double_double_int_bool_double(max_corners, quality_level, min_distance, block_size, use_harris_detector, k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::features2d::GFTTDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * use_harris_detector: false
	/// * k: 0.04
	#[inline]
	pub fn create_with_gradient(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, gradiant_size: i32, use_harris_detector: bool, k: f64) -> Result<core::Ptr<dyn crate::features2d::GFTTDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_create_int_double_double_int_int_bool_double(max_corners, quality_level, min_distance, block_size, gradiant_size, use_harris_detector, k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::features2d::GFTTDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait KAZEConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_KAZE(&self) -> *const c_void;

	#[inline]
	fn get_extended(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_getExtended_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_upright(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_getUpright_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_getThreshold_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_n_octaves(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_getNOctaves_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_n_octave_layers(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_getNOctaveLayers_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_diffusivity(&self) -> Result<crate::features2d::KAZE_DiffusivityType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_getDiffusivity_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_getDefaultName_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait KAZE: crate::features2d::Feature2DTrait + crate::features2d::KAZEConst {
	fn as_raw_mut_KAZE(&mut self) -> *mut c_void;

	#[inline]
	fn set_extended(&mut self, extended: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_setExtended_bool(self.as_raw_mut_KAZE(), extended, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_upright(&mut self, upright: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_setUpright_bool(self.as_raw_mut_KAZE(), upright, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_threshold(&mut self, threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_setThreshold_double(self.as_raw_mut_KAZE(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_n_octaves(&mut self, octaves: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_setNOctaves_int(self.as_raw_mut_KAZE(), octaves, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_n_octave_layers(&mut self, octave_layers: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_setNOctaveLayers_int(self.as_raw_mut_KAZE(), octave_layers, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_diffusivity(&mut self, diff: crate::features2d::KAZE_DiffusivityType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_setDiffusivity_DiffusivityType(self.as_raw_mut_KAZE(), diff, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn KAZE + '_ {
	/// ## C++ default parameters
	/// * extended: false
	/// * upright: false
	/// * threshold: 0.001f
	/// * n_octaves: 4
	/// * n_octave_layers: 4
	/// * diffusivity: KAZE::DIFF_PM_G2
	#[inline]
	pub fn create(extended: bool, upright: bool, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: crate::features2d::KAZE_DiffusivityType) -> Result<core::Ptr<dyn crate::features2d::KAZE>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_create_bool_bool_float_int_int_DiffusivityType(extended, upright, threshold, n_octaves, n_octave_layers, diffusivity, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::features2d::KAZE>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait KeyPointsFilterTraitConst {
	fn as_raw_KeyPointsFilter(&self) -> *const c_void;

}

pub trait KeyPointsFilterTrait: crate::features2d::KeyPointsFilterTraitConst {
	fn as_raw_mut_KeyPointsFilter(&mut self) -> *mut c_void;

}

pub struct KeyPointsFilter {
	ptr: *mut c_void
}

opencv_type_boxed! { KeyPointsFilter }

impl Drop for KeyPointsFilter {
	fn drop(&mut self) {
		extern "C" { fn cv_KeyPointsFilter_delete(instance: *mut c_void); }
		unsafe { cv_KeyPointsFilter_delete(self.as_raw_mut_KeyPointsFilter()) };
	}
}

unsafe impl Send for KeyPointsFilter {}

impl crate::features2d::KeyPointsFilterTraitConst for KeyPointsFilter {
	#[inline] fn as_raw_KeyPointsFilter(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::KeyPointsFilterTrait for KeyPointsFilter {
	#[inline] fn as_raw_mut_KeyPointsFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl KeyPointsFilter {
	#[inline]
	pub fn default() -> Result<crate::features2d::KeyPointsFilter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_KeyPointsFilter(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features2d::KeyPointsFilter::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn run_by_image_border(keypoints: &mut core::Vector<core::KeyPoint>, image_size: core::Size, border_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_runByImageBorder_vector_KeyPoint_R_Size_int(keypoints.as_raw_mut_VectorOfKeyPoint(), image_size.opencv_as_extern(), border_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * max_size: FLT_MAX
	#[inline]
	pub fn run_by_keypoint_size(keypoints: &mut core::Vector<core::KeyPoint>, min_size: f32, max_size: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_runByKeypointSize_vector_KeyPoint_R_float_float(keypoints.as_raw_mut_VectorOfKeyPoint(), min_size, max_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn run_by_pixels_mask(keypoints: &mut core::Vector<core::KeyPoint>, mask: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_runByPixelsMask_vector_KeyPoint_R_const_MatR(keypoints.as_raw_mut_VectorOfKeyPoint(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn remove_duplicated(keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_removeDuplicated_vector_KeyPoint_R(keypoints.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn remove_duplicated_sorted(keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_removeDuplicatedSorted_vector_KeyPoint_R(keypoints.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn retain_best(keypoints: &mut core::Vector<core::KeyPoint>, npoints: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_retainBest_vector_KeyPoint_R_int(keypoints.as_raw_mut_VectorOfKeyPoint(), npoints, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait MSERConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_MSER(&self) -> *const c_void;

	#[inline]
	fn get_delta(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getDelta_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_min_area(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getMinArea_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_area(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getMaxArea_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_pass2_only(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getPass2Only_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getDefaultName_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait MSER: crate::features2d::Feature2DTrait + crate::features2d::MSERConst {
	fn as_raw_mut_MSER(&mut self) -> *mut c_void;

	#[inline]
	fn detect_regions(&mut self, image: &dyn core::ToInputArray, msers: &mut core::Vector<core::Vector<core::Point>>, bboxes: &mut core::Vector<core::Rect>) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_detectRegions_const__InputArrayR_vector_vector_Point__R_vector_Rect_R(self.as_raw_mut_MSER(), image.as_raw__InputArray(), msers.as_raw_mut_VectorOfVectorOfPoint(), bboxes.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_delta(&mut self, delta: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setDelta_int(self.as_raw_mut_MSER(), delta, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_min_area(&mut self, min_area: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setMinArea_int(self.as_raw_mut_MSER(), min_area, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_area(&mut self, max_area: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setMaxArea_int(self.as_raw_mut_MSER(), max_area, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_pass2_only(&mut self, f: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setPass2Only_bool(self.as_raw_mut_MSER(), f, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn MSER + '_ {
	/// ## C++ default parameters
	/// * delta: 5
	/// * min_area: 60
	/// * max_area: 14400
	/// * max_variation: 0.25
	/// * min_diversity: .2
	/// * max_evolution: 200
	/// * area_threshold: 1.01
	/// * min_margin: 0.003
	/// * edge_blur_size: 5
	#[inline]
	pub fn create(delta: i32, min_area: i32, max_area: i32, max_variation: f64, min_diversity: f64, max_evolution: i32, area_threshold: f64, min_margin: f64, edge_blur_size: i32) -> Result<core::Ptr<dyn crate::features2d::MSER>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_create_int_int_int_double_double_int_double_double_int(delta, min_area, max_area, max_variation, min_diversity, max_evolution, area_threshold, min_margin, edge_blur_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::features2d::MSER>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait ORBConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_ORB(&self) -> *const c_void;

	#[inline]
	fn get_max_features(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getMaxFeatures_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_scale_factor(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getScaleFactor_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_n_levels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getNLevels_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_edge_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getEdgeThreshold_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_first_level(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getFirstLevel_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_wta_k(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getWTA_K_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_score_type(&self) -> Result<crate::features2d::ORB_ScoreType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getScoreType_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_patch_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getPatchSize_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_fast_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getFastThreshold_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getDefaultName_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait ORB: crate::features2d::Feature2DTrait + crate::features2d::ORBConst {
	fn as_raw_mut_ORB(&mut self) -> *mut c_void;

	#[inline]
	fn set_max_features(&mut self, max_features: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setMaxFeatures_int(self.as_raw_mut_ORB(), max_features, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_scale_factor(&mut self, scale_factor: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setScaleFactor_double(self.as_raw_mut_ORB(), scale_factor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_n_levels(&mut self, nlevels: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setNLevels_int(self.as_raw_mut_ORB(), nlevels, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_edge_threshold(&mut self, edge_threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setEdgeThreshold_int(self.as_raw_mut_ORB(), edge_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_first_level(&mut self, first_level: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setFirstLevel_int(self.as_raw_mut_ORB(), first_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_wta_k(&mut self, wta_k: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setWTA_K_int(self.as_raw_mut_ORB(), wta_k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_score_type(&mut self, score_type: crate::features2d::ORB_ScoreType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setScoreType_ScoreType(self.as_raw_mut_ORB(), score_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_patch_size(&mut self, patch_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setPatchSize_int(self.as_raw_mut_ORB(), patch_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_fast_threshold(&mut self, fast_threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setFastThreshold_int(self.as_raw_mut_ORB(), fast_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn ORB + '_ {
	pub const kBytes: i32 = 32;
	/// ## C++ default parameters
	/// * nfeatures: 500
	/// * scale_factor: 1.2f
	/// * nlevels: 8
	/// * edge_threshold: 31
	/// * first_level: 0
	/// * wta_k: 2
	/// * score_type: ORB::HARRIS_SCORE
	/// * patch_size: 31
	/// * fast_threshold: 20
	#[inline]
	pub fn create(nfeatures: i32, scale_factor: f32, nlevels: i32, edge_threshold: i32, first_level: i32, wta_k: i32, score_type: crate::features2d::ORB_ScoreType, patch_size: i32, fast_threshold: i32) -> Result<core::Ptr<dyn crate::features2d::ORB>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_create_int_float_int_int_int_int_ScoreType_int_int(nfeatures, scale_factor, nlevels, edge_threshold, first_level, wta_k, score_type, patch_size, fast_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::features2d::ORB>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait SIFTTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_SIFT(&self) -> *const c_void;

	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_getDefaultName_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait SIFTTrait: crate::features2d::Feature2DTrait + crate::features2d::SIFTTraitConst {
	fn as_raw_mut_SIFT(&mut self) -> *mut c_void;

}

pub struct SIFT {
	ptr: *mut c_void
}

opencv_type_boxed! { SIFT }

impl Drop for SIFT {
	fn drop(&mut self) {
		extern "C" { fn cv_SIFT_delete(instance: *mut c_void); }
		unsafe { cv_SIFT_delete(self.as_raw_mut_SIFT()) };
	}
}

unsafe impl Send for SIFT {}

impl core::AlgorithmTraitConst for SIFT {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SIFT {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for SIFT {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for SIFT {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::SIFTTraitConst for SIFT {
	#[inline] fn as_raw_SIFT(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::SIFTTrait for SIFT {
	#[inline] fn as_raw_mut_SIFT(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SIFT {
	/// ## C++ default parameters
	/// * nfeatures: 0
	/// * n_octave_layers: 3
	/// * contrast_threshold: 0.04
	/// * edge_threshold: 10
	/// * sigma: 1.6
	#[inline]
	pub fn create(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64) -> Result<core::Ptr<crate::features2d::SIFT>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_create_int_int_double_double_double(nfeatures, n_octave_layers, contrast_threshold, edge_threshold, sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::SIFT>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create_1(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64, descriptor_type: i32) -> Result<core::Ptr<crate::features2d::SIFT>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_create_int_int_double_double_double_int(nfeatures, n_octave_layers, contrast_threshold, edge_threshold, sigma, descriptor_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::SIFT>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SIFT, core::Algorithm, cv_SIFT_to_Algorithm }

boxed_cast_base! { SIFT, crate::features2d::Feature2D, cv_SIFT_to_Feature2D }

pub trait SimpleBlobDetectorTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_SimpleBlobDetector(&self) -> *const c_void;

	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_getDefaultName_const(self.as_raw_SimpleBlobDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait SimpleBlobDetectorTrait: crate::features2d::Feature2DTrait + crate::features2d::SimpleBlobDetectorTraitConst {
	fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void;

}

pub struct SimpleBlobDetector {
	ptr: *mut c_void
}

opencv_type_boxed! { SimpleBlobDetector }

impl Drop for SimpleBlobDetector {
	fn drop(&mut self) {
		extern "C" { fn cv_SimpleBlobDetector_delete(instance: *mut c_void); }
		unsafe { cv_SimpleBlobDetector_delete(self.as_raw_mut_SimpleBlobDetector()) };
	}
}

unsafe impl Send for SimpleBlobDetector {}

impl core::AlgorithmTraitConst for SimpleBlobDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SimpleBlobDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for SimpleBlobDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for SimpleBlobDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::SimpleBlobDetectorTraitConst for SimpleBlobDetector {
	#[inline] fn as_raw_SimpleBlobDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::SimpleBlobDetectorTrait for SimpleBlobDetector {
	#[inline] fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SimpleBlobDetector {
	/// ## C++ default parameters
	/// * parameters: SimpleBlobDetector::Params()
	#[inline]
	pub fn create(parameters: crate::features2d::SimpleBlobDetector_Params) -> Result<core::Ptr<crate::features2d::SimpleBlobDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_create_const_ParamsR(&parameters, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::SimpleBlobDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SimpleBlobDetector, core::Algorithm, cv_SimpleBlobDetector_to_Algorithm }

boxed_cast_base! { SimpleBlobDetector, crate::features2d::Feature2D, cv_SimpleBlobDetector_to_Feature2D }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SimpleBlobDetector_Params {
	pub threshold_step: f32,
	pub min_threshold: f32,
	pub max_threshold: f32,
	pub min_repeatability: size_t,
	pub min_dist_between_blobs: f32,
	pub filter_by_color: bool,
	pub blob_color: u8,
	pub filter_by_area: bool,
	pub min_area: f32,
	pub max_area: f32,
	pub filter_by_circularity: bool,
	pub min_circularity: f32,
	pub max_circularity: f32,
	pub filter_by_inertia: bool,
	pub min_inertia_ratio: f32,
	pub max_inertia_ratio: f32,
	pub filter_by_convexity: bool,
	pub min_convexity: f32,
	pub max_convexity: f32,
}

opencv_type_simple! { crate::features2d::SimpleBlobDetector_Params }

impl SimpleBlobDetector_Params {
	#[inline]
	pub fn write(self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_Params_write_const_FileStorageR(self.opencv_as_extern(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn default() -> Result<crate::features2d::SimpleBlobDetector_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn read(self, fn_: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_Params_read_const_FileNodeR(self.opencv_as_extern(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
pub use crate::manual::features2d::*;
