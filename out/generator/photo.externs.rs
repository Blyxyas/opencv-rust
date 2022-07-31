extern "C" {
	// colorChange(cv::InputArray, cv::InputArray, cv::OutputArray, float, float, float) /usr/include/opencv2/photo.hpp:755
	pub fn cv_colorChange_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_float_float(src: *const c_void, mask: *const c_void, dst: *const c_void, red_mul: f32, green_mul: f32, blue_mul: f32, ocvrs_return: *mut Result_void);
	// createAlignMTB(int, int, bool) /usr/include/opencv2/photo.hpp:527
	pub fn cv_createAlignMTB_int_int_bool(max_bits: i32, exclude_range: i32, cut: bool, ocvrs_return: *mut Result<*mut c_void>);
	// createCalibrateDebevec(int, float, bool) /usr/include/opencv2/photo.hpp:570
	pub fn cv_createCalibrateDebevec_int_float_bool(samples: i32, lambda: f32, random: bool, ocvrs_return: *mut Result<*mut c_void>);
	// createCalibrateRobertson(int, float) /usr/include/opencv2/photo.hpp:594
	pub fn cv_createCalibrateRobertson_int_float(max_iter: i32, threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
	// createMergeDebevec() /usr/include/opencv2/photo.hpp:628
	pub fn cv_createMergeDebevec(ocvrs_return: *mut Result<*mut c_void>);
	// createMergeMertens(float, float, float) /usr/include/opencv2/photo.hpp:670
	pub fn cv_createMergeMertens_float_float_float(contrast_weight: f32, saturation_weight: f32, exposure_weight: f32, ocvrs_return: *mut Result<*mut c_void>);
	// createMergeRobertson() /usr/include/opencv2/photo.hpp:687
	pub fn cv_createMergeRobertson(ocvrs_return: *mut Result<*mut c_void>);
	// createTonemapDrago(float, float, float) /usr/include/opencv2/photo.hpp:387
	pub fn cv_createTonemapDrago_float_float_float(gamma: f32, saturation: f32, bias: f32, ocvrs_return: *mut Result<*mut c_void>);
	// createTonemapMantiuk(float, float, float) /usr/include/opencv2/photo.hpp:446
	pub fn cv_createTonemapMantiuk_float_float_float(gamma: f32, scale: f32, saturation: f32, ocvrs_return: *mut Result<*mut c_void>);
	// createTonemapReinhard(float, float, float, float) /usr/include/opencv2/photo.hpp:420
	pub fn cv_createTonemapReinhard_float_float_float_float(gamma: f32, intensity: f32, light_adapt: f32, color_adapt: f32, ocvrs_return: *mut Result<*mut c_void>);
	// createTonemap(float) /usr/include/opencv2/photo.hpp:356
	pub fn cv_createTonemap_float(gamma: f32, ocvrs_return: *mut Result<*mut c_void>);
	// fastNlMeansDenoisingColored(cv::InputArray, cv::OutputArray, float, float, int, int, cv::cuda::Stream &) /usr/include/opencv2/photo/cuda.hpp:122
	pub fn cv_cuda_fastNlMeansDenoisingColored_const__InputArrayR_const__OutputArrayR_float_float_int_int_StreamR(src: *const c_void, dst: *const c_void, h_luminance: f32, photo_render: f32, search_window: i32, block_size: i32, stream: *mut c_void, ocvrs_return: *mut Result_void);
	// fastNlMeansDenoising(cv::InputArray, cv::OutputArray, float, int, int, cv::cuda::Stream &) /usr/include/opencv2/photo/cuda.hpp:95
	pub fn cv_cuda_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_float_int_int_StreamR(src: *const c_void, dst: *const c_void, h: f32, search_window: i32, block_size: i32, stream: *mut c_void, ocvrs_return: *mut Result_void);
	// nonLocalMeans(cv::InputArray, cv::OutputArray, float, int, int, int, cv::cuda::Stream &) /usr/include/opencv2/photo/cuda.hpp:67
	pub fn cv_cuda_nonLocalMeans_const__InputArrayR_const__OutputArrayR_float_int_int_int_StreamR(src: *const c_void, dst: *const c_void, h: f32, search_window: i32, block_size: i32, border_mode: i32, stream: *mut c_void, ocvrs_return: *mut Result_void);
	// decolor(cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/photo.hpp:704
	pub fn cv_decolor_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, grayscale: *const c_void, color_boost: *const c_void, ocvrs_return: *mut Result_void);
	// denoise_TVL1(const std::vector<Mat> &, cv::Mat &, double, int) /usr/include/opencv2/photo.hpp:325
	pub fn cv_denoise_TVL1_const_vector_Mat_R_MatR_double_int(observations: *const c_void, result: *mut c_void, lambda: f64, niters: i32, ocvrs_return: *mut Result_void);
	// detailEnhance(cv::InputArray, cv::OutputArray, float, float) /usr/include/opencv2/photo.hpp:822
	pub fn cv_detailEnhance_const__InputArrayR_const__OutputArrayR_float_float(src: *const c_void, dst: *const c_void, sigma_s: f32, sigma_r: f32, ocvrs_return: *mut Result_void);
	// edgePreservingFilter(cv::InputArray, cv::OutputArray, int, float, float) /usr/include/opencv2/photo.hpp:812
	pub fn cv_edgePreservingFilter_const__InputArrayR_const__OutputArrayR_int_float_float(src: *const c_void, dst: *const c_void, flags: i32, sigma_s: f32, sigma_r: f32, ocvrs_return: *mut Result_void);
	// fastNlMeansDenoisingColoredMulti(cv::InputArrayOfArrays, cv::OutputArray, int, int, float, float, int, int) /usr/include/opencv2/photo.hpp:283
	pub fn cv_fastNlMeansDenoisingColoredMulti_const__InputArrayR_const__OutputArrayR_int_int_float_float_int_int(src_imgs: *const c_void, dst: *const c_void, img_to_denoise_index: i32, temporal_window_size: i32, h: f32, h_color: f32, template_window_size: i32, search_window_size: i32, ocvrs_return: *mut Result_void);
	// fastNlMeansDenoisingColored(cv::InputArray, cv::OutputArray, float, float, int, int) /usr/include/opencv2/photo.hpp:198
	pub fn cv_fastNlMeansDenoisingColored_const__InputArrayR_const__OutputArrayR_float_float_int_int(src: *const c_void, dst: *const c_void, h: f32, h_color: f32, template_window_size: i32, search_window_size: i32, ocvrs_return: *mut Result_void);
	// fastNlMeansDenoisingMulti(cv::InputArrayOfArrays, cv::OutputArray, int, int, const std::vector<float> &, int, int, int) /usr/include/opencv2/photo.hpp:254
	pub fn cv_fastNlMeansDenoisingMulti_const__InputArrayR_const__OutputArrayR_int_int_const_vector_float_R_int_int_int(src_imgs: *const c_void, dst: *const c_void, img_to_denoise_index: i32, temporal_window_size: i32, h: *const c_void, template_window_size: i32, search_window_size: i32, norm_type: i32, ocvrs_return: *mut Result_void);
	// fastNlMeansDenoisingMulti(cv::InputArrayOfArrays, cv::OutputArray, int, int, float, int, int) /usr/include/opencv2/photo.hpp:225
	pub fn cv_fastNlMeansDenoisingMulti_const__InputArrayR_const__OutputArrayR_int_int_float_int_int(src_imgs: *const c_void, dst: *const c_void, img_to_denoise_index: i32, temporal_window_size: i32, h: f32, template_window_size: i32, search_window_size: i32, ocvrs_return: *mut Result_void);
	// fastNlMeansDenoising(cv::InputArray, cv::OutputArray, const std::vector<float> &, int, int, int) /usr/include/opencv2/photo.hpp:175
	pub fn cv_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_const_vector_float_R_int_int_int(src: *const c_void, dst: *const c_void, h: *const c_void, template_window_size: i32, search_window_size: i32, norm_type: i32, ocvrs_return: *mut Result_void);
	// fastNlMeansDenoising(cv::InputArray, cv::OutputArray, float, int, int) /usr/include/opencv2/photo.hpp:148
	pub fn cv_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_float_int_int(src: *const c_void, dst: *const c_void, h: f32, template_window_size: i32, search_window_size: i32, ocvrs_return: *mut Result_void);
	// illuminationChange(cv::InputArray, cv::InputArray, cv::OutputArray, float, float) /usr/include/opencv2/photo.hpp:769
	pub fn cv_illuminationChange_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_float(src: *const c_void, mask: *const c_void, dst: *const c_void, alpha: f32, beta: f32, ocvrs_return: *mut Result_void);
	// inpaint(cv::InputArray, cv::InputArray, cv::OutputArray, double, int) /usr/include/opencv2/photo.hpp:120
	pub fn cv_inpaint_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(src: *const c_void, inpaint_mask: *const c_void, dst: *const c_void, inpaint_radius: f64, flags: i32, ocvrs_return: *mut Result_void);
	// pencilSketch(cv::InputArray, cv::OutputArray, cv::OutputArray, float, float, float) /usr/include/opencv2/photo.hpp:837
	pub fn cv_pencilSketch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_float_float_float(src: *const c_void, dst1: *const c_void, dst2: *const c_void, sigma_s: f32, sigma_r: f32, shade_factor: f32, ocvrs_return: *mut Result_void);
	// seamlessClone(cv::InputArray, cv::InputArray, cv::InputArray, cv::Point, cv::OutputArray, int) /usr/include/opencv2/photo.hpp:740
	pub fn cv_seamlessClone_const__InputArrayR_const__InputArrayR_const__InputArrayR_Point_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, mask: *const c_void, p: *const core::Point, blend: *const c_void, flags: i32, ocvrs_return: *mut Result_void);
	// stylization(cv::InputArray, cv::OutputArray, float, float) /usr/include/opencv2/photo.hpp:849
	pub fn cv_stylization_const__InputArrayR_const__OutputArrayR_float_float(src: *const c_void, dst: *const c_void, sigma_s: f32, sigma_r: f32, ocvrs_return: *mut Result_void);
	// textureFlattening(cv::InputArray, cv::InputArray, cv::OutputArray, float, float, int) /usr/include/opencv2/photo.hpp:787
	pub fn cv_textureFlattening_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_float_int(src: *const c_void, mask: *const c_void, dst: *const c_void, low_threshold: f32, high_threshold: f32, kernel_size: i32, ocvrs_return: *mut Result_void);
	// process(cv::InputArrayOfArrays, std::vector<Mat> &, cv::InputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:461
	pub fn cv_AlignExposures_process_const__InputArrayR_vector_Mat_R_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *mut c_void, times: *const c_void, response: *const c_void, ocvrs_return: *mut Result_void);
	// process(cv::InputArrayOfArrays, std::vector<Mat> &, cv::InputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:477
	pub fn cv_AlignMTB_process_const__InputArrayR_vector_Mat_R_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *mut c_void, times: *const c_void, response: *const c_void, ocvrs_return: *mut Result_void);
	// process(cv::InputArrayOfArrays, std::vector<Mat> &) /usr/include/opencv2/photo.hpp:485
	pub fn cv_AlignMTB_process_const__InputArrayR_vector_Mat_R(instance: *mut c_void, src: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result_void);
	// calculateShift(cv::InputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:493
	pub fn cv_AlignMTB_calculateShift_const__InputArrayR_const__InputArrayR(instance: *mut c_void, img0: *const c_void, img1: *const c_void, ocvrs_return: *mut Result<core::Point>);
	// shiftMat(cv::InputArray, cv::OutputArray, const cv::Point) /usr/include/opencv2/photo.hpp:500
	pub fn cv_AlignMTB_shiftMat_const__InputArrayR_const__OutputArrayR_const_Point(instance: *mut c_void, src: *const c_void, dst: *const c_void, shift: *const core::Point, ocvrs_return: *mut Result_void);
	// computeBitmaps(cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/photo.hpp:507
	pub fn cv_AlignMTB_computeBitmaps_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, img: *const c_void, tb: *const c_void, eb: *const c_void, ocvrs_return: *mut Result_void);
	// getMaxBits() /usr/include/opencv2/photo.hpp:509
	pub fn cv_AlignMTB_getMaxBits_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMaxBits(int) /usr/include/opencv2/photo.hpp:510
	pub fn cv_AlignMTB_setMaxBits_int(instance: *mut c_void, max_bits: i32, ocvrs_return: *mut Result_void);
	// getExcludeRange() /usr/include/opencv2/photo.hpp:512
	pub fn cv_AlignMTB_getExcludeRange_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setExcludeRange(int) /usr/include/opencv2/photo.hpp:513
	pub fn cv_AlignMTB_setExcludeRange_int(instance: *mut c_void, exclude_range: i32, ocvrs_return: *mut Result_void);
	// getCut() /usr/include/opencv2/photo.hpp:515
	pub fn cv_AlignMTB_getCut_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setCut(bool) /usr/include/opencv2/photo.hpp:516
	pub fn cv_AlignMTB_setCut_bool(instance: *mut c_void, value: bool, ocvrs_return: *mut Result_void);
	// process(cv::InputArrayOfArrays, cv::OutputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:540
	pub fn cv_CalibrateCRF_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, times: *const c_void, ocvrs_return: *mut Result_void);
	// getLambda() /usr/include/opencv2/photo.hpp:552
	pub fn cv_CalibrateDebevec_getLambda_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setLambda(float) /usr/include/opencv2/photo.hpp:553
	pub fn cv_CalibrateDebevec_setLambda_float(instance: *mut c_void, lambda: f32, ocvrs_return: *mut Result_void);
	// getSamples() /usr/include/opencv2/photo.hpp:555
	pub fn cv_CalibrateDebevec_getSamples_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setSamples(int) /usr/include/opencv2/photo.hpp:556
	pub fn cv_CalibrateDebevec_setSamples_int(instance: *mut c_void, samples: i32, ocvrs_return: *mut Result_void);
	// getRandom() /usr/include/opencv2/photo.hpp:558
	pub fn cv_CalibrateDebevec_getRandom_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setRandom(bool) /usr/include/opencv2/photo.hpp:559
	pub fn cv_CalibrateDebevec_setRandom_bool(instance: *mut c_void, random: bool, ocvrs_return: *mut Result_void);
	// getMaxIter() /usr/include/opencv2/photo.hpp:580
	pub fn cv_CalibrateRobertson_getMaxIter_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMaxIter(int) /usr/include/opencv2/photo.hpp:581
	pub fn cv_CalibrateRobertson_setMaxIter_int(instance: *mut c_void, max_iter: i32, ocvrs_return: *mut Result_void);
	// getThreshold() /usr/include/opencv2/photo.hpp:583
	pub fn cv_CalibrateRobertson_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setThreshold(float) /usr/include/opencv2/photo.hpp:584
	pub fn cv_CalibrateRobertson_setThreshold_float(instance: *mut c_void, threshold: f32, ocvrs_return: *mut Result_void);
	// getRadiance() /usr/include/opencv2/photo.hpp:586
	pub fn cv_CalibrateRobertson_getRadiance_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// process(cv::InputArrayOfArrays, cv::OutputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:621
	pub fn cv_MergeDebevec_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, times: *const c_void, response: *const c_void, ocvrs_return: *mut Result_void);
	// process(cv::InputArrayOfArrays, cv::OutputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:623
	pub fn cv_MergeDebevec_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, times: *const c_void, ocvrs_return: *mut Result_void);
	// process(cv::InputArrayOfArrays, cv::OutputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:609
	pub fn cv_MergeExposures_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, times: *const c_void, response: *const c_void, ocvrs_return: *mut Result_void);
	// process(cv::InputArrayOfArrays, cv::OutputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:644
	pub fn cv_MergeMertens_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, times: *const c_void, response: *const c_void, ocvrs_return: *mut Result_void);
	// process(cv::InputArrayOfArrays, cv::OutputArray) /usr/include/opencv2/photo.hpp:651
	pub fn cv_MergeMertens_process_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// getContrastWeight() /usr/include/opencv2/photo.hpp:653
	pub fn cv_MergeMertens_getContrastWeight_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setContrastWeight(float) /usr/include/opencv2/photo.hpp:654
	pub fn cv_MergeMertens_setContrastWeight_float(instance: *mut c_void, contrast_weiht: f32, ocvrs_return: *mut Result_void);
	// getSaturationWeight() /usr/include/opencv2/photo.hpp:656
	pub fn cv_MergeMertens_getSaturationWeight_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setSaturationWeight(float) /usr/include/opencv2/photo.hpp:657
	pub fn cv_MergeMertens_setSaturationWeight_float(instance: *mut c_void, saturation_weight: f32, ocvrs_return: *mut Result_void);
	// getExposureWeight() /usr/include/opencv2/photo.hpp:659
	pub fn cv_MergeMertens_getExposureWeight_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setExposureWeight(float) /usr/include/opencv2/photo.hpp:660
	pub fn cv_MergeMertens_setExposureWeight_float(instance: *mut c_void, exposure_weight: f32, ocvrs_return: *mut Result_void);
	// process(cv::InputArrayOfArrays, cv::OutputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:680
	pub fn cv_MergeRobertson_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, times: *const c_void, response: *const c_void, ocvrs_return: *mut Result_void);
	// process(cv::InputArrayOfArrays, cv::OutputArray, cv::InputArray) /usr/include/opencv2/photo.hpp:682
	pub fn cv_MergeRobertson_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, times: *const c_void, ocvrs_return: *mut Result_void);
	// process(cv::InputArray, cv::OutputArray) /usr/include/opencv2/photo.hpp:344
	pub fn cv_Tonemap_process_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// getGamma() /usr/include/opencv2/photo.hpp:346
	pub fn cv_Tonemap_getGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setGamma(float) /usr/include/opencv2/photo.hpp:347
	pub fn cv_Tonemap_setGamma_float(instance: *mut c_void, gamma: f32, ocvrs_return: *mut Result_void);
	// getSaturation() /usr/include/opencv2/photo.hpp:372
	pub fn cv_TonemapDrago_getSaturation_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setSaturation(float) /usr/include/opencv2/photo.hpp:373
	pub fn cv_TonemapDrago_setSaturation_float(instance: *mut c_void, saturation: f32, ocvrs_return: *mut Result_void);
	// getBias() /usr/include/opencv2/photo.hpp:375
	pub fn cv_TonemapDrago_getBias_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setBias(float) /usr/include/opencv2/photo.hpp:376
	pub fn cv_TonemapDrago_setBias_float(instance: *mut c_void, bias: f32, ocvrs_return: *mut Result_void);
	// getScale() /usr/include/opencv2/photo.hpp:431
	pub fn cv_TonemapMantiuk_getScale_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setScale(float) /usr/include/opencv2/photo.hpp:432
	pub fn cv_TonemapMantiuk_setScale_float(instance: *mut c_void, scale: f32, ocvrs_return: *mut Result_void);
	// getSaturation() /usr/include/opencv2/photo.hpp:434
	pub fn cv_TonemapMantiuk_getSaturation_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setSaturation(float) /usr/include/opencv2/photo.hpp:435
	pub fn cv_TonemapMantiuk_setSaturation_float(instance: *mut c_void, saturation: f32, ocvrs_return: *mut Result_void);
	// getIntensity() /usr/include/opencv2/photo.hpp:400
	pub fn cv_TonemapReinhard_getIntensity_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setIntensity(float) /usr/include/opencv2/photo.hpp:401
	pub fn cv_TonemapReinhard_setIntensity_float(instance: *mut c_void, intensity: f32, ocvrs_return: *mut Result_void);
	// getLightAdaptation() /usr/include/opencv2/photo.hpp:403
	pub fn cv_TonemapReinhard_getLightAdaptation_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setLightAdaptation(float) /usr/include/opencv2/photo.hpp:404
	pub fn cv_TonemapReinhard_setLightAdaptation_float(instance: *mut c_void, light_adapt: f32, ocvrs_return: *mut Result_void);
	// getColorAdaptation() /usr/include/opencv2/photo.hpp:406
	pub fn cv_TonemapReinhard_getColorAdaptation_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setColorAdaptation(float) /usr/include/opencv2/photo.hpp:407
	pub fn cv_TonemapReinhard_setColorAdaptation_float(instance: *mut c_void, color_adapt: f32, ocvrs_return: *mut Result_void);
}
