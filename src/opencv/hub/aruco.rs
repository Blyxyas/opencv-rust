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

pub const CCW_center: i32 = 0;
pub const CORNER_REFINE_APRILTAG: i32 = 3;
pub const CORNER_REFINE_CONTOUR: i32 = 2;
pub const CORNER_REFINE_NONE: i32 = 0;
pub const CORNER_REFINE_SUBPIX: i32 = 1;
pub const CW_top_left_corner: i32 = 1;
pub const DICT_4X4_100: i32 = 1;
pub const DICT_4X4_1000: i32 = 3;
pub const DICT_4X4_250: i32 = 2;
pub const DICT_4X4_50: i32 = 0;
pub const DICT_5X5_100: i32 = 5;
pub const DICT_5X5_1000: i32 = 7;
pub const DICT_5X5_250: i32 = 6;
pub const DICT_5X5_50: i32 = 4;
pub const DICT_6X6_100: i32 = 9;
pub const DICT_6X6_1000: i32 = 11;
pub const DICT_6X6_250: i32 = 10;
pub const DICT_6X6_50: i32 = 8;
pub const DICT_7X7_100: i32 = 13;
pub const DICT_7X7_1000: i32 = 15;
pub const DICT_7X7_250: i32 = 14;
pub const DICT_7X7_50: i32 = 12;
pub const DICT_APRILTAG_16h5: i32 = 17;
pub const DICT_APRILTAG_25h9: i32 = 18;
pub const DICT_APRILTAG_36h10: i32 = 19;
pub const DICT_APRILTAG_36h11: i32 = 20;
pub const DICT_ARUCO_ORIGINAL: i32 = 16;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CornerRefineMethod {
	CORNER_REFINE_NONE = 0,
	CORNER_REFINE_SUBPIX = 1,
	CORNER_REFINE_CONTOUR = 2,
	CORNER_REFINE_APRILTAG = 3,
}

opencv_type_enum! { crate::aruco::CornerRefineMethod }

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
#[inline]
pub fn custom_dictionary(n_markers: i32, marker_size: i32, random_seed: i32) -> Result<core::Ptr<crate::aruco::Dictionary>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_generateCustomDictionary_int_int_int(n_markers, marker_size, random_seed, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
	Ok(ret)
}

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

#[inline]
pub fn get_predefined_dictionary(name: crate::aruco::PREDEFINED_DICTIONARY_NAME) -> Result<core::Ptr<crate::aruco::Dictionary>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_getPredefinedDictionary_PREDEFINED_DICTIONARY_NAME(name, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
	Ok(ret)
}

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

#[inline]
pub fn test_charuco_corners_collinear(_board: &core::Ptr<crate::aruco::CharucoBoard>, _charuco_ids: &dyn core::ToInputArray) -> Result<bool> {
	input_array_arg!(_charuco_ids);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_testCharucoCornersCollinear_const_Ptr_CharucoBoard_R_const__InputArrayR(_board.as_raw_PtrOfCharucoBoard(), _charuco_ids.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

pub trait BoardTraitConst {
	fn as_raw_Board(&self) -> *const c_void;

	#[inline]
	fn obj_points(&self) -> core::Vector<core::Vector<core::Point3f>> {
		let ret = unsafe { sys::cv_aruco_Board_getPropObjPoints_const(self.as_raw_Board()) };
		let ret = unsafe { core::Vector::<core::Vector<core::Point3f>>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn ids(&self) -> core::Vector<i32> {
		let ret = unsafe { sys::cv_aruco_Board_getPropIds_const(self.as_raw_Board()) };
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		ret
	}
	
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

	#[inline]
	fn set_obj_points(&mut self, mut val: core::Vector<core::Vector<core::Point3f>>) {
		let ret = unsafe { sys::cv_aruco_Board_setPropObjPoints_vector_vector_Point3f__(self.as_raw_mut_Board(), val.as_raw_mut_VectorOfVectorOfPoint3f()) };
		ret
	}
	
	#[inline]
	fn dictionary(&mut self) -> core::Ptr<crate::aruco::Dictionary> {
		let ret = unsafe { sys::cv_aruco_Board_getPropDictionary(self.as_raw_mut_Board()) };
		let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn set_dictionary(&mut self, mut val: core::Ptr<crate::aruco::Dictionary>) {
		let ret = unsafe { sys::cv_aruco_Board_setPropDictionary_Ptr_Dictionary_(self.as_raw_mut_Board(), val.as_raw_mut_PtrOfDictionary()) };
		ret
	}
	
	#[inline]
	fn set_ids(&mut self, mut val: core::Vector<i32>) {
		let ret = unsafe { sys::cv_aruco_Board_setPropIds_vector_int_(self.as_raw_mut_Board(), val.as_raw_mut_VectorOfi32()) };
		ret
	}
	
	#[inline]
	fn set_right_bottom_border(&mut self, val: core::Point3f) {
		let ret = unsafe { sys::cv_aruco_Board_setPropRightBottomBorder_Point3f(self.as_raw_mut_Board(), val.opencv_as_extern()) };
		ret
	}
	
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

pub trait CharucoBoardTraitConst: crate::aruco::BoardTraitConst {
	fn as_raw_CharucoBoard(&self) -> *const c_void;

	#[inline]
	fn chessboard_corners(&self) -> core::Vector<core::Point3f> {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_getPropChessboardCorners_const(self.as_raw_CharucoBoard()) };
		let ret = unsafe { core::Vector::<core::Point3f>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn nearest_marker_idx(&self) -> core::Vector<core::Vector<i32>> {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_getPropNearestMarkerIdx_const(self.as_raw_CharucoBoard()) };
		let ret = unsafe { core::Vector::<core::Vector<i32>>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn nearest_marker_corners(&self) -> core::Vector<core::Vector<i32>> {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_getPropNearestMarkerCorners_const(self.as_raw_CharucoBoard()) };
		let ret = unsafe { core::Vector::<core::Vector<i32>>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn get_chessboard_size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_CharucoBoard_getChessboardSize_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_square_length(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_CharucoBoard_getSquareLength_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
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

	#[inline]
	fn set_chessboard_corners(&mut self, mut val: core::Vector<core::Point3f>) {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_setPropChessboardCorners_vector_Point3f_(self.as_raw_mut_CharucoBoard(), val.as_raw_mut_VectorOfPoint3f()) };
		ret
	}
	
	#[inline]
	fn set_nearest_marker_idx(&mut self, mut val: core::Vector<core::Vector<i32>>) {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_setPropNearestMarkerIdx_vector_vector_int__(self.as_raw_mut_CharucoBoard(), val.as_raw_mut_VectorOfVectorOfi32()) };
		ret
	}
	
	#[inline]
	fn set_nearest_marker_corners(&mut self, mut val: core::Vector<core::Vector<i32>>) {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_setPropNearestMarkerCorners_vector_vector_int__(self.as_raw_mut_CharucoBoard(), val.as_raw_mut_VectorOfVectorOfi32()) };
		ret
	}
	
	/// ## C++ default parameters
	/// * margin_size: 0
	/// * border_bits: 1
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

pub trait DetectorParametersTraitConst {
	fn as_raw_DetectorParameters(&self) -> *const c_void;

	#[inline]
	fn adaptive_thresh_win_size_min(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeMin_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn adaptive_thresh_win_size_max(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeMax_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn adaptive_thresh_win_size_step(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeStep_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn adaptive_thresh_constant(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAdaptiveThreshConstant_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn min_marker_perimeter_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMinMarkerPerimeterRate_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn max_marker_perimeter_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMaxMarkerPerimeterRate_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn polygonal_approx_accuracy_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropPolygonalApproxAccuracyRate_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn min_corner_distance_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMinCornerDistanceRate_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn min_distance_to_border(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMinDistanceToBorder_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn min_marker_distance_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMinMarkerDistanceRate_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn corner_refinement_method(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropCornerRefinementMethod_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn corner_refinement_win_size(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropCornerRefinementWinSize_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn corner_refinement_max_iterations(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropCornerRefinementMaxIterations_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn corner_refinement_min_accuracy(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropCornerRefinementMinAccuracy_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn marker_border_bits(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMarkerBorderBits_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn perspective_remove_pixel_per_cell(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropPerspectiveRemovePixelPerCell_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn perspective_remove_ignored_margin_per_cell(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropPerspectiveRemoveIgnoredMarginPerCell_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn max_erroneous_bits_in_border_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMaxErroneousBitsInBorderRate_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn min_otsu_std_dev(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMinOtsuStdDev_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn error_correction_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropErrorCorrectionRate_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn april_tag_quad_decimate(&self) -> f32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagQuadDecimate_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn april_tag_quad_sigma(&self) -> f32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagQuadSigma_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn april_tag_min_cluster_pixels(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagMinClusterPixels_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn april_tag_max_nmaxima(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagMaxNmaxima_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn april_tag_critical_rad(&self) -> f32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagCriticalRad_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn april_tag_max_line_fit_mse(&self) -> f32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagMaxLineFitMse_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn april_tag_min_white_black_diff(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagMinWhiteBlackDiff_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn april_tag_deglitch(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagDeglitch_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn detect_inverted_marker(&self) -> bool {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropDetectInvertedMarker_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn use_aruco3_detection(&self) -> bool {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropUseAruco3Detection_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn min_side_length_canonical_img(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMinSideLengthCanonicalImg_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
	#[inline]
	fn min_marker_length_ratio_original_img(&self) -> f32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_getPropMinMarkerLengthRatioOriginalImg_const(self.as_raw_DetectorParameters()) };
		ret
	}
	
}

pub trait DetectorParametersTrait: crate::aruco::DetectorParametersTraitConst {
	fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void;

	#[inline]
	fn set_adaptive_thresh_win_size_min(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeMin_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_adaptive_thresh_win_size_max(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeMax_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_adaptive_thresh_win_size_step(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeStep_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_adaptive_thresh_constant(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAdaptiveThreshConstant_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_min_marker_perimeter_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMinMarkerPerimeterRate_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_max_marker_perimeter_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMaxMarkerPerimeterRate_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_polygonal_approx_accuracy_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropPolygonalApproxAccuracyRate_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_min_corner_distance_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMinCornerDistanceRate_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_min_distance_to_border(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMinDistanceToBorder_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_min_marker_distance_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMinMarkerDistanceRate_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_corner_refinement_method(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropCornerRefinementMethod_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_corner_refinement_win_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropCornerRefinementWinSize_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_corner_refinement_max_iterations(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropCornerRefinementMaxIterations_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_corner_refinement_min_accuracy(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropCornerRefinementMinAccuracy_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_marker_border_bits(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMarkerBorderBits_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_perspective_remove_pixel_per_cell(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropPerspectiveRemovePixelPerCell_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_perspective_remove_ignored_margin_per_cell(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropPerspectiveRemoveIgnoredMarginPerCell_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_max_erroneous_bits_in_border_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMaxErroneousBitsInBorderRate_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_min_otsu_std_dev(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMinOtsuStdDev_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_error_correction_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropErrorCorrectionRate_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_april_tag_quad_decimate(&mut self, val: f32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagQuadDecimate_float(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_april_tag_quad_sigma(&mut self, val: f32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagQuadSigma_float(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_april_tag_min_cluster_pixels(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagMinClusterPixels_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_april_tag_max_nmaxima(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagMaxNmaxima_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_april_tag_critical_rad(&mut self, val: f32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagCriticalRad_float(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_april_tag_max_line_fit_mse(&mut self, val: f32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagMaxLineFitMse_float(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_april_tag_min_white_black_diff(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagMinWhiteBlackDiff_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_april_tag_deglitch(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagDeglitch_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_detect_inverted_marker(&mut self, val: bool) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropDetectInvertedMarker_bool(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_use_aruco3_detection(&mut self, val: bool) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropUseAruco3Detection_bool(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_min_side_length_canonical_img(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMinSideLengthCanonicalImg_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_min_marker_length_ratio_original_img(&mut self, val: f32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_setPropMinMarkerLengthRatioOriginalImg_float(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}
	
	#[inline]
	fn read_detector_parameters(&mut self, fn_: &core::FileNode) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_DetectorParameters_readDetectorParameters_const_FileNodeR(self.as_raw_mut_DetectorParameters(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

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
	#[inline]
	pub fn default() -> Result<crate::aruco::DetectorParameters> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_DetectorParameters_DetectorParameters(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::aruco::DetectorParameters::opencv_from_extern(ret) };
		Ok(ret)
	}
	
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

pub trait DictionaryTraitConst {
	fn as_raw_Dictionary(&self) -> *const c_void;

	#[inline]
	fn bytes_list(&self) -> core::Mat {
		let ret = unsafe { sys::cv_aruco_Dictionary_getPropBytesList_const(self.as_raw_Dictionary()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn marker_size(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_Dictionary_getPropMarkerSize_const(self.as_raw_Dictionary()) };
		ret
	}
	
	#[inline]
	fn max_correction_bits(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_Dictionary_getPropMaxCorrectionBits_const(self.as_raw_Dictionary()) };
		ret
	}
	
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

	#[inline]
	fn set_bytes_list(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_aruco_Dictionary_setPropBytesList_Mat(self.as_raw_mut_Dictionary(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_marker_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_Dictionary_setPropMarkerSize_int(self.as_raw_mut_Dictionary(), val) };
		ret
	}
	
	#[inline]
	fn set_max_correction_bits(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_Dictionary_setPropMaxCorrectionBits_int(self.as_raw_mut_Dictionary(), val) };
		ret
	}
	
	#[inline]
	fn read_dictionary(&mut self, fn_: &core::FileNode) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_readDictionary_const_FileNodeR(self.as_raw_mut_Dictionary(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn write_dictionary(&mut self, fs: &mut core::Ptr<core::FileStorage>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_writeDictionary_Ptr_FileStorage_R(self.as_raw_mut_Dictionary(), fs.as_raw_mut_PtrOfFileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

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
	#[inline]
	pub fn new(_bytes_list: &core::Mat, _marker_size: i32, _maxcorr: i32) -> Result<crate::aruco::Dictionary> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_Dictionary_const_MatR_int_int(_bytes_list.as_raw_Mat(), _marker_size, _maxcorr, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::aruco::Dictionary::opencv_from_extern(ret) };
		Ok(ret)
	}
	
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
	#[inline]
	pub fn create_from(n_markers: i32, marker_size: i32, base_dictionary: &core::Ptr<crate::aruco::Dictionary>, random_seed: i32) -> Result<core::Ptr<crate::aruco::Dictionary>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_create_int_int_const_Ptr_Dictionary_R_int(n_markers, marker_size, base_dictionary.as_raw_PtrOfDictionary(), random_seed, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn get(dict: i32) -> Result<core::Ptr<crate::aruco::Dictionary>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_get_int(dict, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn get_byte_list_from_bits(bits: &core::Mat) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_getByteListFromBits_const_MatR(bits.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
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

pub trait EstimateParametersTraitConst {
	fn as_raw_EstimateParameters(&self) -> *const c_void;

	#[inline]
	fn pattern(&self) -> crate::aruco::PatternPos {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_EstimateParameters_getPropPattern_const(self.as_raw_EstimateParameters(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn use_extrinsic_guess(&self) -> bool {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_getPropUseExtrinsicGuess_const(self.as_raw_EstimateParameters()) };
		ret
	}
	
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

	#[inline]
	fn set_pattern(&mut self, val: crate::aruco::PatternPos) {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_setPropPattern_PatternPos(self.as_raw_mut_EstimateParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_use_extrinsic_guess(&mut self, val: bool) {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_setPropUseExtrinsicGuess_bool(self.as_raw_mut_EstimateParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_solve_pnp_method(&mut self, val: crate::calib3d::SolvePnPMethod) {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_setPropSolvePnPMethod_SolvePnPMethod(self.as_raw_mut_EstimateParameters(), val) };
		ret
	}
	
}

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
	#[inline]
	pub fn default() -> Result<crate::aruco::EstimateParameters> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_EstimateParameters_EstimateParameters(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::aruco::EstimateParameters::opencv_from_extern(ret) };
		Ok(ret)
	}
	
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

pub trait GridBoardTraitConst: crate::aruco::BoardTraitConst {
	fn as_raw_GridBoard(&self) -> *const c_void;

	#[inline]
	fn get_grid_size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_GridBoard_getGridSize_const(self.as_raw_GridBoard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_marker_length(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_GridBoard_getMarkerLength_const(self.as_raw_GridBoard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
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
