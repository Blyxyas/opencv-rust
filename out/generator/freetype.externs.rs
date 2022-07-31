extern "C" {
	// createFreeType2() /usr/include/opencv2/freetype.hpp:184
	pub fn cv_freetype_createFreeType2(ocvrs_return: *mut Result<*mut c_void>);
	// loadFontData(cv::String, int) /usr/include/opencv2/freetype.hpp:85
	pub fn cv_freetype_FreeType2_loadFontData_String_int(instance: *mut c_void, font_file_name: *mut c_char, id: i32, ocvrs_return: *mut Result_void);
	// setSplitNumber(int) /usr/include/opencv2/freetype.hpp:96
	pub fn cv_freetype_FreeType2_setSplitNumber_int(instance: *mut c_void, num: i32, ocvrs_return: *mut Result_void);
	// putText(cv::InputOutputArray, const cv::String &, cv::Point, int, cv::Scalar, int, int, bool) /usr/include/opencv2/freetype.hpp:112
	pub fn cv_freetype_FreeType2_putText_const__InputOutputArrayR_const_StringR_Point_int_Scalar_int_int_bool(instance: *mut c_void, img: *const c_void, text: *const c_char, org: *const core::Point, font_height: i32, color: *const core::Scalar, thickness: i32, line_type: i32, bottom_left_origin: bool, ocvrs_return: *mut Result_void);
	// getTextSize(const cv::String &, int, int, int *) /usr/include/opencv2/freetype.hpp:173
	pub fn cv_freetype_FreeType2_getTextSize_const_StringR_int_int_intX(instance: *mut c_void, text: *const c_char, font_height: i32, thickness: i32, base_line: *mut i32, ocvrs_return: *mut Result<core::Size>);
}
