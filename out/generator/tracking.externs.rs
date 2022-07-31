extern "C" {
	// create(const TrackerCSRT::Params &) /usr/include/opencv2/tracking.hpp:81
	pub fn cv_tracking_TrackerCSRT_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setInitialMask(cv::InputArray) /usr/include/opencv2/tracking.hpp:86
	pub fn cv_tracking_TrackerCSRT_setInitialMask_const__InputArrayR(instance: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// use_hog /usr/include/opencv2/tracking.hpp:45
	pub fn cv_tracking_TrackerCSRT_Params_getPropUse_hog_const(instance: *const c_void) -> bool;
	// use_hog /usr/include/opencv2/tracking.hpp:45
	pub fn cv_tracking_TrackerCSRT_Params_setPropUse_hog_bool(instance: *mut c_void, val: bool);
	// use_color_names /usr/include/opencv2/tracking.hpp:46
	pub fn cv_tracking_TrackerCSRT_Params_getPropUse_color_names_const(instance: *const c_void) -> bool;
	// use_color_names /usr/include/opencv2/tracking.hpp:46
	pub fn cv_tracking_TrackerCSRT_Params_setPropUse_color_names_bool(instance: *mut c_void, val: bool);
	// use_gray /usr/include/opencv2/tracking.hpp:47
	pub fn cv_tracking_TrackerCSRT_Params_getPropUse_gray_const(instance: *const c_void) -> bool;
	// use_gray /usr/include/opencv2/tracking.hpp:47
	pub fn cv_tracking_TrackerCSRT_Params_setPropUse_gray_bool(instance: *mut c_void, val: bool);
	// use_rgb /usr/include/opencv2/tracking.hpp:48
	pub fn cv_tracking_TrackerCSRT_Params_getPropUse_rgb_const(instance: *const c_void) -> bool;
	// use_rgb /usr/include/opencv2/tracking.hpp:48
	pub fn cv_tracking_TrackerCSRT_Params_setPropUse_rgb_bool(instance: *mut c_void, val: bool);
	// use_channel_weights /usr/include/opencv2/tracking.hpp:49
	pub fn cv_tracking_TrackerCSRT_Params_getPropUse_channel_weights_const(instance: *const c_void) -> bool;
	// use_channel_weights /usr/include/opencv2/tracking.hpp:49
	pub fn cv_tracking_TrackerCSRT_Params_setPropUse_channel_weights_bool(instance: *mut c_void, val: bool);
	// use_segmentation /usr/include/opencv2/tracking.hpp:50
	pub fn cv_tracking_TrackerCSRT_Params_getPropUse_segmentation_const(instance: *const c_void) -> bool;
	// use_segmentation /usr/include/opencv2/tracking.hpp:50
	pub fn cv_tracking_TrackerCSRT_Params_setPropUse_segmentation_bool(instance: *mut c_void, val: bool);
	// window_function /usr/include/opencv2/tracking.hpp:52
	pub fn cv_tracking_TrackerCSRT_Params_getPropWindow_function_const(instance: *const c_void) -> *mut c_void;
	// window_function /usr/include/opencv2/tracking.hpp:52
	pub fn cv_tracking_TrackerCSRT_Params_setPropWindow_function_string(instance: *mut c_void, val: *mut c_char);
	// kaiser_alpha /usr/include/opencv2/tracking.hpp:53
	pub fn cv_tracking_TrackerCSRT_Params_getPropKaiser_alpha_const(instance: *const c_void) -> f32;
	// kaiser_alpha /usr/include/opencv2/tracking.hpp:53
	pub fn cv_tracking_TrackerCSRT_Params_setPropKaiser_alpha_float(instance: *mut c_void, val: f32);
	// cheb_attenuation /usr/include/opencv2/tracking.hpp:54
	pub fn cv_tracking_TrackerCSRT_Params_getPropCheb_attenuation_const(instance: *const c_void) -> f32;
	// cheb_attenuation /usr/include/opencv2/tracking.hpp:54
	pub fn cv_tracking_TrackerCSRT_Params_setPropCheb_attenuation_float(instance: *mut c_void, val: f32);
	// template_size /usr/include/opencv2/tracking.hpp:56
	pub fn cv_tracking_TrackerCSRT_Params_getPropTemplate_size_const(instance: *const c_void) -> f32;
	// template_size /usr/include/opencv2/tracking.hpp:56
	pub fn cv_tracking_TrackerCSRT_Params_setPropTemplate_size_float(instance: *mut c_void, val: f32);
	// gsl_sigma /usr/include/opencv2/tracking.hpp:57
	pub fn cv_tracking_TrackerCSRT_Params_getPropGsl_sigma_const(instance: *const c_void) -> f32;
	// gsl_sigma /usr/include/opencv2/tracking.hpp:57
	pub fn cv_tracking_TrackerCSRT_Params_setPropGsl_sigma_float(instance: *mut c_void, val: f32);
	// hog_orientations /usr/include/opencv2/tracking.hpp:58
	pub fn cv_tracking_TrackerCSRT_Params_getPropHog_orientations_const(instance: *const c_void) -> f32;
	// hog_orientations /usr/include/opencv2/tracking.hpp:58
	pub fn cv_tracking_TrackerCSRT_Params_setPropHog_orientations_float(instance: *mut c_void, val: f32);
	// hog_clip /usr/include/opencv2/tracking.hpp:59
	pub fn cv_tracking_TrackerCSRT_Params_getPropHog_clip_const(instance: *const c_void) -> f32;
	// hog_clip /usr/include/opencv2/tracking.hpp:59
	pub fn cv_tracking_TrackerCSRT_Params_setPropHog_clip_float(instance: *mut c_void, val: f32);
	// padding /usr/include/opencv2/tracking.hpp:60
	pub fn cv_tracking_TrackerCSRT_Params_getPropPadding_const(instance: *const c_void) -> f32;
	// padding /usr/include/opencv2/tracking.hpp:60
	pub fn cv_tracking_TrackerCSRT_Params_setPropPadding_float(instance: *mut c_void, val: f32);
	// filter_lr /usr/include/opencv2/tracking.hpp:61
	pub fn cv_tracking_TrackerCSRT_Params_getPropFilter_lr_const(instance: *const c_void) -> f32;
	// filter_lr /usr/include/opencv2/tracking.hpp:61
	pub fn cv_tracking_TrackerCSRT_Params_setPropFilter_lr_float(instance: *mut c_void, val: f32);
	// weights_lr /usr/include/opencv2/tracking.hpp:62
	pub fn cv_tracking_TrackerCSRT_Params_getPropWeights_lr_const(instance: *const c_void) -> f32;
	// weights_lr /usr/include/opencv2/tracking.hpp:62
	pub fn cv_tracking_TrackerCSRT_Params_setPropWeights_lr_float(instance: *mut c_void, val: f32);
	// num_hog_channels_used /usr/include/opencv2/tracking.hpp:63
	pub fn cv_tracking_TrackerCSRT_Params_getPropNum_hog_channels_used_const(instance: *const c_void) -> i32;
	// num_hog_channels_used /usr/include/opencv2/tracking.hpp:63
	pub fn cv_tracking_TrackerCSRT_Params_setPropNum_hog_channels_used_int(instance: *mut c_void, val: i32);
	// admm_iterations /usr/include/opencv2/tracking.hpp:64
	pub fn cv_tracking_TrackerCSRT_Params_getPropAdmm_iterations_const(instance: *const c_void) -> i32;
	// admm_iterations /usr/include/opencv2/tracking.hpp:64
	pub fn cv_tracking_TrackerCSRT_Params_setPropAdmm_iterations_int(instance: *mut c_void, val: i32);
	// histogram_bins /usr/include/opencv2/tracking.hpp:65
	pub fn cv_tracking_TrackerCSRT_Params_getPropHistogram_bins_const(instance: *const c_void) -> i32;
	// histogram_bins /usr/include/opencv2/tracking.hpp:65
	pub fn cv_tracking_TrackerCSRT_Params_setPropHistogram_bins_int(instance: *mut c_void, val: i32);
	// histogram_lr /usr/include/opencv2/tracking.hpp:66
	pub fn cv_tracking_TrackerCSRT_Params_getPropHistogram_lr_const(instance: *const c_void) -> f32;
	// histogram_lr /usr/include/opencv2/tracking.hpp:66
	pub fn cv_tracking_TrackerCSRT_Params_setPropHistogram_lr_float(instance: *mut c_void, val: f32);
	// background_ratio /usr/include/opencv2/tracking.hpp:67
	pub fn cv_tracking_TrackerCSRT_Params_getPropBackground_ratio_const(instance: *const c_void) -> i32;
	// background_ratio /usr/include/opencv2/tracking.hpp:67
	pub fn cv_tracking_TrackerCSRT_Params_setPropBackground_ratio_int(instance: *mut c_void, val: i32);
	// number_of_scales /usr/include/opencv2/tracking.hpp:68
	pub fn cv_tracking_TrackerCSRT_Params_getPropNumber_of_scales_const(instance: *const c_void) -> i32;
	// number_of_scales /usr/include/opencv2/tracking.hpp:68
	pub fn cv_tracking_TrackerCSRT_Params_setPropNumber_of_scales_int(instance: *mut c_void, val: i32);
	// scale_sigma_factor /usr/include/opencv2/tracking.hpp:69
	pub fn cv_tracking_TrackerCSRT_Params_getPropScale_sigma_factor_const(instance: *const c_void) -> f32;
	// scale_sigma_factor /usr/include/opencv2/tracking.hpp:69
	pub fn cv_tracking_TrackerCSRT_Params_setPropScale_sigma_factor_float(instance: *mut c_void, val: f32);
	// scale_model_max_area /usr/include/opencv2/tracking.hpp:70
	pub fn cv_tracking_TrackerCSRT_Params_getPropScale_model_max_area_const(instance: *const c_void) -> f32;
	// scale_model_max_area /usr/include/opencv2/tracking.hpp:70
	pub fn cv_tracking_TrackerCSRT_Params_setPropScale_model_max_area_float(instance: *mut c_void, val: f32);
	// scale_lr /usr/include/opencv2/tracking.hpp:71
	pub fn cv_tracking_TrackerCSRT_Params_getPropScale_lr_const(instance: *const c_void) -> f32;
	// scale_lr /usr/include/opencv2/tracking.hpp:71
	pub fn cv_tracking_TrackerCSRT_Params_setPropScale_lr_float(instance: *mut c_void, val: f32);
	// scale_step /usr/include/opencv2/tracking.hpp:72
	pub fn cv_tracking_TrackerCSRT_Params_getPropScale_step_const(instance: *const c_void) -> f32;
	// scale_step /usr/include/opencv2/tracking.hpp:72
	pub fn cv_tracking_TrackerCSRT_Params_setPropScale_step_float(instance: *mut c_void, val: f32);
	// psr_threshold /usr/include/opencv2/tracking.hpp:74
	pub fn cv_tracking_TrackerCSRT_Params_getPropPsr_threshold_const(instance: *const c_void) -> f32;
	// psr_threshold /usr/include/opencv2/tracking.hpp:74
	pub fn cv_tracking_TrackerCSRT_Params_setPropPsr_threshold_float(instance: *mut c_void, val: f32);
	// Params() /usr/include/opencv2/tracking.hpp:43
	pub fn cv_tracking_TrackerCSRT_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
	// create(const TrackerKCF::Params &) /usr/include/opencv2/tracking.hpp:141
	pub fn cv_tracking_TrackerKCF_create_const_ParamsR(parameters: *const crate::tracking::TrackerKCF_Params, ocvrs_return: *mut Result<*mut c_void>);
	// setFeatureExtractor(cv::TrackerKCF::FeatureExtractorCallbackFN, bool) /usr/include/opencv2/tracking.hpp:148
	pub fn cv_tracking_TrackerKCF_setFeatureExtractor_FeatureExtractorCallbackFN_bool(instance: *mut c_void, callback: Option<unsafe extern "C" fn(*const c_void, core::Rect, *mut c_void) -> ()>, pca_func: bool, ocvrs_return: *mut Result_void);
	// Params() /usr/include/opencv2/tracking.hpp:119
	pub fn cv_tracking_TrackerKCF_Params_Params(ocvrs_return: *mut Result<crate::tracking::TrackerKCF_Params>);
}
