extern "C" {
	// haveImageReader(const cv::String &) /usr/include/opencv2/imgcodecs.hpp:317
	pub fn cv_haveImageReader_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<bool>);
	// haveImageWriter(const cv::String &) /usr/include/opencv2/imgcodecs.hpp:323
	pub fn cv_haveImageWriter_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<bool>);
	// imcount(const cv::String &, int) /usr/include/opencv2/imgcodecs.hpp:236
	pub fn cv_imcount_const_StringR_int(filename: *const c_char, flags: i32, ocvrs_return: *mut Result<size_t>);
	// imdecode(cv::InputArray, int) /usr/include/opencv2/imgcodecs.hpp:289
	pub fn cv_imdecode_const__InputArrayR_int(buf: *const c_void, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
	// imdecode(cv::InputArray, int, cv::Mat *) /usr/include/opencv2/imgcodecs.hpp:297
	pub fn cv_imdecode_const__InputArrayR_int_MatX(buf: *const c_void, flags: i32, dst: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// imencode(const cv::String &, cv::InputArray, std::vector<uchar> &, const std::vector<int> &) /usr/include/opencv2/imgcodecs.hpp:309
	pub fn cv_imencode_const_StringR_const__InputArrayR_vector_unsigned_char_R_const_vector_int_R(ext: *const c_char, img: *const c_void, buf: *mut c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
	// imread(const cv::String &, int) /usr/include/opencv2/imgcodecs.hpp:206
	pub fn cv_imread_const_StringR_int(filename: *const c_char, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
	// imreadmulti(const cv::String &, std::vector<Mat> &, int) /usr/include/opencv2/imgcodecs.hpp:216
	pub fn cv_imreadmulti_const_StringR_vector_Mat_R_int(filename: *const c_char, mats: *mut c_void, flags: i32, ocvrs_return: *mut Result<bool>);
	// imreadmulti(const cv::String &, std::vector<Mat> &, int, int, int) /usr/include/opencv2/imgcodecs.hpp:228
	pub fn cv_imreadmulti_const_StringR_vector_Mat_R_int_int_int(filename: *const c_char, mats: *mut c_void, start: i32, count: i32, flags: i32, ocvrs_return: *mut Result<bool>);
	// imwrite(const cv::String &, cv::InputArray, const std::vector<int> &) /usr/include/opencv2/imgcodecs.hpp:267
	pub fn cv_imwrite_const_StringR_const__InputArrayR_const_vector_int_R(filename: *const c_char, img: *const c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
	// imwritemulti(const cv::String &, cv::InputArrayOfArrays, const std::vector<int> &) /usr/include/opencv2/imgcodecs.hpp:272
	pub fn cv_imwritemulti_const_StringR_const__InputArrayR_const_vector_int_R(filename: *const c_char, img: *const c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
}
