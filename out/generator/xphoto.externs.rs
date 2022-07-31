extern "C" {
	// applyChannelGains(cv::InputArray, cv::OutputArray, float, float, float) /usr/include/opencv2/xphoto/white_balance.hpp:225
	pub fn cv_xphoto_applyChannelGains_const__InputArrayR_const__OutputArrayR_float_float_float(src: *const c_void, dst: *const c_void, gain_b: f32, gain_g: f32, gain_r: f32, ocvrs_return: *mut Result_void);
	// bm3dDenoising(cv::InputArray, cv::InputOutputArray, cv::OutputArray, float, int, int, int, int, int, int, float, int, int, int) /usr/include/opencv2/xphoto/bm3d_image_denoising.hpp:115
	pub fn cv_xphoto_bm3dDenoising_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_float_int_int_int_int_int_int_float_int_int_int(src: *const c_void, dst_step1: *const c_void, dst_step2: *const c_void, h: f32, template_window_size: i32, search_window_size: i32, block_matching_step1: i32, block_matching_step2: i32, group_size: i32, sliding_step: i32, beta: f32, norm_type: i32, step: i32, transform_type: i32, ocvrs_return: *mut Result_void);
	// bm3dDenoising(cv::InputArray, cv::OutputArray, float, int, int, int, int, int, int, float, int, int, int) /usr/include/opencv2/xphoto/bm3d_image_denoising.hpp:168
	pub fn cv_xphoto_bm3dDenoising_const__InputArrayR_const__OutputArrayR_float_int_int_int_int_int_int_float_int_int_int(src: *const c_void, dst: *const c_void, h: f32, template_window_size: i32, search_window_size: i32, block_matching_step1: i32, block_matching_step2: i32, group_size: i32, sliding_step: i32, beta: f32, norm_type: i32, step: i32, transform_type: i32, ocvrs_return: *mut Result_void);
	// createGrayworldWB() /usr/include/opencv2/xphoto/white_balance.hpp:152
	pub fn cv_xphoto_createGrayworldWB(ocvrs_return: *mut Result<*mut c_void>);
	// createLearningBasedWB(const cv::String &) /usr/include/opencv2/xphoto/white_balance.hpp:214
	pub fn cv_xphoto_createLearningBasedWB_const_StringR(path_to_model: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// createSimpleWB() /usr/include/opencv2/xphoto/white_balance.hpp:115
	pub fn cv_xphoto_createSimpleWB(ocvrs_return: *mut Result<*mut c_void>);
	// createTonemapDurand(float, float, float, float, float) /usr/include/opencv2/xphoto/tonemap.hpp:53
	pub fn cv_xphoto_createTonemapDurand_float_float_float_float_float(gamma: f32, contrast: f32, saturation: f32, sigma_color: f32, sigma_space: f32, ocvrs_return: *mut Result<*mut c_void>);
	// dctDenoising(const cv::Mat &, cv::Mat &, const double, const int) /usr/include/opencv2/xphoto/dct_image_denoising.hpp:72
	pub fn cv_xphoto_dctDenoising_const_MatR_MatR_const_double_const_int(src: *const c_void, dst: *mut c_void, sigma: f64, psize: i32, ocvrs_return: *mut Result_void);
	// inpaint(const cv::Mat &, const cv::Mat &, cv::Mat &, const int) /usr/include/opencv2/xphoto/inpainting.hpp:113
	pub fn cv_xphoto_inpaint_const_MatR_const_MatR_MatR_const_int(src: *const c_void, mask: *const c_void, dst: *mut c_void, algorithm_type: i32, ocvrs_return: *mut Result_void);
	// oilPainting(cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/xphoto/oilpainting.hpp:36
	pub fn cv_xphoto_oilPainting_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, size: i32, dyn_ratio: i32, ocvrs_return: *mut Result_void);
	// oilPainting(cv::InputArray, cv::OutputArray, int, int, int) /usr/include/opencv2/xphoto/oilpainting.hpp:28
	pub fn cv_xphoto_oilPainting_const__InputArrayR_const__OutputArrayR_int_int_int(src: *const c_void, dst: *const c_void, size: i32, dyn_ratio: i32, code: i32, ocvrs_return: *mut Result_void);
	// getSaturationThreshold() /usr/include/opencv2/xphoto/white_balance.hpp:145
	pub fn cv_xphoto_GrayworldWB_getSaturationThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setSaturationThreshold(float) /usr/include/opencv2/xphoto/white_balance.hpp:147
	pub fn cv_xphoto_GrayworldWB_setSaturationThreshold_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// extractSimpleFeatures(cv::InputArray, cv::OutputArray) /usr/include/opencv2/xphoto/white_balance.hpp:185
	pub fn cv_xphoto_LearningBasedWB_extractSimpleFeatures_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// getRangeMaxVal() /usr/include/opencv2/xphoto/white_balance.hpp:190
	pub fn cv_xphoto_LearningBasedWB_getRangeMaxVal_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setRangeMaxVal(int) /usr/include/opencv2/xphoto/white_balance.hpp:192
	pub fn cv_xphoto_LearningBasedWB_setRangeMaxVal_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getSaturationThreshold() /usr/include/opencv2/xphoto/white_balance.hpp:197
	pub fn cv_xphoto_LearningBasedWB_getSaturationThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setSaturationThreshold(float) /usr/include/opencv2/xphoto/white_balance.hpp:199
	pub fn cv_xphoto_LearningBasedWB_setSaturationThreshold_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getHistBinNum() /usr/include/opencv2/xphoto/white_balance.hpp:205
	pub fn cv_xphoto_LearningBasedWB_getHistBinNum_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setHistBinNum(int) /usr/include/opencv2/xphoto/white_balance.hpp:207
	pub fn cv_xphoto_LearningBasedWB_setHistBinNum_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getInputMin() /usr/include/opencv2/xphoto/white_balance.hpp:84
	pub fn cv_xphoto_SimpleWB_getInputMin_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setInputMin(float) /usr/include/opencv2/xphoto/white_balance.hpp:86
	pub fn cv_xphoto_SimpleWB_setInputMin_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getInputMax() /usr/include/opencv2/xphoto/white_balance.hpp:90
	pub fn cv_xphoto_SimpleWB_getInputMax_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setInputMax(float) /usr/include/opencv2/xphoto/white_balance.hpp:92
	pub fn cv_xphoto_SimpleWB_setInputMax_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getOutputMin() /usr/include/opencv2/xphoto/white_balance.hpp:96
	pub fn cv_xphoto_SimpleWB_getOutputMin_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setOutputMin(float) /usr/include/opencv2/xphoto/white_balance.hpp:98
	pub fn cv_xphoto_SimpleWB_setOutputMin_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getOutputMax() /usr/include/opencv2/xphoto/white_balance.hpp:102
	pub fn cv_xphoto_SimpleWB_getOutputMax_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setOutputMax(float) /usr/include/opencv2/xphoto/white_balance.hpp:104
	pub fn cv_xphoto_SimpleWB_setOutputMax_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getP() /usr/include/opencv2/xphoto/white_balance.hpp:108
	pub fn cv_xphoto_SimpleWB_getP_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setP(float) /usr/include/opencv2/xphoto/white_balance.hpp:110
	pub fn cv_xphoto_SimpleWB_setP_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getSaturation() /usr/include/opencv2/xphoto/tonemap.hpp:28
	pub fn cv_xphoto_TonemapDurand_getSaturation_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setSaturation(float) /usr/include/opencv2/xphoto/tonemap.hpp:29
	pub fn cv_xphoto_TonemapDurand_setSaturation_float(instance: *mut c_void, saturation: f32, ocvrs_return: *mut Result_void);
	// getContrast() /usr/include/opencv2/xphoto/tonemap.hpp:31
	pub fn cv_xphoto_TonemapDurand_getContrast_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setContrast(float) /usr/include/opencv2/xphoto/tonemap.hpp:32
	pub fn cv_xphoto_TonemapDurand_setContrast_float(instance: *mut c_void, contrast: f32, ocvrs_return: *mut Result_void);
	// getSigmaSpace() /usr/include/opencv2/xphoto/tonemap.hpp:34
	pub fn cv_xphoto_TonemapDurand_getSigmaSpace_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setSigmaSpace(float) /usr/include/opencv2/xphoto/tonemap.hpp:35
	pub fn cv_xphoto_TonemapDurand_setSigmaSpace_float(instance: *mut c_void, sigma_space: f32, ocvrs_return: *mut Result_void);
	// getSigmaColor() /usr/include/opencv2/xphoto/tonemap.hpp:37
	pub fn cv_xphoto_TonemapDurand_getSigmaColor_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setSigmaColor(float) /usr/include/opencv2/xphoto/tonemap.hpp:38
	pub fn cv_xphoto_TonemapDurand_setSigmaColor_float(instance: *mut c_void, sigma_color: f32, ocvrs_return: *mut Result_void);
	// balanceWhite(cv::InputArray, cv::OutputArray) /usr/include/opencv2/xphoto/white_balance.hpp:72
	pub fn cv_xphoto_WhiteBalancer_balanceWhite_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
}
