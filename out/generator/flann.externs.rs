extern "C" {
	// flann_distance_type() /usr/include/opencv2/flann.hpp:61
	pub fn cvflann_flann_distance_type(ocvrs_return: *mut Result<crate::flann::flann_distance_t>);
	// set_distance_type(cvflann::flann_distance_t, int) /usr/include/opencv2/flann.hpp:62
	pub fn cvflann_set_distance_type_flann_distance_t_int(distance_type: crate::flann::flann_distance_t, order: i32, ocvrs_return: *mut Result_void);
	// AutotunedIndexParams(float, float, float, float) /usr/include/opencv2/flann/miniflann.hpp:118
	pub fn cv_flann_AutotunedIndexParams_AutotunedIndexParams_float_float_float_float(target_precision: f32, build_weight: f32, memory_weight: f32, sample_fraction: f32, ocvrs_return: *mut Result<*mut c_void>);
	// CompositeIndexParams(int, int, int, cvflann::flann_centers_init_t, float) /usr/include/opencv2/flann/miniflann.hpp:112
	pub fn cv_flann_CompositeIndexParams_CompositeIndexParams_int_int_int_flann_centers_init_t_float(trees: i32, branching: i32, iterations: i32, centers_init: crate::flann::flann_centers_init_t, cb_index: f32, ocvrs_return: *mut Result<*mut c_void>);
	// HierarchicalClusteringIndexParams(int, cvflann::flann_centers_init_t, int, int) /usr/include/opencv2/flann/miniflann.hpp:124
	pub fn cv_flann_HierarchicalClusteringIndexParams_HierarchicalClusteringIndexParams_int_flann_centers_init_t_int_int(branching: i32, centers_init: crate::flann::flann_centers_init_t, trees: i32, leaf_size: i32, ocvrs_return: *mut Result<*mut c_void>);
	// Index() /usr/include/opencv2/flann/miniflann.hpp:153
	pub fn cv_flann_Index_Index(ocvrs_return: *mut Result<*mut c_void>);
	// Index(cv::InputArray, const cv::flann::IndexParams &, cvflann::flann_distance_t) /usr/include/opencv2/flann/miniflann.hpp:154
	pub fn cv_flann_Index_Index_const__InputArrayR_const_IndexParamsR_flann_distance_t(features: *const c_void, params: *const c_void, dist_type: crate::flann::flann_distance_t, ocvrs_return: *mut Result<*mut c_void>);
	// build(cv::InputArray, const cv::flann::IndexParams &, cvflann::flann_distance_t) /usr/include/opencv2/flann/miniflann.hpp:157
	pub fn cv_flann_Index_build_const__InputArrayR_const_IndexParamsR_flann_distance_t(instance: *mut c_void, features: *const c_void, params: *const c_void, dist_type: crate::flann::flann_distance_t, ocvrs_return: *mut Result_void);
	// knnSearch(cv::InputArray, cv::OutputArray, cv::OutputArray, int, const cv::flann::SearchParams &) /usr/include/opencv2/flann/miniflann.hpp:158
	pub fn cv_flann_Index_knnSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_SearchParamsR(instance: *mut c_void, query: *const c_void, indices: *const c_void, dists: *const c_void, knn: i32, params: *const c_void, ocvrs_return: *mut Result_void);
	// radiusSearch(cv::InputArray, cv::OutputArray, cv::OutputArray, double, int, const cv::flann::SearchParams &) /usr/include/opencv2/flann/miniflann.hpp:161
	pub fn cv_flann_Index_radiusSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_int_const_SearchParamsR(instance: *mut c_void, query: *const c_void, indices: *const c_void, dists: *const c_void, radius: f64, max_results: i32, params: *const c_void, ocvrs_return: *mut Result<i32>);
	// save(const cv::String &) /usr/include/opencv2/flann/miniflann.hpp:165
	pub fn cv_flann_Index_save_const_const_StringR(instance: *const c_void, filename: *const c_char, ocvrs_return: *mut Result_void);
	// load(cv::InputArray, const cv::String &) /usr/include/opencv2/flann/miniflann.hpp:166
	pub fn cv_flann_Index_load_const__InputArrayR_const_StringR(instance: *mut c_void, features: *const c_void, filename: *const c_char, ocvrs_return: *mut Result<bool>);
	// release() /usr/include/opencv2/flann/miniflann.hpp:167
	pub fn cv_flann_Index_release(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// getDistance() /usr/include/opencv2/flann/miniflann.hpp:168
	pub fn cv_flann_Index_getDistance_const(instance: *const c_void, ocvrs_return: *mut Result<crate::flann::flann_distance_t>);
	// getAlgorithm() /usr/include/opencv2/flann/miniflann.hpp:169
	pub fn cv_flann_Index_getAlgorithm_const(instance: *const c_void, ocvrs_return: *mut Result<crate::flann::flann_algorithm_t>);
	// params /usr/include/opencv2/flann/miniflann.hpp:93
	pub fn cv_flann_IndexParams_getPropParams(instance: *mut c_void) -> *mut c_void;
	// params /usr/include/opencv2/flann/miniflann.hpp:93
	pub fn cv_flann_IndexParams_setPropParams_voidX(instance: *mut c_void, val: *mut c_void);
	// IndexParams() /usr/include/opencv2/flann/miniflann.hpp:73
	pub fn cv_flann_IndexParams_IndexParams(ocvrs_return: *mut Result<*mut c_void>);
	// getString(const cv::String &, const cv::String &) /usr/include/opencv2/flann/miniflann.hpp:76
	pub fn cv_flann_IndexParams_getString_const_const_StringR_const_StringR(instance: *const c_void, key: *const c_char, default_val: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// getInt(const cv::String &, int) /usr/include/opencv2/flann/miniflann.hpp:77
	pub fn cv_flann_IndexParams_getInt_const_const_StringR_int(instance: *const c_void, key: *const c_char, default_val: i32, ocvrs_return: *mut Result<i32>);
	// getDouble(const cv::String &, double) /usr/include/opencv2/flann/miniflann.hpp:78
	pub fn cv_flann_IndexParams_getDouble_const_const_StringR_double(instance: *const c_void, key: *const c_char, default_val: f64, ocvrs_return: *mut Result<f64>);
	// setString(const cv::String &, const cv::String &) /usr/include/opencv2/flann/miniflann.hpp:80
	pub fn cv_flann_IndexParams_setString_const_StringR_const_StringR(instance: *mut c_void, key: *const c_char, value: *const c_char, ocvrs_return: *mut Result_void);
	// setInt(const cv::String &, int) /usr/include/opencv2/flann/miniflann.hpp:81
	pub fn cv_flann_IndexParams_setInt_const_StringR_int(instance: *mut c_void, key: *const c_char, value: i32, ocvrs_return: *mut Result_void);
	// setDouble(const cv::String &, double) /usr/include/opencv2/flann/miniflann.hpp:82
	pub fn cv_flann_IndexParams_setDouble_const_StringR_double(instance: *mut c_void, key: *const c_char, value: f64, ocvrs_return: *mut Result_void);
	// setFloat(const cv::String &, float) /usr/include/opencv2/flann/miniflann.hpp:83
	pub fn cv_flann_IndexParams_setFloat_const_StringR_float(instance: *mut c_void, key: *const c_char, value: f32, ocvrs_return: *mut Result_void);
	// setBool(const cv::String &, bool) /usr/include/opencv2/flann/miniflann.hpp:84
	pub fn cv_flann_IndexParams_setBool_const_StringR_bool(instance: *mut c_void, key: *const c_char, value: bool, ocvrs_return: *mut Result_void);
	// setAlgorithm(int) /usr/include/opencv2/flann/miniflann.hpp:85
	pub fn cv_flann_IndexParams_setAlgorithm_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result_void);
	// getAll(std::vector<String> &, std::vector<FlannIndexType> &, std::vector<String> &, std::vector<double> &) /usr/include/opencv2/flann/miniflann.hpp:88
	pub fn cv_flann_IndexParams_getAll_const_vector_String_R_vector_FlannIndexType_R_vector_String_R_vector_double_R(instance: *const c_void, names: *mut c_void, types: *mut c_void, str_values: *mut c_void, num_values: *mut c_void, ocvrs_return: *mut Result_void);
	// KDTreeIndexParams(int) /usr/include/opencv2/flann/miniflann.hpp:102
	pub fn cv_flann_KDTreeIndexParams_KDTreeIndexParams_int(trees: i32, ocvrs_return: *mut Result<*mut c_void>);
	// KMeansIndexParams(int, int, cvflann::flann_centers_init_t, float) /usr/include/opencv2/flann/miniflann.hpp:130
	pub fn cv_flann_KMeansIndexParams_KMeansIndexParams_int_int_flann_centers_init_t_float(branching: i32, iterations: i32, centers_init: crate::flann::flann_centers_init_t, cb_index: f32, ocvrs_return: *mut Result<*mut c_void>);
	// LinearIndexParams() /usr/include/opencv2/flann/miniflann.hpp:107
	pub fn cv_flann_LinearIndexParams_LinearIndexParams(ocvrs_return: *mut Result<*mut c_void>);
	// LshIndexParams(int, int, int) /usr/include/opencv2/flann/miniflann.hpp:136
	pub fn cv_flann_LshIndexParams_LshIndexParams_int_int_int(table_number: i32, key_size: i32, multi_probe_level: i32, ocvrs_return: *mut Result<*mut c_void>);
	// SavedIndexParams(const cv::String &) /usr/include/opencv2/flann/miniflann.hpp:141
	pub fn cv_flann_SavedIndexParams_SavedIndexParams_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// SearchParams(int, float, bool, bool) /usr/include/opencv2/flann/miniflann.hpp:146
	pub fn cv_flann_SearchParams_SearchParams_int_float_bool_bool(checks: i32, eps: f32, sorted: bool, explore_all_trees: bool, ocvrs_return: *mut Result<*mut c_void>);
	// SearchParams(int, float, bool) /usr/include/opencv2/flann/miniflann.hpp:147
	pub fn cv_flann_SearchParams_SearchParams_int_float_bool(checks: i32, eps: f32, sorted: bool, ocvrs_return: *mut Result<*mut c_void>);
}
