#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Phase Unwrapping API
//! 
//! Two-dimensional phase unwrapping is found in different applications like terrain elevation estimation
//! in synthetic aperture radar (SAR), field mapping in magnetic resonance imaging or as a way of finding
//! corresponding pixels in structured light reconstruction with sinusoidal patterns.
//! 
//! Given a phase map, wrapped between [-pi; pi], phase unwrapping aims at finding the "true" phase map
//! by adding the right number of 2*pi to each pixel.
//! 
//! The problem is straightforward for perfect wrapped phase map, but real data are usually not noise-free.
//! Among the different algorithms that were developed, quality-guided phase unwrapping methods are fast
//! and efficient. They follow a path that unwraps high quality pixels first,
//! avoiding error propagation from the start.
//! 
//! In this module, a quality-guided phase unwrapping is implemented following the approach described in [histogramUnwrapping](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_histogramUnwrapping) .
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::PhaseUnwrappingConst, super::PhaseUnwrapping, super::HistogramPhaseUnwrappingConst, super::HistogramPhaseUnwrapping };
}

// HistogramPhaseUnwrapping /usr/include/opencv2/phase_unwrapping/histogramphaseunwrapping.hpp:65
pub trait HistogramPhaseUnwrappingConst: crate::phase_unwrapping::PhaseUnwrappingConst {
	fn as_raw_HistogramPhaseUnwrapping(&self) -> *const c_void;

}

pub trait HistogramPhaseUnwrapping: crate::phase_unwrapping::HistogramPhaseUnwrappingConst + crate::phase_unwrapping::PhaseUnwrapping {
	fn as_raw_mut_HistogramPhaseUnwrapping(&mut self) -> *mut c_void;

	// getInverseReliabilityMap(cv::OutputArray) /usr/include/opencv2/phase_unwrapping/histogramphaseunwrapping.hpp:102
	#[inline]
	fn get_inverse_reliability_map(&mut self, reliability_map: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(reliability_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_getInverseReliabilityMap_const__OutputArrayR(self.as_raw_mut_HistogramPhaseUnwrapping(), reliability_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn HistogramPhaseUnwrapping + '_ {
	/// ## C++ default parameters
	/// * parameters: HistogramPhaseUnwrapping::Params()
	// create(const HistogramPhaseUnwrapping::Params &) /usr/include/opencv2/phase_unwrapping/histogramphaseunwrapping.hpp:93
	#[inline]
	pub fn create(parameters: crate::phase_unwrapping::HistogramPhaseUnwrapping_Params) -> Result<core::Ptr<dyn crate::phase_unwrapping::HistogramPhaseUnwrapping>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_create_const_ParamsR(&parameters, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::phase_unwrapping::HistogramPhaseUnwrapping>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
// Params /usr/include/opencv2/phase_unwrapping/histogramphaseunwrapping.hpp:78
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HistogramPhaseUnwrapping_Params {
	pub width: i32,
	pub height: i32,
	pub hist_thresh: f32,
	pub nbr_of_small_bins: i32,
	pub nbr_of_large_bins: i32,
}

opencv_type_simple! { crate::phase_unwrapping::HistogramPhaseUnwrapping_Params }

impl HistogramPhaseUnwrapping_Params {
	// Params() /usr/include/opencv2/phase_unwrapping/histogramphaseunwrapping.hpp:80
	#[inline]
	pub fn default() -> Result<crate::phase_unwrapping::HistogramPhaseUnwrapping_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// PhaseUnwrapping /usr/include/opencv2/phase_unwrapping/phase_unwrapping.hpp:55
pub trait PhaseUnwrappingConst: core::AlgorithmTraitConst {
	fn as_raw_PhaseUnwrapping(&self) -> *const c_void;

}

pub trait PhaseUnwrapping: core::AlgorithmTrait + crate::phase_unwrapping::PhaseUnwrappingConst {
	fn as_raw_mut_PhaseUnwrapping(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * shadow_mask: noArray()
	// unwrapPhaseMap(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/phase_unwrapping/phase_unwrapping.hpp:66
	#[inline]
	fn unwrap_phase_map(&mut self, wrapped_phase_map: &dyn core::ToInputArray, unwrapped_phase_map: &mut dyn core::ToOutputArray, shadow_mask: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(wrapped_phase_map);
		output_array_arg!(unwrapped_phase_map);
		input_array_arg!(shadow_mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_phase_unwrapping_PhaseUnwrapping_unwrapPhaseMap_const__InputArrayR_const__OutputArrayR_const__InputArrayR(self.as_raw_mut_PhaseUnwrapping(), wrapped_phase_map.as_raw__InputArray(), unwrapped_phase_map.as_raw__OutputArray(), shadow_mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
