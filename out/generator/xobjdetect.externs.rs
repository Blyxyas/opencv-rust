extern "C" {
	// read(const cv::FileNode &) /usr/include/opencv2/xobjdetect.hpp:66
	pub fn cv_xobjdetect_WBDetector_read_const_FileNodeR(instance: *mut c_void, node: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/xobjdetect.hpp:71
	pub fn cv_xobjdetect_WBDetector_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// train(const std::string &, const std::string &) /usr/include/opencv2/xobjdetect.hpp:77
	pub fn cv_xobjdetect_WBDetector_train_const_stringR_const_stringR(instance: *mut c_void, pos_samples: *const c_char, neg_imgs: *const c_char, ocvrs_return: *mut Result_void);
	// detect(const cv::Mat &, std::vector<Rect> &, std::vector<double> &) /usr/include/opencv2/xobjdetect.hpp:86
	pub fn cv_xobjdetect_WBDetector_detect_const_MatR_vector_Rect_R_vector_double_R(instance: *mut c_void, img: *const c_void, bboxes: *mut c_void, confidences: *mut c_void, ocvrs_return: *mut Result_void);
	// create() /usr/include/opencv2/xobjdetect.hpp:93
	pub fn cv_xobjdetect_WBDetector_create(ocvrs_return: *mut Result<*mut c_void>);
}
