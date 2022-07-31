#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # ArUco Marker Detection
//! This module is dedicated to square fiducial markers (also known as Augmented Reality Markers)
//! These markers are useful for easy, fast and robust camera pose estimation.ç
//! 
//! The main functionalities are:
//! - Detection of markers in an image
//! - Pose estimation from a single marker or from a board/set of markers
//! - Detection of ChArUco board for high subpixel accuracy
//! - Camera calibration from both, ArUco boards and ChArUco boards.
//! - Detection of ChArUco diamond markers
//! The samples directory includes easy examples of how to use the module.
//! 
//! The implementation is based on the ArUco Library by R. Muñoz-Salinas and S. Garrido-Jurado [Aruco2014](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Aruco2014).
//! 
//! Markers can also be detected based on the AprilTag 2 [wang2016iros](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_wang2016iros) fiducial detection method.
//! ## See also
//! S. Garrido-Jurado, R. Muñoz-Salinas, F. J. Madrid-Cuevas, and M. J. Marín-Jiménez. 2014.
//! "Automatic generation and detection of highly reliable fiducial markers under occlusion".
//! Pattern Recogn. 47, 6 (June 2014), 2280-2292. DOI=10.1016/j.patcog.2014.01.005
//! 
//! http://www.uco.es/investiga/grupos/ava/node/26
//! 
//! This module has been originally developed by Sergio Garrido-Jurado as a project
//! for Google Summer of Code 2015 (GSoC 15).
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::DictionaryTraitConst, super::DictionaryTrait, super::DetectorParametersTraitConst, super::DetectorParametersTrait, super::EstimateParametersTraitConst, super::EstimateParametersTrait, super::BoardTraitConst, super::BoardTrait, super::GridBoardTraitConst, super::GridBoardTrait, super::CharucoBoardTraitConst, super::CharucoBoardTrait };
}

// CCW_center /usr/include/opencv2/aruco.hpp:263
pub const CCW_center: i32 = 0;
// CORNER_REFINE_APRILTAG /usr/include/opencv2/aruco.hpp:86
pub const CORNER_REFINE_APRILTAG: i32 = 3;
// CORNER_REFINE_CONTOUR /usr/include/opencv2/aruco.hpp:85
pub const CORNER_REFINE_CONTOUR: i32 = 2;
// CORNER_REFINE_NONE /usr/include/opencv2/aruco.hpp:83
pub const CORNER_REFINE_NONE: i32 = 0;
// CORNER_REFINE_SUBPIX /usr/include/opencv2/aruco.hpp:84
pub const CORNER_REFINE_SUBPIX: i32 = 1;
// CW_top_left_corner /usr/include/opencv2/aruco.hpp:272
pub const CW_top_left_corner: i32 = 1;
// DICT_4X4_100 /usr/include/opencv2/aruco/dictionary.hpp:158
pub const DICT_4X4_100: i32 = 1;
// DICT_4X4_1000 /usr/include/opencv2/aruco/dictionary.hpp:160
pub const DICT_4X4_1000: i32 = 3;
// DICT_4X4_250 /usr/include/opencv2/aruco/dictionary.hpp:159
pub const DICT_4X4_250: i32 = 2;
// DICT_4X4_50 /usr/include/opencv2/aruco/dictionary.hpp:157
pub const DICT_4X4_50: i32 = 0;
// DICT_5X5_100 /usr/include/opencv2/aruco/dictionary.hpp:162
pub const DICT_5X5_100: i32 = 5;
// DICT_5X5_1000 /usr/include/opencv2/aruco/dictionary.hpp:164
pub const DICT_5X5_1000: i32 = 7;
// DICT_5X5_250 /usr/include/opencv2/aruco/dictionary.hpp:163
pub const DICT_5X5_250: i32 = 6;
// DICT_5X5_50 /usr/include/opencv2/aruco/dictionary.hpp:161
pub const DICT_5X5_50: i32 = 4;
// DICT_6X6_100 /usr/include/opencv2/aruco/dictionary.hpp:166
pub const DICT_6X6_100: i32 = 9;
// DICT_6X6_1000 /usr/include/opencv2/aruco/dictionary.hpp:168
pub const DICT_6X6_1000: i32 = 11;
// DICT_6X6_250 /usr/include/opencv2/aruco/dictionary.hpp:167
pub const DICT_6X6_250: i32 = 10;
// DICT_6X6_50 /usr/include/opencv2/aruco/dictionary.hpp:165
pub const DICT_6X6_50: i32 = 8;
// DICT_7X7_100 /usr/include/opencv2/aruco/dictionary.hpp:170
pub const DICT_7X7_100: i32 = 13;
// DICT_7X7_1000 /usr/include/opencv2/aruco/dictionary.hpp:172
pub const DICT_7X7_1000: i32 = 15;
// DICT_7X7_250 /usr/include/opencv2/aruco/dictionary.hpp:171
pub const DICT_7X7_250: i32 = 14;
// DICT_7X7_50 /usr/include/opencv2/aruco/dictionary.hpp:169
pub const DICT_7X7_50: i32 = 12;
// DICT_APRILTAG_16h5 /usr/include/opencv2/aruco/dictionary.hpp:174
pub const DICT_APRILTAG_16h5: i32 = 17;
// DICT_APRILTAG_25h9 /usr/include/opencv2/aruco/dictionary.hpp:175
pub const DICT_APRILTAG_25h9: i32 = 18;
// DICT_APRILTAG_36h10 /usr/include/opencv2/aruco/dictionary.hpp:176
pub const DICT_APRILTAG_36h10: i32 = 19;
// DICT_APRILTAG_36h11 /usr/include/opencv2/aruco/dictionary.hpp:177
pub const DICT_APRILTAG_36h11: i32 = 20;
// DICT_ARUCO_ORIGINAL /usr/include/opencv2/aruco/dictionary.hpp:173
pub const DICT_ARUCO_ORIGINAL: i32 = 16;
// CornerRefineMethod /usr/include/opencv2/aruco.hpp:82
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CornerRefineMethod {
	CORNER_REFINE_NONE = 0,
	CORNER_REFINE_SUBPIX = 1,
	CORNER_REFINE_CONTOUR = 2,
	CORNER_REFINE_APRILTAG = 3,
}

opencv_type_enum! { crate::aruco::CornerRefineMethod }

// PREDEFINED_DICTIONARY_NAME /usr/include/opencv2/aruco/dictionary.hpp:156
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PREDEFINED_DICTIONARY_NAME {
	DICT_4X4_50 = 0,
	DICT_4X4_100 = 1,
	DICT_4X4_250 = 2,
	DICT_4X4_1000 = 3,
	DICT_5X5_50 = 4,
	DICT_5X5_100 = 5,
	DICT_5X5_250 = 6,
	DICT_5X5_1000 = 7,
	DICT_6X6_50 = 8,
	DICT_6X6_100 = 9,
	DICT_6X6_250 = 10,
	DICT_6X6_1000 = 11,
	DICT_7X7_50 = 12,
	DICT_7X7_100 = 13,
	DICT_7X7_250 = 14,
	DICT_7X7_1000 = 15,
	DICT_ARUCO_ORIGINAL = 16,
	DICT_APRILTAG_16h5 = 17,
	DICT_APRILTAG_25h9 = 18,
	DICT_APRILTAG_36h10 = 19,
	DICT_APRILTAG_36h11 = 20,
}

opencv_type_enum! { crate::aruco::PREDEFINED_DICTIONARY_NAME }

// PatternPos /usr/include/opencv2/aruco.hpp:254
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PatternPos {
	CCW_center = 0,
	CW_top_left_corner = 1,
}

opencv_type_enum! { crate::aruco::PatternPos }

/// ## C++ default parameters
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
// calibrateCameraAruco(cv::InputArrayOfArrays, cv::InputArray, cv::InputArray, const Ptr<cv::aruco::Board> &, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, cv::OutputArray, cv::OutputArray, cv::OutputArray, int, cv::TermCriteria) /usr/include/opencv2/aruco.hpp:648
#[inline]
pub fn calibrate_camera_aruco_extended(corners: &dyn core::ToInputArray, ids: &dyn core::ToInputArray, counter: &dyn core::ToInputArray, board: &core::Ptr<crate::aruco::Board>, image_size: core::Size, camera_matrix: &mut dyn core::ToInputOutputArray, dist_coeffs: &mut dyn core::ToInputOutputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, std_deviations_intrinsics: &mut dyn core::ToOutputArray, std_deviations_extrinsics: &mut dyn core::ToOutputArray, per_view_errors: &mut dyn core::ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
	input_array_arg!(corners);
	input_array_arg!(ids);
	input_array_arg!(counter);
	input_output_array_arg!(camera_matrix);
	input_output_array_arg!(dist_coeffs);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	output_array_arg!(std_deviations_intrinsics);
	output_array_arg!(std_deviations_extrinsics);
	output_array_arg!(per_view_errors);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_Ptr_Board_R_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(corners.as_raw__InputArray(), ids.as_raw__InputArray(), counter.as_raw__InputArray(), board.as_raw_PtrOfBoard(), image_size.opencv_as_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, criteria.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * rvecs: noArray()
/// * tvecs: noArray()
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
// calibrateCameraAruco(cv::InputArrayOfArrays, cv::InputArray, cv::InputArray, const Ptr<cv::aruco::Board> &, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, int, cv::TermCriteria) /usr/include/opencv2/aruco.hpp:659
#[inline]
pub fn calibrate_camera_aruco(corners: &dyn core::ToInputArray, ids: &dyn core::ToInputArray, counter: &dyn core::ToInputArray, board: &core::Ptr<crate::aruco::Board>, image_size: core::Size, camera_matrix: &mut dyn core::ToInputOutputArray, dist_coeffs: &mut dyn core::ToInputOutputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
	input_array_arg!(corners);
	input_array_arg!(ids);
	input_array_arg!(counter);
	input_output_array_arg!(camera_matrix);
	input_output_array_arg!(dist_coeffs);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_Ptr_Board_R_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(corners.as_raw__InputArray(), ids.as_raw__InputArray(), counter.as_raw__InputArray(), board.as_raw_PtrOfBoard(), image_size.opencv_as_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, criteria.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
// calibrateCameraCharuco(cv::InputArrayOfArrays, cv::InputArrayOfArrays, const Ptr<cv::aruco::CharucoBoard> &, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, cv::OutputArray, cv::OutputArray, cv::OutputArray, int, cv::TermCriteria) /usr/include/opencv2/aruco/charuco.hpp:245
#[inline]
pub fn calibrate_camera_charuco_extended(charuco_corners: &dyn core::ToInputArray, charuco_ids: &dyn core::ToInputArray, board: &core::Ptr<crate::aruco::CharucoBoard>, image_size: core::Size, camera_matrix: &mut dyn core::ToInputOutputArray, dist_coeffs: &mut dyn core::ToInputOutputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, std_deviations_intrinsics: &mut dyn core::ToOutputArray, std_deviations_extrinsics: &mut dyn core::ToOutputArray, per_view_errors: &mut dyn core::ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
	input_array_arg!(charuco_corners);
	input_array_arg!(charuco_ids);
	input_output_array_arg!(camera_matrix);
	input_output_array_arg!(dist_coeffs);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	output_array_arg!(std_deviations_intrinsics);
	output_array_arg!(std_deviations_extrinsics);
	output_array_arg!(per_view_errors);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_Ptr_CharucoBoard_R_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), image_size.opencv_as_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, criteria.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * rvecs: noArray()
/// * tvecs: noArray()
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
// calibrateCameraCharuco(cv::InputArrayOfArrays, cv::InputArrayOfArrays, const Ptr<cv::aruco::CharucoBoard> &, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, int, cv::TermCriteria) /usr/include/opencv2/aruco/charuco.hpp:255
#[inline]
pub fn calibrate_camera_charuco(charuco_corners: &dyn core::ToInputArray, charuco_ids: &dyn core::ToInputArray, board: &core::Ptr<crate::aruco::CharucoBoard>, image_size: core::Size, camera_matrix: &mut dyn core::ToInputOutputArray, dist_coeffs: &mut dyn core::ToInputOutputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
	input_array_arg!(charuco_corners);
	input_array_arg!(charuco_ids);
	input_output_array_arg!(camera_matrix);
	input_output_array_arg!(dist_coeffs);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_Ptr_CharucoBoard_R_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), image_size.opencv_as_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, criteria.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * camera_matrix: noArray()
/// * dist_coeffs: noArray()
/// * dictionary: cv::aruco::getPredefinedDictionary(cv::aruco::PREDEFINED_DICTIONARY_NAME::DICT_4X4_50)
// detectCharucoDiamond(cv::InputArray, cv::InputArrayOfArrays, cv::InputArray, float, cv::OutputArrayOfArrays, cv::OutputArray, cv::InputArray, cv::InputArray, Ptr<cv::aruco::Dictionary>) /usr/include/opencv2/aruco/charuco.hpp:286
#[inline]
pub fn detect_charuco_diamond(image: &dyn core::ToInputArray, marker_corners: &dyn core::ToInputArray, marker_ids: &dyn core::ToInputArray, square_marker_length_rate: f32, diamond_corners: &mut dyn core::ToOutputArray, diamond_ids: &mut dyn core::ToOutputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, mut dictionary: core::Ptr<crate::aruco::Dictionary>) -> Result<()> {
	input_array_arg!(image);
	input_array_arg!(marker_corners);
	input_array_arg!(marker_ids);
	output_array_arg!(diamond_corners);
	output_array_arg!(diamond_ids);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_detectCharucoDiamond_const__InputArrayR_const__InputArrayR_const__InputArrayR_float_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_Ptr_Dictionary_(image.as_raw__InputArray(), marker_corners.as_raw__InputArray(), marker_ids.as_raw__InputArray(), square_marker_length_rate, diamond_corners.as_raw__OutputArray(), diamond_ids.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), dictionary.as_raw_mut_PtrOfDictionary(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * parameters: DetectorParameters::create()
/// * rejected_img_points: noArray()
// detectMarkers(cv::InputArray, const Ptr<cv::aruco::Dictionary> &, cv::OutputArrayOfArrays, cv::OutputArray, const Ptr<cv::aruco::DetectorParameters> &, cv::OutputArrayOfArrays) /usr/include/opencv2/aruco.hpp:244
#[inline]
pub fn detect_markers(image: &dyn core::ToInputArray, dictionary: &core::Ptr<crate::aruco::Dictionary>, corners: &mut dyn core::ToOutputArray, ids: &mut dyn core::ToOutputArray, parameters: &core::Ptr<crate::aruco::DetectorParameters>, rejected_img_points: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(image);
	output_array_arg!(corners);
	output_array_arg!(ids);
	output_array_arg!(rejected_img_points);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_detectMarkers_const__InputArrayR_const_Ptr_Dictionary_R_const__OutputArrayR_const__OutputArrayR_const_Ptr_DetectorParameters_R_const__OutputArrayR(image.as_raw__InputArray(), dictionary.as_raw_PtrOfDictionary(), corners.as_raw__OutputArray(), ids.as_raw__OutputArray(), parameters.as_raw_PtrOfDetectorParameters(), rejected_img_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * margin_size: 0
/// * border_bits: 1
// drawCharucoDiamond(const Ptr<cv::aruco::Dictionary> &, cv::Vec4i, int, int, cv::OutputArray, int, int) /usr/include/opencv2/aruco/charuco.hpp:334
#[inline]
pub fn draw_charuco_diamond(dictionary: &core::Ptr<crate::aruco::Dictionary>, ids: core::Vec4i, square_length: i32, marker_length: i32, img: &mut dyn core::ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
	output_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawCharucoDiamond_const_Ptr_Dictionary_R_Vec4i_int_int_const__OutputArrayR_int_int(dictionary.as_raw_PtrOfDictionary(), ids.opencv_as_extern(), square_length, marker_length, img.as_raw__OutputArray(), margin_size, border_bits, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * charuco_ids: noArray()
/// * corner_color: Scalar(255,0,0)
// drawDetectedCornersCharuco(cv::InputOutputArray, cv::InputArray, cv::InputArray, cv::Scalar) /usr/include/opencv2/aruco/charuco.hpp:205
#[inline]
pub fn draw_detected_corners_charuco(image: &mut dyn core::ToInputOutputArray, charuco_corners: &dyn core::ToInputArray, charuco_ids: &dyn core::ToInputArray, corner_color: core::Scalar) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(charuco_corners);
	input_array_arg!(charuco_ids);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), corner_color.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * diamond_ids: noArray()
/// * border_color: Scalar(0,0,255)
// drawDetectedDiamonds(cv::InputOutputArray, cv::InputArrayOfArrays, cv::InputArray, cv::Scalar) /usr/include/opencv2/aruco/charuco.hpp:313
#[inline]
pub fn draw_detected_diamonds(image: &mut dyn core::ToInputOutputArray, diamond_corners: &dyn core::ToInputArray, diamond_ids: &dyn core::ToInputArray, border_color: core::Scalar) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(diamond_corners);
	input_array_arg!(diamond_ids);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), diamond_corners.as_raw__InputArray(), diamond_ids.as_raw__InputArray(), border_color.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * ids: noArray()
/// * border_color: Scalar(0,255,0)
// drawDetectedMarkers(cv::InputOutputArray, cv::InputArrayOfArrays, cv::InputArray, cv::Scalar) /usr/include/opencv2/aruco.hpp:561
#[inline]
pub fn draw_detected_markers(image: &mut dyn core::ToInputOutputArray, corners: &dyn core::ToInputArray, ids: &dyn core::ToInputArray, border_color: core::Scalar) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(corners);
	input_array_arg!(ids);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), corners.as_raw__InputArray(), ids.as_raw__InputArray(), border_color.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * border_bits: 1
// drawMarker(const Ptr<cv::aruco::Dictionary> &, int, int, cv::OutputArray, int) /usr/include/opencv2/aruco.hpp:579
#[inline]
pub fn draw_marker(dictionary: &core::Ptr<crate::aruco::Dictionary>, id: i32, side_pixels: i32, img: &mut dyn core::ToOutputArray, border_bits: i32) -> Result<()> {
	output_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawMarker_const_Ptr_Dictionary_R_int_int_const__OutputArrayR_int(dictionary.as_raw_PtrOfDictionary(), id, side_pixels, img.as_raw__OutputArray(), border_bits, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * margin_size: 0
/// * border_bits: 1
// drawPlanarBoard(const Ptr<cv::aruco::Board> &, cv::Size, cv::OutputArray, int, int) /usr/include/opencv2/aruco.hpp:599
#[inline]
pub fn draw_planar_board(board: &core::Ptr<crate::aruco::Board>, out_size: core::Size, img: &mut dyn core::ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
	output_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawPlanarBoard_const_Ptr_Board_R_Size_const__OutputArrayR_int_int(board.as_raw_PtrOfBoard(), out_size.opencv_as_extern(), img.as_raw__OutputArray(), margin_size, border_bits, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * use_extrinsic_guess: false
// estimatePoseBoard(cv::InputArrayOfArrays, cv::InputArray, const Ptr<cv::aruco::Board> &, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool) /usr/include/opencv2/aruco.hpp:496
#[inline]
pub fn estimate_pose_board(corners: &dyn core::ToInputArray, ids: &dyn core::ToInputArray, board: &core::Ptr<crate::aruco::Board>, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvec: &mut dyn core::ToInputOutputArray, tvec: &mut dyn core::ToInputOutputArray, use_extrinsic_guess: bool) -> Result<i32> {
	input_array_arg!(corners);
	input_array_arg!(ids);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	input_output_array_arg!(rvec);
	input_output_array_arg!(tvec);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_estimatePoseBoard_const__InputArrayR_const__InputArrayR_const_Ptr_Board_R_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool(corners.as_raw__InputArray(), ids.as_raw__InputArray(), board.as_raw_PtrOfBoard(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), use_extrinsic_guess, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * use_extrinsic_guess: false
// estimatePoseCharucoBoard(cv::InputArray, cv::InputArray, const Ptr<cv::aruco::CharucoBoard> &, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool) /usr/include/opencv2/aruco/charuco.hpp:186
#[inline]
pub fn estimate_pose_charuco_board(charuco_corners: &dyn core::ToInputArray, charuco_ids: &dyn core::ToInputArray, board: &core::Ptr<crate::aruco::CharucoBoard>, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvec: &mut dyn core::ToInputOutputArray, tvec: &mut dyn core::ToInputOutputArray, use_extrinsic_guess: bool) -> Result<bool> {
	input_array_arg!(charuco_corners);
	input_array_arg!(charuco_ids);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	input_output_array_arg!(rvec);
	input_output_array_arg!(tvec);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_estimatePoseCharucoBoard_const__InputArrayR_const__InputArrayR_const_Ptr_CharucoBoard_R_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), use_extrinsic_guess, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * _obj_points: noArray()
/// * estimate_parameters: EstimateParameters::create()
// estimatePoseSingleMarkers(cv::InputArrayOfArrays, float, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray, Ptr<cv::aruco::EstimateParameters>) /usr/include/opencv2/aruco.hpp:334
#[inline]
pub fn estimate_pose_single_markers(corners: &dyn core::ToInputArray, marker_length: f32, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, _obj_points: &mut dyn core::ToOutputArray, mut estimate_parameters: core::Ptr<crate::aruco::EstimateParameters>) -> Result<()> {
	input_array_arg!(corners);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	output_array_arg!(_obj_points);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_estimatePoseSingleMarkers_const__InputArrayR_float_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_Ptr_EstimateParameters_(corners.as_raw__InputArray(), marker_length, camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), _obj_points.as_raw__OutputArray(), estimate_parameters.as_raw_mut_PtrOfEstimateParameters(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * random_seed: 0
// generateCustomDictionary(int, int, const Ptr<cv::aruco::Dictionary> &, int) /usr/include/opencv2/aruco/dictionary.hpp:215
#[inline]
pub fn custom_dictionary_from(n_markers: i32, marker_size: i32, base_dictionary: &core::Ptr<crate::aruco::Dictionary>, random_seed: i32) -> Result<core::Ptr<crate::aruco::Dictionary>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_generateCustomDictionary_int_int_const_Ptr_Dictionary_R_int(n_markers, marker_size, base_dictionary.as_raw_PtrOfDictionary(), random_seed, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * random_seed: 0
// generateCustomDictionary(int, int, int) /usr/include/opencv2/aruco/dictionary.hpp:196
#[inline]
pub fn custom_dictionary(n_markers: i32, marker_size: i32, random_seed: i32) -> Result<core::Ptr<crate::aruco::Dictionary>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_generateCustomDictionary_int_int_int(n_markers, marker_size, random_seed, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
	Ok(ret)
}

// getBoardObjectAndImagePoints(const Ptr<cv::aruco::Board> &, cv::InputArrayOfArrays, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/aruco.hpp:676
#[inline]
pub fn get_board_object_and_image_points(board: &core::Ptr<crate::aruco::Board>, detected_corners: &dyn core::ToInputArray, detected_ids: &dyn core::ToInputArray, obj_points: &mut dyn core::ToOutputArray, img_points: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(detected_corners);
	input_array_arg!(detected_ids);
	output_array_arg!(obj_points);
	output_array_arg!(img_points);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_getBoardObjectAndImagePoints_const_Ptr_Board_R_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(board.as_raw_PtrOfBoard(), detected_corners.as_raw__InputArray(), detected_ids.as_raw__InputArray(), obj_points.as_raw__OutputArray(), img_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getPredefinedDictionary(cv::aruco::PREDEFINED_DICTIONARY_NAME) /usr/include/opencv2/aruco/dictionary.hpp:184
#[inline]
pub fn get_predefined_dictionary(name: crate::aruco::PREDEFINED_DICTIONARY_NAME) -> Result<core::Ptr<crate::aruco::Dictionary>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_getPredefinedDictionary_PREDEFINED_DICTIONARY_NAME(name, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
	Ok(ret)
}

// getPredefinedDictionary(int) /usr/include/opencv2/aruco/dictionary.hpp:190
#[inline]
pub fn get_predefined_dictionary_i32(dict: i32) -> Result<core::Ptr<crate::aruco::Dictionary>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_getPredefinedDictionary_int(dict, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * camera_matrix: noArray()
/// * dist_coeffs: noArray()
/// * min_markers: 2
// interpolateCornersCharuco(cv::InputArrayOfArrays, cv::InputArray, cv::InputArray, const Ptr<cv::aruco::CharucoBoard> &, cv::OutputArray, cv::OutputArray, cv::InputArray, cv::InputArray, int) /usr/include/opencv2/aruco/charuco.hpp:158
#[inline]
pub fn interpolate_corners_charuco(marker_corners: &dyn core::ToInputArray, marker_ids: &dyn core::ToInputArray, image: &dyn core::ToInputArray, board: &core::Ptr<crate::aruco::CharucoBoard>, charuco_corners: &mut dyn core::ToOutputArray, charuco_ids: &mut dyn core::ToOutputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, min_markers: i32) -> Result<i32> {
	input_array_arg!(marker_corners);
	input_array_arg!(marker_ids);
	input_array_arg!(image);
	output_array_arg!(charuco_corners);
	output_array_arg!(charuco_ids);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_interpolateCornersCharuco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_Ptr_CharucoBoard_R_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(marker_corners.as_raw__InputArray(), marker_ids.as_raw__InputArray(), image.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), charuco_corners.as_raw__OutputArray(), charuco_ids.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), min_markers, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * camera_matrix: noArray()
/// * dist_coeffs: noArray()
/// * min_rep_distance: 10.f
/// * error_correction_rate: 3.f
/// * check_all_orders: true
/// * recovered_idxs: noArray()
/// * parameters: DetectorParameters::create()
// refineDetectedMarkers(cv::InputArray, const Ptr<cv::aruco::Board> &, cv::InputOutputArrayOfArrays, cv::InputOutputArray, cv::InputOutputArrayOfArrays, cv::InputArray, cv::InputArray, float, float, bool, cv::OutputArray, const Ptr<cv::aruco::DetectorParameters> &) /usr/include/opencv2/aruco.hpp:534
#[inline]
pub fn refine_detected_markers(image: &dyn core::ToInputArray, board: &core::Ptr<crate::aruco::Board>, detected_corners: &mut dyn core::ToInputOutputArray, detected_ids: &mut dyn core::ToInputOutputArray, rejected_corners: &mut dyn core::ToInputOutputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, min_rep_distance: f32, error_correction_rate: f32, check_all_orders: bool, recovered_idxs: &mut dyn core::ToOutputArray, parameters: &core::Ptr<crate::aruco::DetectorParameters>) -> Result<()> {
	input_array_arg!(image);
	input_output_array_arg!(detected_corners);
	input_output_array_arg!(detected_ids);
	input_output_array_arg!(rejected_corners);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(recovered_idxs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_refineDetectedMarkers_const__InputArrayR_const_Ptr_Board_R_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_float_float_bool_const__OutputArrayR_const_Ptr_DetectorParameters_R(image.as_raw__InputArray(), board.as_raw_PtrOfBoard(), detected_corners.as_raw__InputOutputArray(), detected_ids.as_raw__InputOutputArray(), rejected_corners.as_raw__InputOutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), min_rep_distance, error_correction_rate, check_all_orders, recovered_idxs.as_raw__OutputArray(), parameters.as_raw_PtrOfDetectorParameters(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// testCharucoCornersCollinear(const Ptr<cv::aruco::CharucoBoard> &, cv::InputArray) /usr/include/opencv2/aruco/charuco.hpp:349
#[inline]
pub fn test_charuco_corners_collinear(_board: &core::Ptr<crate::aruco::CharucoBoard>, _charuco_ids: &dyn core::ToInputArray) -> Result<bool> {
	input_array_arg!(_charuco_ids);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_testCharucoCornersCollinear_const_Ptr_CharucoBoard_R_const__InputArrayR(_board.as_raw_PtrOfCharucoBoard(), _charuco_ids.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// Board /usr/include/opencv2/aruco.hpp:351
pub trait BoardTraitConst {
	fn as_raw_Board(&self) -> *const c_void;

	// objPoints /usr/include/opencv2/aruco.hpp:384
	#[inline]
	fn obj_points(&self) -> core::Vector<core::Vector<core::Point3f>> {
		let ret = unsafe { sys::cv_aruco_Board_getPropObjPoints_const(self.as_raw_Board()) };
		let ret = unsafe { core::Vector::<core::Vector<core::Point3f>>::opencv_from_extern(ret) };
		ret
	}
	
	// ids /usr/include/opencv2/aruco.hpp:391
	#[inline]
	fn ids(&self) -> core::Vector<i32> {
		let ret = unsafe { sys::cv_aruco_Board_getPropIds_const(self.as_raw_Board()) };
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		ret
	}
	
	// rightBottomBorder /usr/include/opencv2/aruco.hpp:394
	#[inline]
	fn right_bottom_border(&self) -> core::Point3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Board_getPropRightBottomBorder_const(self.as_raw_Board(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
}

pub trait BoardTrait: crate::aruco::BoardTraitConst {
	fn as_raw_mut_Board(&mut self) -> *mut c_void;

	// objPoints /usr/include/opencv2/aruco.hpp:384
	#[inline]
	fn set_obj_points(&mut self, mut val: core::Vector<core::Vector<core::Point3f>>) {
		let ret = unsafe { sys::cv_aruco_Board_setPropObjPoints_vector_vector_Point3f__(self.as_raw_mut_Board(), val.as_raw_mut_VectorOfVectorOfPoint3f()) };
		ret
	}
	
	// dictionary /usr/include/opencv2/aruco.hpp:387
	#[inline]
	fn dictionary(&mut self) -> core::Ptr<crate::aruco::Dictionary> {
		let ret = unsafe { sys::cv_aruco_Board_getPropDictionary(self.as_raw_mut_Board()) };
		let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
		ret
	}
	
	// dictionary /usr/include/opencv2/aruco.hpp:387
	#[inline]
	fn set_dictionary(&mut self, mut val: core::Ptr<crate::aruco::Dictionary>) {
		let ret = unsafe { sys::cv_aruco_Board_setPropDictionary_Ptr_Dictionary_(self.as_raw_mut_Board(), val.as_raw_mut_PtrOfDictionary()) };
		ret
	}
	
	// ids /usr/include/opencv2/aruco.hpp:391
	#[inline]
	fn set_ids(&mut self, mut val: core::Vector<i32>) {
		let ret = unsafe { sys::cv_aruco_Board_setPropIds_vector_int_(self.as_raw_mut_Board(), val.as_raw_mut_VectorOfi32()) };
		ret
	}
	
	// rightBottomBorder /usr/include/opencv2/aruco.hpp:394
	#[inline]
	fn set_right_bottom_border(&mut self, val: core::Point3f) {
		let ret = unsafe { sys::cv_aruco_Board_setPropRightBottomBorder_Point3f(self.as_raw_mut_Board(), val.opencv_as_extern()) };
		ret
	}
	
	// setIds(cv::InputArray) /usr/include/opencv2/aruco.hpp:373
	#[inline]
	fn set_ids_1(&mut self, ids: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(ids);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Board_setIds_const__InputArrayR(self.as_raw_mut_Board(), ids.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Board /usr/include/opencv2/aruco.hpp:351
pub struct Board {
	ptr: *mut c_void
}

opencv_type_boxed! { Board }

impl Drop for Board {
	fn drop(&mut self) {
		extern "C" { fn cv_Board_delete(instance: *mut c_void); }
		unsafe { cv_Board_delete(self.as_raw_mut_Board()) };
	}
}

unsafe impl Send for Board {}

impl crate::aruco::BoardTraitConst for Board {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.as_raw() }
}

impl crate::aruco::BoardTrait for Board {
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Board {
	// create(cv::InputArrayOfArrays, const Ptr<cv::aruco::Dictionary> &, cv::InputArray) /usr/include/opencv2/aruco.hpp:362
	#[inline]
	pub fn create(obj_points: &dyn core::ToInputArray, dictionary: &core::Ptr<crate::aruco::Dictionary>, ids: &dyn core::ToInputArray) -> Result<core::Ptr<crate::aruco::Board>> {
		input_array_arg!(obj_points);
		input_array_arg!(ids);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Board_create_const__InputArrayR_const_Ptr_Dictionary_R_const__InputArrayR(obj_points.as_raw__InputArray(), dictionary.as_raw_PtrOfDictionary(), ids.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::Board>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// CharucoBoard /usr/include/opencv2/aruco/charuco.hpp:62
pub trait CharucoBoardTraitConst: crate::aruco::BoardTraitConst {
	fn as_raw_CharucoBoard(&self) -> *const c_void;

	// chessboardCorners /usr/include/opencv2/aruco/charuco.hpp:66
	#[inline]
	fn chessboard_corners(&self) -> core::Vector<core::Point3f> {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_getPropChessboardCorners_const(self.as_raw_CharucoBoard()) };
		let ret = unsafe { core::Vector::<core::Point3f>::opencv_from_extern(ret) };
		ret
	}
	
	// nearestMarkerIdx /usr/include/opencv2/aruco/charuco.hpp:69
	#[inline]
	fn nearest_marker_idx(&self) -> core::Vector<core::Vector<i32>> {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_getPropNearestMarkerIdx_const(self.as_raw_CharucoBoard()) };
		let ret = unsafe { core::Vector::<core::Vector<i32>>::opencv_from_extern(ret) };
		ret
	}
	
	// nearestMarkerCorners /usr/include/opencv2/aruco/charuco.hpp:70
	#[inline]
	fn nearest_marker_corners(&self) -> core::Vector<core::Vector<i32>> {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_getPropNearestMarkerCorners_const(self.as_raw_CharucoBoard()) };
		let ret = unsafe { core::Vector::<core::Vector<i32>>::opencv_from_extern(ret) };
		ret
	}
	
	// getChessboardSize() /usr/include/opencv2/aruco/charuco.hpp:106
	#[inline]
	fn get_chessboard_size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_CharucoBoard_getChessboardSize_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSquareLength() /usr/include/opencv2/aruco/charuco.hpp:111
	#[inline]
	fn get_square_length(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_CharucoBoard_getSquareLength_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getMarkerLength() /usr/include/opencv2/aruco/charuco.hpp:116
	#[inline]
	fn get_marker_length(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_CharucoBoard_getMarkerLength_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait CharucoBoardTrait: crate::aruco::BoardTrait + crate::aruco::CharucoBoardTraitConst {
	fn as_raw_mut_CharucoBoard(&mut self) -> *mut c_void;

	// chessboardCorners /usr/include/opencv2/aruco/charuco.hpp:66
	#[inline]
	fn set_chessboard_corners(&mut self, mut val: core::Vector<core::Point3f>) {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_setPropChessboardCorners_vector_Point3f_(self.as_raw_mut_CharucoBoard(), val.as_raw_mut_VectorOfPoint3f()) };
		ret
	}
	
	// nearestMarkerIdx /usr/include/opencv2/aruco/charuco.hpp:69
	#[inline]
	fn set_nearest_marker_idx(&mut self, mut val: core::Vector<core::Vector<i32>>) {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_setPropNearestMarkerIdx_vector_vector_int__(self.as_raw_mut_CharucoBoard(), val.as_raw_mut_VectorOfVectorOfi32()) };
		ret
	}
	
	// nearestMarkerCorners /usr/include/opencv2/aruco/charuco.hpp:70
	#[inline]
	fn set_nearest_marker_corners(&mut self, mut val: core::Vector<core::Vector<i32>>) {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_setPropNearestMarkerCorners_vector_vector_int__(self.as_raw_mut_CharucoBoard(), val.as_raw_mut_VectorOfVectorOfi32()) };
		ret
	}
	
	/// ## C++ default parameters
	/// * margin_size: 0
	/// * border_bits: 1
	// draw(cv::Size, cv::OutputArray, int, int) /usr/include/opencv2/aruco/charuco.hpp:83
	#[inline]
	fn draw(&mut self, out_size: core::Size, img: &mut dyn core::ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
		output_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_CharucoBoard_draw_Size_const__OutputArrayR_int_int(self.as_raw_mut_CharucoBoard(), out_size.opencv_as_extern(), img.as_raw__OutputArray(), margin_size, border_bits, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// CharucoBoard /usr/include/opencv2/aruco/charuco.hpp:62
pub struct CharucoBoard {
	ptr: *mut c_void
}

opencv_type_boxed! { CharucoBoard }

impl Drop for CharucoBoard {
	fn drop(&mut self) {
		extern "C" { fn cv_CharucoBoard_delete(instance: *mut c_void); }
		unsafe { cv_CharucoBoard_delete(self.as_raw_mut_CharucoBoard()) };
	}
}

unsafe impl Send for CharucoBoard {}

impl crate::aruco::BoardTraitConst for CharucoBoard {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.as_raw() }
}

impl crate::aruco::BoardTrait for CharucoBoard {
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::aruco::CharucoBoardTraitConst for CharucoBoard {
	#[inline] fn as_raw_CharucoBoard(&self) -> *const c_void { self.as_raw() }
}

impl crate::aruco::CharucoBoardTrait for CharucoBoard {
	#[inline] fn as_raw_mut_CharucoBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CharucoBoard {
	// create(int, int, float, float, const Ptr<cv::aruco::Dictionary> &) /usr/include/opencv2/aruco/charuco.hpp:100
	#[inline]
	pub fn create(squares_x: i32, squares_y: i32, square_length: f32, marker_length: f32, dictionary: &core::Ptr<crate::aruco::Dictionary>) -> Result<core::Ptr<crate::aruco::CharucoBoard>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_CharucoBoard_create_int_int_float_float_const_Ptr_Dictionary_R(squares_x, squares_y, square_length, marker_length, dictionary.as_raw_PtrOfDictionary(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::CharucoBoard>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { CharucoBoard, crate::aruco::Board, cv_CharucoBoard_to_Board }

// DetectorParameters /usr/include/opencv2/aruco.hpp:169
pub trait DetectorParametersTraitConst {
	fn as_raw_DetectorParameters(&self) -> *const c_void;

	// adaptiveThreshWinSizeMin /usr/include/opencv2/aruco.hpp:175
	#[inline]
	fn adaptive_thresh_win_size_min(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeMin_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// adaptiveThreshWinSizeMax /usr/include/opencv2/aruco.hpp:176
	#[inline]
	fn adaptive_thresh_win_size_max(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeMax_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// adaptiveThreshWinSizeStep /usr/include/opencv2/aruco.hpp:177
	#[inline]
	fn adaptive_thresh_win_size_step(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeStep_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// adaptiveThreshConstant /usr/include/opencv2/aruco.hpp:178
	#[inline]
	fn adaptive_thresh_constant(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAdaptiveThreshConstant_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// minMarkerPerimeterRate /usr/include/opencv2/aruco.hpp:179
	#[inline]
	fn min_marker_perimeter_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMinMarkerPerimeterRate_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// maxMarkerPerimeterRate /usr/include/opencv2/aruco.hpp:180
	#[inline]
	fn max_marker_perimeter_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMaxMarkerPerimeterRate_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// polygonalApproxAccuracyRate /usr/include/opencv2/aruco.hpp:181
	#[inline]
	fn polygonal_approx_accuracy_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropPolygonalApproxAccuracyRate_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// minCornerDistanceRate /usr/include/opencv2/aruco.hpp:182
	#[inline]
	fn min_corner_distance_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMinCornerDistanceRate_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// minDistanceToBorder /usr/include/opencv2/aruco.hpp:183
	#[inline]
	fn min_distance_to_border(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMinDistanceToBorder_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// minMarkerDistanceRate /usr/include/opencv2/aruco.hpp:184
	#[inline]
	fn min_marker_distance_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMinMarkerDistanceRate_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// cornerRefinementMethod /usr/include/opencv2/aruco.hpp:185
	#[inline]
	fn corner_refinement_method(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropCornerRefinementMethod_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// cornerRefinementWinSize /usr/include/opencv2/aruco.hpp:186
	#[inline]
	fn corner_refinement_win_size(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropCornerRefinementWinSize_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// cornerRefinementMaxIterations /usr/include/opencv2/aruco.hpp:187
	#[inline]
	fn corner_refinement_max_iterations(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropCornerRefinementMaxIterations_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// cornerRefinementMinAccuracy /usr/include/opencv2/aruco.hpp:188
	#[inline]
	fn corner_refinement_min_accuracy(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropCornerRefinementMinAccuracy_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// markerBorderBits /usr/include/opencv2/aruco.hpp:189
	#[inline]
	fn marker_border_bits(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMarkerBorderBits_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// perspectiveRemovePixelPerCell /usr/include/opencv2/aruco.hpp:190
	#[inline]
	fn perspective_remove_pixel_per_cell(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropPerspectiveRemovePixelPerCell_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// perspectiveRemoveIgnoredMarginPerCell /usr/include/opencv2/aruco.hpp:191
	#[inline]
	fn perspective_remove_ignored_margin_per_cell(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropPerspectiveRemoveIgnoredMarginPerCell_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// maxErroneousBitsInBorderRate /usr/include/opencv2/aruco.hpp:192
	#[inline]
	fn max_erroneous_bits_in_border_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMaxErroneousBitsInBorderRate_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// minOtsuStdDev /usr/include/opencv2/aruco.hpp:193
	#[inline]
	fn min_otsu_std_dev(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMinOtsuStdDev_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// errorCorrectionRate /usr/include/opencv2/aruco.hpp:194
	#[inline]
	fn error_correction_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropErrorCorrectionRate_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// aprilTagQuadDecimate /usr/include/opencv2/aruco.hpp:197
	#[inline]
	fn april_tag_quad_decimate(&self) -> f32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagQuadDecimate_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// aprilTagQuadSigma /usr/include/opencv2/aruco.hpp:198
	#[inline]
	fn april_tag_quad_sigma(&self) -> f32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagQuadSigma_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// aprilTagMinClusterPixels /usr/include/opencv2/aruco.hpp:201
	#[inline]
	fn april_tag_min_cluster_pixels(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagMinClusterPixels_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// aprilTagMaxNmaxima /usr/include/opencv2/aruco.hpp:202
	#[inline]
	fn april_tag_max_nmaxima(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagMaxNmaxima_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// aprilTagCriticalRad /usr/include/opencv2/aruco.hpp:203
	#[inline]
	fn april_tag_critical_rad(&self) -> f32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagCriticalRad_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// aprilTagMaxLineFitMse /usr/include/opencv2/aruco.hpp:204
	#[inline]
	fn april_tag_max_line_fit_mse(&self) -> f32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagMaxLineFitMse_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// aprilTagMinWhiteBlackDiff /usr/include/opencv2/aruco.hpp:205
	#[inline]
	fn april_tag_min_white_black_diff(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagMinWhiteBlackDiff_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// aprilTagDeglitch /usr/include/opencv2/aruco.hpp:206
	#[inline]
	fn april_tag_deglitch(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagDeglitch_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// detectInvertedMarker /usr/include/opencv2/aruco.hpp:209
	#[inline]
	fn detect_inverted_marker(&self) -> bool {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropDetectInvertedMarker_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// useAruco3Detection /usr/include/opencv2/aruco.hpp:213
	#[inline]
	fn use_aruco3_detection(&self) -> bool {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropUseAruco3Detection_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// minSideLengthCanonicalImg /usr/include/opencv2/aruco.hpp:214
	#[inline]
	fn min_side_length_canonical_img(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMinSideLengthCanonicalImg_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	// minMarkerLengthRatioOriginalImg /usr/include/opencv2/aruco.hpp:215
	#[inline]
	fn min_marker_length_ratio_original_img(&self) -> f32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMinMarkerLengthRatioOriginalImg_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
}

pub trait DetectorParametersTrait: crate::aruco::DetectorParametersTraitConst {
	fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void;

	// adaptiveThreshWinSizeMin /usr/include/opencv2/aruco.hpp:175
	#[inline]
	fn set_adaptive_thresh_win_size_min(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeMin_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// adaptiveThreshWinSizeMax /usr/include/opencv2/aruco.hpp:176
	#[inline]
	fn set_adaptive_thresh_win_size_max(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeMax_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// adaptiveThreshWinSizeStep /usr/include/opencv2/aruco.hpp:177
	#[inline]
	fn set_adaptive_thresh_win_size_step(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeStep_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// adaptiveThreshConstant /usr/include/opencv2/aruco.hpp:178
	#[inline]
	fn set_adaptive_thresh_constant(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAdaptiveThreshConstant_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// minMarkerPerimeterRate /usr/include/opencv2/aruco.hpp:179
	#[inline]
	fn set_min_marker_perimeter_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMinMarkerPerimeterRate_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// maxMarkerPerimeterRate /usr/include/opencv2/aruco.hpp:180
	#[inline]
	fn set_max_marker_perimeter_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMaxMarkerPerimeterRate_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// polygonalApproxAccuracyRate /usr/include/opencv2/aruco.hpp:181
	#[inline]
	fn set_polygonal_approx_accuracy_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropPolygonalApproxAccuracyRate_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// minCornerDistanceRate /usr/include/opencv2/aruco.hpp:182
	#[inline]
	fn set_min_corner_distance_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMinCornerDistanceRate_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// minDistanceToBorder /usr/include/opencv2/aruco.hpp:183
	#[inline]
	fn set_min_distance_to_border(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMinDistanceToBorder_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// minMarkerDistanceRate /usr/include/opencv2/aruco.hpp:184
	#[inline]
	fn set_min_marker_distance_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMinMarkerDistanceRate_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// cornerRefinementMethod /usr/include/opencv2/aruco.hpp:185
	#[inline]
	fn set_corner_refinement_method(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropCornerRefinementMethod_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// cornerRefinementWinSize /usr/include/opencv2/aruco.hpp:186
	#[inline]
	fn set_corner_refinement_win_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropCornerRefinementWinSize_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// cornerRefinementMaxIterations /usr/include/opencv2/aruco.hpp:187
	#[inline]
	fn set_corner_refinement_max_iterations(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropCornerRefinementMaxIterations_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// cornerRefinementMinAccuracy /usr/include/opencv2/aruco.hpp:188
	#[inline]
	fn set_corner_refinement_min_accuracy(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropCornerRefinementMinAccuracy_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// markerBorderBits /usr/include/opencv2/aruco.hpp:189
	#[inline]
	fn set_marker_border_bits(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMarkerBorderBits_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// perspectiveRemovePixelPerCell /usr/include/opencv2/aruco.hpp:190
	#[inline]
	fn set_perspective_remove_pixel_per_cell(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropPerspectiveRemovePixelPerCell_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// perspectiveRemoveIgnoredMarginPerCell /usr/include/opencv2/aruco.hpp:191
	#[inline]
	fn set_perspective_remove_ignored_margin_per_cell(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropPerspectiveRemoveIgnoredMarginPerCell_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// maxErroneousBitsInBorderRate /usr/include/opencv2/aruco.hpp:192
	#[inline]
	fn set_max_erroneous_bits_in_border_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMaxErroneousBitsInBorderRate_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// minOtsuStdDev /usr/include/opencv2/aruco.hpp:193
	#[inline]
	fn set_min_otsu_std_dev(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMinOtsuStdDev_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// errorCorrectionRate /usr/include/opencv2/aruco.hpp:194
	#[inline]
	fn set_error_correction_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropErrorCorrectionRate_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// aprilTagQuadDecimate /usr/include/opencv2/aruco.hpp:197
	#[inline]
	fn set_april_tag_quad_decimate(&mut self, val: f32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagQuadDecimate_float(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// aprilTagQuadSigma /usr/include/opencv2/aruco.hpp:198
	#[inline]
	fn set_april_tag_quad_sigma(&mut self, val: f32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagQuadSigma_float(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// aprilTagMinClusterPixels /usr/include/opencv2/aruco.hpp:201
	#[inline]
	fn set_april_tag_min_cluster_pixels(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagMinClusterPixels_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// aprilTagMaxNmaxima /usr/include/opencv2/aruco.hpp:202
	#[inline]
	fn set_april_tag_max_nmaxima(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagMaxNmaxima_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// aprilTagCriticalRad /usr/include/opencv2/aruco.hpp:203
	#[inline]
	fn set_april_tag_critical_rad(&mut self, val: f32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagCriticalRad_float(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// aprilTagMaxLineFitMse /usr/include/opencv2/aruco.hpp:204
	#[inline]
	fn set_april_tag_max_line_fit_mse(&mut self, val: f32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagMaxLineFitMse_float(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// aprilTagMinWhiteBlackDiff /usr/include/opencv2/aruco.hpp:205
	#[inline]
	fn set_april_tag_min_white_black_diff(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagMinWhiteBlackDiff_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// aprilTagDeglitch /usr/include/opencv2/aruco.hpp:206
	#[inline]
	fn set_april_tag_deglitch(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagDeglitch_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// detectInvertedMarker /usr/include/opencv2/aruco.hpp:209
	#[inline]
	fn set_detect_inverted_marker(&mut self, val: bool) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropDetectInvertedMarker_bool(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// useAruco3Detection /usr/include/opencv2/aruco.hpp:213
	#[inline]
	fn set_use_aruco3_detection(&mut self, val: bool) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropUseAruco3Detection_bool(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// minSideLengthCanonicalImg /usr/include/opencv2/aruco.hpp:214
	#[inline]
	fn set_min_side_length_canonical_img(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMinSideLengthCanonicalImg_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// minMarkerLengthRatioOriginalImg /usr/include/opencv2/aruco.hpp:215
	#[inline]
	fn set_min_marker_length_ratio_original_img(&mut self, val: f32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMinMarkerLengthRatioOriginalImg_float(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	// readDetectorParameters(const cv::FileNode &) /usr/include/opencv2/aruco.hpp:173
	#[inline]
	fn read_detector_parameters(&mut self, fn_: &core::FileNode) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_DetectorParameters_readDetectorParameters_const_FileNodeR(self.as_raw_mut_DetectorParameters(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// DetectorParameters /usr/include/opencv2/aruco.hpp:169
pub struct DetectorParameters {
	ptr: *mut c_void
}

opencv_type_boxed! { DetectorParameters }

impl Drop for DetectorParameters {
	fn drop(&mut self) {
		extern "C" { fn cv_DetectorParameters_delete(instance: *mut c_void); }
		unsafe { cv_DetectorParameters_delete(self.as_raw_mut_DetectorParameters()) };
	}
}

unsafe impl Send for DetectorParameters {}

impl crate::aruco::DetectorParametersTraitConst for DetectorParameters {
	#[inline] fn as_raw_DetectorParameters(&self) -> *const c_void { self.as_raw() }
}

impl crate::aruco::DetectorParametersTrait for DetectorParameters {
	#[inline] fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DetectorParameters {
	// DetectorParameters() /usr/include/opencv2/aruco.hpp:171
	#[inline]
	pub fn default() -> Result<crate::aruco::DetectorParameters> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_DetectorParameters_DetectorParameters(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::aruco::DetectorParameters::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// create() /usr/include/opencv2/aruco.hpp:172
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::aruco::DetectorParameters>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_DetectorParameters_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::DetectorParameters>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Dictionary /usr/include/opencv2/aruco/dictionary.hpp:61
pub trait DictionaryTraitConst {
	fn as_raw_Dictionary(&self) -> *const c_void;

	// bytesList /usr/include/opencv2/aruco/dictionary.hpp:64
	#[inline]
	fn bytes_list(&self) -> core::Mat {
		let ret = unsafe { sys::cv_aruco_Dictionary_getPropBytesList_const(self.as_raw_Dictionary()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// markerSize /usr/include/opencv2/aruco/dictionary.hpp:65
	#[inline]
	fn marker_size(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_Dictionary_getPropMarkerSize_const(self.as_raw_Dictionary()) };
		ret
	}
	
	// maxCorrectionBits /usr/include/opencv2/aruco/dictionary.hpp:66
	#[inline]
	fn max_correction_bits(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_Dictionary_getPropMaxCorrectionBits_const(self.as_raw_Dictionary()) };
		ret
	}
	
	// identify(const cv::Mat &, int &, int &, double) /usr/include/opencv2/aruco/dictionary.hpp:120
	#[inline]
	fn identify(&self, only_bits: &core::Mat, idx: &mut i32, rotation: &mut i32, max_correction_rate: f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_identify_const_const_MatR_intR_intR_double(self.as_raw_Dictionary(), only_bits.as_raw_Mat(), idx, rotation, max_correction_rate, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * all_rotations: true
	// getDistanceToId(cv::InputArray, int, bool) /usr/include/opencv2/aruco/dictionary.hpp:126
	#[inline]
	fn get_distance_to_id(&self, bits: &dyn core::ToInputArray, id: i32, all_rotations: bool) -> Result<i32> {
		input_array_arg!(bits);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int_bool(self.as_raw_Dictionary(), bits.as_raw__InputArray(), id, all_rotations, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * border_bits: 1
	// drawMarker(int, int, cv::OutputArray, int) /usr/include/opencv2/aruco/dictionary.hpp:132
	#[inline]
	fn draw_marker(&self, id: i32, side_pixels: i32, _img: &mut dyn core::ToOutputArray, border_bits: i32) -> Result<()> {
		output_array_arg!(_img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_drawMarker_const_int_int_const__OutputArrayR_int(self.as_raw_Dictionary(), id, side_pixels, _img.as_raw__OutputArray(), border_bits, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait DictionaryTrait: crate::aruco::DictionaryTraitConst {
	fn as_raw_mut_Dictionary(&mut self) -> *mut c_void;

	// bytesList /usr/include/opencv2/aruco/dictionary.hpp:64
	#[inline]
	fn set_bytes_list(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_aruco_Dictionary_setPropBytesList_Mat(self.as_raw_mut_Dictionary(), val.as_raw_mut_Mat()) };
		ret
	}
	
	// markerSize /usr/include/opencv2/aruco/dictionary.hpp:65
	#[inline]
	fn set_marker_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_Dictionary_setPropMarkerSize_int(self.as_raw_mut_Dictionary(), val) };
		ret
	}
	
	// maxCorrectionBits /usr/include/opencv2/aruco/dictionary.hpp:66
	#[inline]
	fn set_max_correction_bits(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_Dictionary_setPropMaxCorrectionBits_int(self.as_raw_mut_Dictionary(), val) };
		ret
	}
	
	// readDictionary(const cv::FileNode &) /usr/include/opencv2/aruco/dictionary.hpp:105
	#[inline]
	fn read_dictionary(&mut self, fn_: &core::FileNode) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_readDictionary_const_FileNodeR(self.as_raw_mut_Dictionary(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// writeDictionary(Ptr<cv::FileStorage> &) /usr/include/opencv2/aruco/dictionary.hpp:110
	#[inline]
	fn write_dictionary(&mut self, fs: &mut core::Ptr<core::FileStorage>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_writeDictionary_Ptr_FileStorage_R(self.as_raw_mut_Dictionary(), fs.as_raw_mut_PtrOfFileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Dictionary /usr/include/opencv2/aruco/dictionary.hpp:61
pub struct Dictionary {
	ptr: *mut c_void
}

opencv_type_boxed! { Dictionary }

impl Drop for Dictionary {
	fn drop(&mut self) {
		extern "C" { fn cv_Dictionary_delete(instance: *mut c_void); }
		unsafe { cv_Dictionary_delete(self.as_raw_mut_Dictionary()) };
	}
}

unsafe impl Send for Dictionary {}

impl crate::aruco::DictionaryTraitConst for Dictionary {
	#[inline] fn as_raw_Dictionary(&self) -> *const c_void { self.as_raw() }
}

impl crate::aruco::DictionaryTrait for Dictionary {
	#[inline] fn as_raw_mut_Dictionary(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Dictionary {
	/// ## C++ default parameters
	/// * _bytes_list: Mat()
	/// * _marker_size: 0
	/// * _maxcorr: 0
	// Dictionary(const cv::Mat &, int, int) /usr/include/opencv2/aruco/dictionary.hpp:71
	#[inline]
	pub fn new(_bytes_list: &core::Mat, _marker_size: i32, _maxcorr: i32) -> Result<crate::aruco::Dictionary> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_Dictionary_const_MatR_int_int(_bytes_list.as_raw_Mat(), _marker_size, _maxcorr, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::aruco::Dictionary::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Dictionary(const Ptr<cv::aruco::Dictionary> &) /usr/include/opencv2/aruco/dictionary.hpp:81
	#[inline]
	pub fn copy(_dictionary: &core::Ptr<crate::aruco::Dictionary>) -> Result<crate::aruco::Dictionary> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_Dictionary_const_Ptr_Dictionary_R(_dictionary.as_raw_PtrOfDictionary(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::aruco::Dictionary::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * random_seed: 0
	// create(int, int, int) /usr/include/opencv2/aruco/dictionary.hpp:87
	#[inline]
	pub fn create(n_markers: i32, marker_size: i32, random_seed: i32) -> Result<core::Ptr<crate::aruco::Dictionary>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_create_int_int_int(n_markers, marker_size, random_seed, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * random_seed: 0
	// create(int, int, const Ptr<cv::aruco::Dictionary> &, int) /usr/include/opencv2/aruco/dictionary.hpp:93
	#[inline]
	pub fn create_from(n_markers: i32, marker_size: i32, base_dictionary: &core::Ptr<crate::aruco::Dictionary>, random_seed: i32) -> Result<core::Ptr<crate::aruco::Dictionary>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_create_int_int_const_Ptr_Dictionary_R_int(n_markers, marker_size, base_dictionary.as_raw_PtrOfDictionary(), random_seed, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// get(int) /usr/include/opencv2/aruco/dictionary.hpp:114
	#[inline]
	pub fn get(dict: i32) -> Result<core::Ptr<crate::aruco::Dictionary>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_get_int(dict, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getByteListFromBits(const cv::Mat &) /usr/include/opencv2/aruco/dictionary.hpp:138
	#[inline]
	pub fn get_byte_list_from_bits(bits: &core::Mat) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_getByteListFromBits_const_MatR(bits.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getBitsFromByteList(const cv::Mat &, int) /usr/include/opencv2/aruco/dictionary.hpp:144
	#[inline]
	pub fn get_bits_from_byte_list(byte_list: &core::Mat, marker_size: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_getBitsFromByteList_const_MatR_int(byte_list.as_raw_Mat(), marker_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// EstimateParameters /usr/include/opencv2/aruco.hpp:284
pub trait EstimateParametersTraitConst {
	fn as_raw_EstimateParameters(&self) -> *const c_void;

	// pattern /usr/include/opencv2/aruco.hpp:285
	#[inline]
	fn pattern(&self) -> crate::aruco::PatternPos {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_EstimateParameters_getPropPattern_const(self.as_raw_EstimateParameters(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// useExtrinsicGuess /usr/include/opencv2/aruco.hpp:286
	#[inline]
	fn use_extrinsic_guess(&self) -> bool {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_getPropUseExtrinsicGuess_const(self.as_raw_EstimateParameters()) };
		ret
	}
	
	// solvePnPMethod /usr/include/opencv2/aruco.hpp:287
	#[inline]
	fn solve_pnp_method(&self) -> crate::calib3d::SolvePnPMethod {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_EstimateParameters_getPropSolvePnPMethod_const(self.as_raw_EstimateParameters(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
}

pub trait EstimateParametersTrait: crate::aruco::EstimateParametersTraitConst {
	fn as_raw_mut_EstimateParameters(&mut self) -> *mut c_void;

	// pattern /usr/include/opencv2/aruco.hpp:285
	#[inline]
	fn set_pattern(&mut self, val: crate::aruco::PatternPos) {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_setPropPattern_PatternPos(self.as_raw_mut_EstimateParameters(), val) };
		ret
	}
	
	// useExtrinsicGuess /usr/include/opencv2/aruco.hpp:286
	#[inline]
	fn set_use_extrinsic_guess(&mut self, val: bool) {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_setPropUseExtrinsicGuess_bool(self.as_raw_mut_EstimateParameters(), val) };
		ret
	}
	
	// solvePnPMethod /usr/include/opencv2/aruco.hpp:287
	#[inline]
	fn set_solve_pnp_method(&mut self, val: crate::calib3d::SolvePnPMethod) {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_setPropSolvePnPMethod_SolvePnPMethod(self.as_raw_mut_EstimateParameters(), val) };
		ret
	}
	
}

// EstimateParameters /usr/include/opencv2/aruco.hpp:284
pub struct EstimateParameters {
	ptr: *mut c_void
}

opencv_type_boxed! { EstimateParameters }

impl Drop for EstimateParameters {
	fn drop(&mut self) {
		extern "C" { fn cv_EstimateParameters_delete(instance: *mut c_void); }
		unsafe { cv_EstimateParameters_delete(self.as_raw_mut_EstimateParameters()) };
	}
}

unsafe impl Send for EstimateParameters {}

impl crate::aruco::EstimateParametersTraitConst for EstimateParameters {
	#[inline] fn as_raw_EstimateParameters(&self) -> *const c_void { self.as_raw() }
}

impl crate::aruco::EstimateParametersTrait for EstimateParameters {
	#[inline] fn as_raw_mut_EstimateParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl EstimateParameters {
	// EstimateParameters() /usr/include/opencv2/aruco.hpp:289
	#[inline]
	pub fn default() -> Result<crate::aruco::EstimateParameters> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_EstimateParameters_EstimateParameters(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::aruco::EstimateParameters::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// create() /usr/include/opencv2/aruco.hpp:292
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::aruco::EstimateParameters>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_EstimateParameters_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::EstimateParameters>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// GridBoard /usr/include/opencv2/aruco.hpp:404
pub trait GridBoardTraitConst: crate::aruco::BoardTraitConst {
	fn as_raw_GridBoard(&self) -> *const c_void;

	// getGridSize() /usr/include/opencv2/aruco.hpp:441
	#[inline]
	fn get_grid_size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_GridBoard_getGridSize_const(self.as_raw_GridBoard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getMarkerLength() /usr/include/opencv2/aruco.hpp:446
	#[inline]
	fn get_marker_length(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_GridBoard_getMarkerLength_const(self.as_raw_GridBoard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getMarkerSeparation() /usr/include/opencv2/aruco.hpp:451
	#[inline]
	fn get_marker_separation(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_GridBoard_getMarkerSeparation_const(self.as_raw_GridBoard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait GridBoardTrait: crate::aruco::BoardTrait + crate::aruco::GridBoardTraitConst {
	fn as_raw_mut_GridBoard(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * margin_size: 0
	/// * border_bits: 1
	// draw(cv::Size, cv::OutputArray, int, int) /usr/include/opencv2/aruco.hpp:418
	#[inline]
	fn draw(&mut self, out_size: core::Size, img: &mut dyn core::ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
		output_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_GridBoard_draw_Size_const__OutputArrayR_int_int(self.as_raw_mut_GridBoard(), out_size.opencv_as_extern(), img.as_raw__OutputArray(), margin_size, border_bits, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// GridBoard /usr/include/opencv2/aruco.hpp:404
pub struct GridBoard {
	ptr: *mut c_void
}

opencv_type_boxed! { GridBoard }

impl Drop for GridBoard {
	fn drop(&mut self) {
		extern "C" { fn cv_GridBoard_delete(instance: *mut c_void); }
		unsafe { cv_GridBoard_delete(self.as_raw_mut_GridBoard()) };
	}
}

unsafe impl Send for GridBoard {}

impl crate::aruco::BoardTraitConst for GridBoard {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.as_raw() }
}

impl crate::aruco::BoardTrait for GridBoard {
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::aruco::GridBoardTraitConst for GridBoard {
	#[inline] fn as_raw_GridBoard(&self) -> *const c_void { self.as_raw() }
}

impl crate::aruco::GridBoardTrait for GridBoard {
	#[inline] fn as_raw_mut_GridBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GridBoard {
	/// ## C++ default parameters
	/// * first_marker: 0
	// create(int, int, float, float, const Ptr<cv::aruco::Dictionary> &, int) /usr/include/opencv2/aruco.hpp:435
	#[inline]
	pub fn create(markers_x: i32, markers_y: i32, marker_length: f32, marker_separation: f32, dictionary: &core::Ptr<crate::aruco::Dictionary>, first_marker: i32) -> Result<core::Ptr<crate::aruco::GridBoard>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_GridBoard_create_int_int_float_float_const_Ptr_Dictionary_R_int(markers_x, markers_y, marker_length, marker_separation, dictionary.as_raw_PtrOfDictionary(), first_marker, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::GridBoard>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { GridBoard, crate::aruco::Board, cv_GridBoard_to_Board }
