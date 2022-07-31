extern "C" {
	// getInputSize() /usr/include/opencv2/bioinspired/retina.hpp:215
	pub fn cv_bioinspired_Retina_getInputSize(instance: *mut c_void, ocvrs_return: *mut Result<core::Size>);
	// getOutputSize() /usr/include/opencv2/bioinspired/retina.hpp:221
	pub fn cv_bioinspired_Retina_getOutputSize(instance: *mut c_void, ocvrs_return: *mut Result<core::Size>);
	// setup(cv::String, const bool) /usr/include/opencv2/bioinspired/retina.hpp:233
	pub fn cv_bioinspired_Retina_setup_String_const_bool(instance: *mut c_void, retina_parameter_file: *mut c_char, apply_default_setup_on_failure: bool, ocvrs_return: *mut Result_void);
	// setup(cv::FileStorage &, const bool) /usr/include/opencv2/bioinspired/retina.hpp:239
	pub fn cv_bioinspired_Retina_setup_FileStorageR_const_bool(instance: *mut c_void, fs: *mut c_void, apply_default_setup_on_failure: bool, ocvrs_return: *mut Result_void);
	// setup(cv::bioinspired::RetinaParameters) /usr/include/opencv2/bioinspired/retina.hpp:244
	pub fn cv_bioinspired_Retina_setup_RetinaParameters(instance: *mut c_void, new_parameters: *mut c_void, ocvrs_return: *mut Result_void);
	// getParameters() /usr/include/opencv2/bioinspired/retina.hpp:249
	pub fn cv_bioinspired_Retina_getParameters(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// printSetup() /usr/include/opencv2/bioinspired/retina.hpp:254
	pub fn cv_bioinspired_Retina_printSetup(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// write(cv::String) /usr/include/opencv2/bioinspired/retina.hpp:260
	pub fn cv_bioinspired_Retina_write_const_String(instance: *const c_void, fs: *mut c_char, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/bioinspired/retina.hpp:263
	pub fn cv_bioinspired_Retina_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// setupOPLandIPLParvoChannel(const bool, const bool, const float, const float, const float, const float, const float, const float, const float) /usr/include/opencv2/bioinspired/retina.hpp:299
	pub fn cv_bioinspired_Retina_setupOPLandIPLParvoChannel_const_bool_const_bool_const_float_const_float_const_float_const_float_const_float_const_float_const_float(instance: *mut c_void, color_mode: bool, normalise_output: bool, photoreceptors_local_adaptation_sensitivity: f32, photoreceptors_temporal_constant: f32, photoreceptors_spatial_constant: f32, horizontal_cells_gain: f32, hcells_temporal_constant: f32, hcells_spatial_constant: f32, ganglion_cells_sensitivity: f32, ocvrs_return: *mut Result_void);
	// setupIPLMagnoChannel(const bool, const float, const float, const float, const float, const float, const float, const float) /usr/include/opencv2/bioinspired/retina.hpp:326
	pub fn cv_bioinspired_Retina_setupIPLMagnoChannel_const_bool_const_float_const_float_const_float_const_float_const_float_const_float_const_float(instance: *mut c_void, normalise_output: bool, parasol_cells_beta: f32, parasol_cells_tau: f32, parasol_cells_k: f32, amacrin_cells_temporal_cut_frequency: f32, v0_compression_parameter: f32, local_adaptintegration_tau: f32, local_adaptintegration_k: f32, ocvrs_return: *mut Result_void);
	// run(cv::InputArray) /usr/include/opencv2/bioinspired/retina.hpp:335
	pub fn cv_bioinspired_Retina_run_const__InputArrayR(instance: *mut c_void, input_image: *const c_void, ocvrs_return: *mut Result_void);
	// applyFastToneMapping(cv::InputArray, cv::OutputArray) /usr/include/opencv2/bioinspired/retina.hpp:353
	pub fn cv_bioinspired_Retina_applyFastToneMapping_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, input_image: *const c_void, output_tone_mapped_image: *const c_void, ocvrs_return: *mut Result_void);
	// getParvo(cv::OutputArray) /usr/include/opencv2/bioinspired/retina.hpp:367
	pub fn cv_bioinspired_Retina_getParvo_const__OutputArrayR(instance: *mut c_void, retina_output_parvo: *const c_void, ocvrs_return: *mut Result_void);
	// getParvoRAW(cv::OutputArray) /usr/include/opencv2/bioinspired/retina.hpp:372
	pub fn cv_bioinspired_Retina_getParvoRAW_const__OutputArrayR(instance: *mut c_void, retina_output_parvo: *const c_void, ocvrs_return: *mut Result_void);
	// getMagno(cv::OutputArray) /usr/include/opencv2/bioinspired/retina.hpp:384
	pub fn cv_bioinspired_Retina_getMagno_const__OutputArrayR(instance: *mut c_void, retina_output_magno: *const c_void, ocvrs_return: *mut Result_void);
	// getMagnoRAW(cv::OutputArray) /usr/include/opencv2/bioinspired/retina.hpp:389
	pub fn cv_bioinspired_Retina_getMagnoRAW_const__OutputArrayR(instance: *mut c_void, retina_output_magno: *const c_void, ocvrs_return: *mut Result_void);
	// getMagnoRAW() /usr/include/opencv2/bioinspired/retina.hpp:392
	pub fn cv_bioinspired_Retina_getMagnoRAW_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getParvoRAW() /usr/include/opencv2/bioinspired/retina.hpp:394
	pub fn cv_bioinspired_Retina_getParvoRAW_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setColorSaturation(const bool, const float) /usr/include/opencv2/bioinspired/retina.hpp:402
	pub fn cv_bioinspired_Retina_setColorSaturation_const_bool_const_float(instance: *mut c_void, saturate_colors: bool, color_saturation_value: f32, ocvrs_return: *mut Result_void);
	// clearBuffers() /usr/include/opencv2/bioinspired/retina.hpp:409
	pub fn cv_bioinspired_Retina_clearBuffers(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// activateMovingContoursProcessing(const bool) /usr/include/opencv2/bioinspired/retina.hpp:416
	pub fn cv_bioinspired_Retina_activateMovingContoursProcessing_const_bool(instance: *mut c_void, activate: bool, ocvrs_return: *mut Result_void);
	// activateContoursProcessing(const bool) /usr/include/opencv2/bioinspired/retina.hpp:424
	pub fn cv_bioinspired_Retina_activateContoursProcessing_const_bool(instance: *mut c_void, activate: bool, ocvrs_return: *mut Result_void);
	// create(cv::Size) /usr/include/opencv2/bioinspired/retina.hpp:427
	pub fn cv_bioinspired_Retina_create_Size(input_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
	// create(cv::Size, const bool, int, const bool, const float, const float) /usr/include/opencv2/bioinspired/retina.hpp:444
	pub fn cv_bioinspired_Retina_create_Size_const_bool_int_const_bool_const_float_const_float(input_size: *const core::Size, color_mode: bool, color_sampling_method: i32, use_retina_log_sampling: bool, reduction_factor: f32, sampling_strength: f32, ocvrs_return: *mut Result<*mut c_void>);
	// applyFastToneMapping(cv::InputArray, cv::OutputArray) /usr/include/opencv2/bioinspired/retinafasttonemapping.hpp:119
	pub fn cv_bioinspired_RetinaFastToneMapping_applyFastToneMapping_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, input_image: *const c_void, output_tone_mapped_image: *const c_void, ocvrs_return: *mut Result_void);
	// setup(const float, const float, const float) /usr/include/opencv2/bioinspired/retinafasttonemapping.hpp:128
	pub fn cv_bioinspired_RetinaFastToneMapping_setup_const_float_const_float_const_float(instance: *mut c_void, photoreceptors_neighborhood_radius: f32, ganglioncells_neighborhood_radius: f32, mean_luminance_modulator_k: f32, ocvrs_return: *mut Result_void);
	// create(cv::Size) /usr/include/opencv2/bioinspired/retinafasttonemapping.hpp:130
	pub fn cv_bioinspired_RetinaFastToneMapping_create_Size(input_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
	// OPLandIplParvo /usr/include/opencv2/bioinspired/retina.hpp:178
	pub fn cv_bioinspired_RetinaParameters_getPropOPLandIplParvo_const(instance: *const c_void, ocvrs_return: *mut crate::bioinspired::RetinaParameters_OPLandIplParvoParameters);
	// OPLandIplParvo /usr/include/opencv2/bioinspired/retina.hpp:178
	pub fn cv_bioinspired_RetinaParameters_setPropOPLandIplParvo_OPLandIplParvoParameters(instance: *mut c_void, val: *const crate::bioinspired::RetinaParameters_OPLandIplParvoParameters);
	// IplMagno /usr/include/opencv2/bioinspired/retina.hpp:179
	pub fn cv_bioinspired_RetinaParameters_getPropIplMagno_const(instance: *const c_void, ocvrs_return: *mut crate::bioinspired::RetinaParameters_IplMagnoParameters);
	// IplMagno /usr/include/opencv2/bioinspired/retina.hpp:179
	pub fn cv_bioinspired_RetinaParameters_setPropIplMagno_IplMagnoParameters(instance: *mut c_void, val: *const crate::bioinspired::RetinaParameters_IplMagnoParameters);
	// IplMagnoParameters() /usr/include/opencv2/bioinspired/retina.hpp:166
	pub fn cv_bioinspired_RetinaParameters_IplMagnoParameters_IplMagnoParameters(ocvrs_return: *mut Result<crate::bioinspired::RetinaParameters_IplMagnoParameters>);
	// OPLandIplParvoParameters() /usr/include/opencv2/bioinspired/retina.hpp:152
	pub fn cv_bioinspired_RetinaParameters_OPLandIplParvoParameters_OPLandIplParvoParameters(ocvrs_return: *mut Result<crate::bioinspired::RetinaParameters_OPLandIplParvoParameters>);
	// SegmentationParameters() /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:84
	pub fn cv_bioinspired_SegmentationParameters_SegmentationParameters(ocvrs_return: *mut Result<crate::bioinspired::SegmentationParameters>);
	// getSize() /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:129
	pub fn cv_bioinspired_TransientAreasSegmentationModule_getSize(instance: *mut c_void, ocvrs_return: *mut Result<core::Size>);
	// setup(cv::String, const bool) /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:138
	pub fn cv_bioinspired_TransientAreasSegmentationModule_setup_String_const_bool(instance: *mut c_void, segmentation_parameter_file: *mut c_char, apply_default_setup_on_failure: bool, ocvrs_return: *mut Result_void);
	// setup(cv::FileStorage &, const bool) /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:147
	pub fn cv_bioinspired_TransientAreasSegmentationModule_setup_FileStorageR_const_bool(instance: *mut c_void, fs: *mut c_void, apply_default_setup_on_failure: bool, ocvrs_return: *mut Result_void);
	// setup(cv::bioinspired::SegmentationParameters) /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:155
	pub fn cv_bioinspired_TransientAreasSegmentationModule_setup_SegmentationParameters(instance: *mut c_void, new_parameters: *const crate::bioinspired::SegmentationParameters, ocvrs_return: *mut Result_void);
	// getParameters() /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:159
	pub fn cv_bioinspired_TransientAreasSegmentationModule_getParameters(instance: *mut c_void, ocvrs_return: *mut Result<crate::bioinspired::SegmentationParameters>);
	// printSetup() /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:164
	pub fn cv_bioinspired_TransientAreasSegmentationModule_printSetup(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// write(cv::String) /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:169
	pub fn cv_bioinspired_TransientAreasSegmentationModule_write_const_String(instance: *const c_void, fs: *mut c_char, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:174
	pub fn cv_bioinspired_TransientAreasSegmentationModule_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// run(cv::InputArray, const int) /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:180
	pub fn cv_bioinspired_TransientAreasSegmentationModule_run_const__InputArrayR_const_int(instance: *mut c_void, input_to_segment: *const c_void, channel_index: i32, ocvrs_return: *mut Result_void);
	// getSegmentationPicture(cv::OutputArray) /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:185
	pub fn cv_bioinspired_TransientAreasSegmentationModule_getSegmentationPicture_const__OutputArrayR(instance: *mut c_void, transient_areas: *const c_void, ocvrs_return: *mut Result_void);
	// clearAllBuffers() /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:189
	pub fn cv_bioinspired_TransientAreasSegmentationModule_clearAllBuffers(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// create(cv::Size) /usr/include/opencv2/bioinspired/transientareassegmentationmodule.hpp:194
	pub fn cv_bioinspired_TransientAreasSegmentationModule_create_Size(input_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
}
