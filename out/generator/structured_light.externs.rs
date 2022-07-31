extern "C" {
	// create(const GrayCodePattern::Params &) /usr/include/opencv2/structured_light/graycodepattern.hpp:86
	pub fn cv_structured_light_GrayCodePattern_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int) /usr/include/opencv2/structured_light/graycodepattern.hpp:90
	pub fn cv_structured_light_GrayCodePattern_create_int_int(width: i32, height: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getNumberOfPatternImages() /usr/include/opencv2/structured_light/graycodepattern.hpp:98
	pub fn cv_structured_light_GrayCodePattern_getNumberOfPatternImages_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// setWhiteThreshold(size_t) /usr/include/opencv2/structured_light/graycodepattern.hpp:108
	pub fn cv_structured_light_GrayCodePattern_setWhiteThreshold_size_t(instance: *mut c_void, value: size_t, ocvrs_return: *mut Result_void);
	// setBlackThreshold(size_t) /usr/include/opencv2/structured_light/graycodepattern.hpp:118
	pub fn cv_structured_light_GrayCodePattern_setBlackThreshold_size_t(instance: *mut c_void, value: size_t, ocvrs_return: *mut Result_void);
	// getImagesForShadowMasks(cv::InputOutputArray, cv::InputOutputArray) /usr/include/opencv2/structured_light/graycodepattern.hpp:130
	pub fn cv_structured_light_GrayCodePattern_getImagesForShadowMasks_const_const__InputOutputArrayR_const__InputOutputArrayR(instance: *const c_void, black_image: *const c_void, white_image: *const c_void, ocvrs_return: *mut Result_void);
	// getProjPixel(cv::InputArrayOfArrays, int, int, cv::Point &) /usr/include/opencv2/structured_light/graycodepattern.hpp:143
	pub fn cv_structured_light_GrayCodePattern_getProjPixel_const_const__InputArrayR_int_int_PointR(instance: *const c_void, pattern_images: *const c_void, x: i32, y: i32, proj_pix: *mut core::Point, ocvrs_return: *mut Result<bool>);
	// width /usr/include/opencv2/structured_light/graycodepattern.hpp:79
	pub fn cv_structured_light_GrayCodePattern_Params_getPropWidth_const(instance: *const c_void) -> i32;
	// width /usr/include/opencv2/structured_light/graycodepattern.hpp:79
	pub fn cv_structured_light_GrayCodePattern_Params_setPropWidth_int(instance: *mut c_void, val: i32);
	// height /usr/include/opencv2/structured_light/graycodepattern.hpp:80
	pub fn cv_structured_light_GrayCodePattern_Params_getPropHeight_const(instance: *const c_void) -> i32;
	// height /usr/include/opencv2/structured_light/graycodepattern.hpp:80
	pub fn cv_structured_light_GrayCodePattern_Params_setPropHeight_int(instance: *mut c_void, val: i32);
	// Params() /usr/include/opencv2/structured_light/graycodepattern.hpp:78
	pub fn cv_structured_light_GrayCodePattern_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
	// create(Ptr<SinusoidalPattern::Params>) /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:100
	pub fn cv_structured_light_SinusoidalPattern_create_Ptr_Params_(parameters: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// computePhaseMap(cv::InputArrayOfArrays, cv::OutputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:110
	pub fn cv_structured_light_SinusoidalPattern_computePhaseMap_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR(instance: *mut c_void, pattern_images: *const c_void, wrapped_phase_map: *const c_void, shadow_mask: *const c_void, fundamental: *const c_void, ocvrs_return: *mut Result_void);
	// unwrapPhaseMap(cv::InputArray, cv::OutputArray, cv::Size, cv::InputArray) /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:122
	pub fn cv_structured_light_SinusoidalPattern_unwrapPhaseMap_const__InputArrayR_const__OutputArrayR_Size_const__InputArrayR(instance: *mut c_void, wrapped_phase_map: *const c_void, unwrapped_phase_map: *const c_void, cam_size: *const core::Size, shadow_mask: *const c_void, ocvrs_return: *mut Result_void);
	// findProCamMatches(cv::InputArray, cv::InputArray, cv::OutputArrayOfArrays) /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:133
	pub fn cv_structured_light_SinusoidalPattern_findProCamMatches_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, proj_unwrapped_phase_map: *const c_void, cam_unwrapped_phase_map: *const c_void, matches: *const c_void, ocvrs_return: *mut Result_void);
	// computeDataModulationTerm(cv::InputArrayOfArrays, cv::OutputArray, cv::InputArray) /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:143
	pub fn cv_structured_light_SinusoidalPattern_computeDataModulationTerm_const__InputArrayR_const__OutputArrayR_const__InputArrayR(instance: *mut c_void, pattern_images: *const c_void, data_modulation_term: *const c_void, shadow_mask: *const c_void, ocvrs_return: *mut Result_void);
	// width /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:85
	pub fn cv_structured_light_SinusoidalPattern_Params_getPropWidth_const(instance: *const c_void) -> i32;
	// width /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:85
	pub fn cv_structured_light_SinusoidalPattern_Params_setPropWidth_int(instance: *mut c_void, val: i32);
	// height /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:86
	pub fn cv_structured_light_SinusoidalPattern_Params_getPropHeight_const(instance: *const c_void) -> i32;
	// height /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:86
	pub fn cv_structured_light_SinusoidalPattern_Params_setPropHeight_int(instance: *mut c_void, val: i32);
	// nbrOfPeriods /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:87
	pub fn cv_structured_light_SinusoidalPattern_Params_getPropNbrOfPeriods_const(instance: *const c_void) -> i32;
	// nbrOfPeriods /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:87
	pub fn cv_structured_light_SinusoidalPattern_Params_setPropNbrOfPeriods_int(instance: *mut c_void, val: i32);
	// shiftValue /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:88
	pub fn cv_structured_light_SinusoidalPattern_Params_getPropShiftValue_const(instance: *const c_void) -> f32;
	// shiftValue /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:88
	pub fn cv_structured_light_SinusoidalPattern_Params_setPropShiftValue_float(instance: *mut c_void, val: f32);
	// methodId /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:89
	pub fn cv_structured_light_SinusoidalPattern_Params_getPropMethodId_const(instance: *const c_void) -> i32;
	// methodId /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:89
	pub fn cv_structured_light_SinusoidalPattern_Params_setPropMethodId_int(instance: *mut c_void, val: i32);
	// nbrOfPixelsBetweenMarkers /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:90
	pub fn cv_structured_light_SinusoidalPattern_Params_getPropNbrOfPixelsBetweenMarkers_const(instance: *const c_void) -> i32;
	// nbrOfPixelsBetweenMarkers /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:90
	pub fn cv_structured_light_SinusoidalPattern_Params_setPropNbrOfPixelsBetweenMarkers_int(instance: *mut c_void, val: i32);
	// horizontal /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:91
	pub fn cv_structured_light_SinusoidalPattern_Params_getPropHorizontal_const(instance: *const c_void) -> bool;
	// horizontal /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:91
	pub fn cv_structured_light_SinusoidalPattern_Params_setPropHorizontal_bool(instance: *mut c_void, val: bool);
	// setMarkers /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:92
	pub fn cv_structured_light_SinusoidalPattern_Params_getPropSetMarkers_const(instance: *const c_void) -> bool;
	// setMarkers /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:92
	pub fn cv_structured_light_SinusoidalPattern_Params_setPropSetMarkers_bool(instance: *mut c_void, val: bool);
	// markersLocation /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:93
	pub fn cv_structured_light_SinusoidalPattern_Params_getPropMarkersLocation_const(instance: *const c_void) -> *mut c_void;
	// markersLocation /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:93
	pub fn cv_structured_light_SinusoidalPattern_Params_setPropMarkersLocation_vector_Point2f_(instance: *mut c_void, val: *mut c_void);
	// Params() /usr/include/opencv2/structured_light/sinusoidalpattern.hpp:84
	pub fn cv_structured_light_SinusoidalPattern_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
	// generate(cv::OutputArrayOfArrays) /usr/include/opencv2/structured_light/structured_light.hpp:69
	pub fn cv_structured_light_StructuredLightPattern_generate_const__OutputArrayR(instance: *mut c_void, pattern_images: *const c_void, ocvrs_return: *mut Result<bool>);
	// decode(const std::vector<std::vector<Mat>> &, cv::OutputArray, cv::InputArrayOfArrays, cv::InputArrayOfArrays, int) /usr/include/opencv2/structured_light/structured_light.hpp:81
	pub fn cv_structured_light_StructuredLightPattern_decode_const_const_vector_vector_Mat__R_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(instance: *const c_void, pattern_images: *const c_void, disparity_map: *const c_void, black_images: *const c_void, white_images: *const c_void, flags: i32, ocvrs_return: *mut Result<bool>);
}
