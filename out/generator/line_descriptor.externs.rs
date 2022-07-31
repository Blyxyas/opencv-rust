extern "C" {
	// drawKeylines(const cv::Mat &, const std::vector<KeyLine> &, cv::Mat &, const cv::Scalar &, int) /usr/include/opencv2/line_descriptor/descriptor.hpp:1401
	pub fn cv_line_descriptor_drawKeylines_const_MatR_const_vector_KeyLine_R_MatR_const_ScalarR_int(image: *const c_void, keylines: *const c_void, out_image: *mut c_void, color: *const core::Scalar, flags: i32, ocvrs_return: *mut Result_void);
	// drawLineMatches(const cv::Mat &, const std::vector<KeyLine> &, const cv::Mat &, const std::vector<KeyLine> &, const std::vector<DMatch> &, cv::Mat &, const cv::Scalar &, const cv::Scalar &, const std::vector<char> &, int) /usr/include/opencv2/line_descriptor/descriptor.hpp:1388
	pub fn cv_line_descriptor_drawLineMatches_const_MatR_const_vector_KeyLine_R_const_MatR_const_vector_KeyLine_R_const_vector_DMatch_R_MatR_const_ScalarR_const_ScalarR_const_vector_char_R_int(img1: *const c_void, keylines1: *const c_void, img2: *const c_void, keylines2: *const c_void, matches1to2: *const c_void, out_img: *mut c_void, match_color: *const core::Scalar, single_line_color: *const core::Scalar, matches_mask: *const c_void, flags: i32, ocvrs_return: *mut Result_void);
	// BinaryDescriptor(const BinaryDescriptor::Params &) /usr/include/opencv2/line_descriptor/descriptor.hpp:221
	pub fn cv_line_descriptor_BinaryDescriptor_BinaryDescriptor_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// createBinaryDescriptor() /usr/include/opencv2/line_descriptor/descriptor.hpp:226
	pub fn cv_line_descriptor_BinaryDescriptor_createBinaryDescriptor(ocvrs_return: *mut Result<*mut c_void>);
	// createBinaryDescriptor(cv::line_descriptor::BinaryDescriptor::Params) /usr/include/opencv2/line_descriptor/descriptor.hpp:227
	pub fn cv_line_descriptor_BinaryDescriptor_createBinaryDescriptor_Params(parameters: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getNumOfOctaves() /usr/include/opencv2/line_descriptor/descriptor.hpp:234
	pub fn cv_line_descriptor_BinaryDescriptor_getNumOfOctaves(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// setNumOfOctaves(int) /usr/include/opencv2/line_descriptor/descriptor.hpp:238
	pub fn cv_line_descriptor_BinaryDescriptor_setNumOfOctaves_int(instance: *mut c_void, octaves: i32, ocvrs_return: *mut Result_void);
	// getWidthOfBand() /usr/include/opencv2/line_descriptor/descriptor.hpp:241
	pub fn cv_line_descriptor_BinaryDescriptor_getWidthOfBand(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// setWidthOfBand(int) /usr/include/opencv2/line_descriptor/descriptor.hpp:245
	pub fn cv_line_descriptor_BinaryDescriptor_setWidthOfBand_int(instance: *mut c_void, width: i32, ocvrs_return: *mut Result_void);
	// getReductionRatio() /usr/include/opencv2/line_descriptor/descriptor.hpp:248
	pub fn cv_line_descriptor_BinaryDescriptor_getReductionRatio(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// setReductionRatio(int) /usr/include/opencv2/line_descriptor/descriptor.hpp:252
	pub fn cv_line_descriptor_BinaryDescriptor_setReductionRatio_int(instance: *mut c_void, r_ratio: i32, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &) /usr/include/opencv2/line_descriptor/descriptor.hpp:258
	pub fn cv_line_descriptor_BinaryDescriptor_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/line_descriptor/descriptor.hpp:264
	pub fn cv_line_descriptor_BinaryDescriptor_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// detect(const cv::Mat &, std::vector<KeyLine> &, const cv::Mat &) /usr/include/opencv2/line_descriptor/descriptor.hpp:272
	pub fn cv_line_descriptor_BinaryDescriptor_detect_const_MatR_vector_KeyLine_R_const_MatR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// detect(const std::vector<Mat> &, std::vector<std::vector<KeyLine>> &, const std::vector<Mat> &) /usr/include/opencv2/line_descriptor/descriptor.hpp:280
	pub fn cv_line_descriptor_BinaryDescriptor_detect_const_const_vector_Mat_R_vector_vector_KeyLine__R_const_vector_Mat_R(instance: *const c_void, images: *const c_void, keylines: *mut c_void, masks: *const c_void, ocvrs_return: *mut Result_void);
	// compute(const cv::Mat &, std::vector<KeyLine> &, cv::Mat &, bool) /usr/include/opencv2/line_descriptor/descriptor.hpp:290
	pub fn cv_line_descriptor_BinaryDescriptor_compute_const_const_MatR_vector_KeyLine_R_MatR_bool(instance: *const c_void, image: *const c_void, keylines: *mut c_void, descriptors: *mut c_void, return_float_descr: bool, ocvrs_return: *mut Result_void);
	// compute(const std::vector<Mat> &, std::vector<std::vector<KeyLine>> &, std::vector<Mat> &, bool) /usr/include/opencv2/line_descriptor/descriptor.hpp:299
	pub fn cv_line_descriptor_BinaryDescriptor_compute_const_const_vector_Mat_R_vector_vector_KeyLine__R_vector_Mat_R_bool(instance: *const c_void, images: *const c_void, keylines: *mut c_void, descriptors: *mut c_void, return_float_descr: bool, ocvrs_return: *mut Result_void);
	// descriptorSize() /usr/include/opencv2/line_descriptor/descriptor.hpp:304
	pub fn cv_line_descriptor_BinaryDescriptor_descriptorSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// descriptorType() /usr/include/opencv2/line_descriptor/descriptor.hpp:308
	pub fn cv_line_descriptor_BinaryDescriptor_descriptorType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// defaultNorm() /usr/include/opencv2/line_descriptor/descriptor.hpp:312
	pub fn cv_line_descriptor_BinaryDescriptor_defaultNorm_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// numOfOctave_ /usr/include/opencv2/line_descriptor/descriptor.hpp:195
	pub fn cv_line_descriptor_BinaryDescriptor_Params_getPropNumOfOctave__const(instance: *const c_void) -> i32;
	// numOfOctave_ /usr/include/opencv2/line_descriptor/descriptor.hpp:195
	pub fn cv_line_descriptor_BinaryDescriptor_Params_setPropNumOfOctave__int(instance: *mut c_void, val: i32);
	// widthOfBand_ /usr/include/opencv2/line_descriptor/descriptor.hpp:199
	pub fn cv_line_descriptor_BinaryDescriptor_Params_getPropWidthOfBand__const(instance: *const c_void) -> i32;
	// widthOfBand_ /usr/include/opencv2/line_descriptor/descriptor.hpp:199
	pub fn cv_line_descriptor_BinaryDescriptor_Params_setPropWidthOfBand__int(instance: *mut c_void, val: i32);
	// reductionRatio /usr/include/opencv2/line_descriptor/descriptor.hpp:202
	pub fn cv_line_descriptor_BinaryDescriptor_Params_getPropReductionRatio_const(instance: *const c_void) -> i32;
	// reductionRatio /usr/include/opencv2/line_descriptor/descriptor.hpp:202
	pub fn cv_line_descriptor_BinaryDescriptor_Params_setPropReductionRatio_int(instance: *mut c_void, val: i32);
	// ksize_ /usr/include/opencv2/line_descriptor/descriptor.hpp:204
	pub fn cv_line_descriptor_BinaryDescriptor_Params_getPropKsize__const(instance: *const c_void) -> i32;
	// ksize_ /usr/include/opencv2/line_descriptor/descriptor.hpp:204
	pub fn cv_line_descriptor_BinaryDescriptor_Params_setPropKsize__int(instance: *mut c_void, val: i32);
	// Params() /usr/include/opencv2/line_descriptor/descriptor.hpp:191
	pub fn cv_line_descriptor_BinaryDescriptor_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
	// read(const cv::FileNode &) /usr/include/opencv2/line_descriptor/descriptor.hpp:207
	pub fn cv_line_descriptor_BinaryDescriptor_Params_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/line_descriptor/descriptor.hpp:210
	pub fn cv_line_descriptor_BinaryDescriptor_Params_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// match(const cv::Mat &, const cv::Mat &, std::vector<DMatch> &, const cv::Mat &) /usr/include/opencv2/line_descriptor/descriptor.hpp:1031
	pub fn cv_line_descriptor_BinaryDescriptorMatcher_match_const_const_MatR_const_MatR_vector_DMatch_R_const_MatR(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// match(const cv::Mat &, std::vector<DMatch> &, const std::vector<Mat> &) /usr/include/opencv2/line_descriptor/descriptor.hpp:1040
	pub fn cv_line_descriptor_BinaryDescriptorMatcher_match_const_MatR_vector_DMatch_R_const_vector_Mat_R(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, masks: *const c_void, ocvrs_return: *mut Result_void);
	// knnMatch(const cv::Mat &, const cv::Mat &, std::vector<std::vector<DMatch>> &, int, const cv::Mat &, bool) /usr/include/opencv2/line_descriptor/descriptor.hpp:1053
	pub fn cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_const_MatR_const_MatR_vector_vector_DMatch__R_int_const_MatR_bool(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, k: i32, mask: *const c_void, compact_result: bool, ocvrs_return: *mut Result_void);
	// knnMatch(const cv::Mat &, std::vector<std::vector<DMatch>> &, int, const std::vector<Mat> &, bool) /usr/include/opencv2/line_descriptor/descriptor.hpp:1066
	pub fn cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_MatR_vector_vector_DMatch__R_int_const_vector_Mat_R_bool(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, k: i32, masks: *const c_void, compact_result: bool, ocvrs_return: *mut Result_void);
	// radiusMatch(const cv::Mat &, const cv::Mat &, std::vector<std::vector<DMatch>> &, float, const cv::Mat &, bool) /usr/include/opencv2/line_descriptor/descriptor.hpp:1080
	pub fn cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_const_MatR_const_MatR_vector_vector_DMatch__R_float_const_MatR_bool(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, max_distance: f32, mask: *const c_void, compact_result: bool, ocvrs_return: *mut Result_void);
	// radiusMatch(const cv::Mat &, std::vector<std::vector<DMatch>> &, float, const std::vector<Mat> &, bool) /usr/include/opencv2/line_descriptor/descriptor.hpp:1093
	pub fn cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_MatR_vector_vector_DMatch__R_float_const_vector_Mat_R_bool(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, max_distance: f32, masks: *const c_void, compact_result: bool, ocvrs_return: *mut Result_void);
	// add(const std::vector<Mat> &) /usr/include/opencv2/line_descriptor/descriptor.hpp:1104
	pub fn cv_line_descriptor_BinaryDescriptorMatcher_add_const_vector_Mat_R(instance: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result_void);
	// train() /usr/include/opencv2/line_descriptor/descriptor.hpp:1111
	pub fn cv_line_descriptor_BinaryDescriptorMatcher_train(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// createBinaryDescriptorMatcher() /usr/include/opencv2/line_descriptor/descriptor.hpp:1115
	pub fn cv_line_descriptor_BinaryDescriptorMatcher_createBinaryDescriptorMatcher(ocvrs_return: *mut Result<*mut c_void>);
	// clear() /usr/include/opencv2/line_descriptor/descriptor.hpp:1119
	pub fn cv_line_descriptor_BinaryDescriptorMatcher_clear(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// BinaryDescriptorMatcher() /usr/include/opencv2/line_descriptor/descriptor.hpp:1125
	pub fn cv_line_descriptor_BinaryDescriptorMatcher_BinaryDescriptorMatcher(ocvrs_return: *mut Result<*mut c_void>);
	// getStartPoint() /usr/include/opencv2/line_descriptor/descriptor.hpp:146
	pub fn cv_line_descriptor_KeyLine_getStartPoint_const(instance: *const crate::line_descriptor::KeyLine, ocvrs_return: *mut Result<core::Point2f>);
	// getEndPoint() /usr/include/opencv2/line_descriptor/descriptor.hpp:152
	pub fn cv_line_descriptor_KeyLine_getEndPoint_const(instance: *const crate::line_descriptor::KeyLine, ocvrs_return: *mut Result<core::Point2f>);
	// getStartPointInOctave() /usr/include/opencv2/line_descriptor/descriptor.hpp:158
	pub fn cv_line_descriptor_KeyLine_getStartPointInOctave_const(instance: *const crate::line_descriptor::KeyLine, ocvrs_return: *mut Result<core::Point2f>);
	// getEndPointInOctave() /usr/include/opencv2/line_descriptor/descriptor.hpp:164
	pub fn cv_line_descriptor_KeyLine_getEndPointInOctave_const(instance: *const crate::line_descriptor::KeyLine, ocvrs_return: *mut Result<core::Point2f>);
	// KeyLine() /usr/include/opencv2/line_descriptor/descriptor.hpp:170
	pub fn cv_line_descriptor_KeyLine_KeyLine(ocvrs_return: *mut Result<crate::line_descriptor::KeyLine>);
	// LSDDetector() /usr/include/opencv2/line_descriptor/descriptor.hpp:929
	pub fn cv_line_descriptor_LSDDetector_LSDDetector(ocvrs_return: *mut Result<*mut c_void>);
	// LSDDetector(cv::line_descriptor::LSDParam) /usr/include/opencv2/line_descriptor/descriptor.hpp:934
	pub fn cv_line_descriptor_LSDDetector_LSDDetector_LSDParam(_params: *const crate::line_descriptor::LSDParam, ocvrs_return: *mut Result<*mut c_void>);
	// createLSDDetector() /usr/include/opencv2/line_descriptor/descriptor.hpp:941
	pub fn cv_line_descriptor_LSDDetector_createLSDDetector(ocvrs_return: *mut Result<*mut c_void>);
	// createLSDDetector(cv::line_descriptor::LSDParam) /usr/include/opencv2/line_descriptor/descriptor.hpp:944
	pub fn cv_line_descriptor_LSDDetector_createLSDDetector_LSDParam(params: *const crate::line_descriptor::LSDParam, ocvrs_return: *mut Result<*mut c_void>);
	// detect(const cv::Mat &, std::vector<KeyLine> &, int, int, const cv::Mat &) /usr/include/opencv2/line_descriptor/descriptor.hpp:955
	pub fn cv_line_descriptor_LSDDetector_detect_const_MatR_vector_KeyLine_R_int_int_const_MatR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, scale: i32, num_octaves: i32, mask: *const c_void, ocvrs_return: *mut Result_void);
	// detect(const std::vector<Mat> &, std::vector<std::vector<KeyLine>> &, int, int, const std::vector<Mat> &) /usr/include/opencv2/line_descriptor/descriptor.hpp:964
	pub fn cv_line_descriptor_LSDDetector_detect_const_const_vector_Mat_R_vector_vector_KeyLine__R_int_int_const_vector_Mat_R(instance: *const c_void, images: *const c_void, keylines: *mut c_void, scale: i32, num_octaves: i32, masks: *const c_void, ocvrs_return: *mut Result_void);
	// LSDParam() /usr/include/opencv2/line_descriptor/descriptor.hpp:914
	pub fn cv_line_descriptor_LSDParam_LSDParam(ocvrs_return: *mut Result<crate::line_descriptor::LSDParam>);
}
