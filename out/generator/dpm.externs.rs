extern "C" {
	// isEmpty() /usr/include/opencv2/dpm.hpp:117
	pub fn cv_dpm_DPMDetector_isEmpty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// detect(cv::Mat &, std::vector<ObjectDetection> &) /usr/include/opencv2/dpm.hpp:124
	pub fn cv_dpm_DPMDetector_detect_MatR_vector_ObjectDetection_R(instance: *mut c_void, image: *mut c_void, objects: *mut c_void, ocvrs_return: *mut Result_void);
	// getClassNames() /usr/include/opencv2/dpm.hpp:129
	pub fn cv_dpm_DPMDetector_getClassNames_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getClassCount() /usr/include/opencv2/dpm.hpp:133
	pub fn cv_dpm_DPMDetector_getClassCount_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// create(const std::vector<std::string> &, const std::vector<std::string> &) /usr/include/opencv2/dpm.hpp:142
	pub fn cv_dpm_DPMDetector_create_const_vector_string_R_const_vector_string_R(filenames: *const c_void, class_names: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// rect /usr/include/opencv2/dpm.hpp:112
	pub fn cv_dpm_DPMDetector_ObjectDetection_getPropRect_const(instance: *const c_void, ocvrs_return: *mut core::Rect);
	// rect /usr/include/opencv2/dpm.hpp:112
	pub fn cv_dpm_DPMDetector_ObjectDetection_setPropRect_Rect(instance: *mut c_void, val: *const core::Rect);
	// score /usr/include/opencv2/dpm.hpp:113
	pub fn cv_dpm_DPMDetector_ObjectDetection_getPropScore_const(instance: *const c_void) -> f32;
	// score /usr/include/opencv2/dpm.hpp:113
	pub fn cv_dpm_DPMDetector_ObjectDetection_setPropScore_float(instance: *mut c_void, val: f32);
	// classID /usr/include/opencv2/dpm.hpp:114
	pub fn cv_dpm_DPMDetector_ObjectDetection_getPropClassID_const(instance: *const c_void) -> i32;
	// classID /usr/include/opencv2/dpm.hpp:114
	pub fn cv_dpm_DPMDetector_ObjectDetection_setPropClassID_int(instance: *mut c_void, val: i32);
	// ObjectDetection() /usr/include/opencv2/dpm.hpp:110
	pub fn cv_dpm_DPMDetector_ObjectDetection_ObjectDetection(ocvrs_return: *mut Result<*mut c_void>);
	// ObjectDetection(const cv::Rect &, float, int) /usr/include/opencv2/dpm.hpp:111
	pub fn cv_dpm_DPMDetector_ObjectDetection_ObjectDetection_const_RectR_float_int(rect: *const core::Rect, score: f32, class_id: i32, ocvrs_return: *mut Result<*mut c_void>);
}
