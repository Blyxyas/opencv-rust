extern "C" {
	// convertCorrespondencies(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/rapid.hpp:100
	pub fn cv_rapid_convertCorrespondencies_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputArrayR(cols: *const c_void, src_locations: *const c_void, pts2d: *const c_void, pts3d: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// drawCorrespondencies(cv::InputOutputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/rapid.hpp:30
	pub fn cv_rapid_drawCorrespondencies_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(bundle: *const c_void, cols: *const c_void, colors: *const c_void, ocvrs_return: *mut Result_void);
	// drawSearchLines(cv::InputOutputArray, cv::InputArray, const cv::Scalar &) /usr/include/opencv2/rapid.hpp:39
	pub fn cv_rapid_drawSearchLines_const__InputOutputArrayR_const__InputArrayR_const_ScalarR(img: *const c_void, locations: *const c_void, color: *const core::Scalar, ocvrs_return: *mut Result_void);
	// drawWireframe(cv::InputOutputArray, cv::InputArray, cv::InputArray, const cv::Scalar &, int, bool) /usr/include/opencv2/rapid.hpp:50
	pub fn cv_rapid_drawWireframe_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const_ScalarR_int_bool(img: *const c_void, pts2d: *const c_void, tris: *const c_void, color: *const core::Scalar, typ: i32, cull_backface: bool, ocvrs_return: *mut Result_void);
	// extractControlPoints(int, int, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, const cv::Size &, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rapid.hpp:67
	pub fn cv_rapid_extractControlPoints_int_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(num: i32, len: i32, pts3d: *const c_void, rvec: *const c_void, tvec: *const c_void, k: *const c_void, imsize: *const core::Size, tris: *const c_void, ctl2d: *const c_void, ctl3d: *const c_void, ocvrs_return: *mut Result_void);
	// extractLineBundle(int, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rapid.hpp:79
	pub fn cv_rapid_extractLineBundle_int_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(len: i32, ctl2d: *const c_void, img: *const c_void, bundle: *const c_void, src_locations: *const c_void, ocvrs_return: *mut Result_void);
	// findCorrespondencies(cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rapid.hpp:89
	pub fn cv_rapid_findCorrespondencies_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(bundle: *const c_void, cols: *const c_void, response: *const c_void, ocvrs_return: *mut Result_void);
	// rapid(cv::InputArray, int, int, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, double *) /usr/include/opencv2/rapid.hpp:123
	pub fn cv_rapid_rapid_const__InputArrayR_int_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_doubleX(img: *const c_void, num: i32, len: i32, pts3d: *const c_void, tris: *const c_void, k: *const c_void, rvec: *const c_void, tvec: *const c_void, rmsd: *mut f64, ocvrs_return: *mut Result<f32>);
	// create(cv::InputArray, cv::InputArray, int, uchar) /usr/include/opencv2/rapid.hpp:158
	pub fn cv_rapid_GOSTracker_create_const__InputArrayR_const__InputArrayR_int_unsigned_char(pts3d: *const c_void, tris: *const c_void, hist_bins: i32, sobel_thesh: u8, ocvrs_return: *mut Result<*mut c_void>);
	// create(cv::InputArray, cv::InputArray, int, uchar) /usr/include/opencv2/rapid.hpp:150
	pub fn cv_rapid_OLSTracker_create_const__InputArrayR_const__InputArrayR_int_unsigned_char(pts3d: *const c_void, tris: *const c_void, hist_bins: i32, sobel_thesh: u8, ocvrs_return: *mut Result<*mut c_void>);
	// create(cv::InputArray, cv::InputArray) /usr/include/opencv2/rapid.hpp:141
	pub fn cv_rapid_Rapid_create_const__InputArrayR_const__InputArrayR(pts3d: *const c_void, tris: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// compute(cv::InputArray, int, int, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, const cv::TermCriteria &) /usr/include/opencv2/rapid.hpp:132
	pub fn cv_rapid_Tracker_compute_const__InputArrayR_int_int_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_TermCriteriaR(instance: *mut c_void, img: *const c_void, num: i32, len: i32, k: *const c_void, rvec: *const c_void, tvec: *const c_void, termcrit: *const core::TermCriteria, ocvrs_return: *mut Result<f32>);
	// clearState() /usr/include/opencv2/rapid.hpp:134
	pub fn cv_rapid_Tracker_clearState(instance: *mut c_void, ocvrs_return: *mut Result_void);
}
