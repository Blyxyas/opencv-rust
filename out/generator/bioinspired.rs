#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Biologically inspired vision models and derivated tools
//! 
//! The module provides biological visual systems models (human visual system and others). It also
//! provides derivated objects that take advantage of those bio-inspired models.
//! 
//! @ref bioinspired_retina
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::RetinaParametersTraitConst, super::RetinaParametersTrait, super::RetinaConst, super::Retina, super::RetinaFastToneMappingConst, super::RetinaFastToneMapping, super::TransientAreasSegmentationModuleConst, super::TransientAreasSegmentationModule };
}

// RETINA_COLOR_BAYER /usr/include/opencv2/bioinspired/retina.hpp:86
pub const RETINA_COLOR_BAYER: i32 = 2;
// RETINA_COLOR_DIAGONAL /usr/include/opencv2/bioinspired/retina.hpp:85
pub const RETINA_COLOR_DIAGONAL: i32 = 1;
// RETINA_COLOR_RANDOM /usr/include/opencv2/bioinspired/retina.hpp:84
pub const RETINA_COLOR_RANDOM: i32 = 0;
// Retina /usr/include/opencv2/bioinspired/retina.hpp:207
pub trait RetinaConst: core::AlgorithmTraitConst {
	fn as_raw_Retina(&self) -> *const c_void;

	// write(cv::String) /usr/include/opencv2/bioinspired/retina.hpp:260
	#[inline]
	fn write(&self, fs: &str) -> Result<()> {
		extern_container_arg!(mut fs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_write_const_String(self.as_raw_Retina(), fs.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/bioinspired/retina.hpp:263
	#[inline]
	fn write_to_storage(&self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_write_const_FileStorageR(self.as_raw_Retina(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getMagnoRAW() /usr/include/opencv2/bioinspired/retina.hpp:392
	#[inline]
	fn get_magno_raw(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_getMagnoRAW_const(self.as_raw_Retina(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getParvoRAW() /usr/include/opencv2/bioinspired/retina.hpp:394
	#[inline]
	fn get_parvo_raw(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_getParvoRAW_const(self.as_raw_Retina(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait Retina: core::AlgorithmTrait + crate::bioinspired::RetinaConst {
	fn as_raw_mut_Retina(&mut self) -> *mut c_void;

	// getInputSize() /usr/include/opencv2/bioinspired/retina.hpp:215
	#[inline]
	fn get_input_size(&mut self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_getInputSize(self.as_raw_mut_Retina(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getOutputSize() /usr/include/opencv2/bioinspired/retina.hpp:221
	#[inline]
	fn get_output_size(&mut self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_getOutputSize(self.as_raw_mut_Retina(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * retina_parameter_file: ""
	/// * apply_default_setup_on_failure: true
	// setup(cv::String, const bool) /usr/include/opencv2/bioinspired/retina.hpp:233
	#[inline]
	fn setup_from_file(&mut self, retina_parameter_file: &str, apply_default_setup_on_failure: bool) -> Result<()> {
		extern_container_arg!(mut retina_parameter_file);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_setup_String_const_bool(self.as_raw_mut_Retina(), retina_parameter_file.opencv_as_extern_mut(), apply_default_setup_on_failure, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * apply_default_setup_on_failure: true
	// setup(cv::FileStorage &, const bool) /usr/include/opencv2/bioinspired/retina.hpp:239
	#[inline]
	fn setup_from_storage(&mut self, fs: &mut core::FileStorage, apply_default_setup_on_failure: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_setup_FileStorageR_const_bool(self.as_raw_mut_Retina(), fs.as_raw_mut_FileStorage(), apply_default_setup_on_failure, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setup(cv::bioinspired::RetinaParameters) /usr/include/opencv2/bioinspired/retina.hpp:244
	#[inline]
	fn setup(&mut self, mut new_parameters: crate::bioinspired::RetinaParameters) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_setup_RetinaParameters(self.as_raw_mut_Retina(), new_parameters.as_raw_mut_RetinaParameters(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getParameters() /usr/include/opencv2/bioinspired/retina.hpp:249
	#[inline]
	fn get_parameters(&mut self) -> Result<crate::bioinspired::RetinaParameters> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_getParameters(self.as_raw_mut_Retina(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::bioinspired::RetinaParameters::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// printSetup() /usr/include/opencv2/bioinspired/retina.hpp:254
	#[inline]
	fn print_setup(&mut self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_printSetup(self.as_raw_mut_Retina(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * color_mode: true
	/// * normalise_output: true
	/// * photoreceptors_local_adaptation_sensitivity: 0.7f
	/// * photoreceptors_temporal_constant: 0.5f
	/// * photoreceptors_spatial_constant: 0.53f
	/// * horizontal_cells_gain: 0.f
	/// * hcells_temporal_constant: 1.f
	/// * hcells_spatial_constant: 7.f
	/// * ganglion_cells_sensitivity: 0.7f
	// setupOPLandIPLParvoChannel(const bool, const bool, const float, const float, const float, const float, const float, const float, const float) /usr/include/opencv2/bioinspired/retina.hpp:299
	#[inline]
	fn setup_op_land_ipl_parvo_channel(&mut self, color_mode: bool, normalise_output: bool, photoreceptors_local_adaptation_sensitivity: f32, photoreceptors_temporal_constant: f32, photoreceptors_spatial_constant: f32, horizontal_cells_gain: f32, hcells_temporal_constant: f32, hcells_spatial_constant: f32, ganglion_cells_sensitivity: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_setupOPLandIPLParvoChannel_const_bool_const_bool_const_float_const_float_const_float_const_float_const_float_const_float_const_float(self.as_raw_mut_Retina(), color_mode, normalise_output, photoreceptors_local_adaptation_sensitivity, photoreceptors_temporal_constant, photoreceptors_spatial_constant, horizontal_cells_gain, hcells_temporal_constant, hcells_spatial_constant, ganglion_cells_sensitivity, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * normalise_output: true
	/// * parasol_cells_beta: 0.f
	/// * parasol_cells_tau: 0.f
	/// * parasol_cells_k: 7.f
	/// * amacrin_cells_temporal_cut_frequency: 1.2f
	/// * v0_compression_parameter: 0.95f
	/// * local_adaptintegration_tau: 0.f
	/// * local_adaptintegration_k: 7.f
	// setupIPLMagnoChannel(const bool, const float, const float, const float, const float, const float, const float, const float) /usr/include/opencv2/bioinspired/retina.hpp:326
	#[inline]
	fn setup_ipl_magno_channel(&mut self, normalise_output: bool, parasol_cells_beta: f32, parasol_cells_tau: f32, parasol_cells_k: f32, amacrin_cells_temporal_cut_frequency: f32, v0_compression_parameter: f32, local_adaptintegration_tau: f32, local_adaptintegration_k: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_setupIPLMagnoChannel_const_bool_const_float_const_float_const_float_const_float_const_float_const_float_const_float(self.as_raw_mut_Retina(), normalise_output, parasol_cells_beta, parasol_cells_tau, parasol_cells_k, amacrin_cells_temporal_cut_frequency, v0_compression_parameter, local_adaptintegration_tau, local_adaptintegration_k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// run(cv::InputArray) /usr/include/opencv2/bioinspired/retina.hpp:335
	#[inline]
	fn run(&mut self, input_image: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(input_image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_run_const__InputArrayR(self.as_raw_mut_Retina(), input_image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// applyFastToneMapping(cv::InputArray, cv::OutputArray) /usr/include/opencv2/bioinspired/retina.hpp:353
	#[inline]
	fn apply_fast_tone_mapping(&mut self, input_image: &dyn core::ToInputArray, output_tone_mapped_image: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(input_image);
		output_array_arg!(output_tone_mapped_image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_applyFastToneMapping_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Retina(), input_image.as_raw__InputArray(), output_tone_mapped_image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getParvo(cv::OutputArray) /usr/include/opencv2/bioinspired/retina.hpp:367
	#[inline]
	fn get_parvo(&mut self, retina_output_parvo: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(retina_output_parvo);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_getParvo_const__OutputArrayR(self.as_raw_mut_Retina(), retina_output_parvo.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getParvoRAW(cv::OutputArray) /usr/include/opencv2/bioinspired/retina.hpp:372
	#[inline]
	fn get_parvo_raw_to(&mut self, retina_output_parvo: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(retina_output_parvo);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_getParvoRAW_const__OutputArrayR(self.as_raw_mut_Retina(), retina_output_parvo.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getMagno(cv::OutputArray) /usr/include/opencv2/bioinspired/retina.hpp:384
	#[inline]
	fn get_magno(&mut self, retina_output_magno: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(retina_output_magno);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_getMagno_const__OutputArrayR(self.as_raw_mut_Retina(), retina_output_magno.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getMagnoRAW(cv::OutputArray) /usr/include/opencv2/bioinspired/retina.hpp:389
	#[inline]
	fn get_magno_raw_to(&mut self, retina_output_magno: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(retina_output_magno);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_getMagnoRAW_const__OutputArrayR(self.as_raw_mut_Retina(), retina_output_magno.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * saturate_colors: true
	/// * color_saturation_value: 4.0f
	// setColorSaturation(const bool, const float) /usr/include/opencv2/bioinspired/retina.hpp:402
	#[inline]
	fn set_color_saturation(&mut self, saturate_colors: bool, color_saturation_value: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_setColorSaturation_const_bool_const_float(self.as_raw_mut_Retina(), saturate_colors, color_saturation_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// clearBuffers() /usr/include/opencv2/bioinspired/retina.hpp:409
	#[inline]
	fn clear_buffers(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_clearBuffers(self.as_raw_mut_Retina(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// activateMovingContoursProcessing(const bool) /usr/include/opencv2/bioinspired/retina.hpp:416
	#[inline]
	fn activate_moving_contours_processing(&mut self, activate: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_activateMovingContoursProcessing_const_bool(self.as_raw_mut_Retina(), activate, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// activateContoursProcessing(const bool) /usr/include/opencv2/bioinspired/retina.hpp:424
	#[inline]
	fn activate_contours_processing(&mut self, activate: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_activateContoursProcessing_const_bool(self.as_raw_mut_Retina(), activate, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn Retina + '_ {
	// create(cv::Size) /usr/include/opencv2/bioinspired/retina.hpp:427
	#[inline]
	pub fn create(input_size: core::Size) -> Result<core::Ptr<dyn crate::bioinspired::Retina>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_create_Size(input_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::bioinspired::Retina>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * color_sampling_method: RETINA_COLOR_BAYER
	/// * use_retina_log_sampling: false
	/// * reduction_factor: 1.0f
	/// * sampling_strength: 10.0f
	// create(cv::Size, const bool, int, const bool, const float, const float) /usr/include/opencv2/bioinspired/retina.hpp:444
	#[inline]
	pub fn create_ext(input_size: core::Size, color_mode: bool, color_sampling_method: i32, use_retina_log_sampling: bool, reduction_factor: f32, sampling_strength: f32) -> Result<core::Ptr<dyn crate::bioinspired::Retina>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_Retina_create_Size_const_bool_int_const_bool_const_float_const_float(input_size.opencv_as_extern(), color_mode, color_sampling_method, use_retina_log_sampling, reduction_factor, sampling_strength, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::bioinspired::Retina>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
// RetinaFastToneMapping /usr/include/opencv2/bioinspired/retinafasttonemapping.hpp:99
pub trait RetinaFastToneMappingConst: core::AlgorithmTraitConst {
	fn as_raw_RetinaFastToneMapping(&self) -> *const c_void;

}

pub trait RetinaFastToneMapping: core::AlgorithmTrait + crate::bioinspired::RetinaFastToneMappingConst {
	fn as_raw_mut_RetinaFastToneMapping(&mut self) -> *mut c_void;

	// applyFastToneMapping(cv::InputArray, cv::OutputArray) /usr/include/opencv2/bioinspired/retinafasttonemapping.hpp:119
	#[inline]
	fn apply_fast_tone_mapping(&mut self, input_image: &dyn core::ToInputArray, output_tone_mapped_image: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(input_image);
		output_array_arg!(output_tone_mapped_image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_RetinaFastToneMapping_applyFastToneMapping_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_RetinaFastToneMapping(), input_image.as_raw__InputArray(), output_tone_mapped_image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * photoreceptors_neighborhood_radius: 3.f
	/// * ganglioncells_neighborhood_radius: 1.f
	/// * mean_luminance_modulator_k: 1.f
	// setup(const float, const float, const float) /usr/include/opencv2/bioinspired/retinafasttonemapping.hpp:128
	#[inline]
	fn setup(&mut self, photoreceptors_neighborhood_radius: f32, ganglioncells_neighborhood_radius: f32, mean_luminance_modulator_k: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_RetinaFastToneMapping_setup_const_float_const_float_const_float(self.as_raw_mut_RetinaFastToneMapping(), photoreceptors_neighborhood_radius, ganglioncells_neighborhood_radius, mean_luminance_modulator_k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn RetinaFastToneMapping + '_ {
	// create(cv::Size) /usr/include/opencv2/bioinspired/retinafasttonemapping.hpp:130
	#[inline]
	pub fn create(input_size: core::Size) -> Result<core::Ptr<dyn crate::bioinspired::RetinaFastToneMapping>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_RetinaFastToneMapping_create_Size(input_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::bioinspired::RetinaFastToneMapping>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
// RetinaParameters /usr/include/opencv2/bioinspired/retina.hpp:149
pub trait RetinaParametersTraitConst {
	fn as_raw_RetinaParameters(&self) -> *const c_void;

	// OPLandIplParvo /usr/include/opencv2/bioinspired/retina.hpp:178
	#[inline]
	fn op_land_ipl_parvo(&self) -> crate::bioinspired::RetinaParameters_OPLandIplParvoParameters {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_RetinaParameters_getPropOPLandIplParvo_const(self.as_raw_RetinaParameters(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// IplMagno /usr/include/opencv2/bioinspired/retina.hpp:179
	#[inline]
	fn ipl_magno(&self) -> crate::bioinspired::RetinaParameters_IplMagnoParameters {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_RetinaParameters_getPropIplMagno_const(self.as_raw_RetinaParameters(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
}

pub trait RetinaParametersTrait: crate::bioinspired::RetinaParametersTraitConst {
	fn as_raw_mut_RetinaParameters(&mut self) -> *mut c_void;

	// OPLandIplParvo /usr/include/opencv2/bioinspired/retina.hpp:178
	#[inline]
	fn set_op_land_ipl_parvo(&mut self, val: crate::bioinspired::RetinaParameters_OPLandIplParvoParameters) {
		let ret = unsafe { sys::cv_bioinspired_RetinaParameters_setPropOPLandIplParvo_OPLandIplParvoParameters(self.as_raw_mut_RetinaParameters(), val.opencv_as_extern()) };
		ret
	}
	
	// IplMagno /usr/include/opencv2/bioinspired/retina.hpp:179
	#[inline]
	fn set_ipl_magno(&mut self, val: crate::bioinspired::RetinaParameters_IplMagnoParameters) {
		let ret = unsafe { sys::cv_bioinspired_RetinaParameters_setPropIplMagno_IplMagnoParameters(self.as_raw_mut_RetinaParameters(), val.opencv_as_extern()) };
		ret
	}
	
}

// RetinaParameters /usr/include/opencv2/bioinspired/retina.hpp:149
pub struct RetinaParameters {
	ptr: *mut c_void
}

opencv_type_boxed! { RetinaParameters }

impl Drop for RetinaParameters {
	fn drop(&mut self) {
		extern "C" { fn cv_RetinaParameters_delete(instance: *mut c_void); }
		unsafe { cv_RetinaParameters_delete(self.as_raw_mut_RetinaParameters()) };
	}
}

unsafe impl Send for RetinaParameters {}

impl crate::bioinspired::RetinaParametersTraitConst for RetinaParameters {
	#[inline] fn as_raw_RetinaParameters(&self) -> *const c_void { self.as_raw() }
}

impl crate::bioinspired::RetinaParametersTrait for RetinaParameters {
	#[inline] fn as_raw_mut_RetinaParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RetinaParameters {
}

// IplMagnoParameters /usr/include/opencv2/bioinspired/retina.hpp:165
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RetinaParameters_IplMagnoParameters {
	pub normalise_output: bool,
	pub parasol_cells_beta: f32,
	pub parasol_cells_tau: f32,
	pub parasol_cells_k: f32,
	pub amacrin_cells_temporal_cut_frequency: f32,
	pub v0_compression_parameter: f32,
	pub local_adaptintegration_tau: f32,
	pub local_adaptintegration_k: f32,
}

opencv_type_simple! { crate::bioinspired::RetinaParameters_IplMagnoParameters }

impl RetinaParameters_IplMagnoParameters {
	// IplMagnoParameters() /usr/include/opencv2/bioinspired/retina.hpp:166
	#[inline]
	pub fn default() -> Result<crate::bioinspired::RetinaParameters_IplMagnoParameters> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_RetinaParameters_IplMagnoParameters_IplMagnoParameters(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// OPLandIplParvoParameters /usr/include/opencv2/bioinspired/retina.hpp:151
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RetinaParameters_OPLandIplParvoParameters {
	pub color_mode: bool,
	pub normalise_output: bool,
	pub photoreceptors_local_adaptation_sensitivity: f32,
	pub photoreceptors_temporal_constant: f32,
	pub photoreceptors_spatial_constant: f32,
	pub horizontal_cells_gain: f32,
	pub hcells_temporal_constant: f32,
	pub hcells_spatial_constant: f32,
	pub ganglion_cells_sensitivity: f32,
}

opencv_type_simple! { crate::bioinspired::RetinaParameters_OPLandIplParvoParameters }

impl RetinaParameters_OPLandIplParvoParameters {
	// OPLandIplParvoParameters() /usr/include/opencv2/bioinspired/retina.hpp:152
	#[inline]
	pub fn default() -> Result<crate::bioinspired::RetinaParameters_OPLandIplParvoParameters> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_RetinaParameters_OPLandIplParvoParameters_OPLandIplParvoParameters(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// SegmentationParameters /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:82
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SegmentationParameters {
	pub threshold_on: f32,
	pub threshold_off: f32,
	pub local_energy_temporal_constant: f32,
	pub local_energy_spatial_constant: f32,
	pub neighborhood_energy_temporal_constant: f32,
	pub neighborhood_energy_spatial_constant: f32,
	pub context_energy_temporal_constant: f32,
	pub context_energy_spatial_constant: f32,
}

opencv_type_simple! { crate::bioinspired::SegmentationParameters }

impl SegmentationParameters {
	// SegmentationParameters() /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:84
	#[inline]
	pub fn default() -> Result<crate::bioinspired::SegmentationParameters> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_SegmentationParameters_SegmentationParameters(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// TransientAreasSegmentationModule /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:122
pub trait TransientAreasSegmentationModuleConst: core::AlgorithmTraitConst {
	fn as_raw_TransientAreasSegmentationModule(&self) -> *const c_void;

	// write(cv::String) /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:169
	#[inline]
	fn write(&self, fs: &str) -> Result<()> {
		extern_container_arg!(mut fs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_write_const_String(self.as_raw_TransientAreasSegmentationModule(), fs.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:174
	#[inline]
	fn write_to_storage(&self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_write_const_FileStorageR(self.as_raw_TransientAreasSegmentationModule(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait TransientAreasSegmentationModule: core::AlgorithmTrait + crate::bioinspired::TransientAreasSegmentationModuleConst {
	fn as_raw_mut_TransientAreasSegmentationModule(&mut self) -> *mut c_void;

	// getSize() /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:129
	#[inline]
	fn get_size(&mut self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_getSize(self.as_raw_mut_TransientAreasSegmentationModule(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * segmentation_parameter_file: ""
	/// * apply_default_setup_on_failure: true
	// setup(cv::String, const bool) /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:138
	#[inline]
	fn setup_from_file(&mut self, segmentation_parameter_file: &str, apply_default_setup_on_failure: bool) -> Result<()> {
		extern_container_arg!(mut segmentation_parameter_file);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_setup_String_const_bool(self.as_raw_mut_TransientAreasSegmentationModule(), segmentation_parameter_file.opencv_as_extern_mut(), apply_default_setup_on_failure, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * apply_default_setup_on_failure: true
	// setup(cv::FileStorage &, const bool) /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:147
	#[inline]
	fn setup_from_storage(&mut self, fs: &mut core::FileStorage, apply_default_setup_on_failure: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_setup_FileStorageR_const_bool(self.as_raw_mut_TransientAreasSegmentationModule(), fs.as_raw_mut_FileStorage(), apply_default_setup_on_failure, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setup(cv::bioinspired::SegmentationParameters) /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:155
	#[inline]
	fn setup(&mut self, new_parameters: crate::bioinspired::SegmentationParameters) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_setup_SegmentationParameters(self.as_raw_mut_TransientAreasSegmentationModule(), new_parameters.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getParameters() /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:159
	#[inline]
	fn get_parameters(&mut self) -> Result<crate::bioinspired::SegmentationParameters> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_getParameters(self.as_raw_mut_TransientAreasSegmentationModule(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// printSetup() /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:164
	#[inline]
	fn print_setup(&mut self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_printSetup(self.as_raw_mut_TransientAreasSegmentationModule(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * channel_index: 0
	// run(cv::InputArray, const int) /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:180
	#[inline]
	fn run(&mut self, input_to_segment: &dyn core::ToInputArray, channel_index: i32) -> Result<()> {
		input_array_arg!(input_to_segment);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_run_const__InputArrayR_const_int(self.as_raw_mut_TransientAreasSegmentationModule(), input_to_segment.as_raw__InputArray(), channel_index, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSegmentationPicture(cv::OutputArray) /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:185
	#[inline]
	fn get_segmentation_picture(&mut self, transient_areas: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(transient_areas);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_getSegmentationPicture_const__OutputArrayR(self.as_raw_mut_TransientAreasSegmentationModule(), transient_areas.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// clearAllBuffers() /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:189
	#[inline]
	fn clear_all_buffers(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_clearAllBuffers(self.as_raw_mut_TransientAreasSegmentationModule(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn TransientAreasSegmentationModule + '_ {
	// create(cv::Size) /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:194
	#[inline]
	pub fn create(input_size: core::Size) -> Result<core::Ptr<dyn crate::bioinspired::TransientAreasSegmentationModule>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_create_Size(input_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::bioinspired::TransientAreasSegmentationModule>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}