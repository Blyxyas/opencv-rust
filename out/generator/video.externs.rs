extern "C" {
	// CamShift(cv::InputArray, cv::Rect &, cv::TermCriteria) /usr/include/opencv2/video/tracking.hpp:79
	pub fn cv_CamShift_const__InputArrayR_RectR_TermCriteria(prob_image: *const c_void, window: *mut core::Rect, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<*mut c_void>);
	// buildOpticalFlowPyramid(cv::InputArray, cv::OutputArrayOfArrays, cv::Size, int, bool, int, int, bool) /usr/include/opencv2/video/tracking.hpp:121
	pub fn cv_buildOpticalFlowPyramid_const__InputArrayR_const__OutputArrayR_Size_int_bool_int_int_bool(img: *const c_void, pyramid: *const c_void, win_size: *const core::Size, max_level: i32, with_derivatives: bool, pyr_border: i32, deriv_border: i32, try_reuse_input_image: bool, ocvrs_return: *mut Result<i32>);
	// calcOpticalFlowFarneback(cv::InputArray, cv::InputArray, cv::InputOutputArray, double, int, int, int, int, double, int) /usr/include/opencv2/video/tracking.hpp:223
	pub fn cv_calcOpticalFlowFarneback_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_double_int_int_int_int_double_int(prev: *const c_void, next: *const c_void, flow: *const c_void, pyr_scale: f64, levels: i32, winsize: i32, iterations: i32, poly_n: i32, poly_sigma: f64, flags: i32, ocvrs_return: *mut Result_void);
	// calcOpticalFlowPyrLK(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray, cv::Size, int, cv::TermCriteria, int, double) /usr/include/opencv2/video/tracking.hpp:178
	pub fn cv_calcOpticalFlowPyrLK_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_Size_int_TermCriteria_int_double(prev_img: *const c_void, next_img: *const c_void, prev_pts: *const c_void, next_pts: *const c_void, status: *const c_void, err: *const c_void, win_size: *const core::Size, max_level: i32, criteria: *const core::TermCriteria, flags: i32, min_eig_threshold: f64, ocvrs_return: *mut Result_void);
	// computeECC(cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/video/tracking.hpp:279
	pub fn cv_computeECC_const__InputArrayR_const__InputArrayR_const__InputArrayR(template_image: *const c_void, input_image: *const c_void, input_mask: *const c_void, ocvrs_return: *mut Result<f64>);
	// createBackgroundSubtractorKNN(int, double, bool) /usr/include/opencv2/video/background_segm.hpp:310
	pub fn cv_createBackgroundSubtractorKNN_int_double_bool(history: i32, dist2_threshold: f64, detect_shadows: bool, ocvrs_return: *mut Result<*mut c_void>);
	// createBackgroundSubtractorMOG2(int, double, bool) /usr/include/opencv2/video/background_segm.hpp:221
	pub fn cv_createBackgroundSubtractorMOG2_int_double_bool(history: i32, var_threshold: f64, detect_shadows: bool, ocvrs_return: *mut Result<*mut c_void>);
	// estimateRigidTransform(cv::InputArray, cv::InputArray, bool) /usr/include/opencv2/video/tracking.hpp:258
	pub fn cv_estimateRigidTransform_const__InputArrayR_const__InputArrayR_bool(src: *const c_void, dst: *const c_void, full_affine: bool, ocvrs_return: *mut Result<*mut c_void>);
	// findTransformECC(cv::InputArray, cv::InputArray, cv::InputOutputArray, int, cv::TermCriteria, cv::InputArray) /usr/include/opencv2/video/tracking.hpp:343
	pub fn cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR(template_image: *const c_void, input_image: *const c_void, warp_matrix: *const c_void, motion_type: i32, criteria: *const core::TermCriteria, input_mask: *const c_void, ocvrs_return: *mut Result<f64>);
	// findTransformECC(cv::InputArray, cv::InputArray, cv::InputOutputArray, int, cv::TermCriteria, cv::InputArray, int) /usr/include/opencv2/video/tracking.hpp:336
	pub fn cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR_int(template_image: *const c_void, input_image: *const c_void, warp_matrix: *const c_void, motion_type: i32, criteria: *const core::TermCriteria, input_mask: *const c_void, gauss_filt_size: i32, ocvrs_return: *mut Result<f64>);
	// meanShift(cv::InputArray, cv::Rect &, cv::TermCriteria) /usr/include/opencv2/video/tracking.hpp:104
	pub fn cv_meanShift_const__InputArrayR_RectR_TermCriteria(prob_image: *const c_void, window: *mut core::Rect, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<i32>);
	// readOpticalFlow(const cv::String &) /usr/include/opencv2/video/tracking.hpp:421
	pub fn cv_readOpticalFlow_const_StringR(path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// writeOpticalFlow(const cv::String &, cv::InputArray) /usr/include/opencv2/video/tracking.hpp:431
	pub fn cv_writeOpticalFlow_const_StringR_const__InputArrayR(path: *const c_char, flow: *const c_void, ocvrs_return: *mut Result<bool>);
	// apply(cv::InputArray, cv::OutputArray, double) /usr/include/opencv2/video/background_segm.hpp:72
	pub fn cv_BackgroundSubtractor_apply_const__InputArrayR_const__OutputArrayR_double(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, learning_rate: f64, ocvrs_return: *mut Result_void);
	// getBackgroundImage(cv::OutputArray) /usr/include/opencv2/video/background_segm.hpp:81
	pub fn cv_BackgroundSubtractor_getBackgroundImage_const_const__OutputArrayR(instance: *const c_void, background_image: *const c_void, ocvrs_return: *mut Result_void);
	// getHistory() /usr/include/opencv2/video/background_segm.hpp:234
	pub fn cv_BackgroundSubtractorKNN_getHistory_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setHistory(int) /usr/include/opencv2/video/background_segm.hpp:237
	pub fn cv_BackgroundSubtractorKNN_setHistory_int(instance: *mut c_void, history: i32, ocvrs_return: *mut Result_void);
	// getNSamples() /usr/include/opencv2/video/background_segm.hpp:241
	pub fn cv_BackgroundSubtractorKNN_getNSamples_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setNSamples(int) /usr/include/opencv2/video/background_segm.hpp:246
	pub fn cv_BackgroundSubtractorKNN_setNSamples_int(instance: *mut c_void, _n_n: i32, ocvrs_return: *mut Result_void);
	// getDist2Threshold() /usr/include/opencv2/video/background_segm.hpp:253
	pub fn cv_BackgroundSubtractorKNN_getDist2Threshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setDist2Threshold(double) /usr/include/opencv2/video/background_segm.hpp:256
	pub fn cv_BackgroundSubtractorKNN_setDist2Threshold_double(instance: *mut c_void, _dist2_threshold: f64, ocvrs_return: *mut Result_void);
	// getkNNSamples() /usr/include/opencv2/video/background_segm.hpp:263
	pub fn cv_BackgroundSubtractorKNN_getkNNSamples_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setkNNSamples(int) /usr/include/opencv2/video/background_segm.hpp:266
	pub fn cv_BackgroundSubtractorKNN_setkNNSamples_int(instance: *mut c_void, _nk_nn: i32, ocvrs_return: *mut Result_void);
	// getDetectShadows() /usr/include/opencv2/video/background_segm.hpp:273
	pub fn cv_BackgroundSubtractorKNN_getDetectShadows_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setDetectShadows(bool) /usr/include/opencv2/video/background_segm.hpp:276
	pub fn cv_BackgroundSubtractorKNN_setDetectShadows_bool(instance: *mut c_void, detect_shadows: bool, ocvrs_return: *mut Result_void);
	// getShadowValue() /usr/include/opencv2/video/background_segm.hpp:283
	pub fn cv_BackgroundSubtractorKNN_getShadowValue_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setShadowValue(int) /usr/include/opencv2/video/background_segm.hpp:286
	pub fn cv_BackgroundSubtractorKNN_setShadowValue_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result_void);
	// getShadowThreshold() /usr/include/opencv2/video/background_segm.hpp:295
	pub fn cv_BackgroundSubtractorKNN_getShadowThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setShadowThreshold(double) /usr/include/opencv2/video/background_segm.hpp:298
	pub fn cv_BackgroundSubtractorKNN_setShadowThreshold_double(instance: *mut c_void, threshold: f64, ocvrs_return: *mut Result_void);
	// getHistory() /usr/include/opencv2/video/background_segm.hpp:95
	pub fn cv_BackgroundSubtractorMOG2_getHistory_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setHistory(int) /usr/include/opencv2/video/background_segm.hpp:98
	pub fn cv_BackgroundSubtractorMOG2_setHistory_int(instance: *mut c_void, history: i32, ocvrs_return: *mut Result_void);
	// getNMixtures() /usr/include/opencv2/video/background_segm.hpp:102
	pub fn cv_BackgroundSubtractorMOG2_getNMixtures_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setNMixtures(int) /usr/include/opencv2/video/background_segm.hpp:107
	pub fn cv_BackgroundSubtractorMOG2_setNMixtures_int(instance: *mut c_void, nmixtures: i32, ocvrs_return: *mut Result_void);
	// getBackgroundRatio() /usr/include/opencv2/video/background_segm.hpp:115
	pub fn cv_BackgroundSubtractorMOG2_getBackgroundRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setBackgroundRatio(double) /usr/include/opencv2/video/background_segm.hpp:118
	pub fn cv_BackgroundSubtractorMOG2_setBackgroundRatio_double(instance: *mut c_void, ratio: f64, ocvrs_return: *mut Result_void);
	// getVarThreshold() /usr/include/opencv2/video/background_segm.hpp:125
	pub fn cv_BackgroundSubtractorMOG2_getVarThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setVarThreshold(double) /usr/include/opencv2/video/background_segm.hpp:128
	pub fn cv_BackgroundSubtractorMOG2_setVarThreshold_double(instance: *mut c_void, var_threshold: f64, ocvrs_return: *mut Result_void);
	// getVarThresholdGen() /usr/include/opencv2/video/background_segm.hpp:138
	pub fn cv_BackgroundSubtractorMOG2_getVarThresholdGen_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setVarThresholdGen(double) /usr/include/opencv2/video/background_segm.hpp:141
	pub fn cv_BackgroundSubtractorMOG2_setVarThresholdGen_double(instance: *mut c_void, var_threshold_gen: f64, ocvrs_return: *mut Result_void);
	// getVarInit() /usr/include/opencv2/video/background_segm.hpp:145
	pub fn cv_BackgroundSubtractorMOG2_getVarInit_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setVarInit(double) /usr/include/opencv2/video/background_segm.hpp:148
	pub fn cv_BackgroundSubtractorMOG2_setVarInit_double(instance: *mut c_void, var_init: f64, ocvrs_return: *mut Result_void);
	// getVarMin() /usr/include/opencv2/video/background_segm.hpp:150
	pub fn cv_BackgroundSubtractorMOG2_getVarMin_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setVarMin(double) /usr/include/opencv2/video/background_segm.hpp:151
	pub fn cv_BackgroundSubtractorMOG2_setVarMin_double(instance: *mut c_void, var_min: f64, ocvrs_return: *mut Result_void);
	// getVarMax() /usr/include/opencv2/video/background_segm.hpp:153
	pub fn cv_BackgroundSubtractorMOG2_getVarMax_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setVarMax(double) /usr/include/opencv2/video/background_segm.hpp:154
	pub fn cv_BackgroundSubtractorMOG2_setVarMax_double(instance: *mut c_void, var_max: f64, ocvrs_return: *mut Result_void);
	// getComplexityReductionThreshold() /usr/include/opencv2/video/background_segm.hpp:162
	pub fn cv_BackgroundSubtractorMOG2_getComplexityReductionThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setComplexityReductionThreshold(double) /usr/include/opencv2/video/background_segm.hpp:165
	pub fn cv_BackgroundSubtractorMOG2_setComplexityReductionThreshold_double(instance: *mut c_void, ct: f64, ocvrs_return: *mut Result_void);
	// getDetectShadows() /usr/include/opencv2/video/background_segm.hpp:172
	pub fn cv_BackgroundSubtractorMOG2_getDetectShadows_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setDetectShadows(bool) /usr/include/opencv2/video/background_segm.hpp:175
	pub fn cv_BackgroundSubtractorMOG2_setDetectShadows_bool(instance: *mut c_void, detect_shadows: bool, ocvrs_return: *mut Result_void);
	// getShadowValue() /usr/include/opencv2/video/background_segm.hpp:182
	pub fn cv_BackgroundSubtractorMOG2_getShadowValue_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setShadowValue(int) /usr/include/opencv2/video/background_segm.hpp:185
	pub fn cv_BackgroundSubtractorMOG2_setShadowValue_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result_void);
	// getShadowThreshold() /usr/include/opencv2/video/background_segm.hpp:194
	pub fn cv_BackgroundSubtractorMOG2_getShadowThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setShadowThreshold(double) /usr/include/opencv2/video/background_segm.hpp:197
	pub fn cv_BackgroundSubtractorMOG2_setShadowThreshold_double(instance: *mut c_void, threshold: f64, ocvrs_return: *mut Result_void);
	// apply(cv::InputArray, cv::OutputArray, double) /usr/include/opencv2/video/background_segm.hpp:208
	pub fn cv_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR_double(instance: *mut c_void, image: *const c_void, fgmask: *const c_void, learning_rate: f64, ocvrs_return: *mut Result_void);
	// getFinestScale() /usr/include/opencv2/video/tracking.hpp:597
	pub fn cv_DISOpticalFlow_getFinestScale_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setFinestScale(int) /usr/include/opencv2/video/tracking.hpp:599
	pub fn cv_DISOpticalFlow_setFinestScale_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getPatchSize() /usr/include/opencv2/video/tracking.hpp:604
	pub fn cv_DISOpticalFlow_getPatchSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setPatchSize(int) /usr/include/opencv2/video/tracking.hpp:606
	pub fn cv_DISOpticalFlow_setPatchSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getPatchStride() /usr/include/opencv2/video/tracking.hpp:611
	pub fn cv_DISOpticalFlow_getPatchStride_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setPatchStride(int) /usr/include/opencv2/video/tracking.hpp:613
	pub fn cv_DISOpticalFlow_setPatchStride_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getGradientDescentIterations() /usr/include/opencv2/video/tracking.hpp:618
	pub fn cv_DISOpticalFlow_getGradientDescentIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setGradientDescentIterations(int) /usr/include/opencv2/video/tracking.hpp:620
	pub fn cv_DISOpticalFlow_setGradientDescentIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getVariationalRefinementIterations() /usr/include/opencv2/video/tracking.hpp:626
	pub fn cv_DISOpticalFlow_getVariationalRefinementIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setVariationalRefinementIterations(int) /usr/include/opencv2/video/tracking.hpp:628
	pub fn cv_DISOpticalFlow_setVariationalRefinementIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getVariationalRefinementAlpha() /usr/include/opencv2/video/tracking.hpp:632
	pub fn cv_DISOpticalFlow_getVariationalRefinementAlpha_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setVariationalRefinementAlpha(float) /usr/include/opencv2/video/tracking.hpp:634
	pub fn cv_DISOpticalFlow_setVariationalRefinementAlpha_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getVariationalRefinementDelta() /usr/include/opencv2/video/tracking.hpp:638
	pub fn cv_DISOpticalFlow_getVariationalRefinementDelta_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setVariationalRefinementDelta(float) /usr/include/opencv2/video/tracking.hpp:640
	pub fn cv_DISOpticalFlow_setVariationalRefinementDelta_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getVariationalRefinementGamma() /usr/include/opencv2/video/tracking.hpp:644
	pub fn cv_DISOpticalFlow_getVariationalRefinementGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setVariationalRefinementGamma(float) /usr/include/opencv2/video/tracking.hpp:646
	pub fn cv_DISOpticalFlow_setVariationalRefinementGamma_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getUseMeanNormalization() /usr/include/opencv2/video/tracking.hpp:654
	pub fn cv_DISOpticalFlow_getUseMeanNormalization_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUseMeanNormalization(bool) /usr/include/opencv2/video/tracking.hpp:656
	pub fn cv_DISOpticalFlow_setUseMeanNormalization_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// getUseSpatialPropagation() /usr/include/opencv2/video/tracking.hpp:663
	pub fn cv_DISOpticalFlow_getUseSpatialPropagation_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUseSpatialPropagation(bool) /usr/include/opencv2/video/tracking.hpp:665
	pub fn cv_DISOpticalFlow_setUseSpatialPropagation_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// create(int) /usr/include/opencv2/video/tracking.hpp:671
	pub fn cv_DISOpticalFlow_create_int(preset: i32, ocvrs_return: *mut Result<*mut c_void>);
	// calc(cv::InputArray, cv::InputArray, cv::InputOutputArray) /usr/include/opencv2/video/tracking.hpp:445
	pub fn cv_DenseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(instance: *mut c_void, i0: *const c_void, i1: *const c_void, flow: *const c_void, ocvrs_return: *mut Result_void);
	// collectGarbage() /usr/include/opencv2/video/tracking.hpp:448
	pub fn cv_DenseOpticalFlow_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// getNumLevels() /usr/include/opencv2/video/tracking.hpp:478
	pub fn cv_FarnebackOpticalFlow_getNumLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setNumLevels(int) /usr/include/opencv2/video/tracking.hpp:479
	pub fn cv_FarnebackOpticalFlow_setNumLevels_int(instance: *mut c_void, num_levels: i32, ocvrs_return: *mut Result_void);
	// getPyrScale() /usr/include/opencv2/video/tracking.hpp:481
	pub fn cv_FarnebackOpticalFlow_getPyrScale_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setPyrScale(double) /usr/include/opencv2/video/tracking.hpp:482
	pub fn cv_FarnebackOpticalFlow_setPyrScale_double(instance: *mut c_void, pyr_scale: f64, ocvrs_return: *mut Result_void);
	// getFastPyramids() /usr/include/opencv2/video/tracking.hpp:484
	pub fn cv_FarnebackOpticalFlow_getFastPyramids_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setFastPyramids(bool) /usr/include/opencv2/video/tracking.hpp:485
	pub fn cv_FarnebackOpticalFlow_setFastPyramids_bool(instance: *mut c_void, fast_pyramids: bool, ocvrs_return: *mut Result_void);
	// getWinSize() /usr/include/opencv2/video/tracking.hpp:487
	pub fn cv_FarnebackOpticalFlow_getWinSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setWinSize(int) /usr/include/opencv2/video/tracking.hpp:488
	pub fn cv_FarnebackOpticalFlow_setWinSize_int(instance: *mut c_void, win_size: i32, ocvrs_return: *mut Result_void);
	// getNumIters() /usr/include/opencv2/video/tracking.hpp:490
	pub fn cv_FarnebackOpticalFlow_getNumIters_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setNumIters(int) /usr/include/opencv2/video/tracking.hpp:491
	pub fn cv_FarnebackOpticalFlow_setNumIters_int(instance: *mut c_void, num_iters: i32, ocvrs_return: *mut Result_void);
	// getPolyN() /usr/include/opencv2/video/tracking.hpp:493
	pub fn cv_FarnebackOpticalFlow_getPolyN_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setPolyN(int) /usr/include/opencv2/video/tracking.hpp:494
	pub fn cv_FarnebackOpticalFlow_setPolyN_int(instance: *mut c_void, poly_n: i32, ocvrs_return: *mut Result_void);
	// getPolySigma() /usr/include/opencv2/video/tracking.hpp:496
	pub fn cv_FarnebackOpticalFlow_getPolySigma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setPolySigma(double) /usr/include/opencv2/video/tracking.hpp:497
	pub fn cv_FarnebackOpticalFlow_setPolySigma_double(instance: *mut c_void, poly_sigma: f64, ocvrs_return: *mut Result_void);
	// getFlags() /usr/include/opencv2/video/tracking.hpp:499
	pub fn cv_FarnebackOpticalFlow_getFlags_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setFlags(int) /usr/include/opencv2/video/tracking.hpp:500
	pub fn cv_FarnebackOpticalFlow_setFlags_int(instance: *mut c_void, flags: i32, ocvrs_return: *mut Result_void);
	// create(int, double, bool, int, int, int, double, int) /usr/include/opencv2/video/tracking.hpp:502
	pub fn cv_FarnebackOpticalFlow_create_int_double_bool_int_int_int_double_int(num_levels: i32, pyr_scale: f64, fast_pyramids: bool, win_size: i32, num_iters: i32, poly_n: i32, poly_sigma: f64, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
	// statePre /usr/include/opencv2/video/tracking.hpp:393
	pub fn cv_KalmanFilter_getPropStatePre_const(instance: *const c_void) -> *mut c_void;
	// statePre /usr/include/opencv2/video/tracking.hpp:393
	pub fn cv_KalmanFilter_setPropStatePre_Mat(instance: *mut c_void, val: *mut c_void);
	// statePost /usr/include/opencv2/video/tracking.hpp:394
	pub fn cv_KalmanFilter_getPropStatePost_const(instance: *const c_void) -> *mut c_void;
	// statePost /usr/include/opencv2/video/tracking.hpp:394
	pub fn cv_KalmanFilter_setPropStatePost_Mat(instance: *mut c_void, val: *mut c_void);
	// transitionMatrix /usr/include/opencv2/video/tracking.hpp:395
	pub fn cv_KalmanFilter_getPropTransitionMatrix_const(instance: *const c_void) -> *mut c_void;
	// transitionMatrix /usr/include/opencv2/video/tracking.hpp:395
	pub fn cv_KalmanFilter_setPropTransitionMatrix_Mat(instance: *mut c_void, val: *mut c_void);
	// controlMatrix /usr/include/opencv2/video/tracking.hpp:396
	pub fn cv_KalmanFilter_getPropControlMatrix_const(instance: *const c_void) -> *mut c_void;
	// controlMatrix /usr/include/opencv2/video/tracking.hpp:396
	pub fn cv_KalmanFilter_setPropControlMatrix_Mat(instance: *mut c_void, val: *mut c_void);
	// measurementMatrix /usr/include/opencv2/video/tracking.hpp:397
	pub fn cv_KalmanFilter_getPropMeasurementMatrix_const(instance: *const c_void) -> *mut c_void;
	// measurementMatrix /usr/include/opencv2/video/tracking.hpp:397
	pub fn cv_KalmanFilter_setPropMeasurementMatrix_Mat(instance: *mut c_void, val: *mut c_void);
	// processNoiseCov /usr/include/opencv2/video/tracking.hpp:398
	pub fn cv_KalmanFilter_getPropProcessNoiseCov_const(instance: *const c_void) -> *mut c_void;
	// processNoiseCov /usr/include/opencv2/video/tracking.hpp:398
	pub fn cv_KalmanFilter_setPropProcessNoiseCov_Mat(instance: *mut c_void, val: *mut c_void);
	// measurementNoiseCov /usr/include/opencv2/video/tracking.hpp:399
	pub fn cv_KalmanFilter_getPropMeasurementNoiseCov_const(instance: *const c_void) -> *mut c_void;
	// measurementNoiseCov /usr/include/opencv2/video/tracking.hpp:399
	pub fn cv_KalmanFilter_setPropMeasurementNoiseCov_Mat(instance: *mut c_void, val: *mut c_void);
	// errorCovPre /usr/include/opencv2/video/tracking.hpp:400
	pub fn cv_KalmanFilter_getPropErrorCovPre_const(instance: *const c_void) -> *mut c_void;
	// errorCovPre /usr/include/opencv2/video/tracking.hpp:400
	pub fn cv_KalmanFilter_setPropErrorCovPre_Mat(instance: *mut c_void, val: *mut c_void);
	// gain /usr/include/opencv2/video/tracking.hpp:401
	pub fn cv_KalmanFilter_getPropGain_const(instance: *const c_void) -> *mut c_void;
	// gain /usr/include/opencv2/video/tracking.hpp:401
	pub fn cv_KalmanFilter_setPropGain_Mat(instance: *mut c_void, val: *mut c_void);
	// errorCovPost /usr/include/opencv2/video/tracking.hpp:402
	pub fn cv_KalmanFilter_getPropErrorCovPost_const(instance: *const c_void) -> *mut c_void;
	// errorCovPost /usr/include/opencv2/video/tracking.hpp:402
	pub fn cv_KalmanFilter_setPropErrorCovPost_Mat(instance: *mut c_void, val: *mut c_void);
	// temp1 /usr/include/opencv2/video/tracking.hpp:405
	pub fn cv_KalmanFilter_getPropTemp1_const(instance: *const c_void) -> *mut c_void;
	// temp1 /usr/include/opencv2/video/tracking.hpp:405
	pub fn cv_KalmanFilter_setPropTemp1_Mat(instance: *mut c_void, val: *mut c_void);
	// temp2 /usr/include/opencv2/video/tracking.hpp:406
	pub fn cv_KalmanFilter_getPropTemp2_const(instance: *const c_void) -> *mut c_void;
	// temp2 /usr/include/opencv2/video/tracking.hpp:406
	pub fn cv_KalmanFilter_setPropTemp2_Mat(instance: *mut c_void, val: *mut c_void);
	// temp3 /usr/include/opencv2/video/tracking.hpp:407
	pub fn cv_KalmanFilter_getPropTemp3_const(instance: *const c_void) -> *mut c_void;
	// temp3 /usr/include/opencv2/video/tracking.hpp:407
	pub fn cv_KalmanFilter_setPropTemp3_Mat(instance: *mut c_void, val: *mut c_void);
	// temp4 /usr/include/opencv2/video/tracking.hpp:408
	pub fn cv_KalmanFilter_getPropTemp4_const(instance: *const c_void) -> *mut c_void;
	// temp4 /usr/include/opencv2/video/tracking.hpp:408
	pub fn cv_KalmanFilter_setPropTemp4_Mat(instance: *mut c_void, val: *mut c_void);
	// temp5 /usr/include/opencv2/video/tracking.hpp:409
	pub fn cv_KalmanFilter_getPropTemp5_const(instance: *const c_void) -> *mut c_void;
	// temp5 /usr/include/opencv2/video/tracking.hpp:409
	pub fn cv_KalmanFilter_setPropTemp5_Mat(instance: *mut c_void, val: *mut c_void);
	// KalmanFilter() /usr/include/opencv2/video/tracking.hpp:363
	pub fn cv_KalmanFilter_KalmanFilter(ocvrs_return: *mut Result<*mut c_void>);
	// KalmanFilter(int, int, int, int) /usr/include/opencv2/video/tracking.hpp:370
	pub fn cv_KalmanFilter_KalmanFilter_int_int_int_int(dynam_params: i32, measure_params: i32, control_params: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// init(int, int, int, int) /usr/include/opencv2/video/tracking.hpp:379
	pub fn cv_KalmanFilter_init_int_int_int_int(instance: *mut c_void, dynam_params: i32, measure_params: i32, control_params: i32, typ: i32, ocvrs_return: *mut Result_void);
	// predict(const cv::Mat &) /usr/include/opencv2/video/tracking.hpp:385
	pub fn cv_KalmanFilter_predict_const_MatR(instance: *mut c_void, control: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// correct(const cv::Mat &) /usr/include/opencv2/video/tracking.hpp:391
	pub fn cv_KalmanFilter_correct_const_MatR(instance: *mut c_void, measurement: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// calc(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/video/tracking.hpp:466
	pub fn cv_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, prev_img: *const c_void, next_img: *const c_void, prev_pts: *const c_void, next_pts: *const c_void, status: *const c_void, err: *const c_void, ocvrs_return: *mut Result_void);
	// getWinSize() /usr/include/opencv2/video/tracking.hpp:685
	pub fn cv_SparsePyrLKOpticalFlow_getWinSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// setWinSize(cv::Size) /usr/include/opencv2/video/tracking.hpp:686
	pub fn cv_SparsePyrLKOpticalFlow_setWinSize_Size(instance: *mut c_void, win_size: *const core::Size, ocvrs_return: *mut Result_void);
	// getMaxLevel() /usr/include/opencv2/video/tracking.hpp:688
	pub fn cv_SparsePyrLKOpticalFlow_getMaxLevel_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMaxLevel(int) /usr/include/opencv2/video/tracking.hpp:689
	pub fn cv_SparsePyrLKOpticalFlow_setMaxLevel_int(instance: *mut c_void, max_level: i32, ocvrs_return: *mut Result_void);
	// getTermCriteria() /usr/include/opencv2/video/tracking.hpp:691
	pub fn cv_SparsePyrLKOpticalFlow_getTermCriteria_const(instance: *const c_void, ocvrs_return: *mut Result<core::TermCriteria>);
	// setTermCriteria(cv::TermCriteria &) /usr/include/opencv2/video/tracking.hpp:692
	pub fn cv_SparsePyrLKOpticalFlow_setTermCriteria_TermCriteriaR(instance: *mut c_void, crit: *mut core::TermCriteria, ocvrs_return: *mut Result_void);
	// getFlags() /usr/include/opencv2/video/tracking.hpp:694
	pub fn cv_SparsePyrLKOpticalFlow_getFlags_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setFlags(int) /usr/include/opencv2/video/tracking.hpp:695
	pub fn cv_SparsePyrLKOpticalFlow_setFlags_int(instance: *mut c_void, flags: i32, ocvrs_return: *mut Result_void);
	// getMinEigThreshold() /usr/include/opencv2/video/tracking.hpp:697
	pub fn cv_SparsePyrLKOpticalFlow_getMinEigThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMinEigThreshold(double) /usr/include/opencv2/video/tracking.hpp:698
	pub fn cv_SparsePyrLKOpticalFlow_setMinEigThreshold_double(instance: *mut c_void, min_eig_threshold: f64, ocvrs_return: *mut Result_void);
	// create(cv::Size, int, cv::TermCriteria, int, double) /usr/include/opencv2/video/tracking.hpp:700
	pub fn cv_SparsePyrLKOpticalFlow_create_Size_int_TermCriteria_int_double(win_size: *const core::Size, max_level: i32, crit: *const core::TermCriteria, flags: i32, min_eig_threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
	// init(cv::InputArray, const cv::Rect &) /usr/include/opencv2/video/tracking.hpp:725
	pub fn cv_Tracker_init_const__InputArrayR_const_RectR(instance: *mut c_void, image: *const c_void, bounding_box: *const core::Rect, ocvrs_return: *mut Result_void);
	// update(cv::InputArray, cv::Rect &) /usr/include/opencv2/video/tracking.hpp:737
	pub fn cv_Tracker_update_const__InputArrayR_RectR(instance: *mut c_void, image: *const c_void, bounding_box: *mut core::Rect, ocvrs_return: *mut Result<bool>);
	// create(const TrackerDaSiamRPN::Params &) /usr/include/opencv2/video/tracking.hpp:842
	pub fn cv_TrackerDaSiamRPN_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getTrackingScore() /usr/include/opencv2/video/tracking.hpp:846
	pub fn cv_TrackerDaSiamRPN_getTrackingScore(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
	// model /usr/include/opencv2/video/tracking.hpp:831
	pub fn cv_TrackerDaSiamRPN_Params_getPropModel_const(instance: *const c_void) -> *mut c_void;
	// model /usr/include/opencv2/video/tracking.hpp:831
	pub fn cv_TrackerDaSiamRPN_Params_setPropModel_string(instance: *mut c_void, val: *mut c_char);
	// kernel_cls1 /usr/include/opencv2/video/tracking.hpp:832
	pub fn cv_TrackerDaSiamRPN_Params_getPropKernel_cls1_const(instance: *const c_void) -> *mut c_void;
	// kernel_cls1 /usr/include/opencv2/video/tracking.hpp:832
	pub fn cv_TrackerDaSiamRPN_Params_setPropKernel_cls1_string(instance: *mut c_void, val: *mut c_char);
	// kernel_r1 /usr/include/opencv2/video/tracking.hpp:833
	pub fn cv_TrackerDaSiamRPN_Params_getPropKernel_r1_const(instance: *const c_void) -> *mut c_void;
	// kernel_r1 /usr/include/opencv2/video/tracking.hpp:833
	pub fn cv_TrackerDaSiamRPN_Params_setPropKernel_r1_string(instance: *mut c_void, val: *mut c_char);
	// backend /usr/include/opencv2/video/tracking.hpp:834
	pub fn cv_TrackerDaSiamRPN_Params_getPropBackend_const(instance: *const c_void) -> i32;
	// backend /usr/include/opencv2/video/tracking.hpp:834
	pub fn cv_TrackerDaSiamRPN_Params_setPropBackend_int(instance: *mut c_void, val: i32);
	// target /usr/include/opencv2/video/tracking.hpp:835
	pub fn cv_TrackerDaSiamRPN_Params_getPropTarget_const(instance: *const c_void) -> i32;
	// target /usr/include/opencv2/video/tracking.hpp:835
	pub fn cv_TrackerDaSiamRPN_Params_setPropTarget_int(instance: *mut c_void, val: i32);
	// Params() /usr/include/opencv2/video/tracking.hpp:830
	pub fn cv_TrackerDaSiamRPN_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
	// create(const TrackerGOTURN::Params &) /usr/include/opencv2/video/tracking.hpp:815
	pub fn cv_TrackerGOTURN_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// modelTxt /usr/include/opencv2/video/tracking.hpp:807
	pub fn cv_TrackerGOTURN_Params_getPropModelTxt_const(instance: *const c_void) -> *mut c_void;
	// modelTxt /usr/include/opencv2/video/tracking.hpp:807
	pub fn cv_TrackerGOTURN_Params_setPropModelTxt_string(instance: *mut c_void, val: *mut c_char);
	// modelBin /usr/include/opencv2/video/tracking.hpp:808
	pub fn cv_TrackerGOTURN_Params_getPropModelBin_const(instance: *const c_void) -> *mut c_void;
	// modelBin /usr/include/opencv2/video/tracking.hpp:808
	pub fn cv_TrackerGOTURN_Params_setPropModelBin_string(instance: *mut c_void, val: *mut c_char);
	// Params() /usr/include/opencv2/video/tracking.hpp:806
	pub fn cv_TrackerGOTURN_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
	// create(const TrackerMIL::Params &) /usr/include/opencv2/video/tracking.hpp:774
	pub fn cv_TrackerMIL_create_const_ParamsR(parameters: *const crate::video::TrackerMIL_Params, ocvrs_return: *mut Result<*mut c_void>);
	// Params() /usr/include/opencv2/video/tracking.hpp:759
	pub fn cv_TrackerMIL_Params_Params(ocvrs_return: *mut Result<crate::video::TrackerMIL_Params>);
	// calcUV(cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray) /usr/include/opencv2/video/tracking.hpp:528
	pub fn cv_VariationalRefinement_calcUV_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(instance: *mut c_void, i0: *const c_void, i1: *const c_void, flow_u: *const c_void, flow_v: *const c_void, ocvrs_return: *mut Result_void);
	// getFixedPointIterations() /usr/include/opencv2/video/tracking.hpp:532
	pub fn cv_VariationalRefinement_getFixedPointIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setFixedPointIterations(int) /usr/include/opencv2/video/tracking.hpp:534
	pub fn cv_VariationalRefinement_setFixedPointIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getSorIterations() /usr/include/opencv2/video/tracking.hpp:539
	pub fn cv_VariationalRefinement_getSorIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setSorIterations(int) /usr/include/opencv2/video/tracking.hpp:541
	pub fn cv_VariationalRefinement_setSorIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getOmega() /usr/include/opencv2/video/tracking.hpp:545
	pub fn cv_VariationalRefinement_getOmega_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setOmega(float) /usr/include/opencv2/video/tracking.hpp:547
	pub fn cv_VariationalRefinement_setOmega_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getAlpha() /usr/include/opencv2/video/tracking.hpp:551
	pub fn cv_VariationalRefinement_getAlpha_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setAlpha(float) /usr/include/opencv2/video/tracking.hpp:553
	pub fn cv_VariationalRefinement_setAlpha_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getDelta() /usr/include/opencv2/video/tracking.hpp:557
	pub fn cv_VariationalRefinement_getDelta_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setDelta(float) /usr/include/opencv2/video/tracking.hpp:559
	pub fn cv_VariationalRefinement_setDelta_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getGamma() /usr/include/opencv2/video/tracking.hpp:563
	pub fn cv_VariationalRefinement_getGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setGamma(float) /usr/include/opencv2/video/tracking.hpp:565
	pub fn cv_VariationalRefinement_setGamma_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// create() /usr/include/opencv2/video/tracking.hpp:569
	pub fn cv_VariationalRefinement_create(ocvrs_return: *mut Result<*mut c_void>);
}
