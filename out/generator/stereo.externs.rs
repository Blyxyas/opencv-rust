extern "C" {
	// censusTransform(const cv::Mat &, const cv::Mat &, int, cv::Mat &, cv::Mat &, const int) /usr/include/opencv2/stereo/descriptor.hpp:22
	pub fn cv_stereo_censusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(image1: *const c_void, image2: *const c_void, kernel_size: i32, dist1: *mut c_void, dist2: *mut c_void, typ: i32, ocvrs_return: *mut Result_void);
	// censusTransform(const cv::Mat &, int, cv::Mat &, const int) /usr/include/opencv2/stereo/descriptor.hpp:24
	pub fn cv_stereo_censusTransform_const_MatR_int_MatR_const_int(image1: *const c_void, kernel_size: i32, dist1: *mut c_void, typ: i32, ocvrs_return: *mut Result_void);
	// modifiedCensusTransform(const cv::Mat &, const cv::Mat &, int, cv::Mat &, cv::Mat &, const int, int, const cv::Mat &, const cv::Mat &) /usr/include/opencv2/stereo/descriptor.hpp:29
	pub fn cv_stereo_modifiedCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int_int_const_MatR_const_MatR(img1: *const c_void, img2: *const c_void, kernel_size: i32, dist1: *mut c_void, dist2: *mut c_void, typ: i32, t: i32, integral_image1: *const c_void, integral_image2: *const c_void, ocvrs_return: *mut Result_void);
	// modifiedCensusTransform(const cv::Mat &, int, cv::Mat &, const int, int, const cv::Mat &) /usr/include/opencv2/stereo/descriptor.hpp:31
	pub fn cv_stereo_modifiedCensusTransform_const_MatR_int_MatR_const_int_int_const_MatR(img1: *const c_void, kernel_size: i32, dist: *mut c_void, typ: i32, t: i32, integral_image: *const c_void, ocvrs_return: *mut Result_void);
	// starCensusTransform(const cv::Mat &, const cv::Mat &, int, cv::Mat &, cv::Mat &) /usr/include/opencv2/stereo/descriptor.hpp:39
	pub fn cv_stereo_starCensusTransform_const_MatR_const_MatR_int_MatR_MatR(img1: *const c_void, img2: *const c_void, kernel_size: i32, dist1: *mut c_void, dist2: *mut c_void, ocvrs_return: *mut Result_void);
	// starCensusTransform(const cv::Mat &, int, cv::Mat &) /usr/include/opencv2/stereo/descriptor.hpp:41
	pub fn cv_stereo_starCensusTransform_const_MatR_int_MatR(img1: *const c_void, kernel_size: i32, dist: *mut c_void, ocvrs_return: *mut Result_void);
	// symetricCensusTransform(const cv::Mat &, const cv::Mat &, int, cv::Mat &, cv::Mat &, const int) /usr/include/opencv2/stereo/descriptor.hpp:35
	pub fn cv_stereo_symetricCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(img1: *const c_void, img2: *const c_void, kernel_size: i32, dist1: *mut c_void, dist2: *mut c_void, typ: i32, ocvrs_return: *mut Result_void);
	// symetricCensusTransform(const cv::Mat &, int, cv::Mat &, const int) /usr/include/opencv2/stereo/descriptor.hpp:37
	pub fn cv_stereo_symetricCensusTransform_const_MatR_int_MatR_const_int(img1: *const c_void, kernel_size: i32, dist1: *mut c_void, typ: i32, ocvrs_return: *mut Result_void);
	// MatchQuasiDense() /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:33
	pub fn cv_stereo_MatchQuasiDense_MatchQuasiDense(ocvrs_return: *mut Result<crate::stereo::MatchQuasiDense>);
	// Param /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:190
	pub fn cv_stereo_QuasiDenseStereo_getPropParam_const(instance: *const c_void, ocvrs_return: *mut crate::stereo::PropagationParameters);
	// Param /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:190
	pub fn cv_stereo_QuasiDenseStereo_setPropParam_PropagationParameters(instance: *mut c_void, val: *const crate::stereo::PropagationParameters);
	// loadParameters(cv::String) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:118
	pub fn cv_stereo_QuasiDenseStereo_loadParameters_String(instance: *mut c_void, filepath: *mut c_char, ocvrs_return: *mut Result<i32>);
	// saveParameters(cv::String) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:129
	pub fn cv_stereo_QuasiDenseStereo_saveParameters_String(instance: *mut c_void, filepath: *mut c_char, ocvrs_return: *mut Result<i32>);
	// getSparseMatches(std::vector<MatchQuasiDense> &) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:138
	pub fn cv_stereo_QuasiDenseStereo_getSparseMatches_vector_MatchQuasiDense_R(instance: *mut c_void, s_matches: *mut c_void, ocvrs_return: *mut Result_void);
	// getDenseMatches(std::vector<MatchQuasiDense> &) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:147
	pub fn cv_stereo_QuasiDenseStereo_getDenseMatches_vector_MatchQuasiDense_R(instance: *mut c_void, dense_matches: *mut c_void, ocvrs_return: *mut Result_void);
	// process(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:163
	pub fn cv_stereo_QuasiDenseStereo_process_const_MatR_const_MatR(instance: *mut c_void, img_left: *const c_void, img_right: *const c_void, ocvrs_return: *mut Result_void);
	// getMatch(const int, const int) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:174
	pub fn cv_stereo_QuasiDenseStereo_getMatch_const_int_const_int(instance: *mut c_void, x: i32, y: i32, ocvrs_return: *mut Result<core::Point2f>);
	// getDisparity() /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:184
	pub fn cv_stereo_QuasiDenseStereo_getDisparity(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(cv::Size, cv::String) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:187
	pub fn cv_stereo_QuasiDenseStereo_create_Size_String(mono_img_size: *const core::Size, param_filepath: *mut c_char, ocvrs_return: *mut Result<*mut c_void>);
}
