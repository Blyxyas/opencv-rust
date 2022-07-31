#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Super Resolution
//! 
//! The Super Resolution module contains a set of functions and classes that can be used to solve the
//! problem of resolution enhancement. There are a few methods implemented, most of them are described in
//! the papers [Farsiu03](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Farsiu03) and [Mitzel09](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Mitzel09) .
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::Superres_DenseOpticalFlowExtConst, super::Superres_DenseOpticalFlowExt, super::Superres_FarnebackOpticalFlowConst, super::Superres_FarnebackOpticalFlow, super::Superres_DualTVL1OpticalFlowConst, super::Superres_DualTVL1OpticalFlow, super::Superres_BroxOpticalFlowConst, super::Superres_BroxOpticalFlow, super::Superres_PyrLKOpticalFlowConst, super::Superres_PyrLKOpticalFlow, super::Superres_FrameSourceConst, super::Superres_FrameSource, super::Superres_SuperResolutionConst, super::Superres_SuperResolution };
}

/// ## C++ default parameters
/// * device_id: 0
// createFrameSource_Camera(int) /usr/include/opencv2/superres.hpp:80
#[inline]
pub fn create_frame_source_camera(device_id: i32) -> Result<core::Ptr<dyn crate::superres::Superres_FrameSource>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createFrameSource_Camera_int(device_id, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::superres::Superres_FrameSource>::opencv_from_extern(ret) };
	Ok(ret)
}

// createFrameSource_Empty() /usr/include/opencv2/superres.hpp:75
#[inline]
pub fn create_frame_source_empty() -> Result<core::Ptr<dyn crate::superres::Superres_FrameSource>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createFrameSource_Empty(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::superres::Superres_FrameSource>::opencv_from_extern(ret) };
	Ok(ret)
}

// createFrameSource_Video_CUDA(const cv::String &) /usr/include/opencv2/superres.hpp:78
#[inline]
pub fn create_frame_source_video_cuda(file_name: &str) -> Result<core::Ptr<dyn crate::superres::Superres_FrameSource>> {
	extern_container_arg!(file_name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createFrameSource_Video_CUDA_const_StringR(file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::superres::Superres_FrameSource>::opencv_from_extern(ret) };
	Ok(ret)
}

// createFrameSource_Video(const cv::String &) /usr/include/opencv2/superres.hpp:77
#[inline]
pub fn create_frame_source_video(file_name: &str) -> Result<core::Ptr<dyn crate::superres::Superres_FrameSource>> {
	extern_container_arg!(file_name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createFrameSource_Video_const_StringR(file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::superres::Superres_FrameSource>::opencv_from_extern(ret) };
	Ok(ret)
}

// createOptFlow_Brox_CUDA() /usr/include/opencv2/superres/optical_flow.hpp:177
#[inline]
pub fn create_opt_flow_brox_cuda() -> Result<core::Ptr<dyn crate::superres::Superres_BroxOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createOptFlow_Brox_CUDA(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::superres::Superres_BroxOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

// createOptFlow_DualTVL1() /usr/include/opencv2/superres/optical_flow.hpp:139
#[inline]
pub fn create_opt_flow_dual_tvl1() -> Result<core::Ptr<dyn crate::superres::Superres_DualTVL1OpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createOptFlow_DualTVL1(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::superres::Superres_DualTVL1OpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

// createOptFlow_DualTVL1_CUDA() /usr/include/opencv2/superres/optical_flow.hpp:140
#[inline]
pub fn create_opt_flow_dual_tvl1_cuda() -> Result<core::Ptr<dyn crate::superres::Superres_DualTVL1OpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createOptFlow_DualTVL1_CUDA(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::superres::Superres_DualTVL1OpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

// createOptFlow_Farneback() /usr/include/opencv2/superres/optical_flow.hpp:96
#[inline]
pub fn create_opt_flow_farneback() -> Result<core::Ptr<dyn crate::superres::Superres_FarnebackOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createOptFlow_Farneback(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::superres::Superres_FarnebackOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

// createOptFlow_Farneback_CUDA() /usr/include/opencv2/superres/optical_flow.hpp:97
#[inline]
pub fn create_opt_flow_farneback_cuda() -> Result<core::Ptr<dyn crate::superres::Superres_FarnebackOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createOptFlow_Farneback_CUDA(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::superres::Superres_FarnebackOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

// createOptFlow_PyrLK_CUDA() /usr/include/opencv2/superres/optical_flow.hpp:196
#[inline]
pub fn create_opt_flow_pyr_lk_cuda() -> Result<core::Ptr<dyn crate::superres::Superres_PyrLKOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createOptFlow_PyrLK_CUDA(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::superres::Superres_PyrLKOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Create Bilateral TV-L1 Super Resolution.
/// 
/// This class implements Super Resolution algorithm described in the papers [Farsiu03](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Farsiu03) and
/// [Mitzel09](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Mitzel09) .
/// 
/// Here are important members of the class that control the algorithm, which you can set after
/// constructing the class instance:
/// 
/// *   **int scale** Scale factor.
/// *   **int iterations** Iteration count.
/// *   **double tau** Asymptotic value of steepest descent method.
/// *   **double lambda** Weight parameter to balance data term and smoothness term.
/// *   **double alpha** Parameter of spacial distribution in Bilateral-TV.
/// *   **int btvKernelSize** Kernel size of Bilateral-TV filter.
/// *   **int blurKernelSize** Gaussian blur kernel size.
/// *   **double blurSigma** Gaussian blur sigma.
/// *   **int temporalAreaRadius** Radius of the temporal search area.
/// *   **Ptr\<DenseOpticalFlowExt\> opticalFlow** Dense optical flow algorithm.
// createSuperResolution_BTVL1() /usr/include/opencv2/superres.hpp:199
#[inline]
pub fn create_super_resolution_btvl1() -> Result<core::Ptr<dyn crate::superres::Superres_SuperResolution>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createSuperResolution_BTVL1(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::superres::Superres_SuperResolution>::opencv_from_extern(ret) };
	Ok(ret)
}

// createSuperResolution_BTVL1_CUDA() /usr/include/opencv2/superres.hpp:200
#[inline]
pub fn create_super_resolution_btvl1_cuda() -> Result<core::Ptr<dyn crate::superres::Superres_SuperResolution>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createSuperResolution_BTVL1_CUDA(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::superres::Superres_SuperResolution>::opencv_from_extern(ret) };
	Ok(ret)
}

// BroxOpticalFlow /usr/include/opencv2/superres/optical_flow.hpp:143
pub trait Superres_BroxOpticalFlowConst: crate::superres::Superres_DenseOpticalFlowExtConst {
	fn as_raw_Superres_BroxOpticalFlow(&self) -> *const c_void;

	// getAlpha() /usr/include/opencv2/superres/optical_flow.hpp:148
	#[inline]
	fn get_alpha(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_getAlpha_const(self.as_raw_Superres_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getGamma() /usr/include/opencv2/superres/optical_flow.hpp:153
	#[inline]
	fn get_gamma(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_getGamma_const(self.as_raw_Superres_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getScaleFactor() /usr/include/opencv2/superres/optical_flow.hpp:158
	#[inline]
	fn get_scale_factor(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_getScaleFactor_const(self.as_raw_Superres_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getInnerIterations() /usr/include/opencv2/superres/optical_flow.hpp:163
	#[inline]
	fn get_inner_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_getInnerIterations_const(self.as_raw_Superres_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getOuterIterations() /usr/include/opencv2/superres/optical_flow.hpp:168
	#[inline]
	fn get_outer_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_getOuterIterations_const(self.as_raw_Superres_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSolverIterations() /usr/include/opencv2/superres/optical_flow.hpp:173
	#[inline]
	fn get_solver_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_getSolverIterations_const(self.as_raw_Superres_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Superres_BroxOpticalFlow: crate::superres::Superres_BroxOpticalFlowConst + crate::superres::Superres_DenseOpticalFlowExt {
	fn as_raw_mut_Superres_BroxOpticalFlow(&mut self) -> *mut c_void;

	// setAlpha(double) /usr/include/opencv2/superres/optical_flow.hpp:150
	#[inline]
	fn set_alpha(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_setAlpha_double(self.as_raw_mut_Superres_BroxOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setGamma(double) /usr/include/opencv2/superres/optical_flow.hpp:155
	#[inline]
	fn set_gamma(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_setGamma_double(self.as_raw_mut_Superres_BroxOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setScaleFactor(double) /usr/include/opencv2/superres/optical_flow.hpp:160
	#[inline]
	fn set_scale_factor(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_setScaleFactor_double(self.as_raw_mut_Superres_BroxOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setInnerIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:165
	#[inline]
	fn set_inner_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_setInnerIterations_int(self.as_raw_mut_Superres_BroxOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setOuterIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:170
	#[inline]
	fn set_outer_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_setOuterIterations_int(self.as_raw_mut_Superres_BroxOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setSolverIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:175
	#[inline]
	fn set_solver_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_setSolverIterations_int(self.as_raw_mut_Superres_BroxOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// DenseOpticalFlowExt /usr/include/opencv2/superres/optical_flow.hpp:56
pub trait Superres_DenseOpticalFlowExtConst: core::AlgorithmTraitConst {
	fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void;

}

pub trait Superres_DenseOpticalFlowExt: core::AlgorithmTrait + crate::superres::Superres_DenseOpticalFlowExtConst {
	fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * flow2: noArray()
	// calc(cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/superres/optical_flow.hpp:59
	#[inline]
	fn calc(&mut self, frame0: &dyn core::ToInputArray, frame1: &dyn core::ToInputArray, flow1: &mut dyn core::ToOutputArray, flow2: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(frame0);
		input_array_arg!(frame1);
		output_array_arg!(flow1);
		output_array_arg!(flow2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DenseOpticalFlowExt_calc_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Superres_DenseOpticalFlowExt(), frame0.as_raw__InputArray(), frame1.as_raw__InputArray(), flow1.as_raw__OutputArray(), flow2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// collectGarbage() /usr/include/opencv2/superres/optical_flow.hpp:60
	#[inline]
	fn collect_garbage(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DenseOpticalFlowExt_collectGarbage(self.as_raw_mut_Superres_DenseOpticalFlowExt(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// DualTVL1OpticalFlow /usr/include/opencv2/superres/optical_flow.hpp:103
pub trait Superres_DualTVL1OpticalFlowConst: crate::superres::Superres_DenseOpticalFlowExtConst {
	fn as_raw_Superres_DualTVL1OpticalFlow(&self) -> *const c_void;

	// getTau() /usr/include/opencv2/superres/optical_flow.hpp:107
	#[inline]
	fn get_tau(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_getTau_const(self.as_raw_Superres_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getLambda() /usr/include/opencv2/superres/optical_flow.hpp:111
	#[inline]
	fn get_lambda(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_getLambda_const(self.as_raw_Superres_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getTheta() /usr/include/opencv2/superres/optical_flow.hpp:115
	#[inline]
	fn get_theta(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_getTheta_const(self.as_raw_Superres_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getScalesNumber() /usr/include/opencv2/superres/optical_flow.hpp:119
	#[inline]
	fn get_scales_number(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_getScalesNumber_const(self.as_raw_Superres_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getWarpingsNumber() /usr/include/opencv2/superres/optical_flow.hpp:123
	#[inline]
	fn get_warpings_number(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_getWarpingsNumber_const(self.as_raw_Superres_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getEpsilon() /usr/include/opencv2/superres/optical_flow.hpp:127
	#[inline]
	fn get_epsilon(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_getEpsilon_const(self.as_raw_Superres_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getIterations() /usr/include/opencv2/superres/optical_flow.hpp:131
	#[inline]
	fn get_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_getIterations_const(self.as_raw_Superres_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getUseInitialFlow() /usr/include/opencv2/superres/optical_flow.hpp:135
	#[inline]
	fn get_use_initial_flow(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_getUseInitialFlow_const(self.as_raw_Superres_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Superres_DualTVL1OpticalFlow: crate::superres::Superres_DenseOpticalFlowExt + crate::superres::Superres_DualTVL1OpticalFlowConst {
	fn as_raw_mut_Superres_DualTVL1OpticalFlow(&mut self) -> *mut c_void;

	// setTau(double) /usr/include/opencv2/superres/optical_flow.hpp:109
	#[inline]
	fn set_tau(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_setTau_double(self.as_raw_mut_Superres_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setLambda(double) /usr/include/opencv2/superres/optical_flow.hpp:113
	#[inline]
	fn set_lambda(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_setLambda_double(self.as_raw_mut_Superres_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setTheta(double) /usr/include/opencv2/superres/optical_flow.hpp:117
	#[inline]
	fn set_theta(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_setTheta_double(self.as_raw_mut_Superres_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setScalesNumber(int) /usr/include/opencv2/superres/optical_flow.hpp:121
	#[inline]
	fn set_scales_number(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_setScalesNumber_int(self.as_raw_mut_Superres_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setWarpingsNumber(int) /usr/include/opencv2/superres/optical_flow.hpp:125
	#[inline]
	fn set_warpings_number(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_setWarpingsNumber_int(self.as_raw_mut_Superres_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setEpsilon(double) /usr/include/opencv2/superres/optical_flow.hpp:129
	#[inline]
	fn set_epsilon(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_setEpsilon_double(self.as_raw_mut_Superres_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:133
	#[inline]
	fn set_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_setIterations_int(self.as_raw_mut_Superres_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setUseInitialFlow(bool) /usr/include/opencv2/superres/optical_flow.hpp:137
	#[inline]
	fn set_use_initial_flow(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_setUseInitialFlow_bool(self.as_raw_mut_Superres_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// FarnebackOpticalFlow /usr/include/opencv2/superres/optical_flow.hpp:64
pub trait Superres_FarnebackOpticalFlowConst: crate::superres::Superres_DenseOpticalFlowExtConst {
	fn as_raw_Superres_FarnebackOpticalFlow(&self) -> *const c_void;

	// getPyrScale() /usr/include/opencv2/superres/optical_flow.hpp:68
	#[inline]
	fn get_pyr_scale(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_getPyrScale_const(self.as_raw_Superres_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getLevelsNumber() /usr/include/opencv2/superres/optical_flow.hpp:72
	#[inline]
	fn get_levels_number(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_getLevelsNumber_const(self.as_raw_Superres_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getWindowSize() /usr/include/opencv2/superres/optical_flow.hpp:76
	#[inline]
	fn get_window_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_getWindowSize_const(self.as_raw_Superres_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getIterations() /usr/include/opencv2/superres/optical_flow.hpp:80
	#[inline]
	fn get_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_getIterations_const(self.as_raw_Superres_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getPolyN() /usr/include/opencv2/superres/optical_flow.hpp:84
	#[inline]
	fn get_poly_n(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_getPolyN_const(self.as_raw_Superres_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getPolySigma() /usr/include/opencv2/superres/optical_flow.hpp:88
	#[inline]
	fn get_poly_sigma(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_getPolySigma_const(self.as_raw_Superres_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getFlags() /usr/include/opencv2/superres/optical_flow.hpp:92
	#[inline]
	fn get_flags(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_getFlags_const(self.as_raw_Superres_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Superres_FarnebackOpticalFlow: crate::superres::Superres_DenseOpticalFlowExt + crate::superres::Superres_FarnebackOpticalFlowConst {
	fn as_raw_mut_Superres_FarnebackOpticalFlow(&mut self) -> *mut c_void;

	// setPyrScale(double) /usr/include/opencv2/superres/optical_flow.hpp:70
	#[inline]
	fn set_pyr_scale(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_setPyrScale_double(self.as_raw_mut_Superres_FarnebackOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setLevelsNumber(int) /usr/include/opencv2/superres/optical_flow.hpp:74
	#[inline]
	fn set_levels_number(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_setLevelsNumber_int(self.as_raw_mut_Superres_FarnebackOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setWindowSize(int) /usr/include/opencv2/superres/optical_flow.hpp:78
	#[inline]
	fn set_window_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_setWindowSize_int(self.as_raw_mut_Superres_FarnebackOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:82
	#[inline]
	fn set_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_setIterations_int(self.as_raw_mut_Superres_FarnebackOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setPolyN(int) /usr/include/opencv2/superres/optical_flow.hpp:86
	#[inline]
	fn set_poly_n(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_setPolyN_int(self.as_raw_mut_Superres_FarnebackOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setPolySigma(double) /usr/include/opencv2/superres/optical_flow.hpp:90
	#[inline]
	fn set_poly_sigma(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_setPolySigma_double(self.as_raw_mut_Superres_FarnebackOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setFlags(int) /usr/include/opencv2/superres/optical_flow.hpp:94
	#[inline]
	fn set_flags(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_setFlags_int(self.as_raw_mut_Superres_FarnebackOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// FrameSource /usr/include/opencv2/superres.hpp:66
pub trait Superres_FrameSourceConst {
	fn as_raw_Superres_FrameSource(&self) -> *const c_void;

}

pub trait Superres_FrameSource: crate::superres::Superres_FrameSourceConst {
	fn as_raw_mut_Superres_FrameSource(&mut self) -> *mut c_void;

	// nextFrame(cv::OutputArray) /usr/include/opencv2/superres.hpp:71
	#[inline]
	fn next_frame(&mut self, frame: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(frame);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FrameSource_nextFrame_const__OutputArrayR(self.as_raw_mut_Superres_FrameSource(), frame.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// reset() /usr/include/opencv2/superres.hpp:72
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FrameSource_reset(self.as_raw_mut_Superres_FrameSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// PyrLKOpticalFlow /usr/include/opencv2/superres/optical_flow.hpp:180
pub trait Superres_PyrLKOpticalFlowConst: crate::superres::Superres_DenseOpticalFlowExtConst {
	fn as_raw_Superres_PyrLKOpticalFlow(&self) -> *const c_void;

	// getWindowSize() /usr/include/opencv2/superres/optical_flow.hpp:184
	#[inline]
	fn get_window_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_PyrLKOpticalFlow_getWindowSize_const(self.as_raw_Superres_PyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getMaxLevel() /usr/include/opencv2/superres/optical_flow.hpp:188
	#[inline]
	fn get_max_level(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_PyrLKOpticalFlow_getMaxLevel_const(self.as_raw_Superres_PyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getIterations() /usr/include/opencv2/superres/optical_flow.hpp:192
	#[inline]
	fn get_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_PyrLKOpticalFlow_getIterations_const(self.as_raw_Superres_PyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Superres_PyrLKOpticalFlow: crate::superres::Superres_DenseOpticalFlowExt + crate::superres::Superres_PyrLKOpticalFlowConst {
	fn as_raw_mut_Superres_PyrLKOpticalFlow(&mut self) -> *mut c_void;

	// setWindowSize(int) /usr/include/opencv2/superres/optical_flow.hpp:186
	#[inline]
	fn set_window_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_PyrLKOpticalFlow_setWindowSize_int(self.as_raw_mut_Superres_PyrLKOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setMaxLevel(int) /usr/include/opencv2/superres/optical_flow.hpp:190
	#[inline]
	fn set_max_level(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_PyrLKOpticalFlow_setMaxLevel_int(self.as_raw_mut_Superres_PyrLKOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:194
	#[inline]
	fn set_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_PyrLKOpticalFlow_setIterations_int(self.as_raw_mut_Superres_PyrLKOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Base class for Super Resolution algorithms.
/// 
/// The class is only used to define the common interface for the whole family of Super Resolution
/// algorithms.
// SuperResolution /usr/include/opencv2/superres.hpp:87
pub trait Superres_SuperResolutionConst: core::AlgorithmTraitConst + crate::superres::Superres_FrameSourceConst {
	fn as_raw_Superres_SuperResolution(&self) -> *const c_void;

	/// Scale factor
	/// ## See also
	/// setScale
	// getScale() /usr/include/opencv2/superres.hpp:109
	#[inline]
	fn get_scale(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getScale_const(self.as_raw_Superres_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Iterations count
	/// ## See also
	/// setIterations
	// getIterations() /usr/include/opencv2/superres.hpp:115
	#[inline]
	fn get_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getIterations_const(self.as_raw_Superres_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Asymptotic value of steepest descent method
	/// ## See also
	/// setTau
	// getTau() /usr/include/opencv2/superres.hpp:121
	#[inline]
	fn get_tau(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getTau_const(self.as_raw_Superres_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Weight parameter to balance data term and smoothness term
	/// ## See also
	/// setLambda
	// getLambda() /usr/include/opencv2/superres.hpp:127
	#[inline]
	fn get_lambda(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getLambda_const(self.as_raw_Superres_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Parameter of spacial distribution in Bilateral-TV
	/// ## See also
	/// setAlpha
	// getAlpha() /usr/include/opencv2/superres.hpp:133
	#[inline]
	fn get_alpha(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getAlpha_const(self.as_raw_Superres_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Kernel size of Bilateral-TV filter
	/// ## See also
	/// setKernelSize
	// getKernelSize() /usr/include/opencv2/superres.hpp:139
	#[inline]
	fn get_kernel_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getKernelSize_const(self.as_raw_Superres_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Gaussian blur kernel size
	/// ## See also
	/// setBlurKernelSize
	// getBlurKernelSize() /usr/include/opencv2/superres.hpp:145
	#[inline]
	fn get_blur_kernel_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getBlurKernelSize_const(self.as_raw_Superres_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Gaussian blur sigma
	/// ## See also
	/// setBlurSigma
	// getBlurSigma() /usr/include/opencv2/superres.hpp:151
	#[inline]
	fn get_blur_sigma(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getBlurSigma_const(self.as_raw_Superres_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Radius of the temporal search area
	/// ## See also
	/// setTemporalAreaRadius
	// getTemporalAreaRadius() /usr/include/opencv2/superres.hpp:157
	#[inline]
	fn get_temporal_area_radius(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getTemporalAreaRadius_const(self.as_raw_Superres_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Dense optical flow algorithm
	/// ## See also
	/// setOpticalFlow
	// getOpticalFlow() /usr/include/opencv2/superres.hpp:163
	#[inline]
	fn get_optical_flow(&self) -> Result<core::Ptr<dyn crate::superres::Superres_DenseOpticalFlowExt>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getOpticalFlow_const(self.as_raw_Superres_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::superres::Superres_DenseOpticalFlowExt>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait Superres_SuperResolution: core::AlgorithmTrait + crate::superres::Superres_FrameSource + crate::superres::Superres_SuperResolutionConst {
	fn as_raw_mut_Superres_SuperResolution(&mut self) -> *mut c_void;

	/// Set input frame source for Super Resolution algorithm.
	/// 
	/// ## Parameters
	/// * frameSource: Input frame source
	// setInput(const Ptr<cv::superres::FrameSource> &) /usr/include/opencv2/superres.hpp:94
	#[inline]
	fn set_input(&mut self, frame_source: &core::Ptr<dyn crate::superres::Superres_FrameSource>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setInput_const_Ptr_FrameSource_R(self.as_raw_mut_Superres_SuperResolution(), frame_source.as_raw_PtrOfSuperres_FrameSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Process next frame from input and return output result.
	/// 
	/// ## Parameters
	/// * frame: Output result
	// nextFrame(cv::OutputArray) /usr/include/opencv2/superres.hpp:100
	#[inline]
	fn next_frame(&mut self, frame: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(frame);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_nextFrame_const__OutputArrayR(self.as_raw_mut_Superres_SuperResolution(), frame.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// reset() /usr/include/opencv2/superres.hpp:101
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_reset(self.as_raw_mut_Superres_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Clear all inner buffers.
	// collectGarbage() /usr/include/opencv2/superres.hpp:105
	#[inline]
	fn collect_garbage(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_collectGarbage(self.as_raw_mut_Superres_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Scale factor
	/// ## See also
	/// setScale getScale
	// setScale(int) /usr/include/opencv2/superres.hpp:111
	#[inline]
	fn set_scale(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setScale_int(self.as_raw_mut_Superres_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Iterations count
	/// ## See also
	/// setIterations getIterations
	// setIterations(int) /usr/include/opencv2/superres.hpp:117
	#[inline]
	fn set_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setIterations_int(self.as_raw_mut_Superres_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Asymptotic value of steepest descent method
	/// ## See also
	/// setTau getTau
	// setTau(double) /usr/include/opencv2/superres.hpp:123
	#[inline]
	fn set_tau(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setTau_double(self.as_raw_mut_Superres_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Weight parameter to balance data term and smoothness term
	/// ## See also
	/// setLambda getLambda
	// setLambda(double) /usr/include/opencv2/superres.hpp:129
	#[inline]
	fn set_lambda(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setLambda_double(self.as_raw_mut_Superres_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Parameter of spacial distribution in Bilateral-TV
	/// ## See also
	/// setAlpha getAlpha
	// setAlpha(double) /usr/include/opencv2/superres.hpp:135
	#[inline]
	fn set_alpha(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setAlpha_double(self.as_raw_mut_Superres_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Kernel size of Bilateral-TV filter
	/// ## See also
	/// setKernelSize getKernelSize
	// setKernelSize(int) /usr/include/opencv2/superres.hpp:141
	#[inline]
	fn set_kernel_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setKernelSize_int(self.as_raw_mut_Superres_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Gaussian blur kernel size
	/// ## See also
	/// setBlurKernelSize getBlurKernelSize
	// setBlurKernelSize(int) /usr/include/opencv2/superres.hpp:147
	#[inline]
	fn set_blur_kernel_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setBlurKernelSize_int(self.as_raw_mut_Superres_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Gaussian blur sigma
	/// ## See also
	/// setBlurSigma getBlurSigma
	// setBlurSigma(double) /usr/include/opencv2/superres.hpp:153
	#[inline]
	fn set_blur_sigma(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setBlurSigma_double(self.as_raw_mut_Superres_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Radius of the temporal search area
	/// ## See also
	/// setTemporalAreaRadius getTemporalAreaRadius
	// setTemporalAreaRadius(int) /usr/include/opencv2/superres.hpp:159
	#[inline]
	fn set_temporal_area_radius(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setTemporalAreaRadius_int(self.as_raw_mut_Superres_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Dense optical flow algorithm
	/// ## See also
	/// setOpticalFlow getOpticalFlow
	// setOpticalFlow(const Ptr<cv::superres::DenseOpticalFlowExt> &) /usr/include/opencv2/superres.hpp:165
	#[inline]
	fn set_optical_flow(&mut self, val: &core::Ptr<dyn crate::superres::Superres_DenseOpticalFlowExt>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setOpticalFlow_const_Ptr_DenseOpticalFlowExt_R(self.as_raw_mut_Superres_SuperResolution(), val.as_raw_PtrOfSuperres_DenseOpticalFlowExt(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
