extern "C" {
	// debugDMatch(cv::InputArray, std::vector<cv::KeyPoint>, cv::InputArray, std::vector<cv::KeyPoint>, std::vector<cv::DMatch>, const cvv::impl::CallMetaData &, const char *, const char *, bool) /usr/include/opencv2/cvv/dmatch.hpp:24
	pub fn cvv_impl_debugDMatch_const__InputArrayR_vector_KeyPoint__const__InputArrayR_vector_KeyPoint__vector_DMatch__const_CallMetaDataR_const_charX_const_charX_bool(img1: *const c_void, keypoints1: *mut c_void, img2: *const c_void, keypoints2: *mut c_void, matches: *mut c_void, data: *const c_void, description: *const c_char, view: *const c_char, use_train_descriptor: bool, ocvrs_return: *mut Result_void);
	// debugFilter(cv::InputArray, cv::InputArray, const cvv::impl::CallMetaData &, const char *, const char *) /usr/include/opencv2/cvv/filter.hpp:24
	pub fn cvv_impl_debugFilter_const__InputArrayR_const__InputArrayR_const_CallMetaDataR_const_charX_const_charX(original: *const c_void, result: *const c_void, data: *const c_void, description: *const c_char, view: *const c_char, ocvrs_return: *mut Result_void);
	// finalShow() /usr/include/opencv2/cvv/final_show.hpp:15
	pub fn cvv_impl_finalShow(ocvrs_return: *mut Result_void);
	// showImage(cv::InputArray, const cvv::impl::CallMetaData &, const char *, const char *) /usr/include/opencv2/cvv/show_image.hpp:24
	pub fn cvv_impl_showImage_const__InputArrayR_const_CallMetaDataR_const_charX_const_charX(img: *const c_void, data: *const c_void, description: *const c_char, view: *const c_char, ocvrs_return: *mut Result_void);
	// file /usr/include/opencv2/cvv/call_meta_data.hpp:46
	pub fn cvv_impl_CallMetaData_getPropFile_const(instance: *const c_void) -> *mut c_void;
	// line /usr/include/opencv2/cvv/call_meta_data.hpp:47
	pub fn cvv_impl_CallMetaData_getPropLine_const(instance: *const c_void) -> size_t;
	// function /usr/include/opencv2/cvv/call_meta_data.hpp:48
	pub fn cvv_impl_CallMetaData_getPropFunction_const(instance: *const c_void) -> *mut c_void;
	// isKnown /usr/include/opencv2/cvv/call_meta_data.hpp:53
	pub fn cvv_impl_CallMetaData_getPropIsKnown_const(instance: *const c_void) -> bool;
	// CallMetaData() /usr/include/opencv2/cvv/call_meta_data.hpp:26
	pub fn cvv_impl_CallMetaData_CallMetaData(ocvrs_return: *mut Result<*mut c_void>);
	// CallMetaData(const char *, size_t, const char *) /usr/include/opencv2/cvv/call_meta_data.hpp:36
	pub fn cvv_impl_CallMetaData_CallMetaData_const_charX_size_t_const_charX(file: *const c_char, line: size_t, function: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// operator bool() /usr/include/opencv2/cvv/call_meta_data.hpp:40
	pub fn cvv_impl_CallMetaData_operator_bool(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
}
