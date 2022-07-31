#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Structured Light API
//! 
//! Structured light is considered one of the most effective techniques to acquire 3D models.
//! This technique is based on projecting a light pattern and capturing the illuminated scene
//! from one or more points of view. Since the pattern is coded, correspondences between image
//! points and points of the projected pattern can be quickly found and 3D information easily
//! retrieved.
//! 
//! One of the most commonly exploited coding strategies is based on trmatime-multiplexing. In this
//! case, a set of patterns  are successively projected onto the measuring surface.
//! The codeword for a given pixel is usually formed by  the sequence of illuminance values for that
//! pixel across the projected patterns. Thus, the codification is called  temporal because the bits
//! of the codewords are multiplexed in time [pattern](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_pattern) .
//! 
//! In this module a time-multiplexing coding strategy based on Gray encoding is implemented following the
//! (stereo) approach described in 3DUNDERWORLD algorithm [UNDERWORLD](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_UNDERWORLD) .
//! For more details, see @ref tutorial_structured_light.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::StructuredLightPatternConst, super::StructuredLightPattern, super::GrayCodePattern_ParamsTraitConst, super::GrayCodePattern_ParamsTrait, super::GrayCodePatternConst, super::GrayCodePattern, super::SinusoidalPattern_ParamsTraitConst, super::SinusoidalPattern_ParamsTrait, super::SinusoidalPatternConst, super::SinusoidalPattern };
}

// DECODE_3D_UNDERWORLD /usr/include/opencv2/structured_light/structured_light.hpp:56
pub const DECODE_3D_UNDERWORLD: i32 = 0;
// FAPS /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:60
pub const FAPS: i32 = 2;
// FTP /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:58
pub const FTP: i32 = 0;
// PSP /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:59
pub const PSP: i32 = 1;
// GrayCodePattern /usr/include/opencv2/structured_light/graycodepattern.hpp:68
pub trait GrayCodePatternConst: crate::structured_light::StructuredLightPatternConst {
	fn as_raw_GrayCodePattern(&self) -> *const c_void;

	// getNumberOfPatternImages() /usr/include/opencv2/structured_light/graycodepattern.hpp:98
	#[inline]
	fn get_number_of_pattern_images(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_structured_light_GrayCodePattern_getNumberOfPatternImages_const(self.as_raw_GrayCodePattern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getImagesForShadowMasks(cv::InputOutputArray, cv::InputOutputArray) /usr/include/opencv2/structured_light/graycodepattern.hpp:130
	#[inline]
	fn get_images_for_shadow_masks(&self, black_image: &mut dyn core::ToInputOutputArray, white_image: &mut dyn core::ToInputOutputArray) -> Result<()> {
		input_output_array_arg!(black_image);
		input_output_array_arg!(white_image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_structured_light_GrayCodePattern_getImagesForShadowMasks_const_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_GrayCodePattern(), black_image.as_raw__InputOutputArray(), white_image.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getProjPixel(cv::InputArrayOfArrays, int, int, cv::Point &) /usr/include/opencv2/structured_light/graycodepattern.hpp:143
	#[inline]
	fn get_proj_pixel(&self, pattern_images: &dyn core::ToInputArray, x: i32, y: i32, proj_pix: &mut core::Point) -> Result<bool> {
		input_array_arg!(pattern_images);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_structured_light_GrayCodePattern_getProjPixel_const_const__InputArrayR_int_int_PointR(self.as_raw_GrayCodePattern(), pattern_images.as_raw__InputArray(), x, y, proj_pix, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait GrayCodePattern: crate::structured_light::GrayCodePatternConst + crate::structured_light::StructuredLightPattern {
	fn as_raw_mut_GrayCodePattern(&mut self) -> *mut c_void;

	// setWhiteThreshold(size_t) /usr/include/opencv2/structured_light/graycodepattern.hpp:108
	#[inline]
	fn set_white_threshold(&mut self, value: size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_structured_light_GrayCodePattern_setWhiteThreshold_size_t(self.as_raw_mut_GrayCodePattern(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setBlackThreshold(size_t) /usr/include/opencv2/structured_light/graycodepattern.hpp:118
	#[inline]
	fn set_black_threshold(&mut self, value: size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_structured_light_GrayCodePattern_setBlackThreshold_size_t(self.as_raw_mut_GrayCodePattern(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn GrayCodePattern + '_ {
	/// ## C++ default parameters
	/// * parameters: GrayCodePattern::Params()
	// create(const GrayCodePattern::Params &) /usr/include/opencv2/structured_light/graycodepattern.hpp:86
	#[inline]
	pub fn create(parameters: &crate::structured_light::GrayCodePattern_Params) -> Result<core::Ptr<dyn crate::structured_light::GrayCodePattern>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_structured_light_GrayCodePattern_create_const_ParamsR(parameters.as_raw_GrayCodePattern_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::structured_light::GrayCodePattern>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// create(int, int) /usr/include/opencv2/structured_light/graycodepattern.hpp:90
	#[inline]
	pub fn create_1(width: i32, height: i32) -> Result<core::Ptr<dyn crate::structured_light::GrayCodePattern>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_structured_light_GrayCodePattern_create_int_int(width, height, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::structured_light::GrayCodePattern>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
// Params /usr/include/opencv2/structured_light/graycodepattern.hpp:76
pub trait GrayCodePattern_ParamsTraitConst {
	fn as_raw_GrayCodePattern_Params(&self) -> *const c_void;

	// width /usr/include/opencv2/structured_light/graycodepattern.hpp:79
	#[inline]
	fn width(&self) -> i32 {
		let ret = unsafe { sys::cv_structured_light_GrayCodePattern_Params_getPropWidth_const(self.as_raw_GrayCodePattern_Params()) };
		ret
	}
	
	// height /usr/include/opencv2/structured_light/graycodepattern.hpp:80
	#[inline]
	fn height(&self) -> i32 {
		let ret = unsafe { sys::cv_structured_light_GrayCodePattern_Params_getPropHeight_const(self.as_raw_GrayCodePattern_Params()) };
		ret
	}
	
}

pub trait GrayCodePattern_ParamsTrait: crate::structured_light::GrayCodePattern_ParamsTraitConst {
	fn as_raw_mut_GrayCodePattern_Params(&mut self) -> *mut c_void;

	// width /usr/include/opencv2/structured_light/graycodepattern.hpp:79
	#[inline]
	fn set_width(&mut self, val: i32) {
		let ret = unsafe { sys::cv_structured_light_GrayCodePattern_Params_setPropWidth_int(self.as_raw_mut_GrayCodePattern_Params(), val) };
		ret
	}
	
	// height /usr/include/opencv2/structured_light/graycodepattern.hpp:80
	#[inline]
	fn set_height(&mut self, val: i32) {
		let ret = unsafe { sys::cv_structured_light_GrayCodePattern_Params_setPropHeight_int(self.as_raw_mut_GrayCodePattern_Params(), val) };
		ret
	}
	
}

// Params /usr/include/opencv2/structured_light/graycodepattern.hpp:76
pub struct GrayCodePattern_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { GrayCodePattern_Params }

impl Drop for GrayCodePattern_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_GrayCodePattern_Params_delete(instance: *mut c_void); }
		unsafe { cv_GrayCodePattern_Params_delete(self.as_raw_mut_GrayCodePattern_Params()) };
	}
}

unsafe impl Send for GrayCodePattern_Params {}

impl crate::structured_light::GrayCodePattern_ParamsTraitConst for GrayCodePattern_Params {
	#[inline] fn as_raw_GrayCodePattern_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::structured_light::GrayCodePattern_ParamsTrait for GrayCodePattern_Params {
	#[inline] fn as_raw_mut_GrayCodePattern_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GrayCodePattern_Params {
	// Params() /usr/include/opencv2/structured_light/graycodepattern.hpp:78
	#[inline]
	pub fn default() -> Result<crate::structured_light::GrayCodePattern_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_structured_light_GrayCodePattern_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::structured_light::GrayCodePattern_Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// SinusoidalPattern /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:68
pub trait SinusoidalPatternConst: crate::structured_light::StructuredLightPatternConst {
	fn as_raw_SinusoidalPattern(&self) -> *const c_void;

}

pub trait SinusoidalPattern: crate::structured_light::SinusoidalPatternConst + crate::structured_light::StructuredLightPattern {
	fn as_raw_mut_SinusoidalPattern(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * shadow_mask: noArray()
	/// * fundamental: noArray()
	// computePhaseMap(cv::InputArrayOfArrays, cv::OutputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:110
	#[inline]
	fn compute_phase_map(&mut self, pattern_images: &dyn core::ToInputArray, wrapped_phase_map: &mut dyn core::ToOutputArray, shadow_mask: &mut dyn core::ToOutputArray, fundamental: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(pattern_images);
		output_array_arg!(wrapped_phase_map);
		output_array_arg!(shadow_mask);
		input_array_arg!(fundamental);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_structured_light_SinusoidalPattern_computePhaseMap_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR(self.as_raw_mut_SinusoidalPattern(), pattern_images.as_raw__InputArray(), wrapped_phase_map.as_raw__OutputArray(), shadow_mask.as_raw__OutputArray(), fundamental.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * shadow_mask: noArray()
	// unwrapPhaseMap(cv::InputArray, cv::OutputArray, cv::Size, cv::InputArray) /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:122
	#[inline]
	fn unwrap_phase_map(&mut self, wrapped_phase_map: &dyn core::ToInputArray, unwrapped_phase_map: &mut dyn core::ToOutputArray, cam_size: core::Size, shadow_mask: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(wrapped_phase_map);
		output_array_arg!(unwrapped_phase_map);
		input_array_arg!(shadow_mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_structured_light_SinusoidalPattern_unwrapPhaseMap_const__InputArrayR_const__OutputArrayR_Size_const__InputArrayR(self.as_raw_mut_SinusoidalPattern(), wrapped_phase_map.as_raw__InputArray(), unwrapped_phase_map.as_raw__OutputArray(), cam_size.opencv_as_extern(), shadow_mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// findProCamMatches(cv::InputArray, cv::InputArray, cv::OutputArrayOfArrays) /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:133
	#[inline]
	fn find_pro_cam_matches(&mut self, proj_unwrapped_phase_map: &dyn core::ToInputArray, cam_unwrapped_phase_map: &dyn core::ToInputArray, matches: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(proj_unwrapped_phase_map);
		input_array_arg!(cam_unwrapped_phase_map);
		output_array_arg!(matches);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_structured_light_SinusoidalPattern_findProCamMatches_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_SinusoidalPattern(), proj_unwrapped_phase_map.as_raw__InputArray(), cam_unwrapped_phase_map.as_raw__InputArray(), matches.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// computeDataModulationTerm(cv::InputArrayOfArrays, cv::OutputArray, cv::InputArray) /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:143
	#[inline]
	fn compute_data_modulation_term(&mut self, pattern_images: &dyn core::ToInputArray, data_modulation_term: &mut dyn core::ToOutputArray, shadow_mask: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(pattern_images);
		output_array_arg!(data_modulation_term);
		input_array_arg!(shadow_mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_structured_light_SinusoidalPattern_computeDataModulationTerm_const__InputArrayR_const__OutputArrayR_const__InputArrayR(self.as_raw_mut_SinusoidalPattern(), pattern_images.as_raw__InputArray(), data_modulation_term.as_raw__OutputArray(), shadow_mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn SinusoidalPattern + '_ {
	/// ## C++ default parameters
	/// * parameters: makePtr<SinusoidalPattern::Params>()
	// create(Ptr<SinusoidalPattern::Params>) /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:100
	#[inline]
	pub fn create(mut parameters: core::Ptr<crate::structured_light::SinusoidalPattern_Params>) -> Result<core::Ptr<dyn crate::structured_light::SinusoidalPattern>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_structured_light_SinusoidalPattern_create_Ptr_Params_(parameters.as_raw_mut_PtrOfSinusoidalPattern_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::structured_light::SinusoidalPattern>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
// Params /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:82
pub trait SinusoidalPattern_ParamsTraitConst {
	fn as_raw_SinusoidalPattern_Params(&self) -> *const c_void;

	// width /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:85
	#[inline]
	fn width(&self) -> i32 {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_getPropWidth_const(self.as_raw_SinusoidalPattern_Params()) };
		ret
	}
	
	// height /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:86
	#[inline]
	fn height(&self) -> i32 {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_getPropHeight_const(self.as_raw_SinusoidalPattern_Params()) };
		ret
	}
	
	// nbrOfPeriods /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:87
	#[inline]
	fn nbr_of_periods(&self) -> i32 {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_getPropNbrOfPeriods_const(self.as_raw_SinusoidalPattern_Params()) };
		ret
	}
	
	// shiftValue /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:88
	#[inline]
	fn shift_value(&self) -> f32 {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_getPropShiftValue_const(self.as_raw_SinusoidalPattern_Params()) };
		ret
	}
	
	// methodId /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:89
	#[inline]
	fn method_id(&self) -> i32 {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_getPropMethodId_const(self.as_raw_SinusoidalPattern_Params()) };
		ret
	}
	
	// nbrOfPixelsBetweenMarkers /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:90
	#[inline]
	fn nbr_of_pixels_between_markers(&self) -> i32 {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_getPropNbrOfPixelsBetweenMarkers_const(self.as_raw_SinusoidalPattern_Params()) };
		ret
	}
	
	// horizontal /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:91
	#[inline]
	fn horizontal(&self) -> bool {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_getPropHorizontal_const(self.as_raw_SinusoidalPattern_Params()) };
		ret
	}
	
	// setMarkers /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:92
	#[inline]
	fn set_markers(&self) -> bool {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_getPropSetMarkers_const(self.as_raw_SinusoidalPattern_Params()) };
		ret
	}
	
	// markersLocation /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:93
	#[inline]
	fn markers_location(&self) -> core::Vector<core::Point2f> {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_getPropMarkersLocation_const(self.as_raw_SinusoidalPattern_Params()) };
		let ret = unsafe { core::Vector::<core::Point2f>::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait SinusoidalPattern_ParamsTrait: crate::structured_light::SinusoidalPattern_ParamsTraitConst {
	fn as_raw_mut_SinusoidalPattern_Params(&mut self) -> *mut c_void;

	// width /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:85
	#[inline]
	fn set_width(&mut self, val: i32) {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_setPropWidth_int(self.as_raw_mut_SinusoidalPattern_Params(), val) };
		ret
	}
	
	// height /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:86
	#[inline]
	fn set_height(&mut self, val: i32) {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_setPropHeight_int(self.as_raw_mut_SinusoidalPattern_Params(), val) };
		ret
	}
	
	// nbrOfPeriods /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:87
	#[inline]
	fn set_nbr_of_periods(&mut self, val: i32) {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_setPropNbrOfPeriods_int(self.as_raw_mut_SinusoidalPattern_Params(), val) };
		ret
	}
	
	// shiftValue /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:88
	#[inline]
	fn set_shift_value(&mut self, val: f32) {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_setPropShiftValue_float(self.as_raw_mut_SinusoidalPattern_Params(), val) };
		ret
	}
	
	// methodId /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:89
	#[inline]
	fn set_method_id(&mut self, val: i32) {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_setPropMethodId_int(self.as_raw_mut_SinusoidalPattern_Params(), val) };
		ret
	}
	
	// nbrOfPixelsBetweenMarkers /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:90
	#[inline]
	fn set_nbr_of_pixels_between_markers(&mut self, val: i32) {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_setPropNbrOfPixelsBetweenMarkers_int(self.as_raw_mut_SinusoidalPattern_Params(), val) };
		ret
	}
	
	// horizontal /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:91
	#[inline]
	fn set_horizontal(&mut self, val: bool) {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_setPropHorizontal_bool(self.as_raw_mut_SinusoidalPattern_Params(), val) };
		ret
	}
	
	// setMarkers /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:92
	#[inline]
	fn set_set_markers(&mut self, val: bool) {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_setPropSetMarkers_bool(self.as_raw_mut_SinusoidalPattern_Params(), val) };
		ret
	}
	
	// markersLocation /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:93
	#[inline]
	fn set_markers_location(&mut self, mut val: core::Vector<core::Point2f>) {
		let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_setPropMarkersLocation_vector_Point2f_(self.as_raw_mut_SinusoidalPattern_Params(), val.as_raw_mut_VectorOfPoint2f()) };
		ret
	}
	
}

// Params /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:82
pub struct SinusoidalPattern_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { SinusoidalPattern_Params }

impl Drop for SinusoidalPattern_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_SinusoidalPattern_Params_delete(instance: *mut c_void); }
		unsafe { cv_SinusoidalPattern_Params_delete(self.as_raw_mut_SinusoidalPattern_Params()) };
	}
}

unsafe impl Send for SinusoidalPattern_Params {}

impl crate::structured_light::SinusoidalPattern_ParamsTraitConst for SinusoidalPattern_Params {
	#[inline] fn as_raw_SinusoidalPattern_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::structured_light::SinusoidalPattern_ParamsTrait for SinusoidalPattern_Params {
	#[inline] fn as_raw_mut_SinusoidalPattern_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SinusoidalPattern_Params {
	// Params() /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:84
	#[inline]
	pub fn default() -> Result<crate::structured_light::SinusoidalPattern_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_structured_light_SinusoidalPattern_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::structured_light::SinusoidalPattern_Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// StructuredLightPattern /usr/include/opencv2/structured_light/structured_light.hpp:61
pub trait StructuredLightPatternConst: core::AlgorithmTraitConst {
	fn as_raw_StructuredLightPattern(&self) -> *const c_void;

	/// ## C++ default parameters
	/// * black_images: noArray()
	/// * white_images: noArray()
	/// * flags: DECODE_3D_UNDERWORLD
	// decode(const std::vector<std::vector<Mat>> &, cv::OutputArray, cv::InputArrayOfArrays, cv::InputArrayOfArrays, int) /usr/include/opencv2/structured_light/structured_light.hpp:81
	#[inline]
	fn decode(&self, pattern_images: &core::Vector<core::Vector<core::Mat>>, disparity_map: &mut dyn core::ToOutputArray, black_images: &dyn core::ToInputArray, white_images: &dyn core::ToInputArray, flags: i32) -> Result<bool> {
		output_array_arg!(disparity_map);
		input_array_arg!(black_images);
		input_array_arg!(white_images);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_structured_light_StructuredLightPattern_decode_const_const_vector_vector_Mat__R_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(self.as_raw_StructuredLightPattern(), pattern_images.as_raw_VectorOfVectorOfMat(), disparity_map.as_raw__OutputArray(), black_images.as_raw__InputArray(), white_images.as_raw__InputArray(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait StructuredLightPattern: core::AlgorithmTrait + crate::structured_light::StructuredLightPatternConst {
	fn as_raw_mut_StructuredLightPattern(&mut self) -> *mut c_void;

	// generate(cv::OutputArrayOfArrays) /usr/include/opencv2/structured_light/structured_light.hpp:69
	#[inline]
	fn generate(&mut self, pattern_images: &mut dyn core::ToOutputArray) -> Result<bool> {
		output_array_arg!(pattern_images);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_structured_light_StructuredLightPattern_generate_const__OutputArrayR(self.as_raw_mut_StructuredLightPattern(), pattern_images.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
