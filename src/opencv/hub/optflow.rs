#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Optical Flow Algorithms
//! 
//! Dense optical flow algorithms compute motion for each point:
//! 
//! - cv::optflow::calcOpticalFlowSF
//! - cv::optflow::createOptFlow_DeepFlow
//! 
//! Motion templates is alternative technique for detecting motion and computing its direction.
//! See samples/motempl.py.
//! 
//! - cv::motempl::updateMotionHistory
//! - cv::motempl::calcMotionGradient
//! - cv::motempl::calcGlobalOrientation
//! - cv::motempl::segmentMotion
//! 
//! Functions reading and writing .flo files in "Middlebury" format, see: <http://vision.middlebury.edu/flow/code/flow-code/README.txt>
//! 
//! - cv::optflow::readOpticalFlow
//! - cv::optflow::writeOpticalFlow
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::PCAPriorTraitConst, super::PCAPriorTrait, super::OpticalFlowPCAFlowTraitConst, super::OpticalFlowPCAFlowTrait, super::GPCPatchDescriptorTraitConst, super::GPCPatchDescriptorTrait, super::GPCPatchSampleTraitConst, super::GPCPatchSampleTrait, super::GPCTrainingSamplesTraitConst, super::GPCTrainingSamplesTrait, super::GPCTreeTraitConst, super::GPCTreeTrait, super::GPCDetailsTraitConst, super::GPCDetailsTrait, super::RLOFOpticalFlowParameterTraitConst, super::RLOFOpticalFlowParameterTrait, super::DenseRLOFOpticalFlowConst, super::DenseRLOFOpticalFlow, super::SparseRLOFOpticalFlowConst, super::SparseRLOFOpticalFlow, super::DualTVL1OpticalFlowConst, super::DualTVL1OpticalFlow };
}

pub const GPC_DESCRIPTOR_DCT: i32 = 0;
pub const GPC_DESCRIPTOR_WHT: i32 = 1;
pub const INTERP_EPIC: i32 = 1;
pub const INTERP_GEO: i32 = 0;
pub const INTERP_RIC: i32 = 2;
pub const SR_CROSS: i32 = 1;
pub const SR_FIXED: i32 = 0;
pub const ST_BILINEAR: i32 = 1;
pub const ST_STANDART: i32 = 0;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GPCDescType {
	GPC_DESCRIPTOR_DCT = 0,
	GPC_DESCRIPTOR_WHT = 1,
}

opencv_type_enum! { crate::optflow::GPCDescType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InterpolationType {
	INTERP_GEO = 0,
	INTERP_EPIC = 1,
	INTERP_RIC = 2,
}

opencv_type_enum! { crate::optflow::InterpolationType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SolverType {
	ST_STANDART = 0,
	ST_BILINEAR = 1,
}

opencv_type_enum! { crate::optflow::SolverType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SupportRegionType {
	SR_FIXED = 0,
	SR_CROSS = 1,
}

opencv_type_enum! { crate::optflow::SupportRegionType }

#[inline]
pub fn calc_global_orientation(orientation: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, mhi: &dyn core::ToInputArray, timestamp: f64, duration: f64) -> Result<f64> {
	input_array_arg!(orientation);
	input_array_arg!(mask);
	input_array_arg!(mhi);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_motempl_calcGlobalOrientation_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_double(orientation.as_raw__InputArray(), mask.as_raw__InputArray(), mhi.as_raw__InputArray(), timestamp, duration, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * aperture_size: 3
#[inline]
pub fn calc_motion_gradient(mhi: &dyn core::ToInputArray, mask: &mut dyn core::ToOutputArray, orientation: &mut dyn core::ToOutputArray, delta1: f64, delta2: f64, aperture_size: i32) -> Result<()> {
	input_array_arg!(mhi);
	output_array_arg!(mask);
	output_array_arg!(orientation);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_motempl_calcMotionGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double_int(mhi.as_raw__InputArray(), mask.as_raw__OutputArray(), orientation.as_raw__OutputArray(), delta1, delta2, aperture_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn segment_motion(mhi: &dyn core::ToInputArray, segmask: &mut dyn core::ToOutputArray, bounding_rects: &mut core::Vector<core::Rect>, timestamp: f64, seg_thresh: f64) -> Result<()> {
	input_array_arg!(mhi);
	output_array_arg!(segmask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_motempl_segmentMotion_const__InputArrayR_const__OutputArrayR_vector_Rect_R_double_double(mhi.as_raw__InputArray(), segmask.as_raw__OutputArray(), bounding_rects.as_raw_mut_VectorOfRect(), timestamp, seg_thresh, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn update_motion_history(silhouette: &dyn core::ToInputArray, mhi: &mut dyn core::ToInputOutputArray, timestamp: f64, duration: f64) -> Result<()> {
	input_array_arg!(silhouette);
	input_output_array_arg!(mhi);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_motempl_updateMotionHistory_const__InputArrayR_const__InputOutputArrayR_double_double(silhouette.as_raw__InputArray(), mhi.as_raw__InputOutputArray(), timestamp, duration, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * rlof_param: Ptr<RLOFOpticalFlowParameter>()
/// * forward_backward_threshold: 0
/// * grid_step: Size(6,6)
/// * interp_type: InterpolationType::INTERP_EPIC
/// * epic_k: 128
/// * epic_sigma: 0.05f
/// * epic_lambda: 100.f
/// * ric_sp_size: 15
/// * ric_slic_type: 100
/// * use_post_proc: true
/// * fgs_lambda: 500.0f
/// * fgs_sigma: 1.5f
/// * use_variational_refinement: false
#[inline]
pub fn calc_optical_flow_dense_rlof(i0: &dyn core::ToInputArray, i1: &dyn core::ToInputArray, flow: &mut dyn core::ToInputOutputArray, mut rlof_param: core::Ptr<crate::optflow::RLOFOpticalFlowParameter>, forward_backward_threshold: f32, grid_step: core::Size, interp_type: crate::optflow::InterpolationType, epic_k: i32, epic_sigma: f32, epic_lambda: f32, ric_sp_size: i32, ric_slic_type: i32, use_post_proc: bool, fgs_lambda: f32, fgs_sigma: f32, use_variational_refinement: bool) -> Result<()> {
	input_array_arg!(i0);
	input_array_arg!(i1);
	input_output_array_arg!(flow);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_calcOpticalFlowDenseRLOF_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_Ptr_RLOFOpticalFlowParameter__float_Size_InterpolationType_int_float_float_int_int_bool_float_float_bool(i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow.as_raw__InputOutputArray(), rlof_param.as_raw_mut_PtrOfRLOFOpticalFlowParameter(), forward_backward_threshold, grid_step.opencv_as_extern(), interp_type, epic_k, epic_sigma, epic_lambda, ric_sp_size, ric_slic_type, use_post_proc, fgs_lambda, fgs_sigma, use_variational_refinement, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Calculate an optical flow using "SimpleFlow" algorithm.
/// 
/// ## Parameters
/// * from: First 8-bit 3-channel image.
/// * to: Second 8-bit 3-channel image of the same size as prev
/// * flow: computed flow image that has the same size as prev and type CV_32FC2
/// * layers: Number of layers
/// * averaging_block_size: Size of block through which we sum up when calculate cost function
/// for pixel
/// * max_flow: maximal flow that we search at each level
/// * sigma_dist: vector smooth spatial sigma parameter
/// * sigma_color: vector smooth color sigma parameter
/// * postprocess_window: window size for postprocess cross bilateral filter
/// * sigma_dist_fix: spatial sigma for postprocess cross bilateralf filter
/// * sigma_color_fix: color sigma for postprocess cross bilateral filter
/// * occ_thr: threshold for detecting occlusions
/// * upscale_averaging_radius: window size for bilateral upscale operation
/// * upscale_sigma_dist: spatial sigma for bilateral upscale operation
/// * upscale_sigma_color: color sigma for bilateral upscale operation
/// * speed_up_thr: threshold to detect point with irregular flow - where flow should be
/// recalculated after upscale
/// 
/// See [Tao2012](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Tao2012) . And site of project - <http://graphics.berkeley.edu/papers/Tao-SAN-2012-05/>.
/// 
/// 
/// Note:
///    *   An example using the simpleFlow algorithm can be found at samples/simpleflow_demo.cpp
/// 
/// ## Overloaded parameters
#[inline]
pub fn calc_optical_flow_sf(from: &dyn core::ToInputArray, to: &dyn core::ToInputArray, flow: &mut dyn core::ToOutputArray, layers: i32, averaging_block_size: i32, max_flow: i32) -> Result<()> {
	input_array_arg!(from);
	input_array_arg!(to);
	output_array_arg!(flow);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int(from.as_raw__InputArray(), to.as_raw__InputArray(), flow.as_raw__OutputArray(), layers, averaging_block_size, max_flow, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Calculate an optical flow using "SimpleFlow" algorithm.
/// 
/// ## Parameters
/// * from: First 8-bit 3-channel image.
/// * to: Second 8-bit 3-channel image of the same size as prev
/// * flow: computed flow image that has the same size as prev and type CV_32FC2
/// * layers: Number of layers
/// * averaging_block_size: Size of block through which we sum up when calculate cost function
/// for pixel
/// * max_flow: maximal flow that we search at each level
/// * sigma_dist: vector smooth spatial sigma parameter
/// * sigma_color: vector smooth color sigma parameter
/// * postprocess_window: window size for postprocess cross bilateral filter
/// * sigma_dist_fix: spatial sigma for postprocess cross bilateralf filter
/// * sigma_color_fix: color sigma for postprocess cross bilateral filter
/// * occ_thr: threshold for detecting occlusions
/// * upscale_averaging_radius: window size for bilateral upscale operation
/// * upscale_sigma_dist: spatial sigma for bilateral upscale operation
/// * upscale_sigma_color: color sigma for bilateral upscale operation
/// * speed_up_thr: threshold to detect point with irregular flow - where flow should be
/// recalculated after upscale
/// 
/// See [Tao2012](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Tao2012) . And site of project - <http://graphics.berkeley.edu/papers/Tao-SAN-2012-05/>.
/// 
/// 
/// Note:
///    *   An example using the simpleFlow algorithm can be found at samples/simpleflow_demo.cpp
#[inline]
pub fn calc_optical_flow_sf_1(from: &dyn core::ToInputArray, to: &dyn core::ToInputArray, flow: &mut dyn core::ToOutputArray, layers: i32, averaging_block_size: i32, max_flow: i32, sigma_dist: f64, sigma_color: f64, postprocess_window: i32, sigma_dist_fix: f64, sigma_color_fix: f64, occ_thr: f64, upscale_averaging_radius: i32, upscale_sigma_dist: f64, upscale_sigma_color: f64, speed_up_thr: f64) -> Result<()> {
	input_array_arg!(from);
	input_array_arg!(to);
	output_array_arg!(flow);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int_double_double_int_double_double_double_int_double_double_double(from.as_raw__InputArray(), to.as_raw__InputArray(), flow.as_raw__OutputArray(), layers, averaging_block_size, max_flow, sigma_dist, sigma_color, postprocess_window, sigma_dist_fix, sigma_color_fix, occ_thr, upscale_averaging_radius, upscale_sigma_dist, upscale_sigma_color, speed_up_thr, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * rlof_param: Ptr<RLOFOpticalFlowParameter>()
/// * forward_backward_threshold: 0
#[inline]
pub fn calc_optical_flow_sparse_rlof(prev_img: &dyn core::ToInputArray, next_img: &dyn core::ToInputArray, prev_pts: &dyn core::ToInputArray, next_pts: &mut dyn core::ToInputOutputArray, status: &mut dyn core::ToOutputArray, err: &mut dyn core::ToOutputArray, mut rlof_param: core::Ptr<crate::optflow::RLOFOpticalFlowParameter>, forward_backward_threshold: f32) -> Result<()> {
	input_array_arg!(prev_img);
	input_array_arg!(next_img);
	input_array_arg!(prev_pts);
	input_output_array_arg!(next_pts);
	output_array_arg!(status);
	output_array_arg!(err);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_calcOpticalFlowSparseRLOF_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_Ptr_RLOFOpticalFlowParameter__float(prev_img.as_raw__InputArray(), next_img.as_raw__InputArray(), prev_pts.as_raw__InputArray(), next_pts.as_raw__InputOutputArray(), status.as_raw__OutputArray(), err.as_raw__OutputArray(), rlof_param.as_raw_mut_PtrOfRLOFOpticalFlowParameter(), forward_backward_threshold, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Fast dense optical flow based on PyrLK sparse matches interpolation.
/// 
/// ## Parameters
/// * from: first 8-bit 3-channel or 1-channel image.
/// * to: second 8-bit 3-channel or 1-channel image of the same size as from
/// * flow: computed flow image that has the same size as from and CV_32FC2 type
/// * grid_step: stride used in sparse match computation. Lower values usually
///        result in higher quality but slow down the algorithm.
/// * k: number of nearest-neighbor matches considered, when fitting a locally affine
///        model. Lower values can make the algorithm noticeably faster at the cost of
///        some quality degradation.
/// * sigma: parameter defining how fast the weights decrease in the locally-weighted affine
///        fitting. Higher values can help preserve fine details, lower values can help to get rid
///        of the noise in the output flow.
/// * use_post_proc: defines whether the ximgproc::fastGlobalSmootherFilter() is used
///        for post-processing after interpolation
/// * fgs_lambda: see the respective parameter of the ximgproc::fastGlobalSmootherFilter()
/// * fgs_sigma: see the respective parameter of the ximgproc::fastGlobalSmootherFilter()
/// 
/// ## C++ default parameters
/// * grid_step: 8
/// * k: 128
/// * sigma: 0.05f
/// * use_post_proc: true
/// * fgs_lambda: 500.0f
/// * fgs_sigma: 1.5f
#[inline]
pub fn calc_optical_flow_sparse_to_dense(from: &dyn core::ToInputArray, to: &dyn core::ToInputArray, flow: &mut dyn core::ToOutputArray, grid_step: i32, k: i32, sigma: f32, use_post_proc: bool, fgs_lambda: f32, fgs_sigma: f32) -> Result<()> {
	input_array_arg!(from);
	input_array_arg!(to);
	output_array_arg!(flow);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_calcOpticalFlowSparseToDense_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_float_bool_float_float(from.as_raw__InputArray(), to.as_raw__InputArray(), flow.as_raw__OutputArray(), grid_step, k, sigma, use_post_proc, fgs_lambda, fgs_sigma, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// DeepFlow optical flow algorithm implementation.
/// 
/// The class implements the DeepFlow optical flow algorithm described in [Weinzaepfel2013](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Weinzaepfel2013) . See
/// also <http://lear.inrialpes.fr/src/deepmatching/> .
/// Parameters - class fields - that may be modified after creating a class instance:
/// *   member float alpha
/// Smoothness assumption weight
/// *   member float delta
/// Color constancy assumption weight
/// *   member float gamma
/// Gradient constancy weight
/// *   member float sigma
/// Gaussian smoothing parameter
/// *   member int minSize
/// Minimal dimension of an image in the pyramid (next, smaller images in the pyramid are generated
/// until one of the dimensions reaches this size)
/// *   member float downscaleFactor
/// Scaling factor in the image pyramid (must be \< 1)
/// *   member int fixedPointIterations
/// How many iterations on each level of the pyramid
/// *   member int sorIterations
/// Iterations of Succesive Over-Relaxation (solver)
/// *   member float omega
/// Relaxation factor in SOR
#[inline]
pub fn create_opt_flow_deep_flow() -> Result<core::Ptr<dyn crate::video::DenseOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_DeepFlow(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::video::DenseOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn create_opt_flow_dense_rlof() -> Result<core::Ptr<dyn crate::video::DenseOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_DenseRLOF(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::video::DenseOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Creates instance of cv::DenseOpticalFlow
#[inline]
pub fn create_opt_flow_dual_tvl1() -> Result<core::Ptr<dyn crate::optflow::DualTVL1OpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_DualTVL1(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::optflow::DualTVL1OpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Additional interface to the Farneback's algorithm - calcOpticalFlowFarneback()
#[inline]
pub fn create_opt_flow_farneback() -> Result<core::Ptr<dyn crate::video::DenseOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_Farneback(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::video::DenseOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn create_opt_flow_pca_flow() -> Result<core::Ptr<dyn crate::video::DenseOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_PCAFlow(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::video::DenseOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Additional interface to the SimpleFlow algorithm - calcOpticalFlowSF()
#[inline]
pub fn create_opt_flow_simple_flow() -> Result<core::Ptr<dyn crate::video::DenseOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_SimpleFlow(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::video::DenseOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn create_opt_flow_sparse_rlof() -> Result<core::Ptr<dyn crate::video::SparseOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_SparseRLOF(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::video::SparseOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Additional interface to the SparseToDenseFlow algorithm - calcOpticalFlowSparseToDense()
#[inline]
pub fn create_opt_flow_sparse_to_dense() -> Result<core::Ptr<dyn crate::video::DenseOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_SparseToDense(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::video::DenseOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn read(fn_: &core::FileNode, node: &mut crate::optflow::GPCTree_Node, unnamed: crate::optflow::GPCTree_Node) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_read_const_FileNodeR_NodeR_Node(fn_.as_raw_FileNode(), node, unnamed.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn write(fs: &mut core::FileStorage, name: &str, node: crate::optflow::GPCTree_Node) -> Result<()> {
	extern_container_arg!(name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_write_FileStorageR_const_StringR_const_NodeR(fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), &node, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

pub trait DenseRLOFOpticalFlowConst: crate::video::DenseOpticalFlowConst {
	fn as_raw_DenseRLOFOpticalFlow(&self) -> *const c_void;

	#[inline]
	fn get_rlof_optical_flow_parameter(&self) -> Result<core::Ptr<crate::optflow::RLOFOpticalFlowParameter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getRLOFOpticalFlowParameter_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_forward_backward(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getForwardBackward_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_grid_step(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getGridStep_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_interpolation(&self) -> Result<crate::optflow::InterpolationType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getInterpolation_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_epick(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getEPICK_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_epic_sigma(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getEPICSigma_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_epic_lambda(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getEPICLambda_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_fgs_lambda(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getFgsLambda_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_fgs_sigma(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getFgsSigma_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_use_post_proc(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getUsePostProc_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_use_variational_refinement(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getUseVariationalRefinement_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_ricsp_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getRICSPSize_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_ricslic_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getRICSLICType_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait DenseRLOFOpticalFlow: crate::optflow::DenseRLOFOpticalFlowConst + crate::video::DenseOpticalFlow {
	fn as_raw_mut_DenseRLOFOpticalFlow(&mut self) -> *mut c_void;

	#[inline]
	fn set_rlof_optical_flow_parameter(&mut self, mut val: core::Ptr<crate::optflow::RLOFOpticalFlowParameter>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setRLOFOpticalFlowParameter_Ptr_RLOFOpticalFlowParameter_(self.as_raw_mut_DenseRLOFOpticalFlow(), val.as_raw_mut_PtrOfRLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_forward_backward(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setForwardBackward_float(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_grid_step(&mut self, val: core::Size) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setGridStep_Size(self.as_raw_mut_DenseRLOFOpticalFlow(), val.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_interpolation(&mut self, val: crate::optflow::InterpolationType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setInterpolation_InterpolationType(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_epick(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setEPICK_int(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_epic_sigma(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setEPICSigma_float(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_epic_lambda(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setEPICLambda_float(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_fgs_lambda(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setFgsLambda_float(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_fgs_sigma(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setFgsSigma_float(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_use_post_proc(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setUsePostProc_bool(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_use_variational_refinement(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setUseVariationalRefinement_bool(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_ricsp_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setRICSPSize_int(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_ricslic_type(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setRICSLICType_int(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn DenseRLOFOpticalFlow + '_ {
	/// ## C++ default parameters
	/// * rlof_param: Ptr<RLOFOpticalFlowParameter>()
	/// * forward_backward_threshold: 1.f
	/// * grid_step: Size(6,6)
	/// * interp_type: InterpolationType::INTERP_EPIC
	/// * epic_k: 128
	/// * epic_sigma: 0.05f
	/// * epic_lambda: 999.0f
	/// * ric_sp_size: 15
	/// * ric_slic_type: 100
	/// * use_post_proc: true
	/// * fgs_lambda: 500.0f
	/// * fgs_sigma: 1.5f
	/// * use_variational_refinement: false
	#[inline]
	pub fn create(mut rlof_param: core::Ptr<crate::optflow::RLOFOpticalFlowParameter>, forward_backward_threshold: f32, grid_step: core::Size, interp_type: crate::optflow::InterpolationType, epic_k: i32, epic_sigma: f32, epic_lambda: f32, ric_sp_size: i32, ric_slic_type: i32, use_post_proc: bool, fgs_lambda: f32, fgs_sigma: f32, use_variational_refinement: bool) -> Result<core::Ptr<dyn crate::optflow::DenseRLOFOpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_create_Ptr_RLOFOpticalFlowParameter__float_Size_InterpolationType_int_float_float_int_int_bool_float_float_bool(rlof_param.as_raw_mut_PtrOfRLOFOpticalFlowParameter(), forward_backward_threshold, grid_step.opencv_as_extern(), interp_type, epic_k, epic_sigma, epic_lambda, ric_sp_size, ric_slic_type, use_post_proc, fgs_lambda, fgs_sigma, use_variational_refinement, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::optflow::DenseRLOFOpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
/// "Dual TV L1" Optical Flow Algorithm.
/// 
/// The class implements the "Dual TV L1" optical flow algorithm described in [Zach2007](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Zach2007) and
/// [Javier2012](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Javier2012) .
/// Here are important members of the class that control the algorithm, which you can set after
/// constructing the class instance:
/// 
/// *   member double tau
///    Time step of the numerical scheme.
/// 
/// *   member double lambda
///    Weight parameter for the data term, attachment parameter. This is the most relevant
///    parameter, which determines the smoothness of the output. The smaller this parameter is,
///    the smoother the solutions we obtain. It depends on the range of motions of the images, so
///    its value should be adapted to each image sequence.
/// 
/// *   member double theta
///    Weight parameter for (u - v)\^2, tightness parameter. It serves as a link between the
///    attachment and the regularization terms. In theory, it should have a small value in order
///    to maintain both parts in correspondence. The method is stable for a large range of values
///    of this parameter.
/// 
/// *   member int nscales
///    Number of scales used to create the pyramid of images.
/// 
/// *   member int warps
///    Number of warpings per scale. Represents the number of times that I1(x+u0) and grad(
///    I1(x+u0) ) are computed per scale. This is a parameter that assures the stability of the
///    method. It also affects the running time, so it is a compromise between speed and
///    accuracy.
/// 
/// *   member double epsilon
///    Stopping criterion threshold used in the numerical scheme, which is a trade-off between
///    precision and running time. A small value will yield more accurate solutions at the
///    expense of a slower convergence.
/// 
/// *   member int iterations
///    Stopping criterion iterations number used in the numerical scheme.
/// 
/// C. Zach, T. Pock and H. Bischof, "A Duality Based Approach for Realtime TV-L1 Optical Flow".
/// Javier Sanchez, Enric Meinhardt-Llopis and Gabriele Facciolo. "TV-L1 Optical Flow Estimation".
pub trait DualTVL1OpticalFlowConst: crate::video::DenseOpticalFlowConst {
	fn as_raw_DualTVL1OpticalFlow(&self) -> *const c_void;

	/// Time step of the numerical scheme
	/// ## See also
	/// setTau
	#[inline]
	fn get_tau(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getTau_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Weight parameter for the data term, attachment parameter
	/// ## See also
	/// setLambda
	#[inline]
	fn get_lambda(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getLambda_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Weight parameter for (u - v)^2, tightness parameter
	/// ## See also
	/// setTheta
	#[inline]
	fn get_theta(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getTheta_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// coefficient for additional illumination variation term
	/// ## See also
	/// setGamma
	#[inline]
	fn get_gamma(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getGamma_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Number of scales used to create the pyramid of images
	/// ## See also
	/// setScalesNumber
	#[inline]
	fn get_scales_number(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getScalesNumber_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Number of warpings per scale
	/// ## See also
	/// setWarpingsNumber
	#[inline]
	fn get_warpings_number(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getWarpingsNumber_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Stopping criterion threshold used in the numerical scheme, which is a trade-off between precision and running time
	/// ## See also
	/// setEpsilon
	#[inline]
	fn get_epsilon(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getEpsilon_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Inner iterations (between outlier filtering) used in the numerical scheme
	/// ## See also
	/// setInnerIterations
	#[inline]
	fn get_inner_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getInnerIterations_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Outer iterations (number of inner loops) used in the numerical scheme
	/// ## See also
	/// setOuterIterations
	#[inline]
	fn get_outer_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getOuterIterations_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Use initial flow
	/// ## See also
	/// setUseInitialFlow
	#[inline]
	fn get_use_initial_flow(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getUseInitialFlow_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Step between scales (<1)
	/// ## See also
	/// setScaleStep
	#[inline]
	fn get_scale_step(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getScaleStep_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Median filter kernel size (1 = no filter) (3 or 5)
	/// ## See also
	/// setMedianFiltering
	#[inline]
	fn get_median_filtering(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getMedianFiltering_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait DualTVL1OpticalFlow: crate::optflow::DualTVL1OpticalFlowConst + crate::video::DenseOpticalFlow {
	fn as_raw_mut_DualTVL1OpticalFlow(&mut self) -> *mut c_void;

	/// Time step of the numerical scheme
	/// ## See also
	/// setTau getTau
	#[inline]
	fn set_tau(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setTau_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Weight parameter for the data term, attachment parameter
	/// ## See also
	/// setLambda getLambda
	#[inline]
	fn set_lambda(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setLambda_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Weight parameter for (u - v)^2, tightness parameter
	/// ## See also
	/// setTheta getTheta
	#[inline]
	fn set_theta(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setTheta_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// coefficient for additional illumination variation term
	/// ## See also
	/// setGamma getGamma
	#[inline]
	fn set_gamma(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setGamma_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Number of scales used to create the pyramid of images
	/// ## See also
	/// setScalesNumber getScalesNumber
	#[inline]
	fn set_scales_number(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setScalesNumber_int(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Number of warpings per scale
	/// ## See also
	/// setWarpingsNumber getWarpingsNumber
	#[inline]
	fn set_warpings_number(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setWarpingsNumber_int(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Stopping criterion threshold used in the numerical scheme, which is a trade-off between precision and running time
	/// ## See also
	/// setEpsilon getEpsilon
	#[inline]
	fn set_epsilon(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setEpsilon_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Inner iterations (between outlier filtering) used in the numerical scheme
	/// ## See also
	/// setInnerIterations getInnerIterations
	#[inline]
	fn set_inner_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setInnerIterations_int(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Outer iterations (number of inner loops) used in the numerical scheme
	/// ## See also
	/// setOuterIterations getOuterIterations
	#[inline]
	fn set_outer_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setOuterIterations_int(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Use initial flow
	/// ## See also
	/// setUseInitialFlow getUseInitialFlow
	#[inline]
	fn set_use_initial_flow(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setUseInitialFlow_bool(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Step between scales (<1)
	/// ## See also
	/// setScaleStep getScaleStep
	#[inline]
	fn set_scale_step(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setScaleStep_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Median filter kernel size (1 = no filter) (3 or 5)
	/// ## See also
	/// setMedianFiltering getMedianFiltering
	#[inline]
	fn set_median_filtering(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setMedianFiltering_int(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn DualTVL1OpticalFlow + '_ {
	/// Creates instance of cv::DualTVL1OpticalFlow
	/// 
	/// ## C++ default parameters
	/// * tau: 0.25
	/// * lambda: 0.15
	/// * theta: 0.3
	/// * nscales: 5
	/// * warps: 5
	/// * epsilon: 0.01
	/// * innner_iterations: 30
	/// * outer_iterations: 10
	/// * scale_step: 0.8
	/// * gamma: 0.0
	/// * median_filtering: 5
	/// * use_initial_flow: false
	#[inline]
	pub fn create(tau: f64, lambda: f64, theta: f64, nscales: i32, warps: i32, epsilon: f64, innner_iterations: i32, outer_iterations: i32, scale_step: f64, gamma: f64, median_filtering: i32, use_initial_flow: bool) -> Result<core::Ptr<dyn crate::optflow::DualTVL1OpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_create_double_double_double_int_int_double_int_int_double_double_int_bool(tau, lambda, theta, nscales, warps, epsilon, innner_iterations, outer_iterations, scale_step, gamma, median_filtering, use_initial_flow, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::optflow::DualTVL1OpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait GPCDetailsTraitConst {
	fn as_raw_GPCDetails(&self) -> *const c_void;

}

pub trait GPCDetailsTrait: crate::optflow::GPCDetailsTraitConst {
	fn as_raw_mut_GPCDetails(&mut self) -> *mut c_void;

}

pub struct GPCDetails {
	ptr: *mut c_void
}

opencv_type_boxed! { GPCDetails }

impl Drop for GPCDetails {
	fn drop(&mut self) {
		extern "C" { fn cv_GPCDetails_delete(instance: *mut c_void); }
		unsafe { cv_GPCDetails_delete(self.as_raw_mut_GPCDetails()) };
	}
}

unsafe impl Send for GPCDetails {}

impl crate::optflow::GPCDetailsTraitConst for GPCDetails {
	#[inline] fn as_raw_GPCDetails(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::GPCDetailsTrait for GPCDetails {
	#[inline] fn as_raw_mut_GPCDetails(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GPCDetails {
	#[inline]
	pub fn get_all_descriptors_for_image(img_ch: &core::Mat, descr: &mut core::Vector<crate::optflow::GPCPatchDescriptor>, mp: crate::optflow::GPCMatchingParams, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCDetails_getAllDescriptorsForImage_const_MatX_vector_GPCPatchDescriptor_R_const_GPCMatchingParamsR_int(img_ch.as_raw_Mat(), descr.as_raw_mut_VectorOfGPCPatchDescriptor(), &mp, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn get_coordinates_from_index(index: size_t, sz: core::Size, x: &mut i32, y: &mut i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCDetails_getCoordinatesFromIndex_size_t_Size_intR_intR(index, sz.opencv_as_extern(), x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPCMatchingParams {
	pub use_opencl: bool,
}

opencv_type_simple! { crate::optflow::GPCMatchingParams }

impl GPCMatchingParams {
	/// ## C++ default parameters
	/// * _use_opencl: false
	#[inline]
	pub fn new(_use_opencl: bool) -> Result<crate::optflow::GPCMatchingParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCMatchingParams_GPCMatchingParams_bool(_use_opencl, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn copy(params: crate::optflow::GPCMatchingParams) -> Result<crate::optflow::GPCMatchingParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCMatchingParams_GPCMatchingParams_const_GPCMatchingParamsR(&params, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait GPCPatchDescriptorTraitConst {
	fn as_raw_GPCPatchDescriptor(&self) -> *const c_void;

	#[inline]
	fn feature(&self) -> core::VecN<f64, 18> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCPatchDescriptor_getPropFeature_const(self.as_raw_GPCPatchDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn dot(&self, coef: core::VecN<f64, 18>) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCPatchDescriptor_dot_const_const_Vec_double__18_R(self.as_raw_GPCPatchDescriptor(), &coef, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn is_separated(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCPatchDescriptor_isSeparated_const(self.as_raw_GPCPatchDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait GPCPatchDescriptorTrait: crate::optflow::GPCPatchDescriptorTraitConst {
	fn as_raw_mut_GPCPatchDescriptor(&mut self) -> *mut c_void;

	#[inline]
	fn set_feature(&mut self, val: core::VecN<f64, 18>) {
		let ret = unsafe { sys::cv_optflow_GPCPatchDescriptor_setPropFeature_Vec_double__18_(self.as_raw_mut_GPCPatchDescriptor(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn mark_as_separated(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCPatchDescriptor_markAsSeparated(self.as_raw_mut_GPCPatchDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct GPCPatchDescriptor {
	ptr: *mut c_void
}

opencv_type_boxed! { GPCPatchDescriptor }

impl Drop for GPCPatchDescriptor {
	fn drop(&mut self) {
		extern "C" { fn cv_GPCPatchDescriptor_delete(instance: *mut c_void); }
		unsafe { cv_GPCPatchDescriptor_delete(self.as_raw_mut_GPCPatchDescriptor()) };
	}
}

unsafe impl Send for GPCPatchDescriptor {}

impl crate::optflow::GPCPatchDescriptorTraitConst for GPCPatchDescriptor {
	#[inline] fn as_raw_GPCPatchDescriptor(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::GPCPatchDescriptorTrait for GPCPatchDescriptor {
	#[inline] fn as_raw_mut_GPCPatchDescriptor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GPCPatchDescriptor {
	pub const nFeatures: u32 = 18;
}

pub trait GPCPatchSampleTraitConst {
	fn as_raw_GPCPatchSample(&self) -> *const c_void;

	#[inline]
	fn ref_(&self) -> crate::optflow::GPCPatchDescriptor {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_getPropRef_const(self.as_raw_GPCPatchSample()) };
		let ret = unsafe { crate::optflow::GPCPatchDescriptor::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn pos(&self) -> crate::optflow::GPCPatchDescriptor {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_getPropPos_const(self.as_raw_GPCPatchSample()) };
		let ret = unsafe { crate::optflow::GPCPatchDescriptor::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn neg(&self) -> crate::optflow::GPCPatchDescriptor {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_getPropNeg_const(self.as_raw_GPCPatchSample()) };
		let ret = unsafe { crate::optflow::GPCPatchDescriptor::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn get_directions(&self, refdir: &mut bool, posdir: &mut bool, negdir: &mut bool, coef: core::VecN<f64, 18>, rhs: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCPatchSample_getDirections_const_boolR_boolR_boolR_const_Vec_double__18_R_double(self.as_raw_GPCPatchSample(), refdir, posdir, negdir, &coef, rhs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait GPCPatchSampleTrait: crate::optflow::GPCPatchSampleTraitConst {
	fn as_raw_mut_GPCPatchSample(&mut self) -> *mut c_void;

	#[inline]
	fn set_ref(&mut self, mut val: crate::optflow::GPCPatchDescriptor) {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_setPropRef_GPCPatchDescriptor(self.as_raw_mut_GPCPatchSample(), val.as_raw_mut_GPCPatchDescriptor()) };
		ret
	}
	
	#[inline]
	fn set_pos(&mut self, mut val: crate::optflow::GPCPatchDescriptor) {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_setPropPos_GPCPatchDescriptor(self.as_raw_mut_GPCPatchSample(), val.as_raw_mut_GPCPatchDescriptor()) };
		ret
	}
	
	#[inline]
	fn set_neg(&mut self, mut val: crate::optflow::GPCPatchDescriptor) {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_setPropNeg_GPCPatchDescriptor(self.as_raw_mut_GPCPatchSample(), val.as_raw_mut_GPCPatchDescriptor()) };
		ret
	}
	
}

pub struct GPCPatchSample {
	ptr: *mut c_void
}

opencv_type_boxed! { GPCPatchSample }

impl Drop for GPCPatchSample {
	fn drop(&mut self) {
		extern "C" { fn cv_GPCPatchSample_delete(instance: *mut c_void); }
		unsafe { cv_GPCPatchSample_delete(self.as_raw_mut_GPCPatchSample()) };
	}
}

unsafe impl Send for GPCPatchSample {}

impl crate::optflow::GPCPatchSampleTraitConst for GPCPatchSample {
	#[inline] fn as_raw_GPCPatchSample(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::GPCPatchSampleTrait for GPCPatchSample {
	#[inline] fn as_raw_mut_GPCPatchSample(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GPCPatchSample {
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPCTrainingParams {
	pub max_tree_depth: u32,
	pub min_number_of_samples: i32,
	pub descriptor_type: i32,
	pub print_progress: bool,
}

opencv_type_simple! { crate::optflow::GPCTrainingParams }

impl GPCTrainingParams {
	#[inline]
	pub fn check(self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTrainingParams_check_const(self.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * _max_tree_depth: 20
	/// * _min_number_of_samples: 3
	/// * _descriptor_type: GPC_DESCRIPTOR_DCT
	/// * _print_progress: true
	#[inline]
	pub fn new(_max_tree_depth: u32, _min_number_of_samples: i32, _descriptor_type: crate::optflow::GPCDescType, _print_progress: bool) -> Result<crate::optflow::GPCTrainingParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTrainingParams_GPCTrainingParams_unsigned_int_int_GPCDescType_bool(_max_tree_depth, _min_number_of_samples, _descriptor_type, _print_progress, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait GPCTrainingSamplesTraitConst {
	fn as_raw_GPCTrainingSamples(&self) -> *const c_void;

	#[inline]
	fn size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTrainingSamples_size_const(self.as_raw_GPCTrainingSamples(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn typ(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTrainingSamples_type_const(self.as_raw_GPCTrainingSamples(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait GPCTrainingSamplesTrait: crate::optflow::GPCTrainingSamplesTraitConst {
	fn as_raw_mut_GPCTrainingSamples(&mut self) -> *mut c_void;

}

pub struct GPCTrainingSamples {
	ptr: *mut c_void
}

opencv_type_boxed! { GPCTrainingSamples }

impl Drop for GPCTrainingSamples {
	fn drop(&mut self) {
		extern "C" { fn cv_GPCTrainingSamples_delete(instance: *mut c_void); }
		unsafe { cv_GPCTrainingSamples_delete(self.as_raw_mut_GPCTrainingSamples()) };
	}
}

unsafe impl Send for GPCTrainingSamples {}

impl crate::optflow::GPCTrainingSamplesTraitConst for GPCTrainingSamples {
	#[inline] fn as_raw_GPCTrainingSamples(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::GPCTrainingSamplesTrait for GPCTrainingSamples {
	#[inline] fn as_raw_mut_GPCTrainingSamples(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GPCTrainingSamples {
	#[inline]
	pub fn create(images_from: &core::Vector<String>, images_to: &core::Vector<String>, gt: &core::Vector<String>, descriptor_type: i32) -> Result<core::Ptr<crate::optflow::GPCTrainingSamples>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTrainingSamples_create_const_vector_String_R_const_vector_String_R_const_vector_String_R_int(images_from.as_raw_VectorOfString(), images_to.as_raw_VectorOfString(), gt.as_raw_VectorOfString(), descriptor_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::GPCTrainingSamples>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create_1(images_from: &dyn core::ToInputArray, images_to: &dyn core::ToInputArray, gt: &dyn core::ToInputArray, descriptor_type: i32) -> Result<core::Ptr<crate::optflow::GPCTrainingSamples>> {
		input_array_arg!(images_from);
		input_array_arg!(images_to);
		input_array_arg!(gt);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTrainingSamples_create_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(images_from.as_raw__InputArray(), images_to.as_raw__InputArray(), gt.as_raw__InputArray(), descriptor_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::GPCTrainingSamples>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait GPCTreeTraitConst: core::AlgorithmTraitConst {
	fn as_raw_GPCTree(&self) -> *const c_void;

	#[inline]
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_write_const_FileStorageR(self.as_raw_GPCTree(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn find_leaf_for_patch(&self, descr: &crate::optflow::GPCPatchDescriptor) -> Result<u32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_findLeafForPatch_const_const_GPCPatchDescriptorR(self.as_raw_GPCTree(), descr.as_raw_GPCPatchDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn equals(&self, t: &crate::optflow::GPCTree) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_operatorEQ_const_const_GPCTreeR(self.as_raw_GPCTree(), t.as_raw_GPCTree(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_descriptor_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_getDescriptorType_const(self.as_raw_GPCTree(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait GPCTreeTrait: core::AlgorithmTrait + crate::optflow::GPCTreeTraitConst {
	fn as_raw_mut_GPCTree(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * params: GPCTrainingParams()
	#[inline]
	fn train(&mut self, samples: &mut crate::optflow::GPCTrainingSamples, params: crate::optflow::GPCTrainingParams) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_train_GPCTrainingSamplesR_const_GPCTrainingParams(self.as_raw_mut_GPCTree(), samples.as_raw_mut_GPCTrainingSamples(), params.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_read_const_FileNodeR(self.as_raw_mut_GPCTree(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct GPCTree {
	ptr: *mut c_void
}

opencv_type_boxed! { GPCTree }

impl Drop for GPCTree {
	fn drop(&mut self) {
		extern "C" { fn cv_GPCTree_delete(instance: *mut c_void); }
		unsafe { cv_GPCTree_delete(self.as_raw_mut_GPCTree()) };
	}
}

unsafe impl Send for GPCTree {}

impl core::AlgorithmTraitConst for GPCTree {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for GPCTree {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::optflow::GPCTreeTraitConst for GPCTree {
	#[inline] fn as_raw_GPCTree(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::GPCTreeTrait for GPCTree {
	#[inline] fn as_raw_mut_GPCTree(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GPCTree {
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::optflow::GPCTree>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::GPCTree>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { GPCTree, core::Algorithm, cv_GPCTree_to_Algorithm }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPCTree_Node {
	pub coef: core::VecN<f64, 18>,
	pub rhs: f64,
	pub left: u32,
	pub right: u32,
}

opencv_type_simple! { crate::optflow::GPCTree_Node }

impl GPCTree_Node {
	#[inline]
	pub fn equals(self, n: crate::optflow::GPCTree_Node) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_Node_operatorEQ_const_const_NodeR(self.opencv_as_extern(), &n, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait OpticalFlowPCAFlowTraitConst: crate::video::DenseOpticalFlowConst {
	fn as_raw_OpticalFlowPCAFlow(&self) -> *const c_void;

}

pub trait OpticalFlowPCAFlowTrait: crate::optflow::OpticalFlowPCAFlowTraitConst + crate::video::DenseOpticalFlow {
	fn as_raw_mut_OpticalFlowPCAFlow(&mut self) -> *mut c_void;

	#[inline]
	fn calc(&mut self, i0: &dyn core::ToInputArray, i1: &dyn core::ToInputArray, flow: &mut dyn core::ToInputOutputArray) -> Result<()> {
		input_array_arg!(i0);
		input_array_arg!(i1);
		input_output_array_arg!(flow);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_OpticalFlowPCAFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(self.as_raw_mut_OpticalFlowPCAFlow(), i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn collect_garbage(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_OpticalFlowPCAFlow_collectGarbage(self.as_raw_mut_OpticalFlowPCAFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct OpticalFlowPCAFlow {
	ptr: *mut c_void
}

opencv_type_boxed! { OpticalFlowPCAFlow }

impl Drop for OpticalFlowPCAFlow {
	fn drop(&mut self) {
		extern "C" { fn cv_OpticalFlowPCAFlow_delete(instance: *mut c_void); }
		unsafe { cv_OpticalFlowPCAFlow_delete(self.as_raw_mut_OpticalFlowPCAFlow()) };
	}
}

unsafe impl Send for OpticalFlowPCAFlow {}

impl core::AlgorithmTraitConst for OpticalFlowPCAFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for OpticalFlowPCAFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::video::DenseOpticalFlowConst for OpticalFlowPCAFlow {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::DenseOpticalFlow for OpticalFlowPCAFlow {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::optflow::OpticalFlowPCAFlowTraitConst for OpticalFlowPCAFlow {
	#[inline] fn as_raw_OpticalFlowPCAFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::OpticalFlowPCAFlowTrait for OpticalFlowPCAFlow {
	#[inline] fn as_raw_mut_OpticalFlowPCAFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl OpticalFlowPCAFlow {
	/// ## C++ default parameters
	/// * _prior: Ptr<constPCAPrior>()
	/// * _basis_size: Size(18,14)
	/// * _sparse_rate: 0.024
	/// * _retained_corners_fraction: 0.2
	/// * _occlusions_threshold: 0.0003
	/// * _damping_factor: 0.00002
	/// * _clahe_clip: 14
	#[inline]
	pub fn new(_prior: core::Ptr<crate::optflow::PCAPrior>, _basis_size: core::Size, _sparse_rate: f32, _retained_corners_fraction: f32, _occlusions_threshold: f32, _damping_factor: f32, _clahe_clip: f32) -> Result<crate::optflow::OpticalFlowPCAFlow> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_OpticalFlowPCAFlow_OpticalFlowPCAFlow_Ptr_const_PCAPrior__const_Size_float_float_float_float_float(_prior.as_raw_PtrOfPCAPrior(), _basis_size.opencv_as_extern(), _sparse_rate, _retained_corners_fraction, _occlusions_threshold, _damping_factor, _clahe_clip, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::optflow::OpticalFlowPCAFlow::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { OpticalFlowPCAFlow, core::Algorithm, cv_OpticalFlowPCAFlow_to_Algorithm }

pub trait PCAPriorTraitConst {
	fn as_raw_PCAPrior(&self) -> *const c_void;

	#[inline]
	fn get_padding(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_PCAPrior_getPadding_const(self.as_raw_PCAPrior(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_basis_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_PCAPrior_getBasisSize_const(self.as_raw_PCAPrior(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn fill_constraints(&self, a1: &mut f32, a2: &mut f32, b1: &mut f32, b2: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_PCAPrior_fillConstraints_const_floatX_floatX_floatX_floatX(self.as_raw_PCAPrior(), a1, a2, b1, b2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait PCAPriorTrait: crate::optflow::PCAPriorTraitConst {
	fn as_raw_mut_PCAPrior(&mut self) -> *mut c_void;

}

pub struct PCAPrior {
	ptr: *mut c_void
}

opencv_type_boxed! { PCAPrior }

impl Drop for PCAPrior {
	fn drop(&mut self) {
		extern "C" { fn cv_PCAPrior_delete(instance: *mut c_void); }
		unsafe { cv_PCAPrior_delete(self.as_raw_mut_PCAPrior()) };
	}
}

unsafe impl Send for PCAPrior {}

impl crate::optflow::PCAPriorTraitConst for PCAPrior {
	#[inline] fn as_raw_PCAPrior(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::PCAPriorTrait for PCAPrior {
	#[inline] fn as_raw_mut_PCAPrior(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PCAPrior {
	#[inline]
	pub fn new(path_to_prior: &str) -> Result<crate::optflow::PCAPrior> {
		extern_container_arg!(path_to_prior);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_PCAPrior_PCAPrior_const_charX(path_to_prior.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::optflow::PCAPrior::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait RLOFOpticalFlowParameterTraitConst {
	fn as_raw_RLOFOpticalFlowParameter(&self) -> *const c_void;

	#[inline]
	fn solver_type(&self) -> crate::optflow::SolverType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropSolverType_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn support_region_type(&self) -> crate::optflow::SupportRegionType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropSupportRegionType_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn norm_sigma0(&self) -> f32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropNormSigma0_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}
	
	#[inline]
	fn norm_sigma1(&self) -> f32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropNormSigma1_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}
	
	#[inline]
	fn small_win_size(&self) -> i32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropSmallWinSize_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}
	
	#[inline]
	fn large_win_size(&self) -> i32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropLargeWinSize_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}
	
	#[inline]
	fn cross_segmentation_threshold(&self) -> i32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropCrossSegmentationThreshold_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}
	
	#[inline]
	fn max_level(&self) -> i32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropMaxLevel_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}
	
	#[inline]
	fn use_initial_flow(&self) -> bool {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropUseInitialFlow_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}
	
	#[inline]
	fn use_illumination_model(&self) -> bool {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropUseIlluminationModel_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}
	
	#[inline]
	fn use_global_motion_prior(&self) -> bool {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropUseGlobalMotionPrior_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}
	
	#[inline]
	fn max_iteration(&self) -> i32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropMaxIteration_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}
	
	#[inline]
	fn min_eigen_value(&self) -> f32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropMinEigenValue_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}
	
	#[inline]
	fn global_motion_ransac_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropGlobalMotionRansacThreshold_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}
	
	#[inline]
	fn get_solver_type(&self) -> Result<crate::optflow::SolverType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getSolverType_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_support_region_type(&self) -> Result<crate::optflow::SupportRegionType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getSupportRegionType_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_norm_sigma0(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getNormSigma0_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_norm_sigma1(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getNormSigma1_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_small_win_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getSmallWinSize_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_large_win_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getLargeWinSize_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_cross_segmentation_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getCrossSegmentationThreshold_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_level(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getMaxLevel_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_use_initial_flow(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getUseInitialFlow_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_use_illumination_model(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getUseIlluminationModel_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_use_global_motion_prior(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getUseGlobalMotionPrior_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_iteration(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getMaxIteration_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_min_eigen_value(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getMinEigenValue_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_global_motion_ransac_threshold(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getGlobalMotionRansacThreshold_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait RLOFOpticalFlowParameterTrait: crate::optflow::RLOFOpticalFlowParameterTraitConst {
	fn as_raw_mut_RLOFOpticalFlowParameter(&mut self) -> *mut c_void;

	#[inline]
	fn set_solver_type(&mut self, val: crate::optflow::SolverType) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropSolverType_SolverType(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}
	
	#[inline]
	fn set_support_region_type(&mut self, val: crate::optflow::SupportRegionType) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropSupportRegionType_SupportRegionType(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}
	
	#[inline]
	fn set_norm_sigma0(&mut self, val: f32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropNormSigma0_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}
	
	#[inline]
	fn set_norm_sigma1(&mut self, val: f32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropNormSigma1_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}
	
	#[inline]
	fn set_small_win_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropSmallWinSize_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}
	
	#[inline]
	fn set_large_win_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropLargeWinSize_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}
	
	#[inline]
	fn set_cross_segmentation_threshold(&mut self, val: i32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropCrossSegmentationThreshold_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}
	
	#[inline]
	fn set_max_level(&mut self, val: i32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropMaxLevel_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}
	
	#[inline]
	fn set_use_initial_flow(&mut self, val: bool) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropUseInitialFlow_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}
	
	#[inline]
	fn set_use_illumination_model(&mut self, val: bool) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropUseIlluminationModel_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}
	
	#[inline]
	fn set_use_global_motion_prior(&mut self, val: bool) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropUseGlobalMotionPrior_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}
	
	#[inline]
	fn set_max_iteration(&mut self, val: i32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropMaxIteration_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}
	
	#[inline]
	fn set_min_eigen_value(&mut self, val: f32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropMinEigenValue_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}
	
	#[inline]
	fn set_global_motion_ransac_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropGlobalMotionRansacThreshold_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}
	
	#[inline]
	fn set_use_m_estimator(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setUseMEstimator_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_solver_type_1(&mut self, val: crate::optflow::SolverType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setSolverType_SolverType(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_support_region_type_1(&mut self, val: crate::optflow::SupportRegionType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setSupportRegionType_SupportRegionType(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_norm_sigma0_1(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setNormSigma0_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_norm_sigma1_1(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setNormSigma1_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_small_win_size_1(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setSmallWinSize_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_large_win_size_1(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setLargeWinSize_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_cross_segmentation_threshold_1(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setCrossSegmentationThreshold_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_level_1(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setMaxLevel_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_use_initial_flow_1(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setUseInitialFlow_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_use_illumination_model_1(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setUseIlluminationModel_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_use_global_motion_prior_1(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setUseGlobalMotionPrior_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_iteration_1(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setMaxIteration_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_min_eigen_value_1(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setMinEigenValue_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_global_motion_ransac_threshold_1(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setGlobalMotionRansacThreshold_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct RLOFOpticalFlowParameter {
	ptr: *mut c_void
}

opencv_type_boxed! { RLOFOpticalFlowParameter }

impl Drop for RLOFOpticalFlowParameter {
	fn drop(&mut self) {
		extern "C" { fn cv_RLOFOpticalFlowParameter_delete(instance: *mut c_void); }
		unsafe { cv_RLOFOpticalFlowParameter_delete(self.as_raw_mut_RLOFOpticalFlowParameter()) };
	}
}

unsafe impl Send for RLOFOpticalFlowParameter {}

impl crate::optflow::RLOFOpticalFlowParameterTraitConst for RLOFOpticalFlowParameter {
	#[inline] fn as_raw_RLOFOpticalFlowParameter(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::RLOFOpticalFlowParameterTrait for RLOFOpticalFlowParameter {
	#[inline] fn as_raw_mut_RLOFOpticalFlowParameter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RLOFOpticalFlowParameter {
	#[inline]
	pub fn default() -> Result<crate::optflow::RLOFOpticalFlowParameter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_RLOFOpticalFlowParameter(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::optflow::RLOFOpticalFlowParameter::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::optflow::RLOFOpticalFlowParameter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait SparseRLOFOpticalFlowConst: crate::video::SparseOpticalFlowConst {
	fn as_raw_SparseRLOFOpticalFlow(&self) -> *const c_void;

	#[inline]
	fn get_rlof_optical_flow_parameter(&self) -> Result<core::Ptr<crate::optflow::RLOFOpticalFlowParameter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_SparseRLOFOpticalFlow_getRLOFOpticalFlowParameter_const(self.as_raw_SparseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_forward_backward(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_SparseRLOFOpticalFlow_getForwardBackward_const(self.as_raw_SparseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait SparseRLOFOpticalFlow: crate::optflow::SparseRLOFOpticalFlowConst + crate::video::SparseOpticalFlow {
	fn as_raw_mut_SparseRLOFOpticalFlow(&mut self) -> *mut c_void;

	#[inline]
	fn set_rlof_optical_flow_parameter(&mut self, mut val: core::Ptr<crate::optflow::RLOFOpticalFlowParameter>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_SparseRLOFOpticalFlow_setRLOFOpticalFlowParameter_Ptr_RLOFOpticalFlowParameter_(self.as_raw_mut_SparseRLOFOpticalFlow(), val.as_raw_mut_PtrOfRLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_forward_backward(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_SparseRLOFOpticalFlow_setForwardBackward_float(self.as_raw_mut_SparseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn SparseRLOFOpticalFlow + '_ {
	/// ## C++ default parameters
	/// * rlof_param: Ptr<RLOFOpticalFlowParameter>()
	/// * forward_backward_threshold: 1.f
	#[inline]
	pub fn create(mut rlof_param: core::Ptr<crate::optflow::RLOFOpticalFlowParameter>, forward_backward_threshold: f32) -> Result<core::Ptr<dyn crate::optflow::SparseRLOFOpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_SparseRLOFOpticalFlow_create_Ptr_RLOFOpticalFlowParameter__float(rlof_param.as_raw_mut_PtrOfRLOFOpticalFlowParameter(), forward_backward_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::optflow::SparseRLOFOpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}