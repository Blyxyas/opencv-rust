extern "C" {
	// WeChatQRCode(const std::string &, const std::string &, const std::string &, const std::string &) /usr/include/opencv2/wechat_qrcode.hpp:36
	pub fn cv_wechat_qrcode_WeChatQRCode_WeChatQRCode_const_stringR_const_stringR_const_stringR_const_stringR(detector_prototxt_path: *const c_char, detector_caffe_model_path: *const c_char, super_resolution_prototxt_path: *const c_char, super_resolution_caffe_model_path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// detectAndDecode(cv::InputArray, cv::OutputArrayOfArrays) /usr/include/opencv2/wechat_qrcode.hpp:51
	pub fn cv_wechat_qrcode_WeChatQRCode_detectAndDecode_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, img: *const c_void, points: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
}
