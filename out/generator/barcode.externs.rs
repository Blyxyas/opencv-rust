extern "C" {
	// BarcodeDetector(const std::string &, const std::string &) /usr/include/opencv2/barcode.hpp:59
	pub fn cv_barcode_BarcodeDetector_BarcodeDetector_const_stringR_const_stringR(prototxt_path: *const c_char, model_path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// detect(cv::InputArray, cv::OutputArray) /usr/include/opencv2/barcode.hpp:70
	pub fn cv_barcode_BarcodeDetector_detect_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, img: *const c_void, points: *const c_void, ocvrs_return: *mut Result<bool>);
	// decode(cv::InputArray, cv::InputArray, std::vector<std::string> &, std::vector<BarcodeType> &) /usr/include/opencv2/barcode.hpp:81
	pub fn cv_barcode_BarcodeDetector_decode_const_const__InputArrayR_const__InputArrayR_vector_string_R_vector_BarcodeType_R(instance: *const c_void, img: *const c_void, points: *const c_void, decoded_info: *mut c_void, decoded_type: *mut c_void, ocvrs_return: *mut Result<bool>);
	// detectAndDecode(cv::InputArray, std::vector<std::string> &, std::vector<BarcodeType> &, cv::OutputArray) /usr/include/opencv2/barcode.hpp:91
	pub fn cv_barcode_BarcodeDetector_detectAndDecode_const_const__InputArrayR_vector_string_R_vector_BarcodeType_R_const__OutputArrayR(instance: *const c_void, img: *const c_void, decoded_info: *mut c_void, decoded_type: *mut c_void, points: *const c_void, ocvrs_return: *mut Result<bool>);
}
