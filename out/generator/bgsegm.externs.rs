extern "C" {
	// createBackgroundSubtractorCNT(int, bool, int, bool) /usr/include/opencv2/bgsegm.hpp:240
	pub fn cv_bgsegm_createBackgroundSubtractorCNT_int_bool_int_bool(min_pixel_stability: i32, use_history: bool, max_pixel_stability: i32, is_parallel: bool, ocvrs_return: *mut Result<*mut c_void>);
	// createBackgroundSubtractorGMG(int, double) /usr/include/opencv2/bgsegm.hpp:185
	pub fn cv_bgsegm_createBackgroundSubtractorGMG_int_double(initialization_frames: i32, decision_threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
	// createBackgroundSubtractorGSOC(int, int, float, float, int, float, float, float, float, float, float) /usr/include/opencv2/bgsegm.hpp:302
	pub fn cv_bgsegm_createBackgroundSubtractorGSOC_int_int_float_float_int_float_float_float_float_float_float(mc: i32, n_samples: i32, replace_rate: f32, propagation_rate: f32, hits_threshold: i32, alpha: f32, beta: f32, blinking_supression_decay: f32, blinking_supression_multiplier: f32, noise_removal_threshold_fac_bg: f32, noise_removal_threshold_fac_fg: f32, ocvrs_return: *mut Result<*mut c_void>);
	// createBackgroundSubtractorLSBP(int, int, int, float, float, float, float, float, float, float, float, int, int) /usr/include/opencv2/bgsegm.hpp:322
	pub fn cv_bgsegm_createBackgroundSubtractorLSBP_int_int_int_float_float_float_float_float_float_float_float_int_int(mc: i32, n_samples: i32, lsbp_radius: i32, tlower: f32, tupper: f32, tinc: f32, tdec: f32, rscale: f32, rincdec: f32, noise_removal_threshold_fac_bg: f32, noise_removal_threshold_fac_fg: f32, lsb_pthreshold: i32, min_count: i32, ocvrs_return: *mut Result<*mut c_void>);
	// createBackgroundSubtractorMOG(int, int, double, double) /usr/include/opencv2/bgsegm.hpp:87
	pub fn cv_bgsegm_createBackgroundSubtractorMOG_int_int_double_double(history: i32, nmixtures: i32, background_ratio: f64, noise_sigma: f64, ocvrs_return: *mut Result<*mut c_void>);
	// createSyntheticSequenceGenerator(cv::InputArray, cv::InputArray, double, double, double, double) /usr/include/opencv2/bgsegm.hpp:372
	pub fn cv_bgsegm_createSyntheticSequenceGenerator_const__InputArrayR_const__InputArrayR_double_double_double_double(background: *const c_void, object: *const c_void, amplitude: f64, wavelength: f64, wavespeed: f64, objspeed: f64, ocvrs_return: *mut Result<*mut c_void>);
	// apply(cv::InputArray, cv::OutputArray, double) /usr/include/opencv2/bgsegm.hpp:199
	pub fn cv_bgsegm_BackgroundSubtractorCNT_apply_const__InputArrayR_const__OutputArrayR_double(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, learning_rate: f64, ocvrs_return: *mut Result_void);
	// getBackgroundImage(cv::OutputArray) /usr/include/opencv2/bgsegm.hpp:200
	pub fn cv_bgsegm_BackgroundSubtractorCNT_getBackgroundImage_const_const__OutputArrayR(instance: *const c_void, background_image: *const c_void, ocvrs_return: *mut Result_void);
	// getMinPixelStability() /usr/include/opencv2/bgsegm.hpp:204
	pub fn cv_bgsegm_BackgroundSubtractorCNT_getMinPixelStability_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMinPixelStability(int) /usr/include/opencv2/bgsegm.hpp:207
	pub fn cv_bgsegm_BackgroundSubtractorCNT_setMinPixelStability_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result_void);
	// getMaxPixelStability() /usr/include/opencv2/bgsegm.hpp:211
	pub fn cv_bgsegm_BackgroundSubtractorCNT_getMaxPixelStability_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMaxPixelStability(int) /usr/include/opencv2/bgsegm.hpp:214
	pub fn cv_bgsegm_BackgroundSubtractorCNT_setMaxPixelStability_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result_void);
	// getUseHistory() /usr/include/opencv2/bgsegm.hpp:218
	pub fn cv_bgsegm_BackgroundSubtractorCNT_getUseHistory_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUseHistory(bool) /usr/include/opencv2/bgsegm.hpp:221
	pub fn cv_bgsegm_BackgroundSubtractorCNT_setUseHistory_bool(instance: *mut c_void, value: bool, ocvrs_return: *mut Result_void);
	// getIsParallel() /usr/include/opencv2/bgsegm.hpp:225
	pub fn cv_bgsegm_BackgroundSubtractorCNT_getIsParallel_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setIsParallel(bool) /usr/include/opencv2/bgsegm.hpp:228
	pub fn cv_bgsegm_BackgroundSubtractorCNT_setIsParallel_bool(instance: *mut c_void, value: bool, ocvrs_return: *mut Result_void);
	// getMaxFeatures() /usr/include/opencv2/bgsegm.hpp:104
	pub fn cv_bgsegm_BackgroundSubtractorGMG_getMaxFeatures_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMaxFeatures(int) /usr/include/opencv2/bgsegm.hpp:107
	pub fn cv_bgsegm_BackgroundSubtractorGMG_setMaxFeatures_int(instance: *mut c_void, max_features: i32, ocvrs_return: *mut Result_void);
	// getDefaultLearningRate() /usr/include/opencv2/bgsegm.hpp:114
	pub fn cv_bgsegm_BackgroundSubtractorGMG_getDefaultLearningRate_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setDefaultLearningRate(double) /usr/include/opencv2/bgsegm.hpp:117
	pub fn cv_bgsegm_BackgroundSubtractorGMG_setDefaultLearningRate_double(instance: *mut c_void, lr: f64, ocvrs_return: *mut Result_void);
	// getNumFrames() /usr/include/opencv2/bgsegm.hpp:121
	pub fn cv_bgsegm_BackgroundSubtractorGMG_getNumFrames_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setNumFrames(int) /usr/include/opencv2/bgsegm.hpp:124
	pub fn cv_bgsegm_BackgroundSubtractorGMG_setNumFrames_int(instance: *mut c_void, nframes: i32, ocvrs_return: *mut Result_void);
	// getQuantizationLevels() /usr/include/opencv2/bgsegm.hpp:130
	pub fn cv_bgsegm_BackgroundSubtractorGMG_getQuantizationLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setQuantizationLevels(int) /usr/include/opencv2/bgsegm.hpp:133
	pub fn cv_bgsegm_BackgroundSubtractorGMG_setQuantizationLevels_int(instance: *mut c_void, nlevels: i32, ocvrs_return: *mut Result_void);
	// getBackgroundPrior() /usr/include/opencv2/bgsegm.hpp:137
	pub fn cv_bgsegm_BackgroundSubtractorGMG_getBackgroundPrior_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setBackgroundPrior(double) /usr/include/opencv2/bgsegm.hpp:140
	pub fn cv_bgsegm_BackgroundSubtractorGMG_setBackgroundPrior_double(instance: *mut c_void, bgprior: f64, ocvrs_return: *mut Result_void);
	// getSmoothingRadius() /usr/include/opencv2/bgsegm.hpp:144
	pub fn cv_bgsegm_BackgroundSubtractorGMG_getSmoothingRadius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setSmoothingRadius(int) /usr/include/opencv2/bgsegm.hpp:147
	pub fn cv_bgsegm_BackgroundSubtractorGMG_setSmoothingRadius_int(instance: *mut c_void, radius: i32, ocvrs_return: *mut Result_void);
	// getDecisionThreshold() /usr/include/opencv2/bgsegm.hpp:153
	pub fn cv_bgsegm_BackgroundSubtractorGMG_getDecisionThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setDecisionThreshold(double) /usr/include/opencv2/bgsegm.hpp:156
	pub fn cv_bgsegm_BackgroundSubtractorGMG_setDecisionThreshold_double(instance: *mut c_void, thresh: f64, ocvrs_return: *mut Result_void);
	// getUpdateBackgroundModel() /usr/include/opencv2/bgsegm.hpp:160
	pub fn cv_bgsegm_BackgroundSubtractorGMG_getUpdateBackgroundModel_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUpdateBackgroundModel(bool) /usr/include/opencv2/bgsegm.hpp:163
	pub fn cv_bgsegm_BackgroundSubtractorGMG_setUpdateBackgroundModel_bool(instance: *mut c_void, update: bool, ocvrs_return: *mut Result_void);
	// getMinVal() /usr/include/opencv2/bgsegm.hpp:167
	pub fn cv_bgsegm_BackgroundSubtractorGMG_getMinVal_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMinVal(double) /usr/include/opencv2/bgsegm.hpp:170
	pub fn cv_bgsegm_BackgroundSubtractorGMG_setMinVal_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getMaxVal() /usr/include/opencv2/bgsegm.hpp:174
	pub fn cv_bgsegm_BackgroundSubtractorGMG_getMaxVal_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxVal(double) /usr/include/opencv2/bgsegm.hpp:177
	pub fn cv_bgsegm_BackgroundSubtractorGMG_setMaxVal_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// apply(cv::InputArray, cv::OutputArray, double) /usr/include/opencv2/bgsegm.hpp:258
	pub fn cv_bgsegm_BackgroundSubtractorGSOC_apply_const__InputArrayR_const__OutputArrayR_double(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, learning_rate: f64, ocvrs_return: *mut Result_void);
	// getBackgroundImage(cv::OutputArray) /usr/include/opencv2/bgsegm.hpp:260
	pub fn cv_bgsegm_BackgroundSubtractorGSOC_getBackgroundImage_const_const__OutputArrayR(instance: *const c_void, background_image: *const c_void, ocvrs_return: *mut Result_void);
	// apply(cv::InputArray, cv::OutputArray, double) /usr/include/opencv2/bgsegm.hpp:269
	pub fn cv_bgsegm_BackgroundSubtractorLSBP_apply_const__InputArrayR_const__OutputArrayR_double(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, learning_rate: f64, ocvrs_return: *mut Result_void);
	// getBackgroundImage(cv::OutputArray) /usr/include/opencv2/bgsegm.hpp:271
	pub fn cv_bgsegm_BackgroundSubtractorLSBP_getBackgroundImage_const_const__OutputArrayR(instance: *const c_void, background_image: *const c_void, ocvrs_return: *mut Result_void);
	// calcLocalSVDValues(cv::OutputArray, const cv::Mat &) /usr/include/opencv2/bgsegm.hpp:279
	pub fn cv_bgsegm_BackgroundSubtractorLSBPDesc_calcLocalSVDValues_const__OutputArrayR_const_MatR(local_svd_values: *const c_void, frame: *const c_void, ocvrs_return: *mut Result_void);
	// computeFromLocalSVDValues(cv::OutputArray, const cv::Mat &, const cv::Point2i *) /usr/include/opencv2/bgsegm.hpp:281
	pub fn cv_bgsegm_BackgroundSubtractorLSBPDesc_computeFromLocalSVDValues_const__OutputArrayR_const_MatR_const_Point2iX(desc: *const c_void, local_svd_values: *const c_void, lsbp_sample_points: *const core::Point2i, ocvrs_return: *mut Result_void);
	// compute(cv::OutputArray, const cv::Mat &, const cv::Point2i *) /usr/include/opencv2/bgsegm.hpp:283
	pub fn cv_bgsegm_BackgroundSubtractorLSBPDesc_compute_const__OutputArrayR_const_MatR_const_Point2iX(desc: *const c_void, frame: *const c_void, lsbp_sample_points: *const core::Point2i, ocvrs_return: *mut Result_void);
	// getHistory() /usr/include/opencv2/bgsegm.hpp:65
	pub fn cv_bgsegm_BackgroundSubtractorMOG_getHistory_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setHistory(int) /usr/include/opencv2/bgsegm.hpp:66
	pub fn cv_bgsegm_BackgroundSubtractorMOG_setHistory_int(instance: *mut c_void, nframes: i32, ocvrs_return: *mut Result_void);
	// getNMixtures() /usr/include/opencv2/bgsegm.hpp:68
	pub fn cv_bgsegm_BackgroundSubtractorMOG_getNMixtures_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setNMixtures(int) /usr/include/opencv2/bgsegm.hpp:69
	pub fn cv_bgsegm_BackgroundSubtractorMOG_setNMixtures_int(instance: *mut c_void, nmix: i32, ocvrs_return: *mut Result_void);
	// getBackgroundRatio() /usr/include/opencv2/bgsegm.hpp:71
	pub fn cv_bgsegm_BackgroundSubtractorMOG_getBackgroundRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setBackgroundRatio(double) /usr/include/opencv2/bgsegm.hpp:72
	pub fn cv_bgsegm_BackgroundSubtractorMOG_setBackgroundRatio_double(instance: *mut c_void, background_ratio: f64, ocvrs_return: *mut Result_void);
	// getNoiseSigma() /usr/include/opencv2/bgsegm.hpp:74
	pub fn cv_bgsegm_BackgroundSubtractorMOG_getNoiseSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setNoiseSigma(double) /usr/include/opencv2/bgsegm.hpp:75
	pub fn cv_bgsegm_BackgroundSubtractorMOG_setNoiseSigma_double(instance: *mut c_void, noise_sigma: f64, ocvrs_return: *mut Result_void);
	// SyntheticSequenceGenerator(cv::InputArray, cv::InputArray, double, double, double, double) /usr/include/opencv2/bgsegm.hpp:353
	pub fn cv_bgsegm_SyntheticSequenceGenerator_SyntheticSequenceGenerator_const__InputArrayR_const__InputArrayR_double_double_double_double(background: *const c_void, object: *const c_void, amplitude: f64, wavelength: f64, wavespeed: f64, objspeed: f64, ocvrs_return: *mut Result<*mut c_void>);
	// getNextFrame(cv::OutputArray, cv::OutputArray) /usr/include/opencv2/bgsegm.hpp:360
	pub fn cv_bgsegm_SyntheticSequenceGenerator_getNextFrame_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, frame: *const c_void, gt_mask: *const c_void, ocvrs_return: *mut Result_void);
}
