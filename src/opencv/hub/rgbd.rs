#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # RGB-Depth Processing
//! 
//! @ref kinfu_icp
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::Linemod_TemplateTraitConst, super::Linemod_TemplateTrait, super::Linemod_QuantizedPyramidConst, super::Linemod_QuantizedPyramid, super::Linemod_ModalityConst, super::Linemod_Modality, super::Linemod_ColorGradientTraitConst, super::Linemod_ColorGradientTrait, super::Linemod_DepthNormalTraitConst, super::Linemod_DepthNormalTrait, super::Linemod_MatchTraitConst, super::Linemod_MatchTrait, super::Linemod_DetectorTraitConst, super::Linemod_DetectorTrait, super::RgbdNormalsTraitConst, super::RgbdNormalsTrait, super::DepthCleanerTraitConst, super::DepthCleanerTrait, super::RgbdPlaneTraitConst, super::RgbdPlaneTrait, super::RgbdFrameTraitConst, super::RgbdFrameTrait, super::OdometryFrameTraitConst, super::OdometryFrameTrait, super::OdometryConst, super::Odometry, super::RgbdOdometryTraitConst, super::RgbdOdometryTrait, super::ICPOdometryTraitConst, super::ICPOdometryTrait, super::RgbdICPOdometryTraitConst, super::RgbdICPOdometryTrait, super::FastICPOdometryTraitConst, super::FastICPOdometryTrait, super::Kinfu_VolumeConst, super::Kinfu_Volume, super::Kinfu_VolumeParamsTraitConst, super::Kinfu_VolumeParamsTrait, super::Kinfu_ParamsTraitConst, super::Kinfu_ParamsTrait, super::Kinfu_KinFuConst, super::Kinfu_KinFu, super::Dynafu_DynaFuConst, super::Dynafu_DynaFu, super::ParamsTraitConst, super::ParamsTrait, super::LargeKinfuConst, super::LargeKinfu, super::Kinfu_Detail_PoseGraphConst, super::Kinfu_Detail_PoseGraph, super::ColoredKinfu_ParamsTraitConst, super::ColoredKinfu_ParamsTrait, super::ColoredKinfu_ColoredKinFuConst, super::ColoredKinfu_ColoredKinFu };
}

pub const Kinfu_VolumeType_COLOREDTSDF: i32 = 2;
pub const Kinfu_VolumeType_HASHTSDF: i32 = 1;
pub const Kinfu_VolumeType_TSDF: i32 = 0;
pub const OdometryFrame_CACHE_ALL: i32 = 3;
pub const OdometryFrame_CACHE_DST: i32 = 2;
pub const OdometryFrame_CACHE_SRC: i32 = 1;
pub const Odometry_RIGID_BODY_MOTION: i32 = 4;
pub const Odometry_ROTATION: i32 = 1;
pub const Odometry_TRANSLATION: i32 = 2;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DepthCleaner_DEPTH_CLEANER_METHOD {
	DEPTH_CLEANER_NIL = 0,
}

opencv_type_enum! { crate::rgbd::DepthCleaner_DEPTH_CLEANER_METHOD }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Kinfu_VolumeType {
	TSDF = 0,
	HASHTSDF = 1,
	COLOREDTSDF = 2,
}

opencv_type_enum! { crate::rgbd::Kinfu_VolumeType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RgbdNormals_RGBD_NORMALS_METHOD {
	RGBD_NORMALS_METHOD_FALS = 0,
	RGBD_NORMALS_METHOD_LINEMOD = 1,
	RGBD_NORMALS_METHOD_SRI = 2,
}

opencv_type_enum! { crate::rgbd::RgbdNormals_RGBD_NORMALS_METHOD }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RgbdPlane_RGBD_PLANE_METHOD {
	RGBD_PLANE_METHOD_DEFAULT = 0,
}

opencv_type_enum! { crate::rgbd::RgbdPlane_RGBD_PLANE_METHOD }

pub type Dynafu_Params = crate::rgbd::Kinfu_Params;
#[inline]
pub fn make_volume(_volume_type: crate::rgbd::Kinfu_VolumeType, _voxel_size: f32, _pose: core::Matx44f, _raycast_step_factor: f32, _trunc_dist: f32, _max_weight: i32, _truncate_threshold: f32, _resolution: core::Vec3i) -> Result<core::Ptr<dyn crate::rgbd::Kinfu_Volume>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_kinfu_makeVolume_VolumeType_float_Matx44f_float_float_int_float_Vec3i(_volume_type, _voxel_size, _pose.opencv_as_extern(), _raycast_step_factor, _trunc_dist, _max_weight, _truncate_threshold, _resolution.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::rgbd::Kinfu_Volume>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn colormap(quantized: &core::Mat, dst: &mut core::Mat) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_linemod_colormap_const_MatR_MatR(quantized.as_raw_Mat(), dst.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * size: 10
#[inline]
pub fn draw_features(img: &mut dyn core::ToInputOutputArray, templates: &core::Vector<crate::rgbd::Linemod_Template>, tl: core::Point2i, size: i32) -> Result<()> {
	input_output_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_linemod_drawFeatures_const__InputOutputArrayR_const_vector_Template_R_const_Point2iR_int(img.as_raw__InputOutputArray(), templates.as_raw_VectorOfLinemod_Template(), &tl, size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn get_default_line() -> Result<core::Ptr<crate::rgbd::Linemod_Detector>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_linemod_getDefaultLINE(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::rgbd::Linemod_Detector>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn get_default_linemod() -> Result<core::Ptr<crate::rgbd::Linemod_Detector>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_linemod_getDefaultLINEMOD(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::rgbd::Linemod_Detector>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn depth_to3d_sparse(depth: &dyn core::ToInputArray, in_k: &dyn core::ToInputArray, in_points: &dyn core::ToInputArray, points3d: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(depth);
	input_array_arg!(in_k);
	input_array_arg!(in_points);
	output_array_arg!(points3d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_depthTo3dSparse_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(depth.as_raw__InputArray(), in_k.as_raw__InputArray(), in_points.as_raw__InputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * mask: noArray()
#[inline]
pub fn depth_to3d(depth: &dyn core::ToInputArray, k: &dyn core::ToInputArray, points3d: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(depth);
	input_array_arg!(k);
	output_array_arg!(points3d);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(depth.as_raw__InputArray(), k.as_raw__InputArray(), points3d.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn is_valid_depth_1(depth: &f64) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_isValidDepth_const_doubleR(depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn is_valid_depth(depth: &f32) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_isValidDepth_const_floatR(depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn is_valid_depth_4(depth: &i32) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_isValidDepth_const_intR(depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn is_valid_depth_2(depth: &i16) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_isValidDepth_const_shortR(depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn is_valid_depth_5(depth: &u32) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_isValidDepth_const_unsigned_intR(depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn is_valid_depth_3(depth: &u16) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_isValidDepth_const_unsigned_shortR(depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * depth_dilation: false
#[inline]
pub fn register_depth(unregistered_camera_matrix: &dyn core::ToInputArray, registered_camera_matrix: &dyn core::ToInputArray, registered_dist_coeffs: &dyn core::ToInputArray, rt: &dyn core::ToInputArray, unregistered_depth: &dyn core::ToInputArray, output_image_plane_size: core::Size, registered_depth: &mut dyn core::ToOutputArray, depth_dilation: bool) -> Result<()> {
	input_array_arg!(unregistered_camera_matrix);
	input_array_arg!(registered_camera_matrix);
	input_array_arg!(registered_dist_coeffs);
	input_array_arg!(rt);
	input_array_arg!(unregistered_depth);
	output_array_arg!(registered_depth);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR_bool(unregistered_camera_matrix.as_raw__InputArray(), registered_camera_matrix.as_raw__InputArray(), registered_dist_coeffs.as_raw__InputArray(), rt.as_raw__InputArray(), unregistered_depth.as_raw__InputArray(), &output_image_plane_size, registered_depth.as_raw__OutputArray(), depth_dilation, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * depth_factor: 1000.0
#[inline]
pub fn rescale_depth(in_: &dyn core::ToInputArray, depth: i32, out: &mut dyn core::ToOutputArray, depth_factor: f64) -> Result<()> {
	input_array_arg!(in_);
	output_array_arg!(out);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_rescaleDepth_const__InputArrayR_int_const__OutputArrayR_double(in_.as_raw__InputArray(), depth, out.as_raw__OutputArray(), depth_factor, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * warped_depth: noArray()
/// * warped_mask: noArray()
#[inline]
pub fn warp_frame(image: &core::Mat, depth: &core::Mat, mask: &core::Mat, rt: &core::Mat, camera_matrix: &core::Mat, dist_coeff: &core::Mat, warped_image: &mut dyn core::ToOutputArray, warped_depth: &mut dyn core::ToOutputArray, warped_mask: &mut dyn core::ToOutputArray) -> Result<()> {
	output_array_arg!(warped_image);
	output_array_arg!(warped_depth);
	output_array_arg!(warped_mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_warpFrame_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(image.as_raw_Mat(), depth.as_raw_Mat(), mask.as_raw_Mat(), rt.as_raw_Mat(), camera_matrix.as_raw_Mat(), dist_coeff.as_raw_Mat(), warped_image.as_raw__OutputArray(), warped_depth.as_raw__OutputArray(), warped_mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

pub trait ColoredKinfu_ColoredKinFuConst {
	fn as_raw_ColoredKinfu_ColoredKinFu(&self) -> *const c_void;

	#[inline]
	fn get_params(&self) -> Result<crate::rgbd::ColoredKinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_getParams_const(self.as_raw_ColoredKinfu_ColoredKinFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::ColoredKinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn render(&self, image: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR(self.as_raw_ColoredKinfu_ColoredKinFu(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn render_1(&self, image: &mut dyn core::ToOutputArray, camera_pose: core::Matx44f) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR_const_Matx44fR(self.as_raw_ColoredKinfu_ColoredKinFu(), image.as_raw__OutputArray(), &camera_pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * colors: noArray()
	#[inline]
	fn get_cloud(&self, points: &mut dyn core::ToOutputArray, normals: &mut dyn core::ToOutputArray, colors: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(normals);
		output_array_arg!(colors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_ColoredKinfu_ColoredKinFu(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), colors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_points(&self, points: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_getPoints_const_const__OutputArrayR(self.as_raw_ColoredKinfu_ColoredKinFu(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_normals(&self, points: &dyn core::ToInputArray, normals: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_ColoredKinfu_ColoredKinFu(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_pose(&self) -> Result<core::Affine3f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_getPose_const(self.as_raw_ColoredKinfu_ColoredKinFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait ColoredKinfu_ColoredKinFu: crate::rgbd::ColoredKinfu_ColoredKinFuConst {
	fn as_raw_mut_ColoredKinfu_ColoredKinFu(&mut self) -> *mut c_void;

	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_reset(self.as_raw_mut_ColoredKinfu_ColoredKinFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn update(&mut self, depth: &dyn core::ToInputArray, rgb: &dyn core::ToInputArray) -> Result<bool> {
		input_array_arg!(depth);
		input_array_arg!(rgb);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_update_const__InputArrayR_const__InputArrayR(self.as_raw_mut_ColoredKinfu_ColoredKinFu(), depth.as_raw__InputArray(), rgb.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn ColoredKinfu_ColoredKinFu + '_ {
	#[inline]
	pub fn create(_params: &core::Ptr<crate::rgbd::ColoredKinfu_Params>) -> Result<core::Ptr<dyn crate::rgbd::ColoredKinfu_ColoredKinFu>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_create_const_Ptr_Params_R(_params.as_raw_PtrOfColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::rgbd::ColoredKinfu_ColoredKinFu>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait ColoredKinfu_ParamsTraitConst {
	fn as_raw_ColoredKinfu_Params(&self) -> *const c_void;

	#[inline]
	fn frame_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_getPropFrameSize_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn rgb_frame_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_getPropRgb_frameSize_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn volume_type(&self) -> crate::rgbd::Kinfu_VolumeType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_getPropVolumeType_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn intr(&self) -> core::Matx33f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_getPropIntr_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn rgb_intr(&self) -> core::Matx33f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_getPropRgb_intr_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn depth_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_getPropDepthFactor_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}
	
	#[inline]
	fn bilateral_sigma_depth(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_getPropBilateral_sigma_depth_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}
	
	#[inline]
	fn bilateral_sigma_spatial(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_getPropBilateral_sigma_spatial_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}
	
	#[inline]
	fn bilateral_kernel_size(&self) -> i32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_getPropBilateral_kernel_size_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}
	
	#[inline]
	fn pyramid_levels(&self) -> i32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_getPropPyramidLevels_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}
	
	#[inline]
	fn volume_dims(&self) -> core::Vec3i {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_getPropVolumeDims_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn voxel_size(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_getPropVoxelSize_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}
	
	#[inline]
	fn tsdf_min_camera_movement(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_getPropTsdf_min_camera_movement_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}
	
	#[inline]
	fn volume_pose(&self) -> core::Affine3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_getPropVolumePose_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn tsdf_trunc_dist(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_getPropTsdf_trunc_dist_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}
	
	#[inline]
	fn tsdf_max_weight(&self) -> i32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_getPropTsdf_max_weight_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}
	
	#[inline]
	fn raycast_step_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_getPropRaycast_step_factor_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}
	
	#[inline]
	fn light_pose(&self) -> core::Vec3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_getPropLightPose_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn icp_dist_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_getPropIcpDistThresh_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}
	
	#[inline]
	fn icp_angle_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_getPropIcpAngleThresh_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}
	
	#[inline]
	fn icp_iterations(&self) -> core::Vector<i32> {
		let ret = unsafe { sys::cv_colored_kinfu_Params_getPropIcpIterations_const(self.as_raw_ColoredKinfu_Params()) };
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn truncate_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_getPropTruncateThreshold_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}
	
}

pub trait ColoredKinfu_ParamsTrait: crate::rgbd::ColoredKinfu_ParamsTraitConst {
	fn as_raw_mut_ColoredKinfu_Params(&mut self) -> *mut c_void;

	#[inline]
	fn set_frame_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropFrameSize_Size(self.as_raw_mut_ColoredKinfu_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_rgb_frame_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropRgb_frameSize_Size(self.as_raw_mut_ColoredKinfu_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_volume_type(&mut self, val: crate::rgbd::Kinfu_VolumeType) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropVolumeType_VolumeType(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_intr(&mut self, val: core::Matx33f) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropIntr_Matx33f(self.as_raw_mut_ColoredKinfu_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_rgb_intr(&mut self, val: core::Matx33f) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropRgb_intr_Matx33f(self.as_raw_mut_ColoredKinfu_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_depth_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropDepthFactor_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_bilateral_sigma_depth(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropBilateral_sigma_depth_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_bilateral_sigma_spatial(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropBilateral_sigma_spatial_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_bilateral_kernel_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropBilateral_kernel_size_int(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_pyramid_levels(&mut self, val: i32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropPyramidLevels_int(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_volume_dims(&mut self, val: core::Vec3i) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropVolumeDims_Vec3i(self.as_raw_mut_ColoredKinfu_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_voxel_size(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropVoxelSize_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_tsdf_min_camera_movement(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropTsdf_min_camera_movement_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_volume_pose(&mut self, val: core::Affine3f) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropVolumePose_Affine3f(self.as_raw_mut_ColoredKinfu_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_tsdf_trunc_dist(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropTsdf_trunc_dist_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_tsdf_max_weight(&mut self, val: i32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropTsdf_max_weight_int(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_raycast_step_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropRaycast_step_factor_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_light_pose(&mut self, val: core::Vec3f) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropLightPose_Vec3f(self.as_raw_mut_ColoredKinfu_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_icp_dist_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropIcpDistThresh_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_icp_angle_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropIcpAngleThresh_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_icp_iterations(&mut self, mut val: core::Vector<i32>) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropIcpIterations_vector_int_(self.as_raw_mut_ColoredKinfu_Params(), val.as_raw_mut_VectorOfi32()) };
		ret
	}
	
	#[inline]
	fn set_truncate_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_setPropTruncateThreshold_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_initial_volume_pose(&mut self, r: core::Matx33f, t: core::Vec3f) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(self.as_raw_mut_ColoredKinfu_Params(), r.opencv_as_extern(), t.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_initial_volume_pose_1(&mut self, homogen_tf: core::Matx44f) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_setInitialVolumePose_Matx44f(self.as_raw_mut_ColoredKinfu_Params(), homogen_tf.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct ColoredKinfu_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { ColoredKinfu_Params }

impl Drop for ColoredKinfu_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_ColoredKinfu_Params_delete(instance: *mut c_void); }
		unsafe { cv_ColoredKinfu_Params_delete(self.as_raw_mut_ColoredKinfu_Params()) };
	}
}

unsafe impl Send for ColoredKinfu_Params {}

impl crate::rgbd::ColoredKinfu_ParamsTraitConst for ColoredKinfu_Params {
	#[inline] fn as_raw_ColoredKinfu_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::ColoredKinfu_ParamsTrait for ColoredKinfu_Params {
	#[inline] fn as_raw_mut_ColoredKinfu_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ColoredKinfu_Params {
	#[inline]
	pub fn default() -> Result<crate::rgbd::ColoredKinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::ColoredKinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new(volume_initial_pose_rot: core::Matx33f, volume_initial_pose_transl: core::Vec3f) -> Result<crate::rgbd::ColoredKinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_Params_Matx33f_Vec3f(volume_initial_pose_rot.opencv_as_extern(), volume_initial_pose_transl.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::ColoredKinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new_1(volume_initial_pose: core::Matx44f) -> Result<crate::rgbd::ColoredKinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_Params_Matx44f(volume_initial_pose.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::ColoredKinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn default_params() -> Result<core::Ptr<crate::rgbd::ColoredKinfu_Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_defaultParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::ColoredKinfu_Params>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn coarse_params() -> Result<core::Ptr<crate::rgbd::ColoredKinfu_Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_coarseParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::ColoredKinfu_Params>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn hash_tsdf_params(is_coarse: bool) -> Result<core::Ptr<crate::rgbd::ColoredKinfu_Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_hashTSDFParams_bool(is_coarse, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::ColoredKinfu_Params>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn colored_tsdf_params(is_coarse: bool) -> Result<core::Ptr<crate::rgbd::ColoredKinfu_Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_coloredTSDFParams_bool(is_coarse, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::ColoredKinfu_Params>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait Dynafu_DynaFuConst {
	fn as_raw_Dynafu_DynaFu(&self) -> *const c_void;

	#[inline]
	fn get_params(&self) -> Result<crate::rgbd::Kinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_getParams_const(self.as_raw_Dynafu_DynaFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Kinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * camera_pose: Matx44f::eye()
	#[inline]
	fn render(&self, image: &mut dyn core::ToOutputArray, camera_pose: core::Matx44f) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_render_const_const__OutputArrayR_const_Matx44fR(self.as_raw_Dynafu_DynaFu(), image.as_raw__OutputArray(), &camera_pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_cloud(&self, points: &mut dyn core::ToOutputArray, normals: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_Dynafu_DynaFu(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_points(&self, points: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_getPoints_const_const__OutputArrayR(self.as_raw_Dynafu_DynaFu(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_normals(&self, points: &dyn core::ToInputArray, normals: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_getNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_Dynafu_DynaFu(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_pose(&self) -> Result<core::Affine3f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_getPose_const(self.as_raw_Dynafu_DynaFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_nodes_pos(&self) -> Result<core::Vector<core::Point3f>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_getNodesPos_const(self.as_raw_Dynafu_DynaFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Point3f>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn march_cubes(&self, vertices: &mut dyn core::ToOutputArray, edges: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(vertices);
		output_array_arg!(edges);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_marchCubes_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_Dynafu_DynaFu(), vertices.as_raw__OutputArray(), edges.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Dynafu_DynaFu: crate::rgbd::Dynafu_DynaFuConst {
	fn as_raw_mut_Dynafu_DynaFu(&mut self) -> *mut c_void;

	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_reset(self.as_raw_mut_Dynafu_DynaFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn update(&mut self, depth: &dyn core::ToInputArray) -> Result<bool> {
		input_array_arg!(depth);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_update_const__InputArrayR(self.as_raw_mut_Dynafu_DynaFu(), depth.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * warp: true
	#[inline]
	fn render_surface(&mut self, depth_image: &mut dyn core::ToOutputArray, vert_image: &mut dyn core::ToOutputArray, norm_image: &mut dyn core::ToOutputArray, warp: bool) -> Result<()> {
		output_array_arg!(depth_image);
		output_array_arg!(vert_image);
		output_array_arg!(norm_image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_renderSurface_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_bool(self.as_raw_mut_Dynafu_DynaFu(), depth_image.as_raw__OutputArray(), vert_image.as_raw__OutputArray(), norm_image.as_raw__OutputArray(), warp, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn Dynafu_DynaFu + '_ {
	#[inline]
	pub fn create(_params: &core::Ptr<crate::rgbd::Kinfu_Params>) -> Result<core::Ptr<dyn crate::rgbd::Dynafu_DynaFu>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_create_const_Ptr_Params_R(_params.as_raw_PtrOfKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::rgbd::Dynafu_DynaFu>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Kinfu_Intr {
	pub fx: f32,
	pub fy: f32,
	pub cx: f32,
	pub cy: f32,
}

opencv_type_simple! { crate::rgbd::Kinfu_Intr }

impl Kinfu_Intr {
	#[inline]
	pub fn scale(self, pyr: i32) -> Result<crate::rgbd::Kinfu_Intr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_scale_const_int(self.opencv_as_extern(), pyr, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn make_reprojector(self) -> Result<crate::rgbd::Kinfu_Intr_Reprojector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_makeReprojector_const(self.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn make_projector(self) -> Result<crate::rgbd::Kinfu_Intr_Projector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_makeProjector_const(self.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn get_mat(self) -> Result<core::Matx33f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_getMat_const(self.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn default() -> Result<crate::rgbd::Kinfu_Intr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_Intr(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn new(_fx: f32, _fy: f32, _cx: f32, _cy: f32) -> Result<crate::rgbd::Kinfu_Intr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_Intr_float_float_float_float(_fx, _fy, _cx, _cy, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn new_1(m: core::Matx33f) -> Result<crate::rgbd::Kinfu_Intr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_Intr_Matx33f(m.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Kinfu_Intr_Projector {
	pub fx: f32,
	pub fy: f32,
	pub cx: f32,
	pub cy: f32,
}

opencv_type_simple! { crate::rgbd::Kinfu_Intr_Projector }

impl Kinfu_Intr_Projector {
	#[inline]
	pub fn new(intr: crate::rgbd::Kinfu_Intr) -> Result<crate::rgbd::Kinfu_Intr_Projector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_Projector_Projector_Intr(intr.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Kinfu_Intr_Reprojector {
	pub fxinv: f32,
	pub fyinv: f32,
	pub cx: f32,
	pub cy: f32,
}

opencv_type_simple! { crate::rgbd::Kinfu_Intr_Reprojector }

impl Kinfu_Intr_Reprojector {
	#[inline]
	pub fn default() -> Result<crate::rgbd::Kinfu_Intr_Reprojector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_Reprojector_Reprojector(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn new(intr: crate::rgbd::Kinfu_Intr) -> Result<crate::rgbd::Kinfu_Intr_Reprojector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_Reprojector_Reprojector_Intr(intr.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Kinfu_KinFuConst {
	fn as_raw_Kinfu_KinFu(&self) -> *const c_void;

	#[inline]
	fn get_params(&self) -> Result<crate::rgbd::Kinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_getParams_const(self.as_raw_Kinfu_KinFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Kinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn render(&self, image: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_render_const_const__OutputArrayR(self.as_raw_Kinfu_KinFu(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn render_1(&self, image: &mut dyn core::ToOutputArray, camera_pose: core::Matx44f) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_render_const_const__OutputArrayR_const_Matx44fR(self.as_raw_Kinfu_KinFu(), image.as_raw__OutputArray(), &camera_pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_cloud(&self, points: &mut dyn core::ToOutputArray, normals: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_Kinfu_KinFu(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_points(&self, points: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_getPoints_const_const__OutputArrayR(self.as_raw_Kinfu_KinFu(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_normals(&self, points: &dyn core::ToInputArray, normals: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_Kinfu_KinFu(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_pose(&self) -> Result<core::Affine3f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_getPose_const(self.as_raw_Kinfu_KinFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Kinfu_KinFu: crate::rgbd::Kinfu_KinFuConst {
	fn as_raw_mut_Kinfu_KinFu(&mut self) -> *mut c_void;

	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_reset(self.as_raw_mut_Kinfu_KinFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn update(&mut self, depth: &dyn core::ToInputArray) -> Result<bool> {
		input_array_arg!(depth);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_update_const__InputArrayR(self.as_raw_mut_Kinfu_KinFu(), depth.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn Kinfu_KinFu + '_ {
	#[inline]
	pub fn create(_params: &core::Ptr<crate::rgbd::Kinfu_Params>) -> Result<core::Ptr<dyn crate::rgbd::Kinfu_KinFu>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_create_const_Ptr_Params_R(_params.as_raw_PtrOfKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::rgbd::Kinfu_KinFu>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait Kinfu_ParamsTraitConst {
	fn as_raw_Kinfu_Params(&self) -> *const c_void;

	#[inline]
	fn frame_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_getPropFrameSize_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn volume_type(&self) -> crate::rgbd::Kinfu_VolumeType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_getPropVolumeType_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn intr(&self) -> core::Matx33f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_getPropIntr_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn rgb_intr(&self) -> core::Matx33f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_getPropRgb_intr_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn depth_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_getPropDepthFactor_const(self.as_raw_Kinfu_Params()) };
		ret
	}
	
	#[inline]
	fn bilateral_sigma_depth(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_getPropBilateral_sigma_depth_const(self.as_raw_Kinfu_Params()) };
		ret
	}
	
	#[inline]
	fn bilateral_sigma_spatial(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_getPropBilateral_sigma_spatial_const(self.as_raw_Kinfu_Params()) };
		ret
	}
	
	#[inline]
	fn bilateral_kernel_size(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_Params_getPropBilateral_kernel_size_const(self.as_raw_Kinfu_Params()) };
		ret
	}
	
	#[inline]
	fn pyramid_levels(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_Params_getPropPyramidLevels_const(self.as_raw_Kinfu_Params()) };
		ret
	}
	
	#[inline]
	fn volume_dims(&self) -> core::Vec3i {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_getPropVolumeDims_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn voxel_size(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_getPropVoxelSize_const(self.as_raw_Kinfu_Params()) };
		ret
	}
	
	#[inline]
	fn tsdf_min_camera_movement(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_getPropTsdf_min_camera_movement_const(self.as_raw_Kinfu_Params()) };
		ret
	}
	
	#[inline]
	fn volume_pose(&self) -> core::Affine3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_getPropVolumePose_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn tsdf_trunc_dist(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_getPropTsdf_trunc_dist_const(self.as_raw_Kinfu_Params()) };
		ret
	}
	
	#[inline]
	fn tsdf_max_weight(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_Params_getPropTsdf_max_weight_const(self.as_raw_Kinfu_Params()) };
		ret
	}
	
	#[inline]
	fn raycast_step_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_getPropRaycast_step_factor_const(self.as_raw_Kinfu_Params()) };
		ret
	}
	
	#[inline]
	fn light_pose(&self) -> core::Vec3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_getPropLightPose_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn icp_dist_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_getPropIcpDistThresh_const(self.as_raw_Kinfu_Params()) };
		ret
	}
	
	#[inline]
	fn icp_angle_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_getPropIcpAngleThresh_const(self.as_raw_Kinfu_Params()) };
		ret
	}
	
	#[inline]
	fn icp_iterations(&self) -> core::Vector<i32> {
		let ret = unsafe { sys::cv_kinfu_Params_getPropIcpIterations_const(self.as_raw_Kinfu_Params()) };
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn truncate_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_getPropTruncateThreshold_const(self.as_raw_Kinfu_Params()) };
		ret
	}
	
}

pub trait Kinfu_ParamsTrait: crate::rgbd::Kinfu_ParamsTraitConst {
	fn as_raw_mut_Kinfu_Params(&mut self) -> *mut c_void;

	#[inline]
	fn set_frame_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropFrameSize_Size(self.as_raw_mut_Kinfu_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_volume_type(&mut self, val: crate::rgbd::Kinfu_VolumeType) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropVolumeType_VolumeType(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_intr(&mut self, val: core::Matx33f) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropIntr_Matx33f(self.as_raw_mut_Kinfu_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_rgb_intr(&mut self, val: core::Matx33f) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropRgb_intr_Matx33f(self.as_raw_mut_Kinfu_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_depth_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropDepthFactor_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_bilateral_sigma_depth(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropBilateral_sigma_depth_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_bilateral_sigma_spatial(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropBilateral_sigma_spatial_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_bilateral_kernel_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropBilateral_kernel_size_int(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_pyramid_levels(&mut self, val: i32) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropPyramidLevels_int(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_volume_dims(&mut self, val: core::Vec3i) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropVolumeDims_Vec3i(self.as_raw_mut_Kinfu_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_voxel_size(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropVoxelSize_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_tsdf_min_camera_movement(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropTsdf_min_camera_movement_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_volume_pose(&mut self, val: core::Affine3f) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropVolumePose_Affine3f(self.as_raw_mut_Kinfu_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_tsdf_trunc_dist(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropTsdf_trunc_dist_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_tsdf_max_weight(&mut self, val: i32) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropTsdf_max_weight_int(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_raycast_step_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropRaycast_step_factor_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_light_pose(&mut self, val: core::Vec3f) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropLightPose_Vec3f(self.as_raw_mut_Kinfu_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_icp_dist_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropIcpDistThresh_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_icp_angle_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropIcpAngleThresh_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_icp_iterations(&mut self, mut val: core::Vector<i32>) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropIcpIterations_vector_int_(self.as_raw_mut_Kinfu_Params(), val.as_raw_mut_VectorOfi32()) };
		ret
	}
	
	#[inline]
	fn set_truncate_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_setPropTruncateThreshold_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_initial_volume_pose(&mut self, r: core::Matx33f, t: core::Vec3f) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(self.as_raw_mut_Kinfu_Params(), r.opencv_as_extern(), t.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_initial_volume_pose_1(&mut self, homogen_tf: core::Matx44f) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_setInitialVolumePose_Matx44f(self.as_raw_mut_Kinfu_Params(), homogen_tf.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct Kinfu_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { Kinfu_Params }

impl Drop for Kinfu_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_Kinfu_Params_delete(instance: *mut c_void); }
		unsafe { cv_Kinfu_Params_delete(self.as_raw_mut_Kinfu_Params()) };
	}
}

unsafe impl Send for Kinfu_Params {}

impl crate::rgbd::Kinfu_ParamsTraitConst for Kinfu_Params {
	#[inline] fn as_raw_Kinfu_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Kinfu_ParamsTrait for Kinfu_Params {
	#[inline] fn as_raw_mut_Kinfu_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Kinfu_Params {
	#[inline]
	pub fn default() -> Result<crate::rgbd::Kinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Kinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new(volume_initial_pose_rot: core::Matx33f, volume_initial_pose_transl: core::Vec3f) -> Result<crate::rgbd::Kinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_Params_Matx33f_Vec3f(volume_initial_pose_rot.opencv_as_extern(), volume_initial_pose_transl.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Kinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new_1(volume_initial_pose: core::Matx44f) -> Result<crate::rgbd::Kinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_Params_Matx44f(volume_initial_pose.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Kinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn default_params() -> Result<core::Ptr<crate::rgbd::Kinfu_Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_defaultParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_Params>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn coarse_params() -> Result<core::Ptr<crate::rgbd::Kinfu_Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_coarseParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_Params>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn hash_tsdf_params(is_coarse: bool) -> Result<core::Ptr<crate::rgbd::Kinfu_Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_hashTSDFParams_bool(is_coarse, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_Params>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn colored_tsdf_params(is_coarse: bool) -> Result<core::Ptr<crate::rgbd::Kinfu_Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_coloredTSDFParams_bool(is_coarse, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_Params>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait Kinfu_VolumeConst {
	fn as_raw_Kinfu_Volume(&self) -> *const c_void;

	#[inline]
	fn voxel_size(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Volume_getPropVoxelSize_const(self.as_raw_Kinfu_Volume()) };
		ret
	}
	
	#[inline]
	fn voxel_size_inv(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Volume_getPropVoxelSizeInv_const(self.as_raw_Kinfu_Volume()) };
		ret
	}
	
	#[inline]
	fn pose(&self) -> core::Affine3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_getPropPose_const(self.as_raw_Kinfu_Volume(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn raycast_step_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Volume_getPropRaycastStepFactor_const(self.as_raw_Kinfu_Volume()) };
		ret
	}
	
	#[inline]
	fn raycast(&self, camera_pose: core::Matx44f, intrinsics: crate::rgbd::Kinfu_Intr, frame_size: core::Size, points: &mut dyn core::ToOutputArray, normals: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_raycast_const_const_Matx44fR_const_IntrR_const_SizeR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Kinfu_Volume(), &camera_pose, &intrinsics, &frame_size, points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn raycast_1(&self, camera_pose: core::Matx44f, intrinsics: crate::rgbd::Kinfu_Intr, frame_size: core::Size, points: &mut dyn core::ToOutputArray, normals: &mut dyn core::ToOutputArray, colors: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(normals);
		output_array_arg!(colors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_raycast_const_const_Matx44fR_const_IntrR_const_SizeR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Kinfu_Volume(), &camera_pose, &intrinsics, &frame_size, points.as_raw__OutputArray(), normals.as_raw__OutputArray(), colors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn fetch_normals(&self, points: &dyn core::ToInputArray, _normals: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(_normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_fetchNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_Kinfu_Volume(), points.as_raw__InputArray(), _normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn fetch_points_normals(&self, points: &mut dyn core::ToOutputArray, normals: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_fetchPointsNormals_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_Kinfu_Volume(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn fetch_points_normals_colors(&self, unnamed: &mut dyn core::ToOutputArray, unnamed_1: &mut dyn core::ToOutputArray, unnamed_2: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(unnamed);
		output_array_arg!(unnamed_1);
		output_array_arg!(unnamed_2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_fetchPointsNormalsColors_const_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Kinfu_Volume(), unnamed.as_raw__OutputArray(), unnamed_1.as_raw__OutputArray(), unnamed_2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Kinfu_Volume: crate::rgbd::Kinfu_VolumeConst {
	fn as_raw_mut_Kinfu_Volume(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * frame_id: 0
	#[inline]
	fn integrate(&mut self, _depth: &dyn core::ToInputArray, depth_factor: f32, camera_pose: core::Matx44f, intrinsics: crate::rgbd::Kinfu_Intr, frame_id: i32) -> Result<()> {
		input_array_arg!(_depth);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_integrate_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_int(self.as_raw_mut_Kinfu_Volume(), _depth.as_raw__InputArray(), depth_factor, &camera_pose, &intrinsics, frame_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * frame_id: 0
	#[inline]
	fn integrate_1(&mut self, _depth: &dyn core::ToInputArray, _rgb: &dyn core::ToInputArray, depth_factor: f32, camera_pose: core::Matx44f, intrinsics: crate::rgbd::Kinfu_Intr, rgb_intrinsics: crate::rgbd::Kinfu_Intr, frame_id: i32) -> Result<()> {
		input_array_arg!(_depth);
		input_array_arg!(_rgb);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_integrate_const__InputArrayR_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_IntrR_const_int(self.as_raw_mut_Kinfu_Volume(), _depth.as_raw__InputArray(), _rgb.as_raw__InputArray(), depth_factor, &camera_pose, &intrinsics, &rgb_intrinsics, frame_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_reset(self.as_raw_mut_Kinfu_Volume(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Kinfu_VolumeParamsTraitConst {
	fn as_raw_Kinfu_VolumeParams(&self) -> *const c_void;

	#[inline]
	fn typ(&self) -> crate::rgbd::Kinfu_VolumeType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_VolumeParams_getPropType_const(self.as_raw_Kinfu_VolumeParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn resolution(&self) -> core::Vec3i {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_VolumeParams_getPropResolution_const(self.as_raw_Kinfu_VolumeParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn unit_resolution(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_getPropUnitResolution_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}
	
	#[inline]
	fn pose(&self) -> core::Affine3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_VolumeParams_getPropPose_const(self.as_raw_Kinfu_VolumeParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn voxel_size(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_getPropVoxelSize_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}
	
	#[inline]
	fn tsdf_trunc_dist(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_getPropTsdfTruncDist_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}
	
	#[inline]
	fn max_weight(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_getPropMaxWeight_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}
	
	#[inline]
	fn depth_trunc_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_getPropDepthTruncThreshold_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}
	
	#[inline]
	fn raycast_step_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_getPropRaycastStepFactor_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}
	
}

pub trait Kinfu_VolumeParamsTrait: crate::rgbd::Kinfu_VolumeParamsTraitConst {
	fn as_raw_mut_Kinfu_VolumeParams(&mut self) -> *mut c_void;

	#[inline]
	fn set_type(&mut self, val: crate::rgbd::Kinfu_VolumeType) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_setPropType_VolumeType(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}
	
	#[inline]
	fn set_resolution(&mut self, val: core::Vec3i) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_setPropResolution_Vec3i(self.as_raw_mut_Kinfu_VolumeParams(), val.opencv_as_extern()) };
		ret
	}
	
	/// ## C++ default parameters
	/// * val: {0}
	#[inline]
	fn set_unit_resolution(&mut self, val: i32) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_setPropUnitResolution_int(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}
	
	#[inline]
	fn set_pose(&mut self, val: core::Affine3f) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_setPropPose_Affine3f(self.as_raw_mut_Kinfu_VolumeParams(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_voxel_size(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_setPropVoxelSize_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}
	
	#[inline]
	fn set_tsdf_trunc_dist(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_setPropTsdfTruncDist_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}
	
	#[inline]
	fn set_max_weight(&mut self, val: i32) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_setPropMaxWeight_int(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}
	
	#[inline]
	fn set_depth_trunc_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_setPropDepthTruncThreshold_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}
	
	#[inline]
	fn set_raycast_step_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_setPropRaycastStepFactor_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}
	
}

pub struct Kinfu_VolumeParams {
	ptr: *mut c_void
}

opencv_type_boxed! { Kinfu_VolumeParams }

impl Drop for Kinfu_VolumeParams {
	fn drop(&mut self) {
		extern "C" { fn cv_Kinfu_VolumeParams_delete(instance: *mut c_void); }
		unsafe { cv_Kinfu_VolumeParams_delete(self.as_raw_mut_Kinfu_VolumeParams()) };
	}
}

unsafe impl Send for Kinfu_VolumeParams {}

impl crate::rgbd::Kinfu_VolumeParamsTraitConst for Kinfu_VolumeParams {
	#[inline] fn as_raw_Kinfu_VolumeParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Kinfu_VolumeParamsTrait for Kinfu_VolumeParams {
	#[inline] fn as_raw_mut_Kinfu_VolumeParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Kinfu_VolumeParams {
	#[inline]
	pub fn default_params(_volume_type: crate::rgbd::Kinfu_VolumeType) -> Result<core::Ptr<crate::rgbd::Kinfu_VolumeParams>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_VolumeParams_defaultParams_VolumeType(_volume_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_VolumeParams>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn coarse_params(_volume_type: crate::rgbd::Kinfu_VolumeType) -> Result<core::Ptr<crate::rgbd::Kinfu_VolumeParams>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_VolumeParams_coarseParams_VolumeType(_volume_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_VolumeParams>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait Kinfu_Detail_PoseGraphConst {
	fn as_raw_Kinfu_Detail_PoseGraph(&self) -> *const c_void;

	#[inline]
	fn is_node_exist(&self, node_id: size_t) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_isNodeExist_const_size_t(self.as_raw_Kinfu_Detail_PoseGraph(), node_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn is_node_fixed(&self, node_id: size_t) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_isNodeFixed_const_size_t(self.as_raw_Kinfu_Detail_PoseGraph(), node_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_node_pose(&self, node_id: size_t) -> Result<core::Affine3d> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_getNodePose_const_size_t(self.as_raw_Kinfu_Detail_PoseGraph(), node_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_nodes_ids(&self) -> Result<core::Vector<size_t>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_getNodesIds_const(self.as_raw_Kinfu_Detail_PoseGraph(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_num_nodes(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_getNumNodes_const(self.as_raw_Kinfu_Detail_PoseGraph(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_edge_start(&self, i: size_t) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_getEdgeStart_const_size_t(self.as_raw_Kinfu_Detail_PoseGraph(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_edge_end(&self, i: size_t) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_getEdgeEnd_const_size_t(self.as_raw_Kinfu_Detail_PoseGraph(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_num_edges(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_getNumEdges_const(self.as_raw_Kinfu_Detail_PoseGraph(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn is_valid(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_isValid_const(self.as_raw_Kinfu_Detail_PoseGraph(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn calc_energy(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_calcEnergy_const(self.as_raw_Kinfu_Detail_PoseGraph(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Kinfu_Detail_PoseGraph: crate::rgbd::Kinfu_Detail_PoseGraphConst {
	fn as_raw_mut_Kinfu_Detail_PoseGraph(&mut self) -> *mut c_void;

	#[inline]
	fn add_node(&mut self, _node_id: size_t, _pose: core::Affine3d, fixed: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_addNode_size_t_const_Affine3dR_bool(self.as_raw_mut_Kinfu_Detail_PoseGraph(), _node_id, &_pose, fixed, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_node_fixed(&mut self, node_id: size_t, fixed: bool) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_setNodeFixed_size_t_bool(self.as_raw_mut_Kinfu_Detail_PoseGraph(), node_id, fixed, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * _information: Matx66f::eye()
	#[inline]
	fn add_edge(&mut self, _source_node_id: size_t, _target_node_id: size_t, _transformation: core::Affine3f, _information: core::Matx66f) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_addEdge_size_t_size_t_const_Affine3fR_const_Matx66fR(self.as_raw_mut_Kinfu_Detail_PoseGraph(), _source_node_id, _target_node_id, &_transformation, &_information, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * tc: cv::TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,1e-6)
	#[inline]
	fn optimize(&mut self, tc: core::TermCriteria) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_optimize_const_TermCriteriaR(self.as_raw_mut_Kinfu_Detail_PoseGraph(), &tc, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn Kinfu_Detail_PoseGraph + '_ {
	#[inline]
	pub fn create() -> Result<core::Ptr<dyn crate::rgbd::Kinfu_Detail_PoseGraph>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::rgbd::Kinfu_Detail_PoseGraph>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait LargeKinfuConst {
	fn as_raw_LargeKinfu(&self) -> *const c_void;

	#[inline]
	fn get_params(&self) -> Result<crate::rgbd::Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_getParams_const(self.as_raw_LargeKinfu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn render(&self, image: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR(self.as_raw_LargeKinfu(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn render_1(&self, image: &mut dyn core::ToOutputArray, camera_pose: core::Matx44f) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR_const_Matx44fR(self.as_raw_LargeKinfu(), image.as_raw__OutputArray(), &camera_pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_cloud(&self, points: &mut dyn core::ToOutputArray, normals: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_getCloud_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_LargeKinfu(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_points(&self, points: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_getPoints_const_const__OutputArrayR(self.as_raw_LargeKinfu(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_normals(&self, points: &dyn core::ToInputArray, normals: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_getNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_LargeKinfu(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_pose(&self) -> Result<core::Affine3f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_getPose_const(self.as_raw_LargeKinfu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait LargeKinfu: crate::rgbd::LargeKinfuConst {
	fn as_raw_mut_LargeKinfu(&mut self) -> *mut c_void;

	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_reset(self.as_raw_mut_LargeKinfu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn update(&mut self, depth: &dyn core::ToInputArray) -> Result<bool> {
		input_array_arg!(depth);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_update_const__InputArrayR(self.as_raw_mut_LargeKinfu(), depth.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn LargeKinfu + '_ {
	#[inline]
	pub fn create(_params: &core::Ptr<crate::rgbd::Params>) -> Result<core::Ptr<dyn crate::rgbd::LargeKinfu>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_create_const_Ptr_Params_R(_params.as_raw_PtrOfParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::rgbd::LargeKinfu>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait ParamsTraitConst {
	fn as_raw_Params(&self) -> *const c_void;

	#[inline]
	fn frame_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_getPropFrameSize_const(self.as_raw_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn intr(&self) -> core::Matx33f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_getPropIntr_const(self.as_raw_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn rgb_intr(&self) -> core::Matx33f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_getPropRgb_intr_const(self.as_raw_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn depth_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_getPropDepthFactor_const(self.as_raw_Params()) };
		ret
	}
	
	#[inline]
	fn bilateral_sigma_depth(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_getPropBilateral_sigma_depth_const(self.as_raw_Params()) };
		ret
	}
	
	#[inline]
	fn bilateral_sigma_spatial(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_getPropBilateral_sigma_spatial_const(self.as_raw_Params()) };
		ret
	}
	
	#[inline]
	fn bilateral_kernel_size(&self) -> i32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_getPropBilateral_kernel_size_const(self.as_raw_Params()) };
		ret
	}
	
	#[inline]
	fn pyramid_levels(&self) -> i32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_getPropPyramidLevels_const(self.as_raw_Params()) };
		ret
	}
	
	#[inline]
	fn tsdf_min_camera_movement(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_getPropTsdf_min_camera_movement_const(self.as_raw_Params()) };
		ret
	}
	
	#[inline]
	fn light_pose(&self) -> core::Vec3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_getPropLightPose_const(self.as_raw_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn icp_dist_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_getPropIcpDistThresh_const(self.as_raw_Params()) };
		ret
	}
	
	#[inline]
	fn icp_angle_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_getPropIcpAngleThresh_const(self.as_raw_Params()) };
		ret
	}
	
	#[inline]
	fn icp_iterations(&self) -> core::Vector<i32> {
		let ret = unsafe { sys::cv_large_kinfu_Params_getPropIcpIterations_const(self.as_raw_Params()) };
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn truncate_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_getPropTruncateThreshold_const(self.as_raw_Params()) };
		ret
	}
	
	#[inline]
	fn volume_params(&self) -> crate::rgbd::Kinfu_VolumeParams {
		let ret = unsafe { sys::cv_large_kinfu_Params_getPropVolumeParams_const(self.as_raw_Params()) };
		let ret = unsafe { crate::rgbd::Kinfu_VolumeParams::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait ParamsTrait: crate::rgbd::ParamsTraitConst {
	fn as_raw_mut_Params(&mut self) -> *mut c_void;

	#[inline]
	fn set_frame_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_large_kinfu_Params_setPropFrameSize_Size(self.as_raw_mut_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_intr(&mut self, val: core::Matx33f) {
		let ret = unsafe { sys::cv_large_kinfu_Params_setPropIntr_Matx33f(self.as_raw_mut_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_rgb_intr(&mut self, val: core::Matx33f) {
		let ret = unsafe { sys::cv_large_kinfu_Params_setPropRgb_intr_Matx33f(self.as_raw_mut_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_depth_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_setPropDepthFactor_float(self.as_raw_mut_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_bilateral_sigma_depth(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_setPropBilateral_sigma_depth_float(self.as_raw_mut_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_bilateral_sigma_spatial(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_setPropBilateral_sigma_spatial_float(self.as_raw_mut_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_bilateral_kernel_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_setPropBilateral_kernel_size_int(self.as_raw_mut_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_pyramid_levels(&mut self, val: i32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_setPropPyramidLevels_int(self.as_raw_mut_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_tsdf_min_camera_movement(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_setPropTsdf_min_camera_movement_float(self.as_raw_mut_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_light_pose(&mut self, val: core::Vec3f) {
		let ret = unsafe { sys::cv_large_kinfu_Params_setPropLightPose_Vec3f(self.as_raw_mut_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_icp_dist_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_setPropIcpDistThresh_float(self.as_raw_mut_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_icp_angle_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_setPropIcpAngleThresh_float(self.as_raw_mut_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_icp_iterations(&mut self, mut val: core::Vector<i32>) {
		let ret = unsafe { sys::cv_large_kinfu_Params_setPropIcpIterations_vector_int_(self.as_raw_mut_Params(), val.as_raw_mut_VectorOfi32()) };
		ret
	}
	
	#[inline]
	fn set_truncate_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_setPropTruncateThreshold_float(self.as_raw_mut_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_volume_params(&mut self, mut val: crate::rgbd::Kinfu_VolumeParams) {
		let ret = unsafe { sys::cv_large_kinfu_Params_setPropVolumeParams_VolumeParams(self.as_raw_mut_Params(), val.as_raw_mut_Kinfu_VolumeParams()) };
		ret
	}
	
}

pub struct Params {
	ptr: *mut c_void
}

opencv_type_boxed! { Params }

impl Drop for Params {
	fn drop(&mut self) {
		extern "C" { fn cv_Params_delete(instance: *mut c_void); }
		unsafe { cv_Params_delete(self.as_raw_mut_Params()) };
	}
}

unsafe impl Send for Params {}

impl crate::rgbd::ParamsTraitConst for Params {
	#[inline] fn as_raw_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::ParamsTrait for Params {
	#[inline] fn as_raw_mut_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Params {
	#[inline]
	pub fn default_params() -> Result<core::Ptr<crate::rgbd::Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_defaultParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Params>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn coarse_params() -> Result<core::Ptr<crate::rgbd::Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_coarseParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Params>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn hash_tsdf_params(is_coarse: bool) -> Result<core::Ptr<crate::rgbd::Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_hashTSDFParams_bool(is_coarse, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Params>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait Linemod_ColorGradientTraitConst: crate::rgbd::Linemod_ModalityConst {
	fn as_raw_Linemod_ColorGradient(&self) -> *const c_void;

	#[inline]
	fn weak_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_linemod_ColorGradient_getPropWeak_threshold_const(self.as_raw_Linemod_ColorGradient()) };
		ret
	}
	
	#[inline]
	fn num_features(&self) -> size_t {
		let ret = unsafe { sys::cv_linemod_ColorGradient_getPropNum_features_const(self.as_raw_Linemod_ColorGradient()) };
		ret
	}
	
	#[inline]
	fn strong_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_linemod_ColorGradient_getPropStrong_threshold_const(self.as_raw_Linemod_ColorGradient()) };
		ret
	}
	
	#[inline]
	fn name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_ColorGradient_name_const(self.as_raw_Linemod_ColorGradient(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_ColorGradient_write_const_FileStorageR(self.as_raw_Linemod_ColorGradient(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Linemod_ColorGradientTrait: crate::rgbd::Linemod_ColorGradientTraitConst + crate::rgbd::Linemod_Modality {
	fn as_raw_mut_Linemod_ColorGradient(&mut self) -> *mut c_void;

	#[inline]
	fn set_weak_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_linemod_ColorGradient_setPropWeak_threshold_float(self.as_raw_mut_Linemod_ColorGradient(), val) };
		ret
	}
	
	#[inline]
	fn set_num_features(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_linemod_ColorGradient_setPropNum_features_size_t(self.as_raw_mut_Linemod_ColorGradient(), val) };
		ret
	}
	
	#[inline]
	fn set_strong_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_linemod_ColorGradient_setPropStrong_threshold_float(self.as_raw_mut_Linemod_ColorGradient(), val) };
		ret
	}
	
	#[inline]
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_ColorGradient_read_const_FileNodeR(self.as_raw_mut_Linemod_ColorGradient(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct Linemod_ColorGradient {
	ptr: *mut c_void
}

opencv_type_boxed! { Linemod_ColorGradient }

impl Drop for Linemod_ColorGradient {
	fn drop(&mut self) {
		extern "C" { fn cv_Linemod_ColorGradient_delete(instance: *mut c_void); }
		unsafe { cv_Linemod_ColorGradient_delete(self.as_raw_mut_Linemod_ColorGradient()) };
	}
}

unsafe impl Send for Linemod_ColorGradient {}

impl crate::rgbd::Linemod_ModalityConst for Linemod_ColorGradient {
	#[inline] fn as_raw_Linemod_Modality(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Linemod_Modality for Linemod_ColorGradient {
	#[inline] fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::Linemod_ColorGradientTraitConst for Linemod_ColorGradient {
	#[inline] fn as_raw_Linemod_ColorGradient(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Linemod_ColorGradientTrait for Linemod_ColorGradient {
	#[inline] fn as_raw_mut_Linemod_ColorGradient(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Linemod_ColorGradient {
	#[inline]
	pub fn default() -> Result<crate::rgbd::Linemod_ColorGradient> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_ColorGradient_ColorGradient(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Linemod_ColorGradient::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new(weak_threshold: f32, num_features: size_t, strong_threshold: f32) -> Result<crate::rgbd::Linemod_ColorGradient> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_ColorGradient_ColorGradient_float_size_t_float(weak_threshold, num_features, strong_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Linemod_ColorGradient::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create(weak_threshold: f32, num_features: size_t, strong_threshold: f32) -> Result<core::Ptr<crate::rgbd::Linemod_ColorGradient>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_ColorGradient_create_float_size_t_float(weak_threshold, num_features, strong_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Linemod_ColorGradient>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait Linemod_DepthNormalTraitConst: crate::rgbd::Linemod_ModalityConst {
	fn as_raw_Linemod_DepthNormal(&self) -> *const c_void;

	#[inline]
	fn distance_threshold(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_DepthNormal_getPropDistance_threshold_const(self.as_raw_Linemod_DepthNormal()) };
		ret
	}
	
	#[inline]
	fn difference_threshold(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_DepthNormal_getPropDifference_threshold_const(self.as_raw_Linemod_DepthNormal()) };
		ret
	}
	
	#[inline]
	fn num_features(&self) -> size_t {
		let ret = unsafe { sys::cv_linemod_DepthNormal_getPropNum_features_const(self.as_raw_Linemod_DepthNormal()) };
		ret
	}
	
	#[inline]
	fn extract_threshold(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_DepthNormal_getPropExtract_threshold_const(self.as_raw_Linemod_DepthNormal()) };
		ret
	}
	
	#[inline]
	fn name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_DepthNormal_name_const(self.as_raw_Linemod_DepthNormal(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_DepthNormal_write_const_FileStorageR(self.as_raw_Linemod_DepthNormal(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Linemod_DepthNormalTrait: crate::rgbd::Linemod_DepthNormalTraitConst + crate::rgbd::Linemod_Modality {
	fn as_raw_mut_Linemod_DepthNormal(&mut self) -> *mut c_void;

	#[inline]
	fn set_distance_threshold(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_DepthNormal_setPropDistance_threshold_int(self.as_raw_mut_Linemod_DepthNormal(), val) };
		ret
	}
	
	#[inline]
	fn set_difference_threshold(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_DepthNormal_setPropDifference_threshold_int(self.as_raw_mut_Linemod_DepthNormal(), val) };
		ret
	}
	
	#[inline]
	fn set_num_features(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_linemod_DepthNormal_setPropNum_features_size_t(self.as_raw_mut_Linemod_DepthNormal(), val) };
		ret
	}
	
	#[inline]
	fn set_extract_threshold(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_DepthNormal_setPropExtract_threshold_int(self.as_raw_mut_Linemod_DepthNormal(), val) };
		ret
	}
	
	#[inline]
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_DepthNormal_read_const_FileNodeR(self.as_raw_mut_Linemod_DepthNormal(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct Linemod_DepthNormal {
	ptr: *mut c_void
}

opencv_type_boxed! { Linemod_DepthNormal }

impl Drop for Linemod_DepthNormal {
	fn drop(&mut self) {
		extern "C" { fn cv_Linemod_DepthNormal_delete(instance: *mut c_void); }
		unsafe { cv_Linemod_DepthNormal_delete(self.as_raw_mut_Linemod_DepthNormal()) };
	}
}

unsafe impl Send for Linemod_DepthNormal {}

impl crate::rgbd::Linemod_ModalityConst for Linemod_DepthNormal {
	#[inline] fn as_raw_Linemod_Modality(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Linemod_Modality for Linemod_DepthNormal {
	#[inline] fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::Linemod_DepthNormalTraitConst for Linemod_DepthNormal {
	#[inline] fn as_raw_Linemod_DepthNormal(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Linemod_DepthNormalTrait for Linemod_DepthNormal {
	#[inline] fn as_raw_mut_Linemod_DepthNormal(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Linemod_DepthNormal {
	#[inline]
	pub fn default() -> Result<crate::rgbd::Linemod_DepthNormal> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_DepthNormal_DepthNormal(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Linemod_DepthNormal::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new(distance_threshold: i32, difference_threshold: i32, num_features: size_t, extract_threshold: i32) -> Result<crate::rgbd::Linemod_DepthNormal> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_DepthNormal_DepthNormal_int_int_size_t_int(distance_threshold, difference_threshold, num_features, extract_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Linemod_DepthNormal::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create(distance_threshold: i32, difference_threshold: i32, num_features: size_t, extract_threshold: i32) -> Result<core::Ptr<crate::rgbd::Linemod_DepthNormal>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_DepthNormal_create_int_int_size_t_int(distance_threshold, difference_threshold, num_features, extract_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Linemod_DepthNormal>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait Linemod_DetectorTraitConst {
	fn as_raw_Linemod_Detector(&self) -> *const c_void;

	/// ## C++ default parameters
	/// * class_ids: std::vector<String>()
	/// * quantized_images: noArray()
	/// * masks: std::vector<Mat>()
	#[inline]
	fn match_(&self, sources: &core::Vector<core::Mat>, threshold: f32, matches: &mut core::Vector<crate::rgbd::Linemod_Match>, class_ids: &core::Vector<String>, quantized_images: &mut dyn core::ToOutputArray, masks: &core::Vector<core::Mat>) -> Result<()> {
		output_array_arg!(quantized_images);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_match_const_const_vector_Mat_R_float_vector_Match_R_const_vector_String_R_const__OutputArrayR_const_vector_Mat_R(self.as_raw_Linemod_Detector(), sources.as_raw_VectorOfMat(), threshold, matches.as_raw_mut_VectorOfLinemod_Match(), class_ids.as_raw_VectorOfString(), quantized_images.as_raw__OutputArray(), masks.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_modalities(&self) -> Result<core::Vector<core::Ptr<dyn crate::rgbd::Linemod_Modality>>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_getModalities_const(self.as_raw_Linemod_Detector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Ptr<dyn crate::rgbd::Linemod_Modality>>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_t(&self, pyramid_level: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_getT_const_int(self.as_raw_Linemod_Detector(), pyramid_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn pyramid_levels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_pyramidLevels_const(self.as_raw_Linemod_Detector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_templates(&self, class_id: &str, template_id: i32) -> Result<core::Vector<crate::rgbd::Linemod_Template>> {
		extern_container_arg!(class_id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_getTemplates_const_const_StringR_int(self.as_raw_Linemod_Detector(), class_id.opencv_as_extern(), template_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<crate::rgbd::Linemod_Template>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn num_templates(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_numTemplates_const(self.as_raw_Linemod_Detector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn num_templates_1(&self, class_id: &str) -> Result<i32> {
		extern_container_arg!(class_id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_numTemplates_const_const_StringR(self.as_raw_Linemod_Detector(), class_id.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn num_classes(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_numClasses_const(self.as_raw_Linemod_Detector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn class_ids(&self) -> Result<core::Vector<String>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_classIds_const(self.as_raw_Linemod_Detector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<String>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_write_const_FileStorageR(self.as_raw_Linemod_Detector(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn write_class(&self, class_id: &str, fs: &mut core::FileStorage) -> Result<()> {
		extern_container_arg!(class_id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_writeClass_const_const_StringR_FileStorageR(self.as_raw_Linemod_Detector(), class_id.opencv_as_extern(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * format: "templates_%s.yml.gz"
	#[inline]
	fn write_classes(&self, format: &str) -> Result<()> {
		extern_container_arg!(format);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_writeClasses_const_const_StringR(self.as_raw_Linemod_Detector(), format.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Linemod_DetectorTrait: crate::rgbd::Linemod_DetectorTraitConst {
	fn as_raw_mut_Linemod_Detector(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * bounding_box: NULL
	#[inline]
	fn add_template(&mut self, sources: &core::Vector<core::Mat>, class_id: &str, object_mask: &core::Mat, bounding_box: &mut core::Rect) -> Result<i32> {
		extern_container_arg!(class_id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_addTemplate_const_vector_Mat_R_const_StringR_const_MatR_RectX(self.as_raw_mut_Linemod_Detector(), sources.as_raw_VectorOfMat(), class_id.opencv_as_extern(), object_mask.as_raw_Mat(), bounding_box, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn add_synthetic_template(&mut self, templates: &core::Vector<crate::rgbd::Linemod_Template>, class_id: &str) -> Result<i32> {
		extern_container_arg!(class_id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_addSyntheticTemplate_const_vector_Template_R_const_StringR(self.as_raw_mut_Linemod_Detector(), templates.as_raw_VectorOfLinemod_Template(), class_id.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_read_const_FileNodeR(self.as_raw_mut_Linemod_Detector(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * class_id_override: ""
	#[inline]
	fn read_class(&mut self, fn_: &core::FileNode, class_id_override: &str) -> Result<String> {
		extern_container_arg!(class_id_override);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_readClass_const_FileNodeR_const_StringR(self.as_raw_mut_Linemod_Detector(), fn_.as_raw_FileNode(), class_id_override.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * format: "templates_%s.yml.gz"
	#[inline]
	fn read_classes(&mut self, class_ids: &core::Vector<String>, format: &str) -> Result<()> {
		extern_container_arg!(format);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_readClasses_const_vector_String_R_const_StringR(self.as_raw_mut_Linemod_Detector(), class_ids.as_raw_VectorOfString(), format.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct Linemod_Detector {
	ptr: *mut c_void
}

opencv_type_boxed! { Linemod_Detector }

impl Drop for Linemod_Detector {
	fn drop(&mut self) {
		extern "C" { fn cv_Linemod_Detector_delete(instance: *mut c_void); }
		unsafe { cv_Linemod_Detector_delete(self.as_raw_mut_Linemod_Detector()) };
	}
}

unsafe impl Send for Linemod_Detector {}

impl crate::rgbd::Linemod_DetectorTraitConst for Linemod_Detector {
	#[inline] fn as_raw_Linemod_Detector(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Linemod_DetectorTrait for Linemod_Detector {
	#[inline] fn as_raw_mut_Linemod_Detector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Linemod_Detector {
	#[inline]
	pub fn default() -> Result<crate::rgbd::Linemod_Detector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_Detector(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Linemod_Detector::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new(modalities: &core::Vector<core::Ptr<dyn crate::rgbd::Linemod_Modality>>, t_pyramid: &core::Vector<i32>) -> Result<crate::rgbd::Linemod_Detector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_Detector_const_vector_Ptr_Modality__R_const_vector_int_R(modalities.as_raw_VectorOfPtrOfLinemod_Modality(), t_pyramid.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Linemod_Detector::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Linemod_Feature {
	pub x: i32,
	pub y: i32,
	pub label: i32,
}

opencv_type_simple! { crate::rgbd::Linemod_Feature }

impl Linemod_Feature {
	#[inline]
	pub fn write(self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Feature_write_const_FileStorageR(self.opencv_as_extern(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn default() -> Result<crate::rgbd::Linemod_Feature> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Feature_Feature(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn new(x: i32, y: i32, label: i32) -> Result<crate::rgbd::Linemod_Feature> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Feature_Feature_int_int_int(x, y, label, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn read(self, fn_: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Feature_read_const_FileNodeR(self.opencv_as_extern(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Linemod_MatchTraitConst {
	fn as_raw_Linemod_Match(&self) -> *const c_void;

	#[inline]
	fn x(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Match_getPropX_const(self.as_raw_Linemod_Match()) };
		ret
	}
	
	#[inline]
	fn y(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Match_getPropY_const(self.as_raw_Linemod_Match()) };
		ret
	}
	
	#[inline]
	fn similarity(&self) -> f32 {
		let ret = unsafe { sys::cv_linemod_Match_getPropSimilarity_const(self.as_raw_Linemod_Match()) };
		ret
	}
	
	#[inline]
	fn class_id(&self) -> String {
		let ret = unsafe { sys::cv_linemod_Match_getPropClass_id_const(self.as_raw_Linemod_Match()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn template_id(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Match_getPropTemplate_id_const(self.as_raw_Linemod_Match()) };
		ret
	}
	
	#[inline]
	fn equals(&self, rhs: &crate::rgbd::Linemod_Match) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Match_operatorEQ_const_const_MatchR(self.as_raw_Linemod_Match(), rhs.as_raw_Linemod_Match(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Linemod_MatchTrait: crate::rgbd::Linemod_MatchTraitConst {
	fn as_raw_mut_Linemod_Match(&mut self) -> *mut c_void;

	#[inline]
	fn set_x(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Match_setPropX_int(self.as_raw_mut_Linemod_Match(), val) };
		ret
	}
	
	#[inline]
	fn set_y(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Match_setPropY_int(self.as_raw_mut_Linemod_Match(), val) };
		ret
	}
	
	#[inline]
	fn set_similarity(&mut self, val: f32) {
		let ret = unsafe { sys::cv_linemod_Match_setPropSimilarity_float(self.as_raw_mut_Linemod_Match(), val) };
		ret
	}
	
	#[inline]
	fn set_class_id(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_linemod_Match_setPropClass_id_String(self.as_raw_mut_Linemod_Match(), val.opencv_as_extern_mut()) };
		ret
	}
	
	#[inline]
	fn set_template_id(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Match_setPropTemplate_id_int(self.as_raw_mut_Linemod_Match(), val) };
		ret
	}
	
}

pub struct Linemod_Match {
	ptr: *mut c_void
}

opencv_type_boxed! { Linemod_Match }

impl Drop for Linemod_Match {
	fn drop(&mut self) {
		extern "C" { fn cv_Linemod_Match_delete(instance: *mut c_void); }
		unsafe { cv_Linemod_Match_delete(self.as_raw_mut_Linemod_Match()) };
	}
}

unsafe impl Send for Linemod_Match {}

impl crate::rgbd::Linemod_MatchTraitConst for Linemod_Match {
	#[inline] fn as_raw_Linemod_Match(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Linemod_MatchTrait for Linemod_Match {
	#[inline] fn as_raw_mut_Linemod_Match(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Linemod_Match {
	#[inline]
	pub fn default() -> Result<crate::rgbd::Linemod_Match> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Match_Match(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Linemod_Match::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new(x: i32, y: i32, similarity: f32, class_id: &str, template_id: i32) -> Result<crate::rgbd::Linemod_Match> {
		extern_container_arg!(class_id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Match_Match_int_int_float_const_StringR_int(x, y, similarity, class_id.opencv_as_extern(), template_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Linemod_Match::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait Linemod_ModalityConst {
	fn as_raw_Linemod_Modality(&self) -> *const c_void;

	/// ## C++ default parameters
	/// * mask: Mat()
	#[inline]
	fn process(&self, src: &core::Mat, mask: &core::Mat) -> Result<core::Ptr<dyn crate::rgbd::Linemod_QuantizedPyramid>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Modality_process_const_const_MatR_const_MatR(self.as_raw_Linemod_Modality(), src.as_raw_Mat(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::rgbd::Linemod_QuantizedPyramid>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Modality_name_const(self.as_raw_Linemod_Modality(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Modality_write_const_FileStorageR(self.as_raw_Linemod_Modality(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Linemod_Modality: crate::rgbd::Linemod_ModalityConst {
	fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void;

	#[inline]
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Modality_read_const_FileNodeR(self.as_raw_mut_Linemod_Modality(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn Linemod_Modality + '_ {
	#[inline]
	pub fn create(modality_type: &str) -> Result<core::Ptr<dyn crate::rgbd::Linemod_Modality>> {
		extern_container_arg!(modality_type);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Modality_create_const_StringR(modality_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::rgbd::Linemod_Modality>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create_1(fn_: &core::FileNode) -> Result<core::Ptr<dyn crate::rgbd::Linemod_Modality>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Modality_create_const_FileNodeR(fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::rgbd::Linemod_Modality>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait Linemod_QuantizedPyramidConst {
	fn as_raw_Linemod_QuantizedPyramid(&self) -> *const c_void;

	#[inline]
	fn quantize(&self, dst: &mut core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_QuantizedPyramid_quantize_const_MatR(self.as_raw_Linemod_QuantizedPyramid(), dst.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn extract_template(&self, templ: &mut crate::rgbd::Linemod_Template) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_QuantizedPyramid_extractTemplate_const_TemplateR(self.as_raw_Linemod_QuantizedPyramid(), templ.as_raw_mut_Linemod_Template(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Linemod_QuantizedPyramid: crate::rgbd::Linemod_QuantizedPyramidConst {
	fn as_raw_mut_Linemod_QuantizedPyramid(&mut self) -> *mut c_void;

	#[inline]
	fn pyr_down(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_QuantizedPyramid_pyrDown(self.as_raw_mut_Linemod_QuantizedPyramid(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Linemod_TemplateTraitConst {
	fn as_raw_Linemod_Template(&self) -> *const c_void;

	#[inline]
	fn width(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Template_getPropWidth_const(self.as_raw_Linemod_Template()) };
		ret
	}
	
	#[inline]
	fn height(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Template_getPropHeight_const(self.as_raw_Linemod_Template()) };
		ret
	}
	
	#[inline]
	fn pyramid_level(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Template_getPropPyramid_level_const(self.as_raw_Linemod_Template()) };
		ret
	}
	
	#[inline]
	fn features(&self) -> core::Vector<crate::rgbd::Linemod_Feature> {
		let ret = unsafe { sys::cv_linemod_Template_getPropFeatures_const(self.as_raw_Linemod_Template()) };
		let ret = unsafe { core::Vector::<crate::rgbd::Linemod_Feature>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Template_write_const_FileStorageR(self.as_raw_Linemod_Template(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Linemod_TemplateTrait: crate::rgbd::Linemod_TemplateTraitConst {
	fn as_raw_mut_Linemod_Template(&mut self) -> *mut c_void;

	#[inline]
	fn set_width(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Template_setPropWidth_int(self.as_raw_mut_Linemod_Template(), val) };
		ret
	}
	
	#[inline]
	fn set_height(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Template_setPropHeight_int(self.as_raw_mut_Linemod_Template(), val) };
		ret
	}
	
	#[inline]
	fn set_pyramid_level(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Template_setPropPyramid_level_int(self.as_raw_mut_Linemod_Template(), val) };
		ret
	}
	
	#[inline]
	fn set_features(&mut self, mut val: core::Vector<crate::rgbd::Linemod_Feature>) {
		let ret = unsafe { sys::cv_linemod_Template_setPropFeatures_vector_Feature_(self.as_raw_mut_Linemod_Template(), val.as_raw_mut_VectorOfLinemod_Feature()) };
		ret
	}
	
	#[inline]
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Template_read_const_FileNodeR(self.as_raw_mut_Linemod_Template(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct Linemod_Template {
	ptr: *mut c_void
}

opencv_type_boxed! { Linemod_Template }

impl Drop for Linemod_Template {
	fn drop(&mut self) {
		extern "C" { fn cv_Linemod_Template_delete(instance: *mut c_void); }
		unsafe { cv_Linemod_Template_delete(self.as_raw_mut_Linemod_Template()) };
	}
}

unsafe impl Send for Linemod_Template {}

impl crate::rgbd::Linemod_TemplateTraitConst for Linemod_Template {
	#[inline] fn as_raw_Linemod_Template(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Linemod_TemplateTrait for Linemod_Template {
	#[inline] fn as_raw_mut_Linemod_Template(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Linemod_Template {
}

pub trait DepthCleanerTraitConst: core::AlgorithmTraitConst {
	fn as_raw_DepthCleaner(&self) -> *const c_void;

	#[inline]
	fn initialize(&self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_initialize_const(self.as_raw_DepthCleaner(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_window_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_getWindowSize_const(self.as_raw_DepthCleaner(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_depth(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_getDepth_const(self.as_raw_DepthCleaner(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_method(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_getMethod_const(self.as_raw_DepthCleaner(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait DepthCleanerTrait: core::AlgorithmTrait + crate::rgbd::DepthCleanerTraitConst {
	fn as_raw_mut_DepthCleaner(&mut self) -> *mut c_void;

	#[inline]
	fn set_window_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_setWindowSize_int(self.as_raw_mut_DepthCleaner(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_depth(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_setDepth_int(self.as_raw_mut_DepthCleaner(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_method(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_setMethod_int(self.as_raw_mut_DepthCleaner(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct DepthCleaner {
	ptr: *mut c_void
}

opencv_type_boxed! { DepthCleaner }

impl Drop for DepthCleaner {
	fn drop(&mut self) {
		extern "C" { fn cv_DepthCleaner_delete(instance: *mut c_void); }
		unsafe { cv_DepthCleaner_delete(self.as_raw_mut_DepthCleaner()) };
	}
}

unsafe impl Send for DepthCleaner {}

impl core::AlgorithmTraitConst for DepthCleaner {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for DepthCleaner {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::DepthCleanerTraitConst for DepthCleaner {
	#[inline] fn as_raw_DepthCleaner(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::DepthCleanerTrait for DepthCleaner {
	#[inline] fn as_raw_mut_DepthCleaner(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DepthCleaner {
	#[inline]
	pub fn default() -> Result<crate::rgbd::DepthCleaner> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_DepthCleaner(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::DepthCleaner::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * window_size: 5
	/// * method: DepthCleaner::DEPTH_CLEANER_NIL
	#[inline]
	pub fn new(depth: i32, window_size: i32, method: i32) -> Result<crate::rgbd::DepthCleaner> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_DepthCleaner_int_int_int(depth, window_size, method, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::DepthCleaner::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * window_size: 5
	/// * method: DepthCleaner::DEPTH_CLEANER_NIL
	#[inline]
	pub fn create(depth: i32, window_size: i32, method: i32) -> Result<core::Ptr<crate::rgbd::DepthCleaner>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_create_int_int_int(depth, window_size, method, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::DepthCleaner>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { DepthCleaner, core::Algorithm, cv_DepthCleaner_to_Algorithm }

pub trait FastICPOdometryTraitConst: crate::rgbd::OdometryConst {
	fn as_raw_FastICPOdometry(&self) -> *const c_void;

	#[inline]
	fn prepare_frame_cache(&self, frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(self.as_raw_FastICPOdometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_camera_matrix(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_getCameraMatrix_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_max_dist_diff(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_getMaxDistDiff_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_angle_threshold(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_getAngleThreshold_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_sigma_depth(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_getSigmaDepth_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_sigma_spatial(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_getSigmaSpatial_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_kernel_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_getKernelSize_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_iteration_counts(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_getIterationCounts_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_transform_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_getTransformType_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait FastICPOdometryTrait: crate::rgbd::FastICPOdometryTraitConst + crate::rgbd::Odometry {
	fn as_raw_mut_FastICPOdometry(&mut self) -> *mut c_void;

	#[inline]
	fn set_camera_matrix(&mut self, val: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_setCameraMatrix_const_MatR(self.as_raw_mut_FastICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_dist_diff(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_setMaxDistDiff_float(self.as_raw_mut_FastICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_angle_threshold(&mut self, f: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_setAngleThreshold_float(self.as_raw_mut_FastICPOdometry(), f, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_sigma_depth(&mut self, f: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_setSigmaDepth_float(self.as_raw_mut_FastICPOdometry(), f, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_sigma_spatial(&mut self, f: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_setSigmaSpatial_float(self.as_raw_mut_FastICPOdometry(), f, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_kernel_size(&mut self, f: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_setKernelSize_int(self.as_raw_mut_FastICPOdometry(), f, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_iteration_counts(&mut self, val: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_setIterationCounts_const_MatR(self.as_raw_mut_FastICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_transform_type(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_setTransformType_int(self.as_raw_mut_FastICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct FastICPOdometry {
	ptr: *mut c_void
}

opencv_type_boxed! { FastICPOdometry }

impl Drop for FastICPOdometry {
	fn drop(&mut self) {
		extern "C" { fn cv_FastICPOdometry_delete(instance: *mut c_void); }
		unsafe { cv_FastICPOdometry_delete(self.as_raw_mut_FastICPOdometry()) };
	}
}

unsafe impl Send for FastICPOdometry {}

impl core::AlgorithmTraitConst for FastICPOdometry {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for FastICPOdometry {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::OdometryConst for FastICPOdometry {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Odometry for FastICPOdometry {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::FastICPOdometryTraitConst for FastICPOdometry {
	#[inline] fn as_raw_FastICPOdometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::FastICPOdometryTrait for FastICPOdometry {
	#[inline] fn as_raw_mut_FastICPOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FastICPOdometry {
	#[inline]
	pub fn default() -> Result<crate::rgbd::FastICPOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_FastICPOdometry(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::FastICPOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * max_dist_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * angle_threshold: (float)(30.*CV_PI/180.)
	/// * sigma_depth: 0.04f
	/// * sigma_spatial: 4.5f
	/// * kernel_size: 7
	/// * iter_counts: std::vector<int>()
	#[inline]
	pub fn new(camera_matrix: &core::Mat, max_dist_diff: f32, angle_threshold: f32, sigma_depth: f32, sigma_spatial: f32, kernel_size: i32, iter_counts: &core::Vector<i32>) -> Result<crate::rgbd::FastICPOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_FastICPOdometry_const_MatR_float_float_float_float_int_const_vector_int_R(camera_matrix.as_raw_Mat(), max_dist_diff, angle_threshold, sigma_depth, sigma_spatial, kernel_size, iter_counts.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::FastICPOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * max_dist_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * angle_threshold: (float)(30.*CV_PI/180.)
	/// * sigma_depth: 0.04f
	/// * sigma_spatial: 4.5f
	/// * kernel_size: 7
	/// * iter_counts: std::vector<int>()
	#[inline]
	pub fn create(camera_matrix: &core::Mat, max_dist_diff: f32, angle_threshold: f32, sigma_depth: f32, sigma_spatial: f32, kernel_size: i32, iter_counts: &core::Vector<i32>) -> Result<core::Ptr<crate::rgbd::FastICPOdometry>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_create_const_MatR_float_float_float_float_int_const_vector_int_R(camera_matrix.as_raw_Mat(), max_dist_diff, angle_threshold, sigma_depth, sigma_spatial, kernel_size, iter_counts.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::FastICPOdometry>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { FastICPOdometry, core::Algorithm, cv_FastICPOdometry_to_Algorithm }

pub trait ICPOdometryTraitConst: crate::rgbd::OdometryConst {
	fn as_raw_ICPOdometry(&self) -> *const c_void;

	#[inline]
	fn prepare_frame_cache(&self, frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(self.as_raw_ICPOdometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_camera_matrix(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getCameraMatrix_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_min_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMinDepth_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMaxDepth_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_depth_diff(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMaxDepthDiff_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_iteration_counts(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getIterationCounts_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_max_points_part(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMaxPointsPart_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_transform_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getTransformType_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_translation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMaxTranslation_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_rotation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMaxRotation_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_normals_computer(&self) -> Result<core::Ptr<crate::rgbd::RgbdNormals>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getNormalsComputer_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdNormals>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait ICPOdometryTrait: crate::rgbd::ICPOdometryTraitConst + crate::rgbd::Odometry {
	fn as_raw_mut_ICPOdometry(&mut self) -> *mut c_void;

	#[inline]
	fn set_camera_matrix(&mut self, val: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setCameraMatrix_const_MatR(self.as_raw_mut_ICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_min_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setMinDepth_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setMaxDepth_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_depth_diff(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setMaxDepthDiff_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_iteration_counts(&mut self, val: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setIterationCounts_const_MatR(self.as_raw_mut_ICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_points_part(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setMaxPointsPart_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_transform_type(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setTransformType_int(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_translation(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setMaxTranslation_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_rotation(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setMaxRotation_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct ICPOdometry {
	ptr: *mut c_void
}

opencv_type_boxed! { ICPOdometry }

impl Drop for ICPOdometry {
	fn drop(&mut self) {
		extern "C" { fn cv_ICPOdometry_delete(instance: *mut c_void); }
		unsafe { cv_ICPOdometry_delete(self.as_raw_mut_ICPOdometry()) };
	}
}

unsafe impl Send for ICPOdometry {}

impl core::AlgorithmTraitConst for ICPOdometry {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ICPOdometry {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::OdometryConst for ICPOdometry {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Odometry for ICPOdometry {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::ICPOdometryTraitConst for ICPOdometry {
	#[inline] fn as_raw_ICPOdometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::ICPOdometryTrait for ICPOdometry {
	#[inline] fn as_raw_mut_ICPOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ICPOdometry {
	#[inline]
	pub fn default() -> Result<crate::rgbd::ICPOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_ICPOdometry(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::ICPOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * iter_counts: std::vector<int>()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	#[inline]
	pub fn new(camera_matrix: &core::Mat, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: &core::Vector<i32>, transform_type: i32) -> Result<crate::rgbd::ICPOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_ICPOdometry_const_MatR_float_float_float_float_const_vector_int_R_int(camera_matrix.as_raw_Mat(), min_depth, max_depth, max_depth_diff, max_points_part, iter_counts.as_raw_VectorOfi32(), transform_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::ICPOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * camera_matrix: Mat()
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * iter_counts: std::vector<int>()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	#[inline]
	pub fn create(camera_matrix: &core::Mat, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: &core::Vector<i32>, transform_type: i32) -> Result<core::Ptr<crate::rgbd::ICPOdometry>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_create_const_MatR_float_float_float_float_const_vector_int_R_int(camera_matrix.as_raw_Mat(), min_depth, max_depth, max_depth_diff, max_points_part, iter_counts.as_raw_VectorOfi32(), transform_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::ICPOdometry>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ICPOdometry, core::Algorithm, cv_ICPOdometry_to_Algorithm }

pub trait OdometryConst: core::AlgorithmTraitConst {
	fn as_raw_Odometry(&self) -> *const c_void;

	/// ## C++ default parameters
	/// * init_rt: Mat()
	#[inline]
	fn compute(&self, src_image: &core::Mat, src_depth: &core::Mat, src_mask: &core::Mat, dst_image: &core::Mat, dst_depth: &core::Mat, dst_mask: &core::Mat, rt: &mut dyn core::ToOutputArray, init_rt: &core::Mat) -> Result<bool> {
		output_array_arg!(rt);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_compute_const_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const_MatR(self.as_raw_Odometry(), src_image.as_raw_Mat(), src_depth.as_raw_Mat(), src_mask.as_raw_Mat(), dst_image.as_raw_Mat(), dst_depth.as_raw_Mat(), dst_mask.as_raw_Mat(), rt.as_raw__OutputArray(), init_rt.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * init_rt: Mat()
	#[inline]
	fn compute2(&self, src_frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, dst_frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, rt: &mut dyn core::ToOutputArray, init_rt: &core::Mat) -> Result<bool> {
		output_array_arg!(rt);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_compute_const_Ptr_OdometryFrame_R_Ptr_OdometryFrame_R_const__OutputArrayR_const_MatR(self.as_raw_Odometry(), src_frame.as_raw_mut_PtrOfOdometryFrame(), dst_frame.as_raw_mut_PtrOfOdometryFrame(), rt.as_raw__OutputArray(), init_rt.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn prepare_frame_cache(&self, frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(self.as_raw_Odometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_camera_matrix(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_getCameraMatrix_const(self.as_raw_Odometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_transform_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_getTransformType_const(self.as_raw_Odometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Odometry: core::AlgorithmTrait + crate::rgbd::OdometryConst {
	fn as_raw_mut_Odometry(&mut self) -> *mut c_void;

	#[inline]
	fn set_camera_matrix(&mut self, val: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_setCameraMatrix_const_MatR(self.as_raw_mut_Odometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_transform_type(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_setTransformType_int(self.as_raw_mut_Odometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn Odometry + '_ {
	#[inline]
	pub fn default_min_depth() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MIN_DEPTH(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn default_max_depth() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_DEPTH(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn default_max_depth_diff() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_DEPTH_DIFF(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn default_max_points_part() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_POINTS_PART(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn default_max_translation() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_TRANSLATION(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn default_max_rotation() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_ROTATION(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn create(odometry_type: &str) -> Result<core::Ptr<dyn crate::rgbd::Odometry>> {
		extern_container_arg!(odometry_type);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_create_const_StringR(odometry_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::rgbd::Odometry>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait OdometryFrameTraitConst: crate::rgbd::RgbdFrameTraitConst {
	fn as_raw_OdometryFrame(&self) -> *const c_void;

	#[inline]
	fn pyramid_image(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramidImage_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn pyramid_depth(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramidDepth_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn pyramid_mask(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramidMask_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn pyramid_cloud(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramidCloud_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn pyramid_d_i_dx(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramid_dI_dx_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn pyramid_d_i_dy(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramid_dI_dy_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn pyramid_textured_mask(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramidTexturedMask_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn pyramid_normals(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramidNormals_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn pyramid_normals_mask(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramidNormalsMask_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait OdometryFrameTrait: crate::rgbd::OdometryFrameTraitConst + crate::rgbd::RgbdFrameTrait {
	fn as_raw_mut_OdometryFrame(&mut self) -> *mut c_void;

	#[inline]
	fn set_pyramid_image(&mut self, mut val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramidImage_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
		ret
	}
	
	#[inline]
	fn set_pyramid_depth(&mut self, mut val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramidDepth_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
		ret
	}
	
	#[inline]
	fn set_pyramid_mask(&mut self, mut val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramidMask_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
		ret
	}
	
	#[inline]
	fn set_pyramid_cloud(&mut self, mut val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramidCloud_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
		ret
	}
	
	#[inline]
	fn set_pyramid_d_i_dx(&mut self, mut val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramid_dI_dx_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
		ret
	}
	
	#[inline]
	fn set_pyramid_d_i_dy(&mut self, mut val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramid_dI_dy_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
		ret
	}
	
	#[inline]
	fn set_pyramid_textured_mask(&mut self, mut val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramidTexturedMask_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
		ret
	}
	
	#[inline]
	fn set_pyramid_normals(&mut self, mut val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramidNormals_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
		ret
	}
	
	#[inline]
	fn set_pyramid_normals_mask(&mut self, mut val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramidNormalsMask_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
		ret
	}
	
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_OdometryFrame_release(self.as_raw_mut_OdometryFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn release_pyramids(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_OdometryFrame_releasePyramids(self.as_raw_mut_OdometryFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct OdometryFrame {
	ptr: *mut c_void
}

opencv_type_boxed! { OdometryFrame }

impl Drop for OdometryFrame {
	fn drop(&mut self) {
		extern "C" { fn cv_OdometryFrame_delete(instance: *mut c_void); }
		unsafe { cv_OdometryFrame_delete(self.as_raw_mut_OdometryFrame()) };
	}
}

unsafe impl Send for OdometryFrame {}

impl crate::rgbd::RgbdFrameTraitConst for OdometryFrame {
	#[inline] fn as_raw_RgbdFrame(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::RgbdFrameTrait for OdometryFrame {
	#[inline] fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::OdometryFrameTraitConst for OdometryFrame {
	#[inline] fn as_raw_OdometryFrame(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::OdometryFrameTrait for OdometryFrame {
	#[inline] fn as_raw_mut_OdometryFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl OdometryFrame {
	#[inline]
	pub fn default() -> Result<crate::rgbd::OdometryFrame> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_OdometryFrame_OdometryFrame(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::OdometryFrame::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * mask: Mat()
	/// * normals: Mat()
	/// * id: -1
	#[inline]
	pub fn new(image: &core::Mat, depth: &core::Mat, mask: &core::Mat, normals: &core::Mat, id: i32) -> Result<crate::rgbd::OdometryFrame> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_OdometryFrame_OdometryFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(image.as_raw_Mat(), depth.as_raw_Mat(), mask.as_raw_Mat(), normals.as_raw_Mat(), id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::OdometryFrame::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * image: Mat()
	/// * depth: Mat()
	/// * mask: Mat()
	/// * normals: Mat()
	/// * id: -1
	#[inline]
	pub fn create(image: &core::Mat, depth: &core::Mat, mask: &core::Mat, normals: &core::Mat, id: i32) -> Result<core::Ptr<crate::rgbd::OdometryFrame>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_OdometryFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(image.as_raw_Mat(), depth.as_raw_Mat(), mask.as_raw_Mat(), normals.as_raw_Mat(), id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::OdometryFrame>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { OdometryFrame, crate::rgbd::RgbdFrame, cv_OdometryFrame_to_RgbdFrame }

pub trait RgbdFrameTraitConst {
	fn as_raw_RgbdFrame(&self) -> *const c_void;

	#[inline]
	fn id(&self) -> i32 {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_getPropID_const(self.as_raw_RgbdFrame()) };
		ret
	}
	
	#[inline]
	fn image(&self) -> core::Mat {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_getPropImage_const(self.as_raw_RgbdFrame()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn depth(&self) -> core::Mat {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_getPropDepth_const(self.as_raw_RgbdFrame()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn mask(&self) -> core::Mat {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_getPropMask_const(self.as_raw_RgbdFrame()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn normals(&self) -> core::Mat {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_getPropNormals_const(self.as_raw_RgbdFrame()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait RgbdFrameTrait: crate::rgbd::RgbdFrameTraitConst {
	fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void;

	#[inline]
	fn set_id(&mut self, val: i32) {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_setPropID_int(self.as_raw_mut_RgbdFrame(), val) };
		ret
	}
	
	#[inline]
	fn set_image(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_setPropImage_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_depth(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_setPropDepth_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_mask(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_setPropMask_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_normals(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_setPropNormals_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdFrame_release(self.as_raw_mut_RgbdFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct RgbdFrame {
	ptr: *mut c_void
}

opencv_type_boxed! { RgbdFrame }

impl Drop for RgbdFrame {
	fn drop(&mut self) {
		extern "C" { fn cv_RgbdFrame_delete(instance: *mut c_void); }
		unsafe { cv_RgbdFrame_delete(self.as_raw_mut_RgbdFrame()) };
	}
}

unsafe impl Send for RgbdFrame {}

impl crate::rgbd::RgbdFrameTraitConst for RgbdFrame {
	#[inline] fn as_raw_RgbdFrame(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::RgbdFrameTrait for RgbdFrame {
	#[inline] fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RgbdFrame {
	#[inline]
	pub fn default() -> Result<crate::rgbd::RgbdFrame> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdFrame_RgbdFrame(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdFrame::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * mask: Mat()
	/// * normals: Mat()
	/// * id: -1
	#[inline]
	pub fn new(image: &core::Mat, depth: &core::Mat, mask: &core::Mat, normals: &core::Mat, id: i32) -> Result<crate::rgbd::RgbdFrame> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdFrame_RgbdFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(image.as_raw_Mat(), depth.as_raw_Mat(), mask.as_raw_Mat(), normals.as_raw_Mat(), id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdFrame::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * image: Mat()
	/// * depth: Mat()
	/// * mask: Mat()
	/// * normals: Mat()
	/// * id: -1
	#[inline]
	pub fn create(image: &core::Mat, depth: &core::Mat, mask: &core::Mat, normals: &core::Mat, id: i32) -> Result<core::Ptr<crate::rgbd::RgbdFrame>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(image.as_raw_Mat(), depth.as_raw_Mat(), mask.as_raw_Mat(), normals.as_raw_Mat(), id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdFrame>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_descendant! { RgbdFrame, crate::rgbd::OdometryFrame, cv_RgbdFrame_to_OdometryFrame }

pub trait RgbdICPOdometryTraitConst: crate::rgbd::OdometryConst {
	fn as_raw_RgbdICPOdometry(&self) -> *const c_void;

	#[inline]
	fn prepare_frame_cache(&self, frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(self.as_raw_RgbdICPOdometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_camera_matrix(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getCameraMatrix_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_min_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMinDepth_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxDepth_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_depth_diff(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxDepthDiff_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_points_part(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxPointsPart_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_iteration_counts(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getIterationCounts_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_min_gradient_magnitudes(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMinGradientMagnitudes_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_transform_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getTransformType_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_translation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxTranslation_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_rotation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxRotation_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_normals_computer(&self) -> Result<core::Ptr<crate::rgbd::RgbdNormals>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getNormalsComputer_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdNormals>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait RgbdICPOdometryTrait: crate::rgbd::Odometry + crate::rgbd::RgbdICPOdometryTraitConst {
	fn as_raw_mut_RgbdICPOdometry(&mut self) -> *mut c_void;

	#[inline]
	fn set_camera_matrix(&mut self, val: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setCameraMatrix_const_MatR(self.as_raw_mut_RgbdICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_min_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMinDepth_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxDepth_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_depth_diff(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxDepthDiff_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_points_part(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxPointsPart_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_iteration_counts(&mut self, val: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setIterationCounts_const_MatR(self.as_raw_mut_RgbdICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_min_gradient_magnitudes(&mut self, val: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMinGradientMagnitudes_const_MatR(self.as_raw_mut_RgbdICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_transform_type(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setTransformType_int(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_translation(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxTranslation_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_rotation(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxRotation_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct RgbdICPOdometry {
	ptr: *mut c_void
}

opencv_type_boxed! { RgbdICPOdometry }

impl Drop for RgbdICPOdometry {
	fn drop(&mut self) {
		extern "C" { fn cv_RgbdICPOdometry_delete(instance: *mut c_void); }
		unsafe { cv_RgbdICPOdometry_delete(self.as_raw_mut_RgbdICPOdometry()) };
	}
}

unsafe impl Send for RgbdICPOdometry {}

impl core::AlgorithmTraitConst for RgbdICPOdometry {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for RgbdICPOdometry {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::OdometryConst for RgbdICPOdometry {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Odometry for RgbdICPOdometry {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::RgbdICPOdometryTraitConst for RgbdICPOdometry {
	#[inline] fn as_raw_RgbdICPOdometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::RgbdICPOdometryTrait for RgbdICPOdometry {
	#[inline] fn as_raw_mut_RgbdICPOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RgbdICPOdometry {
	#[inline]
	pub fn default() -> Result<crate::rgbd::RgbdICPOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_RgbdICPOdometry(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdICPOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * iter_counts: std::vector<int>()
	/// * min_gradient_magnitudes: std::vector<float>()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	#[inline]
	pub fn new(camera_matrix: &core::Mat, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: &core::Vector<i32>, min_gradient_magnitudes: &core::Vector<f32>, transform_type: i32) -> Result<crate::rgbd::RgbdICPOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_RgbdICPOdometry_const_MatR_float_float_float_float_const_vector_int_R_const_vector_float_R_int(camera_matrix.as_raw_Mat(), min_depth, max_depth, max_depth_diff, max_points_part, iter_counts.as_raw_VectorOfi32(), min_gradient_magnitudes.as_raw_VectorOff32(), transform_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdICPOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * camera_matrix: Mat()
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * iter_counts: std::vector<int>()
	/// * min_gradient_magnitudes: std::vector<float>()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	#[inline]
	pub fn create(camera_matrix: &core::Mat, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: &core::Vector<i32>, min_gradient_magnitudes: &core::Vector<f32>, transform_type: i32) -> Result<core::Ptr<crate::rgbd::RgbdICPOdometry>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_create_const_MatR_float_float_float_float_const_vector_int_R_const_vector_float_R_int(camera_matrix.as_raw_Mat(), min_depth, max_depth, max_depth_diff, max_points_part, iter_counts.as_raw_VectorOfi32(), min_gradient_magnitudes.as_raw_VectorOff32(), transform_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdICPOdometry>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { RgbdICPOdometry, core::Algorithm, cv_RgbdICPOdometry_to_Algorithm }

pub trait RgbdNormalsTraitConst: core::AlgorithmTraitConst {
	fn as_raw_RgbdNormals(&self) -> *const c_void;

	#[inline]
	fn initialize(&self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_initialize_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_rows(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_getRows_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_cols(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_getCols_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_window_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_getWindowSize_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_depth(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_getDepth_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_k(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_getK_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_method(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_getMethod_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait RgbdNormalsTrait: core::AlgorithmTrait + crate::rgbd::RgbdNormalsTraitConst {
	fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void;

	#[inline]
	fn set_rows(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_setRows_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_cols(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_setCols_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_window_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_setWindowSize_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_depth(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_setDepth_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_k(&mut self, val: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_setK_const_MatR(self.as_raw_mut_RgbdNormals(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_method(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_setMethod_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct RgbdNormals {
	ptr: *mut c_void
}

opencv_type_boxed! { RgbdNormals }

impl Drop for RgbdNormals {
	fn drop(&mut self) {
		extern "C" { fn cv_RgbdNormals_delete(instance: *mut c_void); }
		unsafe { cv_RgbdNormals_delete(self.as_raw_mut_RgbdNormals()) };
	}
}

unsafe impl Send for RgbdNormals {}

impl core::AlgorithmTraitConst for RgbdNormals {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for RgbdNormals {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::RgbdNormalsTraitConst for RgbdNormals {
	#[inline] fn as_raw_RgbdNormals(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::RgbdNormalsTrait for RgbdNormals {
	#[inline] fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RgbdNormals {
	#[inline]
	pub fn default() -> Result<crate::rgbd::RgbdNormals> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_RgbdNormals(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdNormals::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * window_size: 5
	/// * method: RgbdNormals::RGBD_NORMALS_METHOD_FALS
	#[inline]
	pub fn new(rows: i32, cols: i32, depth: i32, k: &dyn core::ToInputArray, window_size: i32, method: i32) -> Result<crate::rgbd::RgbdNormals> {
		input_array_arg!(k);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_RgbdNormals_int_int_int_const__InputArrayR_int_int(rows, cols, depth, k.as_raw__InputArray(), window_size, method, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdNormals::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * window_size: 5
	/// * method: RgbdNormals::RGBD_NORMALS_METHOD_FALS
	#[inline]
	pub fn create(rows: i32, cols: i32, depth: i32, k: &dyn core::ToInputArray, window_size: i32, method: i32) -> Result<core::Ptr<crate::rgbd::RgbdNormals>> {
		input_array_arg!(k);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_create_int_int_int_const__InputArrayR_int_int(rows, cols, depth, k.as_raw__InputArray(), window_size, method, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdNormals>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { RgbdNormals, core::Algorithm, cv_RgbdNormals_to_Algorithm }

pub trait RgbdOdometryTraitConst: crate::rgbd::OdometryConst {
	fn as_raw_RgbdOdometry(&self) -> *const c_void;

	#[inline]
	fn prepare_frame_cache(&self, frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(self.as_raw_RgbdOdometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_camera_matrix(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getCameraMatrix_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_min_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMinDepth_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxDepth_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_depth_diff(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxDepthDiff_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_iteration_counts(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getIterationCounts_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_min_gradient_magnitudes(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMinGradientMagnitudes_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_max_points_part(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxPointsPart_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_transform_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getTransformType_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_translation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxTranslation_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_rotation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxRotation_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait RgbdOdometryTrait: crate::rgbd::Odometry + crate::rgbd::RgbdOdometryTraitConst {
	fn as_raw_mut_RgbdOdometry(&mut self) -> *mut c_void;

	#[inline]
	fn set_camera_matrix(&mut self, val: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setCameraMatrix_const_MatR(self.as_raw_mut_RgbdOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_min_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMinDepth_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxDepth_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_depth_diff(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxDepthDiff_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_iteration_counts(&mut self, val: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setIterationCounts_const_MatR(self.as_raw_mut_RgbdOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_min_gradient_magnitudes(&mut self, val: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMinGradientMagnitudes_const_MatR(self.as_raw_mut_RgbdOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_points_part(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxPointsPart_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_transform_type(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setTransformType_int(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_translation(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxTranslation_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_rotation(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxRotation_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct RgbdOdometry {
	ptr: *mut c_void
}

opencv_type_boxed! { RgbdOdometry }

impl Drop for RgbdOdometry {
	fn drop(&mut self) {
		extern "C" { fn cv_RgbdOdometry_delete(instance: *mut c_void); }
		unsafe { cv_RgbdOdometry_delete(self.as_raw_mut_RgbdOdometry()) };
	}
}

unsafe impl Send for RgbdOdometry {}

impl core::AlgorithmTraitConst for RgbdOdometry {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for RgbdOdometry {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::OdometryConst for RgbdOdometry {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Odometry for RgbdOdometry {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::RgbdOdometryTraitConst for RgbdOdometry {
	#[inline] fn as_raw_RgbdOdometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::RgbdOdometryTrait for RgbdOdometry {
	#[inline] fn as_raw_mut_RgbdOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RgbdOdometry {
	#[inline]
	pub fn default() -> Result<crate::rgbd::RgbdOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_RgbdOdometry(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * iter_counts: std::vector<int>()
	/// * min_gradient_magnitudes: std::vector<float>()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	#[inline]
	pub fn new(camera_matrix: &core::Mat, min_depth: f32, max_depth: f32, max_depth_diff: f32, iter_counts: &core::Vector<i32>, min_gradient_magnitudes: &core::Vector<f32>, max_points_part: f32, transform_type: i32) -> Result<crate::rgbd::RgbdOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_RgbdOdometry_const_MatR_float_float_float_const_vector_int_R_const_vector_float_R_float_int(camera_matrix.as_raw_Mat(), min_depth, max_depth, max_depth_diff, iter_counts.as_raw_VectorOfi32(), min_gradient_magnitudes.as_raw_VectorOff32(), max_points_part, transform_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * camera_matrix: Mat()
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * iter_counts: std::vector<int>()
	/// * min_gradient_magnitudes: std::vector<float>()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	#[inline]
	pub fn create(camera_matrix: &core::Mat, min_depth: f32, max_depth: f32, max_depth_diff: f32, iter_counts: &core::Vector<i32>, min_gradient_magnitudes: &core::Vector<f32>, max_points_part: f32, transform_type: i32) -> Result<core::Ptr<crate::rgbd::RgbdOdometry>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_create_const_MatR_float_float_float_const_vector_int_R_const_vector_float_R_float_int(camera_matrix.as_raw_Mat(), min_depth, max_depth, max_depth_diff, iter_counts.as_raw_VectorOfi32(), min_gradient_magnitudes.as_raw_VectorOff32(), max_points_part, transform_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdOdometry>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { RgbdOdometry, core::Algorithm, cv_RgbdOdometry_to_Algorithm }

pub trait RgbdPlaneTraitConst: core::AlgorithmTraitConst {
	fn as_raw_RgbdPlane(&self) -> *const c_void;

	#[inline]
	fn get_block_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getBlockSize_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_min_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getMinSize_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_method(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getMethod_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getThreshold_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_sensor_error_a(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getSensorErrorA_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_sensor_error_b(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getSensorErrorB_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_sensor_error_c(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getSensorErrorC_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait RgbdPlaneTrait: core::AlgorithmTrait + crate::rgbd::RgbdPlaneTraitConst {
	fn as_raw_mut_RgbdPlane(&mut self) -> *mut c_void;

	#[inline]
	fn set_block_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setBlockSize_int(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_min_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setMinSize_int(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_method(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setMethod_int(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_threshold(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setThreshold_double(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_sensor_error_a(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setSensorErrorA_double(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_sensor_error_b(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setSensorErrorB_double(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_sensor_error_c(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setSensorErrorC_double(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct RgbdPlane {
	ptr: *mut c_void
}

opencv_type_boxed! { RgbdPlane }

impl Drop for RgbdPlane {
	fn drop(&mut self) {
		extern "C" { fn cv_RgbdPlane_delete(instance: *mut c_void); }
		unsafe { cv_RgbdPlane_delete(self.as_raw_mut_RgbdPlane()) };
	}
}

unsafe impl Send for RgbdPlane {}

impl core::AlgorithmTraitConst for RgbdPlane {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for RgbdPlane {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::RgbdPlaneTraitConst for RgbdPlane {
	#[inline] fn as_raw_RgbdPlane(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::RgbdPlaneTrait for RgbdPlane {
	#[inline] fn as_raw_mut_RgbdPlane(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RgbdPlane {
	/// ## C++ default parameters
	/// * method: RgbdPlane::RGBD_PLANE_METHOD_DEFAULT
	#[inline]
	pub fn new(method: i32) -> Result<crate::rgbd::RgbdPlane> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_RgbdPlane_int(method, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdPlane::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * sensor_error_a: 0
	/// * sensor_error_b: 0
	/// * sensor_error_c: 0
	#[inline]
	pub fn new_1(method: i32, block_size: i32, min_size: i32, threshold: f64, sensor_error_a: f64, sensor_error_b: f64, sensor_error_c: f64) -> Result<crate::rgbd::RgbdPlane> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_RgbdPlane_int_int_int_double_double_double_double(method, block_size, min_size, threshold, sensor_error_a, sensor_error_b, sensor_error_c, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdPlane::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * sensor_error_a: 0
	/// * sensor_error_b: 0
	/// * sensor_error_c: 0
	#[inline]
	pub fn create(method: i32, block_size: i32, min_size: i32, threshold: f64, sensor_error_a: f64, sensor_error_b: f64, sensor_error_c: f64) -> Result<core::Ptr<crate::rgbd::RgbdPlane>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_create_int_int_int_double_double_double_double(method, block_size, min_size, threshold, sensor_error_a, sensor_error_b, sensor_error_c, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdPlane>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { RgbdPlane, core::Algorithm, cv_RgbdPlane_to_Algorithm }
