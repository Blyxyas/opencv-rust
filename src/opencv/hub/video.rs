#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Video Analysis
//!   # Motion Analysis
//!   # Object Tracking
//!   # C API
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::KalmanFilterTraitConst, super::KalmanFilterTrait, super::DenseOpticalFlowConst, super::DenseOpticalFlow, super::SparseOpticalFlowConst, super::SparseOpticalFlow, super::FarnebackOpticalFlowConst, super::FarnebackOpticalFlow, super::VariationalRefinementConst, super::VariationalRefinement, super::DISOpticalFlowConst, super::DISOpticalFlow, super::SparsePyrLKOpticalFlowConst, super::SparsePyrLKOpticalFlow, super::TrackerConst, super::Tracker, super::TrackerMILConst, super::TrackerMIL, super::TrackerGOTURN_ParamsTraitConst, super::TrackerGOTURN_ParamsTrait, super::TrackerGOTURNConst, super::TrackerGOTURN, super::TrackerDaSiamRPN_ParamsTraitConst, super::TrackerDaSiamRPN_ParamsTrait, super::TrackerDaSiamRPNConst, super::TrackerDaSiamRPN, super::BackgroundSubtractorConst, super::BackgroundSubtractor, super::BackgroundSubtractorMOG2Const, super::BackgroundSubtractorMOG2, super::BackgroundSubtractorKNNConst, super::BackgroundSubtractorKNN };
}

pub const DISOpticalFlow_PRESET_FAST: i32 = 1;
pub const DISOpticalFlow_PRESET_MEDIUM: i32 = 2;
pub const DISOpticalFlow_PRESET_ULTRAFAST: i32 = 0;
pub const MOTION_AFFINE: i32 = 2;
pub const MOTION_EUCLIDEAN: i32 = 1;
pub const MOTION_HOMOGRAPHY: i32 = 3;
pub const MOTION_TRANSLATION: i32 = 0;
pub const OPTFLOW_FARNEBACK_GAUSSIAN: i32 = 256;
pub const OPTFLOW_LK_GET_MIN_EIGENVALS: i32 = 8;
pub const OPTFLOW_USE_INITIAL_FLOW: i32 = 4;
#[inline]
pub fn cam_shift(prob_image: &dyn core::ToInputArray, window: &mut core::Rect, criteria: core::TermCriteria) -> Result<core::RotatedRect> {
	input_array_arg!(prob_image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_CamShift_const__InputArrayR_RectR_TermCriteria(prob_image.as_raw__InputArray(), window, criteria.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::RotatedRect::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * with_derivatives: true
/// * pyr_border: BORDER_REFLECT_101
/// * deriv_border: BORDER_CONSTANT
/// * try_reuse_input_image: true
#[inline]
pub fn build_optical_flow_pyramid(img: &dyn core::ToInputArray, pyramid: &mut dyn core::ToOutputArray, win_size: core::Size, max_level: i32, with_derivatives: bool, pyr_border: i32, deriv_border: i32, try_reuse_input_image: bool) -> Result<i32> {
	input_array_arg!(img);
	output_array_arg!(pyramid);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_buildOpticalFlowPyramid_const__InputArrayR_const__OutputArrayR_Size_int_bool_int_int_bool(img.as_raw__InputArray(), pyramid.as_raw__OutputArray(), win_size.opencv_as_extern(), max_level, with_derivatives, pyr_border, deriv_border, try_reuse_input_image, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn calc_optical_flow_farneback(prev: &dyn core::ToInputArray, next: &dyn core::ToInputArray, flow: &mut dyn core::ToInputOutputArray, pyr_scale: f64, levels: i32, winsize: i32, iterations: i32, poly_n: i32, poly_sigma: f64, flags: i32) -> Result<()> {
	input_array_arg!(prev);
	input_array_arg!(next);
	input_output_array_arg!(flow);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_calcOpticalFlowFarneback_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_double_int_int_int_int_double_int(prev.as_raw__InputArray(), next.as_raw__InputArray(), flow.as_raw__InputOutputArray(), pyr_scale, levels, winsize, iterations, poly_n, poly_sigma, flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * win_size: Size(21,21)
/// * max_level: 3
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,0.01)
/// * flags: 0
/// * min_eig_threshold: 1e-4
#[inline]
pub fn calc_optical_flow_pyr_lk(prev_img: &dyn core::ToInputArray, next_img: &dyn core::ToInputArray, prev_pts: &dyn core::ToInputArray, next_pts: &mut dyn core::ToInputOutputArray, status: &mut dyn core::ToOutputArray, err: &mut dyn core::ToOutputArray, win_size: core::Size, max_level: i32, criteria: core::TermCriteria, flags: i32, min_eig_threshold: f64) -> Result<()> {
	input_array_arg!(prev_img);
	input_array_arg!(next_img);
	input_array_arg!(prev_pts);
	input_output_array_arg!(next_pts);
	output_array_arg!(status);
	output_array_arg!(err);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_calcOpticalFlowPyrLK_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_Size_int_TermCriteria_int_double(prev_img.as_raw__InputArray(), next_img.as_raw__InputArray(), prev_pts.as_raw__InputArray(), next_pts.as_raw__InputOutputArray(), status.as_raw__OutputArray(), err.as_raw__OutputArray(), win_size.opencv_as_extern(), max_level, criteria.opencv_as_extern(), flags, min_eig_threshold, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * input_mask: noArray()
#[inline]
pub fn compute_ecc(template_image: &dyn core::ToInputArray, input_image: &dyn core::ToInputArray, input_mask: &dyn core::ToInputArray) -> Result<f64> {
	input_array_arg!(template_image);
	input_array_arg!(input_image);
	input_array_arg!(input_mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_computeECC_const__InputArrayR_const__InputArrayR_const__InputArrayR(template_image.as_raw__InputArray(), input_image.as_raw__InputArray(), input_mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * history: 500
/// * dist2_threshold: 400.0
/// * detect_shadows: true
#[inline]
pub fn create_background_subtractor_knn(history: i32, dist2_threshold: f64, detect_shadows: bool) -> Result<core::Ptr<dyn crate::video::BackgroundSubtractorKNN>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createBackgroundSubtractorKNN_int_double_bool(history, dist2_threshold, detect_shadows, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::video::BackgroundSubtractorKNN>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * history: 500
/// * var_threshold: 16
/// * detect_shadows: true
#[inline]
pub fn create_background_subtractor_mog2(history: i32, var_threshold: f64, detect_shadows: bool) -> Result<core::Ptr<dyn crate::video::BackgroundSubtractorMOG2>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createBackgroundSubtractorMOG2_int_double_bool(history, var_threshold, detect_shadows, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::video::BackgroundSubtractorMOG2>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn estimate_rigid_transform(src: &dyn core::ToInputArray, dst: &dyn core::ToInputArray, full_affine: bool) -> Result<core::Mat> {
	input_array_arg!(src);
	input_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_estimateRigidTransform_const__InputArrayR_const__InputArrayR_bool(src.as_raw__InputArray(), dst.as_raw__InputArray(), full_affine, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * motion_type: MOTION_AFFINE
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,50,0.001)
/// * input_mask: noArray()
#[inline]
pub fn find_transform_ecc_1(template_image: &dyn core::ToInputArray, input_image: &dyn core::ToInputArray, warp_matrix: &mut dyn core::ToInputOutputArray, motion_type: i32, criteria: core::TermCriteria, input_mask: &dyn core::ToInputArray) -> Result<f64> {
	input_array_arg!(template_image);
	input_array_arg!(input_image);
	input_output_array_arg!(warp_matrix);
	input_array_arg!(input_mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR(template_image.as_raw__InputArray(), input_image.as_raw__InputArray(), warp_matrix.as_raw__InputOutputArray(), motion_type, criteria.opencv_as_extern(), input_mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn find_transform_ecc(template_image: &dyn core::ToInputArray, input_image: &dyn core::ToInputArray, warp_matrix: &mut dyn core::ToInputOutputArray, motion_type: i32, criteria: core::TermCriteria, input_mask: &dyn core::ToInputArray, gauss_filt_size: i32) -> Result<f64> {
	input_array_arg!(template_image);
	input_array_arg!(input_image);
	input_output_array_arg!(warp_matrix);
	input_array_arg!(input_mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR_int(template_image.as_raw__InputArray(), input_image.as_raw__InputArray(), warp_matrix.as_raw__InputOutputArray(), motion_type, criteria.opencv_as_extern(), input_mask.as_raw__InputArray(), gauss_filt_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn mean_shift(prob_image: &dyn core::ToInputArray, window: &mut core::Rect, criteria: core::TermCriteria) -> Result<i32> {
	input_array_arg!(prob_image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_meanShift_const__InputArrayR_RectR_TermCriteria(prob_image.as_raw__InputArray(), window, criteria.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn read_optical_flow(path: &str) -> Result<core::Mat> {
	extern_container_arg!(path);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_readOpticalFlow_const_StringR(path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn write_optical_flow(path: &str, flow: &dyn core::ToInputArray) -> Result<bool> {
	extern_container_arg!(path);
	input_array_arg!(flow);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_writeOpticalFlow_const_StringR_const__InputArrayR(path.opencv_as_extern(), flow.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

pub trait BackgroundSubtractorConst: core::AlgorithmTraitConst {
	fn as_raw_BackgroundSubtractor(&self) -> *const c_void;

	#[inline]
	fn get_background_image(&self, background_image: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(background_image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractor_getBackgroundImage_const_const__OutputArrayR(self.as_raw_BackgroundSubtractor(), background_image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait BackgroundSubtractor: core::AlgorithmTrait + crate::video::BackgroundSubtractorConst {
	fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * learning_rate: -1
	#[inline]
	fn apply(&mut self, image: &dyn core::ToInputArray, fgmask: &mut dyn core::ToOutputArray, learning_rate: f64) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(fgmask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractor_apply_const__InputArrayR_const__OutputArrayR_double(self.as_raw_mut_BackgroundSubtractor(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait BackgroundSubtractorKNNConst: crate::video::BackgroundSubtractorConst {
	fn as_raw_BackgroundSubtractorKNN(&self) -> *const c_void;

	#[inline]
	fn get_history(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_getHistory_const(self.as_raw_BackgroundSubtractorKNN(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_n_samples(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_getNSamples_const(self.as_raw_BackgroundSubtractorKNN(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_dist2_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_getDist2Threshold_const(self.as_raw_BackgroundSubtractorKNN(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn getk_nn_samples(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_getkNNSamples_const(self.as_raw_BackgroundSubtractorKNN(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_detect_shadows(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_getDetectShadows_const(self.as_raw_BackgroundSubtractorKNN(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_shadow_value(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_getShadowValue_const(self.as_raw_BackgroundSubtractorKNN(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_shadow_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_getShadowThreshold_const(self.as_raw_BackgroundSubtractorKNN(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait BackgroundSubtractorKNN: crate::video::BackgroundSubtractor + crate::video::BackgroundSubtractorKNNConst {
	fn as_raw_mut_BackgroundSubtractorKNN(&mut self) -> *mut c_void;

	#[inline]
	fn set_history(&mut self, history: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_setHistory_int(self.as_raw_mut_BackgroundSubtractorKNN(), history, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_n_samples(&mut self, _n_n: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_setNSamples_int(self.as_raw_mut_BackgroundSubtractorKNN(), _n_n, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_dist2_threshold(&mut self, _dist2_threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_setDist2Threshold_double(self.as_raw_mut_BackgroundSubtractorKNN(), _dist2_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn setk_nn_samples(&mut self, _nk_nn: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_setkNNSamples_int(self.as_raw_mut_BackgroundSubtractorKNN(), _nk_nn, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_detect_shadows(&mut self, detect_shadows: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_setDetectShadows_bool(self.as_raw_mut_BackgroundSubtractorKNN(), detect_shadows, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_shadow_value(&mut self, value: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_setShadowValue_int(self.as_raw_mut_BackgroundSubtractorKNN(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_shadow_threshold(&mut self, threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorKNN_setShadowThreshold_double(self.as_raw_mut_BackgroundSubtractorKNN(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait BackgroundSubtractorMOG2Const: crate::video::BackgroundSubtractorConst {
	fn as_raw_BackgroundSubtractorMOG2(&self) -> *const c_void;

	#[inline]
	fn get_history(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getHistory_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_n_mixtures(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getNMixtures_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_background_ratio(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getBackgroundRatio_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_var_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarThreshold_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_var_threshold_gen(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarThresholdGen_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_var_init(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarInit_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_var_min(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarMin_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_var_max(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getVarMax_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_complexity_reduction_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getComplexityReductionThreshold_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_detect_shadows(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getDetectShadows_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_shadow_value(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getShadowValue_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_shadow_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_getShadowThreshold_const(self.as_raw_BackgroundSubtractorMOG2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait BackgroundSubtractorMOG2: crate::video::BackgroundSubtractor + crate::video::BackgroundSubtractorMOG2Const {
	fn as_raw_mut_BackgroundSubtractorMOG2(&mut self) -> *mut c_void;

	#[inline]
	fn set_history(&mut self, history: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setHistory_int(self.as_raw_mut_BackgroundSubtractorMOG2(), history, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_n_mixtures(&mut self, nmixtures: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setNMixtures_int(self.as_raw_mut_BackgroundSubtractorMOG2(), nmixtures, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_background_ratio(&mut self, ratio: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setBackgroundRatio_double(self.as_raw_mut_BackgroundSubtractorMOG2(), ratio, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_var_threshold(&mut self, var_threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarThreshold_double(self.as_raw_mut_BackgroundSubtractorMOG2(), var_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_var_threshold_gen(&mut self, var_threshold_gen: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarThresholdGen_double(self.as_raw_mut_BackgroundSubtractorMOG2(), var_threshold_gen, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_var_init(&mut self, var_init: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarInit_double(self.as_raw_mut_BackgroundSubtractorMOG2(), var_init, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_var_min(&mut self, var_min: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarMin_double(self.as_raw_mut_BackgroundSubtractorMOG2(), var_min, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_var_max(&mut self, var_max: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setVarMax_double(self.as_raw_mut_BackgroundSubtractorMOG2(), var_max, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_complexity_reduction_threshold(&mut self, ct: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setComplexityReductionThreshold_double(self.as_raw_mut_BackgroundSubtractorMOG2(), ct, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_detect_shadows(&mut self, detect_shadows: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setDetectShadows_bool(self.as_raw_mut_BackgroundSubtractorMOG2(), detect_shadows, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_shadow_value(&mut self, value: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setShadowValue_int(self.as_raw_mut_BackgroundSubtractorMOG2(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_shadow_threshold(&mut self, threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_setShadowThreshold_double(self.as_raw_mut_BackgroundSubtractorMOG2(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * learning_rate: -1
	#[inline]
	fn apply(&mut self, image: &dyn core::ToInputArray, fgmask: &mut dyn core::ToOutputArray, learning_rate: f64) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(fgmask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR_double(self.as_raw_mut_BackgroundSubtractorMOG2(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait DISOpticalFlowConst: crate::video::DenseOpticalFlowConst {
	fn as_raw_DISOpticalFlow(&self) -> *const c_void;

	#[inline]
	fn get_finest_scale(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_getFinestScale_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_patch_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_getPatchSize_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_patch_stride(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_getPatchStride_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_gradient_descent_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_getGradientDescentIterations_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_variational_refinement_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_getVariationalRefinementIterations_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_variational_refinement_alpha(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_getVariationalRefinementAlpha_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_variational_refinement_delta(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_getVariationalRefinementDelta_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_variational_refinement_gamma(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_getVariationalRefinementGamma_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_use_mean_normalization(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_getUseMeanNormalization_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_use_spatial_propagation(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_getUseSpatialPropagation_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait DISOpticalFlow: crate::video::DISOpticalFlowConst + crate::video::DenseOpticalFlow {
	fn as_raw_mut_DISOpticalFlow(&mut self) -> *mut c_void;

	#[inline]
	fn set_finest_scale(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_setFinestScale_int(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_patch_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_setPatchSize_int(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_patch_stride(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_setPatchStride_int(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_gradient_descent_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_setGradientDescentIterations_int(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_variational_refinement_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_setVariationalRefinementIterations_int(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_variational_refinement_alpha(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_setVariationalRefinementAlpha_float(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_variational_refinement_delta(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_setVariationalRefinementDelta_float(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_variational_refinement_gamma(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_setVariationalRefinementGamma_float(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_use_mean_normalization(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_setUseMeanNormalization_bool(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_use_spatial_propagation(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_setUseSpatialPropagation_bool(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn DISOpticalFlow + '_ {
	/// ## C++ default parameters
	/// * preset: DISOpticalFlow::PRESET_FAST
	#[inline]
	pub fn create(preset: i32) -> Result<core::Ptr<dyn crate::video::DISOpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DISOpticalFlow_create_int(preset, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::video::DISOpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait DenseOpticalFlowConst: core::AlgorithmTraitConst {
	fn as_raw_DenseOpticalFlow(&self) -> *const c_void;

}

pub trait DenseOpticalFlow: core::AlgorithmTrait + crate::video::DenseOpticalFlowConst {
	fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void;

	#[inline]
	fn calc(&mut self, i0: &dyn core::ToInputArray, i1: &dyn core::ToInputArray, flow: &mut dyn core::ToInputOutputArray) -> Result<()> {
		input_array_arg!(i0);
		input_array_arg!(i1);
		input_output_array_arg!(flow);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DenseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(self.as_raw_mut_DenseOpticalFlow(), i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn collect_garbage(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DenseOpticalFlow_collectGarbage(self.as_raw_mut_DenseOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait FarnebackOpticalFlowConst: crate::video::DenseOpticalFlowConst {
	fn as_raw_FarnebackOpticalFlow(&self) -> *const c_void;

	#[inline]
	fn get_num_levels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_getNumLevels_const(self.as_raw_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_pyr_scale(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_getPyrScale_const(self.as_raw_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_fast_pyramids(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_getFastPyramids_const(self.as_raw_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_win_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_getWinSize_const(self.as_raw_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_num_iters(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_getNumIters_const(self.as_raw_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_poly_n(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_getPolyN_const(self.as_raw_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_poly_sigma(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_getPolySigma_const(self.as_raw_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_flags(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_getFlags_const(self.as_raw_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait FarnebackOpticalFlow: crate::video::DenseOpticalFlow + crate::video::FarnebackOpticalFlowConst {
	fn as_raw_mut_FarnebackOpticalFlow(&mut self) -> *mut c_void;

	#[inline]
	fn set_num_levels(&mut self, num_levels: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_setNumLevels_int(self.as_raw_mut_FarnebackOpticalFlow(), num_levels, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_pyr_scale(&mut self, pyr_scale: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_setPyrScale_double(self.as_raw_mut_FarnebackOpticalFlow(), pyr_scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_fast_pyramids(&mut self, fast_pyramids: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_setFastPyramids_bool(self.as_raw_mut_FarnebackOpticalFlow(), fast_pyramids, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_win_size(&mut self, win_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_setWinSize_int(self.as_raw_mut_FarnebackOpticalFlow(), win_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_num_iters(&mut self, num_iters: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_setNumIters_int(self.as_raw_mut_FarnebackOpticalFlow(), num_iters, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_poly_n(&mut self, poly_n: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_setPolyN_int(self.as_raw_mut_FarnebackOpticalFlow(), poly_n, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_poly_sigma(&mut self, poly_sigma: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_setPolySigma_double(self.as_raw_mut_FarnebackOpticalFlow(), poly_sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_flags(&mut self, flags: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_setFlags_int(self.as_raw_mut_FarnebackOpticalFlow(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn FarnebackOpticalFlow + '_ {
	/// ## C++ default parameters
	/// * num_levels: 5
	/// * pyr_scale: 0.5
	/// * fast_pyramids: false
	/// * win_size: 13
	/// * num_iters: 10
	/// * poly_n: 5
	/// * poly_sigma: 1.1
	/// * flags: 0
	#[inline]
	pub fn create(num_levels: i32, pyr_scale: f64, fast_pyramids: bool, win_size: i32, num_iters: i32, poly_n: i32, poly_sigma: f64, flags: i32) -> Result<core::Ptr<dyn crate::video::FarnebackOpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FarnebackOpticalFlow_create_int_double_bool_int_int_int_double_int(num_levels, pyr_scale, fast_pyramids, win_size, num_iters, poly_n, poly_sigma, flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::video::FarnebackOpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait KalmanFilterTraitConst {
	fn as_raw_KalmanFilter(&self) -> *const c_void;

	#[inline]
	fn state_pre(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_getPropStatePre_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn state_post(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_getPropStatePost_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn transition_matrix(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_getPropTransitionMatrix_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn control_matrix(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_getPropControlMatrix_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn measurement_matrix(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_getPropMeasurementMatrix_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn process_noise_cov(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_getPropProcessNoiseCov_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn measurement_noise_cov(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_getPropMeasurementNoiseCov_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn error_cov_pre(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_getPropErrorCovPre_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn gain(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_getPropGain_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn error_cov_post(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_getPropErrorCovPost_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn temp1(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_getPropTemp1_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn temp2(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_getPropTemp2_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn temp3(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_getPropTemp3_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn temp4(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_getPropTemp4_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn temp5(&self) -> core::Mat {
		let ret = unsafe { sys::cv_KalmanFilter_getPropTemp5_const(self.as_raw_KalmanFilter()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait KalmanFilterTrait: crate::video::KalmanFilterTraitConst {
	fn as_raw_mut_KalmanFilter(&mut self) -> *mut c_void;

	#[inline]
	fn set_state_pre(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_setPropStatePre_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_state_post(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_setPropStatePost_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_transition_matrix(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_setPropTransitionMatrix_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_control_matrix(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_setPropControlMatrix_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_measurement_matrix(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_setPropMeasurementMatrix_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_process_noise_cov(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_setPropProcessNoiseCov_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_measurement_noise_cov(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_setPropMeasurementNoiseCov_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_error_cov_pre(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_setPropErrorCovPre_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_gain(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_setPropGain_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_error_cov_post(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_setPropErrorCovPost_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_temp1(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_setPropTemp1_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_temp2(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_setPropTemp2_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_temp3(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_setPropTemp3_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_temp4(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_setPropTemp4_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_temp5(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_KalmanFilter_setPropTemp5_Mat(self.as_raw_mut_KalmanFilter(), val.as_raw_mut_Mat()) };
		ret
	}
	
	/// ## C++ default parameters
	/// * control_params: 0
	/// * typ: CV_32F
	#[inline]
	fn init(&mut self, dynam_params: i32, measure_params: i32, control_params: i32, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KalmanFilter_init_int_int_int_int(self.as_raw_mut_KalmanFilter(), dynam_params, measure_params, control_params, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * control: Mat()
	#[inline]
	fn predict(&mut self, control: &core::Mat) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KalmanFilter_predict_const_MatR(self.as_raw_mut_KalmanFilter(), control.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn correct(&mut self, measurement: &core::Mat) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KalmanFilter_correct_const_MatR(self.as_raw_mut_KalmanFilter(), measurement.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub struct KalmanFilter {
	ptr: *mut c_void
}

opencv_type_boxed! { KalmanFilter }

impl Drop for KalmanFilter {
	fn drop(&mut self) {
		extern "C" { fn cv_KalmanFilter_delete(instance: *mut c_void); }
		unsafe { cv_KalmanFilter_delete(self.as_raw_mut_KalmanFilter()) };
	}
}

unsafe impl Send for KalmanFilter {}

impl crate::video::KalmanFilterTraitConst for KalmanFilter {
	#[inline] fn as_raw_KalmanFilter(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::KalmanFilterTrait for KalmanFilter {
	#[inline] fn as_raw_mut_KalmanFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl KalmanFilter {
	#[inline]
	pub fn default() -> Result<crate::video::KalmanFilter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KalmanFilter_KalmanFilter(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::video::KalmanFilter::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * control_params: 0
	/// * typ: CV_32F
	#[inline]
	pub fn new(dynam_params: i32, measure_params: i32, control_params: i32, typ: i32) -> Result<crate::video::KalmanFilter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KalmanFilter_KalmanFilter_int_int_int_int(dynam_params, measure_params, control_params, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::video::KalmanFilter::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait SparseOpticalFlowConst: core::AlgorithmTraitConst {
	fn as_raw_SparseOpticalFlow(&self) -> *const c_void;

}

pub trait SparseOpticalFlow: core::AlgorithmTrait + crate::video::SparseOpticalFlowConst {
	fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * err: cv::noArray()
	#[inline]
	fn calc(&mut self, prev_img: &dyn core::ToInputArray, next_img: &dyn core::ToInputArray, prev_pts: &dyn core::ToInputArray, next_pts: &mut dyn core::ToInputOutputArray, status: &mut dyn core::ToOutputArray, err: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(prev_img);
		input_array_arg!(next_img);
		input_array_arg!(prev_pts);
		input_output_array_arg!(next_pts);
		output_array_arg!(status);
		output_array_arg!(err);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_SparseOpticalFlow(), prev_img.as_raw__InputArray(), next_img.as_raw__InputArray(), prev_pts.as_raw__InputArray(), next_pts.as_raw__InputOutputArray(), status.as_raw__OutputArray(), err.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait SparsePyrLKOpticalFlowConst: crate::video::SparseOpticalFlowConst {
	fn as_raw_SparsePyrLKOpticalFlow(&self) -> *const c_void;

	#[inline]
	fn get_win_size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getWinSize_const(self.as_raw_SparsePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_level(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getMaxLevel_const(self.as_raw_SparsePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_term_criteria(&self) -> Result<core::TermCriteria> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getTermCriteria_const(self.as_raw_SparsePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_flags(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getFlags_const(self.as_raw_SparsePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_min_eig_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_getMinEigThreshold_const(self.as_raw_SparsePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait SparsePyrLKOpticalFlow: crate::video::SparseOpticalFlow + crate::video::SparsePyrLKOpticalFlowConst {
	fn as_raw_mut_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void;

	#[inline]
	fn set_win_size(&mut self, win_size: core::Size) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setWinSize_Size(self.as_raw_mut_SparsePyrLKOpticalFlow(), win_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_level(&mut self, max_level: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setMaxLevel_int(self.as_raw_mut_SparsePyrLKOpticalFlow(), max_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_term_criteria(&mut self, crit: &mut core::TermCriteria) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setTermCriteria_TermCriteriaR(self.as_raw_mut_SparsePyrLKOpticalFlow(), crit, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_flags(&mut self, flags: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setFlags_int(self.as_raw_mut_SparsePyrLKOpticalFlow(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_min_eig_threshold(&mut self, min_eig_threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_setMinEigThreshold_double(self.as_raw_mut_SparsePyrLKOpticalFlow(), min_eig_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn SparsePyrLKOpticalFlow + '_ {
	/// ## C++ default parameters
	/// * win_size: Size(21,21)
	/// * max_level: 3
	/// * crit: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,0.01)
	/// * flags: 0
	/// * min_eig_threshold: 1e-4
	#[inline]
	pub fn create(win_size: core::Size, max_level: i32, crit: core::TermCriteria, flags: i32, min_eig_threshold: f64) -> Result<core::Ptr<dyn crate::video::SparsePyrLKOpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SparsePyrLKOpticalFlow_create_Size_int_TermCriteria_int_double(win_size.opencv_as_extern(), max_level, crit.opencv_as_extern(), flags, min_eig_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::video::SparsePyrLKOpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait TrackerConst {
	fn as_raw_Tracker(&self) -> *const c_void;

}

pub trait Tracker: crate::video::TrackerConst {
	fn as_raw_mut_Tracker(&mut self) -> *mut c_void;

	#[inline]
	fn init(&mut self, image: &dyn core::ToInputArray, bounding_box: core::Rect) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Tracker_init_const__InputArrayR_const_RectR(self.as_raw_mut_Tracker(), image.as_raw__InputArray(), &bounding_box, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn update(&mut self, image: &dyn core::ToInputArray, bounding_box: &mut core::Rect) -> Result<bool> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Tracker_update_const__InputArrayR_RectR(self.as_raw_mut_Tracker(), image.as_raw__InputArray(), bounding_box, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait TrackerDaSiamRPNConst: crate::video::TrackerConst {
	fn as_raw_TrackerDaSiamRPN(&self) -> *const c_void;

}

pub trait TrackerDaSiamRPN: crate::video::Tracker + crate::video::TrackerDaSiamRPNConst {
	fn as_raw_mut_TrackerDaSiamRPN(&mut self) -> *mut c_void;

	#[inline]
	fn get_tracking_score(&mut self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerDaSiamRPN_getTrackingScore(self.as_raw_mut_TrackerDaSiamRPN(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn TrackerDaSiamRPN + '_ {
	/// ## C++ default parameters
	/// * parameters: TrackerDaSiamRPN::Params()
	#[inline]
	pub fn create(parameters: &crate::video::TrackerDaSiamRPN_Params) -> Result<core::Ptr<dyn crate::video::TrackerDaSiamRPN>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerDaSiamRPN_create_const_ParamsR(parameters.as_raw_TrackerDaSiamRPN_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::video::TrackerDaSiamRPN>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait TrackerDaSiamRPN_ParamsTraitConst {
	fn as_raw_TrackerDaSiamRPN_Params(&self) -> *const c_void;

	#[inline]
	fn model(&self) -> String {
		let ret = unsafe { sys::cv_TrackerDaSiamRPN_Params_getPropModel_const(self.as_raw_TrackerDaSiamRPN_Params()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn kernel_cls1(&self) -> String {
		let ret = unsafe { sys::cv_TrackerDaSiamRPN_Params_getPropKernel_cls1_const(self.as_raw_TrackerDaSiamRPN_Params()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn kernel_r1(&self) -> String {
		let ret = unsafe { sys::cv_TrackerDaSiamRPN_Params_getPropKernel_r1_const(self.as_raw_TrackerDaSiamRPN_Params()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn backend(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerDaSiamRPN_Params_getPropBackend_const(self.as_raw_TrackerDaSiamRPN_Params()) };
		ret
	}
	
	#[inline]
	fn target(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerDaSiamRPN_Params_getPropTarget_const(self.as_raw_TrackerDaSiamRPN_Params()) };
		ret
	}
	
}

pub trait TrackerDaSiamRPN_ParamsTrait: crate::video::TrackerDaSiamRPN_ParamsTraitConst {
	fn as_raw_mut_TrackerDaSiamRPN_Params(&mut self) -> *mut c_void;

	#[inline]
	fn set_model(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_TrackerDaSiamRPN_Params_setPropModel_string(self.as_raw_mut_TrackerDaSiamRPN_Params(), val.opencv_as_extern_mut()) };
		ret
	}
	
	#[inline]
	fn set_kernel_cls1(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_TrackerDaSiamRPN_Params_setPropKernel_cls1_string(self.as_raw_mut_TrackerDaSiamRPN_Params(), val.opencv_as_extern_mut()) };
		ret
	}
	
	#[inline]
	fn set_kernel_r1(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_TrackerDaSiamRPN_Params_setPropKernel_r1_string(self.as_raw_mut_TrackerDaSiamRPN_Params(), val.opencv_as_extern_mut()) };
		ret
	}
	
	#[inline]
	fn set_backend(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerDaSiamRPN_Params_setPropBackend_int(self.as_raw_mut_TrackerDaSiamRPN_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_target(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerDaSiamRPN_Params_setPropTarget_int(self.as_raw_mut_TrackerDaSiamRPN_Params(), val) };
		ret
	}
	
}

pub struct TrackerDaSiamRPN_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerDaSiamRPN_Params }

impl Drop for TrackerDaSiamRPN_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerDaSiamRPN_Params_delete(instance: *mut c_void); }
		unsafe { cv_TrackerDaSiamRPN_Params_delete(self.as_raw_mut_TrackerDaSiamRPN_Params()) };
	}
}

unsafe impl Send for TrackerDaSiamRPN_Params {}

impl crate::video::TrackerDaSiamRPN_ParamsTraitConst for TrackerDaSiamRPN_Params {
	#[inline] fn as_raw_TrackerDaSiamRPN_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::TrackerDaSiamRPN_ParamsTrait for TrackerDaSiamRPN_Params {
	#[inline] fn as_raw_mut_TrackerDaSiamRPN_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerDaSiamRPN_Params {
	#[inline]
	pub fn default() -> Result<crate::video::TrackerDaSiamRPN_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerDaSiamRPN_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::video::TrackerDaSiamRPN_Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait TrackerGOTURNConst: crate::video::TrackerConst {
	fn as_raw_TrackerGOTURN(&self) -> *const c_void;

}

pub trait TrackerGOTURN: crate::video::Tracker + crate::video::TrackerGOTURNConst {
	fn as_raw_mut_TrackerGOTURN(&mut self) -> *mut c_void;

}

impl dyn TrackerGOTURN + '_ {
	/// ## C++ default parameters
	/// * parameters: TrackerGOTURN::Params()
	#[inline]
	pub fn create(parameters: &crate::video::TrackerGOTURN_Params) -> Result<core::Ptr<dyn crate::video::TrackerGOTURN>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerGOTURN_create_const_ParamsR(parameters.as_raw_TrackerGOTURN_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::video::TrackerGOTURN>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait TrackerGOTURN_ParamsTraitConst {
	fn as_raw_TrackerGOTURN_Params(&self) -> *const c_void;

	#[inline]
	fn model_txt(&self) -> String {
		let ret = unsafe { sys::cv_TrackerGOTURN_Params_getPropModelTxt_const(self.as_raw_TrackerGOTURN_Params()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn model_bin(&self) -> String {
		let ret = unsafe { sys::cv_TrackerGOTURN_Params_getPropModelBin_const(self.as_raw_TrackerGOTURN_Params()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait TrackerGOTURN_ParamsTrait: crate::video::TrackerGOTURN_ParamsTraitConst {
	fn as_raw_mut_TrackerGOTURN_Params(&mut self) -> *mut c_void;

	#[inline]
	fn set_model_txt(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_TrackerGOTURN_Params_setPropModelTxt_string(self.as_raw_mut_TrackerGOTURN_Params(), val.opencv_as_extern_mut()) };
		ret
	}
	
	#[inline]
	fn set_model_bin(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_TrackerGOTURN_Params_setPropModelBin_string(self.as_raw_mut_TrackerGOTURN_Params(), val.opencv_as_extern_mut()) };
		ret
	}
	
}

pub struct TrackerGOTURN_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerGOTURN_Params }

impl Drop for TrackerGOTURN_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerGOTURN_Params_delete(instance: *mut c_void); }
		unsafe { cv_TrackerGOTURN_Params_delete(self.as_raw_mut_TrackerGOTURN_Params()) };
	}
}

unsafe impl Send for TrackerGOTURN_Params {}

impl crate::video::TrackerGOTURN_ParamsTraitConst for TrackerGOTURN_Params {
	#[inline] fn as_raw_TrackerGOTURN_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::TrackerGOTURN_ParamsTrait for TrackerGOTURN_Params {
	#[inline] fn as_raw_mut_TrackerGOTURN_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerGOTURN_Params {
	#[inline]
	pub fn default() -> Result<crate::video::TrackerGOTURN_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerGOTURN_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::video::TrackerGOTURN_Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait TrackerMILConst: crate::video::TrackerConst {
	fn as_raw_TrackerMIL(&self) -> *const c_void;

}

pub trait TrackerMIL: crate::video::Tracker + crate::video::TrackerMILConst {
	fn as_raw_mut_TrackerMIL(&mut self) -> *mut c_void;

}

impl dyn TrackerMIL + '_ {
	/// ## C++ default parameters
	/// * parameters: TrackerMIL::Params()
	#[inline]
	pub fn create(parameters: crate::video::TrackerMIL_Params) -> Result<core::Ptr<dyn crate::video::TrackerMIL>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerMIL_create_const_ParamsR(&parameters, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::video::TrackerMIL>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TrackerMIL_Params {
	pub sampler_init_in_radius: f32,
	pub sampler_init_max_neg_num: i32,
	pub sampler_search_win_size: f32,
	pub sampler_track_in_radius: f32,
	pub sampler_track_max_pos_num: i32,
	pub sampler_track_max_neg_num: i32,
	pub feature_set_num_features: i32,
}

opencv_type_simple! { crate::video::TrackerMIL_Params }

impl TrackerMIL_Params {
	#[inline]
	pub fn default() -> Result<crate::video::TrackerMIL_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerMIL_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait VariationalRefinementConst: crate::video::DenseOpticalFlowConst {
	fn as_raw_VariationalRefinement(&self) -> *const c_void;

	#[inline]
	fn get_fixed_point_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VariationalRefinement_getFixedPointIterations_const(self.as_raw_VariationalRefinement(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_sor_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VariationalRefinement_getSorIterations_const(self.as_raw_VariationalRefinement(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_omega(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VariationalRefinement_getOmega_const(self.as_raw_VariationalRefinement(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_alpha(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VariationalRefinement_getAlpha_const(self.as_raw_VariationalRefinement(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_delta(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VariationalRefinement_getDelta_const(self.as_raw_VariationalRefinement(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_gamma(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VariationalRefinement_getGamma_const(self.as_raw_VariationalRefinement(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait VariationalRefinement: crate::video::DenseOpticalFlow + crate::video::VariationalRefinementConst {
	fn as_raw_mut_VariationalRefinement(&mut self) -> *mut c_void;

	#[inline]
	fn calc_uv(&mut self, i0: &dyn core::ToInputArray, i1: &dyn core::ToInputArray, flow_u: &mut dyn core::ToInputOutputArray, flow_v: &mut dyn core::ToInputOutputArray) -> Result<()> {
		input_array_arg!(i0);
		input_array_arg!(i1);
		input_output_array_arg!(flow_u);
		input_output_array_arg!(flow_v);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VariationalRefinement_calcUV_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_mut_VariationalRefinement(), i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow_u.as_raw__InputOutputArray(), flow_v.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_fixed_point_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VariationalRefinement_setFixedPointIterations_int(self.as_raw_mut_VariationalRefinement(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_sor_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VariationalRefinement_setSorIterations_int(self.as_raw_mut_VariationalRefinement(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_omega(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VariationalRefinement_setOmega_float(self.as_raw_mut_VariationalRefinement(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_alpha(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VariationalRefinement_setAlpha_float(self.as_raw_mut_VariationalRefinement(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_delta(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VariationalRefinement_setDelta_float(self.as_raw_mut_VariationalRefinement(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_gamma(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VariationalRefinement_setGamma_float(self.as_raw_mut_VariationalRefinement(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn VariationalRefinement + '_ {
	#[inline]
	pub fn create() -> Result<core::Ptr<dyn crate::video::VariationalRefinement>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VariationalRefinement_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::video::VariationalRefinement>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}