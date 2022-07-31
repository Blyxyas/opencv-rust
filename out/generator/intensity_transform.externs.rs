extern "C" {
	// BIMEF(cv::InputArray, cv::OutputArray, float, float, float) /usr/include/opencv2/intensity_transform.hpp:88
	pub fn cv_intensity_transform_BIMEF_const__InputArrayR_const__OutputArrayR_float_float_float(input: *const c_void, output: *const c_void, mu: f32, a: f32, b: f32, ocvrs_return: *mut Result_void);
	// BIMEF(cv::InputArray, cv::OutputArray, float, float, float, float) /usr/include/opencv2/intensity_transform.hpp:106
	pub fn cv_intensity_transform_BIMEF_const__InputArrayR_const__OutputArrayR_float_float_float_float(input: *const c_void, output: *const c_void, k: f32, mu: f32, a: f32, b: f32, ocvrs_return: *mut Result_void);
	// autoscaling(const cv::Mat, cv::Mat &) /usr/include/opencv2/intensity_transform.hpp:60
	pub fn cv_intensity_transform_autoscaling_const_Mat_MatR(input: *const c_void, output: *mut c_void, ocvrs_return: *mut Result_void);
	// contrastStretching(const cv::Mat, cv::Mat &, const int, const int, const int, const int) /usr/include/opencv2/intensity_transform.hpp:73
	pub fn cv_intensity_transform_contrastStretching_const_Mat_MatR_const_int_const_int_const_int_const_int(input: *const c_void, output: *mut c_void, r1: i32, s1: i32, r2: i32, s2: i32, ocvrs_return: *mut Result_void);
	// gammaCorrection(const cv::Mat, cv::Mat &, const float) /usr/include/opencv2/intensity_transform.hpp:51
	pub fn cv_intensity_transform_gammaCorrection_const_Mat_MatR_const_float(input: *const c_void, output: *mut c_void, gamma: f32, ocvrs_return: *mut Result_void);
	// logTransform(const cv::Mat, cv::Mat &) /usr/include/opencv2/intensity_transform.hpp:41
	pub fn cv_intensity_transform_logTransform_const_Mat_MatR(input: *const c_void, output: *mut c_void, ocvrs_return: *mut Result_void);
}
