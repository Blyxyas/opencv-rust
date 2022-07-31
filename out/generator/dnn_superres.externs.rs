extern "C" {
	// create() /usr/include/opencv2/dnn_superres.hpp:60
	pub fn cv_dnn_superres_DnnSuperResImpl_create(ocvrs_return: *mut Result<*mut c_void>);
	// DnnSuperResImpl() /usr/include/opencv2/dnn_superres.hpp:64
	pub fn cv_dnn_superres_DnnSuperResImpl_DnnSuperResImpl(ocvrs_return: *mut Result<*mut c_void>);
	// DnnSuperResImpl(const cv::String &, int) /usr/include/opencv2/dnn_superres.hpp:74
	pub fn cv_dnn_superres_DnnSuperResImpl_DnnSuperResImpl_const_StringR_int(algo: *const c_char, scale: i32, ocvrs_return: *mut Result<*mut c_void>);
	// readModel(const cv::String &) /usr/include/opencv2/dnn_superres.hpp:79
	pub fn cv_dnn_superres_DnnSuperResImpl_readModel_const_StringR(instance: *mut c_void, path: *const c_char, ocvrs_return: *mut Result_void);
	// readModel(const cv::String &, const cv::String &) /usr/include/opencv2/dnn_superres.hpp:85
	pub fn cv_dnn_superres_DnnSuperResImpl_readModel_const_StringR_const_StringR(instance: *mut c_void, weights: *const c_char, definition: *const c_char, ocvrs_return: *mut Result_void);
	// setModel(const cv::String &, int) /usr/include/opencv2/dnn_superres.hpp:95
	pub fn cv_dnn_superres_DnnSuperResImpl_setModel_const_StringR_int(instance: *mut c_void, algo: *const c_char, scale: i32, ocvrs_return: *mut Result_void);
	// setPreferableBackend(int) /usr/include/opencv2/dnn_superres.hpp:99
	pub fn cv_dnn_superres_DnnSuperResImpl_setPreferableBackend_int(instance: *mut c_void, backend_id: i32, ocvrs_return: *mut Result_void);
	// setPreferableTarget(int) /usr/include/opencv2/dnn_superres.hpp:103
	pub fn cv_dnn_superres_DnnSuperResImpl_setPreferableTarget_int(instance: *mut c_void, target_id: i32, ocvrs_return: *mut Result_void);
	// upsample(cv::InputArray, cv::OutputArray) /usr/include/opencv2/dnn_superres.hpp:109
	pub fn cv_dnn_superres_DnnSuperResImpl_upsample_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, img: *const c_void, result: *const c_void, ocvrs_return: *mut Result_void);
	// upsampleMultioutput(cv::InputArray, std::vector<Mat> &, const std::vector<int> &, const std::vector<String> &) /usr/include/opencv2/dnn_superres.hpp:117
	pub fn cv_dnn_superres_DnnSuperResImpl_upsampleMultioutput_const__InputArrayR_vector_Mat_R_const_vector_int_R_const_vector_String_R(instance: *mut c_void, img: *const c_void, imgs_new: *mut c_void, scale_factors: *const c_void, node_names: *const c_void, ocvrs_return: *mut Result_void);
	// getScale() /usr/include/opencv2/dnn_superres.hpp:122
	pub fn cv_dnn_superres_DnnSuperResImpl_getScale(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// getAlgorithm() /usr/include/opencv2/dnn_superres.hpp:127
	pub fn cv_dnn_superres_DnnSuperResImpl_getAlgorithm(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
}
