#include "ocvrs_common.hpp"
#include <opencv2/line_descriptor.hpp>
#include "line_descriptor_types.hpp"

extern "C" {
	// drawKeylines(const cv::Mat &, const std::vector<KeyLine> &, cv::Mat &, const cv::Scalar &, int) /usr/include/opencv2/line_descriptor/descriptor.hpp:1401
	void cv_line_descriptor_drawKeylines_const_MatR_const_vector_KeyLine_R_MatR_const_ScalarR_int(const cv::Mat* image, const std::vector<cv::line_descriptor::KeyLine>* keylines, cv::Mat* outImage, const cv::Scalar* color, int flags, Result_void* ocvrs_return) {
		try {
			cv::line_descriptor::drawKeylines(*image, *keylines, *outImage, *color, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// drawLineMatches(const cv::Mat &, const std::vector<KeyLine> &, const cv::Mat &, const std::vector<KeyLine> &, const std::vector<DMatch> &, cv::Mat &, const cv::Scalar &, const cv::Scalar &, const std::vector<char> &, int) /usr/include/opencv2/line_descriptor/descriptor.hpp:1388
	void cv_line_descriptor_drawLineMatches_const_MatR_const_vector_KeyLine_R_const_MatR_const_vector_KeyLine_R_const_vector_DMatch_R_MatR_const_ScalarR_const_ScalarR_const_vector_char_R_int(const cv::Mat* img1, const std::vector<cv::line_descriptor::KeyLine>* keylines1, const cv::Mat* img2, const std::vector<cv::line_descriptor::KeyLine>* keylines2, const std::vector<cv::DMatch>* matches1to2, cv::Mat* outImg, const cv::Scalar* matchColor, const cv::Scalar* singleLineColor, const std::vector<char>* matchesMask, int flags, Result_void* ocvrs_return) {
		try {
			cv::line_descriptor::drawLineMatches(*img1, *keylines1, *img2, *keylines2, *matches1to2, *outImg, *matchColor, *singleLineColor, *matchesMask, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::Algorithm* cv_BinaryDescriptor_to_Algorithm(cv::line_descriptor::BinaryDescriptor* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_BinaryDescriptor_delete(cv::line_descriptor::BinaryDescriptor* instance) {
		delete instance;
	}
	// BinaryDescriptor(const BinaryDescriptor::Params &) /usr/include/opencv2/line_descriptor/descriptor.hpp:221
	void cv_line_descriptor_BinaryDescriptor_BinaryDescriptor_const_ParamsR(const cv::line_descriptor::BinaryDescriptor::Params* parameters, Result<cv::line_descriptor::BinaryDescriptor*>* ocvrs_return) {
		try {
			cv::line_descriptor::BinaryDescriptor* ret = new cv::line_descriptor::BinaryDescriptor(*parameters);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::line_descriptor::BinaryDescriptor*>))
	}
	
	// createBinaryDescriptor() /usr/include/opencv2/line_descriptor/descriptor.hpp:226
	void cv_line_descriptor_BinaryDescriptor_createBinaryDescriptor(Result<cv::Ptr<cv::line_descriptor::BinaryDescriptor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::line_descriptor::BinaryDescriptor> ret = cv::line_descriptor::BinaryDescriptor::createBinaryDescriptor();
			Ok(new cv::Ptr<cv::line_descriptor::BinaryDescriptor>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::line_descriptor::BinaryDescriptor>*>))
	}
	
	// createBinaryDescriptor(cv::line_descriptor::BinaryDescriptor::Params) /usr/include/opencv2/line_descriptor/descriptor.hpp:227
	void cv_line_descriptor_BinaryDescriptor_createBinaryDescriptor_Params(cv::line_descriptor::BinaryDescriptor::Params* parameters, Result<cv::Ptr<cv::line_descriptor::BinaryDescriptor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::line_descriptor::BinaryDescriptor> ret = cv::line_descriptor::BinaryDescriptor::createBinaryDescriptor(*parameters);
			Ok(new cv::Ptr<cv::line_descriptor::BinaryDescriptor>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::line_descriptor::BinaryDescriptor>*>))
	}
	
	// getNumOfOctaves() /usr/include/opencv2/line_descriptor/descriptor.hpp:234
	void cv_line_descriptor_BinaryDescriptor_getNumOfOctaves(cv::line_descriptor::BinaryDescriptor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumOfOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setNumOfOctaves(int) /usr/include/opencv2/line_descriptor/descriptor.hpp:238
	void cv_line_descriptor_BinaryDescriptor_setNumOfOctaves_int(cv::line_descriptor::BinaryDescriptor* instance, int octaves, Result_void* ocvrs_return) {
		try {
			instance->setNumOfOctaves(octaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWidthOfBand() /usr/include/opencv2/line_descriptor/descriptor.hpp:241
	void cv_line_descriptor_BinaryDescriptor_getWidthOfBand(cv::line_descriptor::BinaryDescriptor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWidthOfBand();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setWidthOfBand(int) /usr/include/opencv2/line_descriptor/descriptor.hpp:245
	void cv_line_descriptor_BinaryDescriptor_setWidthOfBand_int(cv::line_descriptor::BinaryDescriptor* instance, int width, Result_void* ocvrs_return) {
		try {
			instance->setWidthOfBand(width);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getReductionRatio() /usr/include/opencv2/line_descriptor/descriptor.hpp:248
	void cv_line_descriptor_BinaryDescriptor_getReductionRatio(cv::line_descriptor::BinaryDescriptor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getReductionRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setReductionRatio(int) /usr/include/opencv2/line_descriptor/descriptor.hpp:252
	void cv_line_descriptor_BinaryDescriptor_setReductionRatio_int(cv::line_descriptor::BinaryDescriptor* instance, int rRatio, Result_void* ocvrs_return) {
		try {
			instance->setReductionRatio(rRatio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/line_descriptor/descriptor.hpp:258
	void cv_line_descriptor_BinaryDescriptor_read_const_FileNodeR(cv::line_descriptor::BinaryDescriptor* instance, const cv::FileNode* fn, Result_void* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/line_descriptor/descriptor.hpp:264
	void cv_line_descriptor_BinaryDescriptor_write_const_FileStorageR(const cv::line_descriptor::BinaryDescriptor* instance, cv::FileStorage* fs, Result_void* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detect(const cv::Mat &, std::vector<KeyLine> &, const cv::Mat &) /usr/include/opencv2/line_descriptor/descriptor.hpp:272
	void cv_line_descriptor_BinaryDescriptor_detect_const_MatR_vector_KeyLine_R_const_MatR(cv::line_descriptor::BinaryDescriptor* instance, const cv::Mat* image, std::vector<cv::line_descriptor::KeyLine>* keypoints, const cv::Mat* mask, Result_void* ocvrs_return) {
		try {
			instance->detect(*image, *keypoints, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detect(const std::vector<Mat> &, std::vector<std::vector<KeyLine>> &, const std::vector<Mat> &) /usr/include/opencv2/line_descriptor/descriptor.hpp:280
	void cv_line_descriptor_BinaryDescriptor_detect_const_const_vector_Mat_R_vector_vector_KeyLine__R_const_vector_Mat_R(const cv::line_descriptor::BinaryDescriptor* instance, const std::vector<cv::Mat>* images, std::vector<std::vector<cv::line_descriptor::KeyLine>>* keylines, const std::vector<cv::Mat>* masks, Result_void* ocvrs_return) {
		try {
			instance->detect(*images, *keylines, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// compute(const cv::Mat &, std::vector<KeyLine> &, cv::Mat &, bool) /usr/include/opencv2/line_descriptor/descriptor.hpp:290
	void cv_line_descriptor_BinaryDescriptor_compute_const_const_MatR_vector_KeyLine_R_MatR_bool(const cv::line_descriptor::BinaryDescriptor* instance, const cv::Mat* image, std::vector<cv::line_descriptor::KeyLine>* keylines, cv::Mat* descriptors, bool returnFloatDescr, Result_void* ocvrs_return) {
		try {
			instance->compute(*image, *keylines, *descriptors, returnFloatDescr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// compute(const std::vector<Mat> &, std::vector<std::vector<KeyLine>> &, std::vector<Mat> &, bool) /usr/include/opencv2/line_descriptor/descriptor.hpp:299
	void cv_line_descriptor_BinaryDescriptor_compute_const_const_vector_Mat_R_vector_vector_KeyLine__R_vector_Mat_R_bool(const cv::line_descriptor::BinaryDescriptor* instance, const std::vector<cv::Mat>* images, std::vector<std::vector<cv::line_descriptor::KeyLine>>* keylines, std::vector<cv::Mat>* descriptors, bool returnFloatDescr, Result_void* ocvrs_return) {
		try {
			instance->compute(*images, *keylines, *descriptors, returnFloatDescr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// descriptorSize() /usr/include/opencv2/line_descriptor/descriptor.hpp:304
	void cv_line_descriptor_BinaryDescriptor_descriptorSize_const(const cv::line_descriptor::BinaryDescriptor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// descriptorType() /usr/include/opencv2/line_descriptor/descriptor.hpp:308
	void cv_line_descriptor_BinaryDescriptor_descriptorType_const(const cv::line_descriptor::BinaryDescriptor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// defaultNorm() /usr/include/opencv2/line_descriptor/descriptor.hpp:312
	void cv_line_descriptor_BinaryDescriptor_defaultNorm_const(const cv::line_descriptor::BinaryDescriptor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->defaultNorm();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// numOfOctave_ /usr/include/opencv2/line_descriptor/descriptor.hpp:195
	int cv_line_descriptor_BinaryDescriptor_Params_getPropNumOfOctave__const(const cv::line_descriptor::BinaryDescriptor::Params* instance) {
			int ret = instance->numOfOctave_;
			return ret;
	}
	
	// numOfOctave_ /usr/include/opencv2/line_descriptor/descriptor.hpp:195
	void cv_line_descriptor_BinaryDescriptor_Params_setPropNumOfOctave__int(cv::line_descriptor::BinaryDescriptor::Params* instance, int val) {
			instance->numOfOctave_ = val;
	}
	
	// widthOfBand_ /usr/include/opencv2/line_descriptor/descriptor.hpp:199
	int cv_line_descriptor_BinaryDescriptor_Params_getPropWidthOfBand__const(const cv::line_descriptor::BinaryDescriptor::Params* instance) {
			int ret = instance->widthOfBand_;
			return ret;
	}
	
	// widthOfBand_ /usr/include/opencv2/line_descriptor/descriptor.hpp:199
	void cv_line_descriptor_BinaryDescriptor_Params_setPropWidthOfBand__int(cv::line_descriptor::BinaryDescriptor::Params* instance, int val) {
			instance->widthOfBand_ = val;
	}
	
	// reductionRatio /usr/include/opencv2/line_descriptor/descriptor.hpp:202
	int cv_line_descriptor_BinaryDescriptor_Params_getPropReductionRatio_const(const cv::line_descriptor::BinaryDescriptor::Params* instance) {
			int ret = instance->reductionRatio;
			return ret;
	}
	
	// reductionRatio /usr/include/opencv2/line_descriptor/descriptor.hpp:202
	void cv_line_descriptor_BinaryDescriptor_Params_setPropReductionRatio_int(cv::line_descriptor::BinaryDescriptor::Params* instance, int val) {
			instance->reductionRatio = val;
	}
	
	// ksize_ /usr/include/opencv2/line_descriptor/descriptor.hpp:204
	int cv_line_descriptor_BinaryDescriptor_Params_getPropKsize__const(const cv::line_descriptor::BinaryDescriptor::Params* instance) {
			int ret = instance->ksize_;
			return ret;
	}
	
	// ksize_ /usr/include/opencv2/line_descriptor/descriptor.hpp:204
	void cv_line_descriptor_BinaryDescriptor_Params_setPropKsize__int(cv::line_descriptor::BinaryDescriptor::Params* instance, int val) {
			instance->ksize_ = val;
	}
	
	void cv_BinaryDescriptor_Params_delete(cv::line_descriptor::BinaryDescriptor::Params* instance) {
		delete instance;
	}
	// Params() /usr/include/opencv2/line_descriptor/descriptor.hpp:191
	void cv_line_descriptor_BinaryDescriptor_Params_Params(Result<cv::line_descriptor::BinaryDescriptor::Params*>* ocvrs_return) {
		try {
			cv::line_descriptor::BinaryDescriptor::Params* ret = new cv::line_descriptor::BinaryDescriptor::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::line_descriptor::BinaryDescriptor::Params*>))
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/line_descriptor/descriptor.hpp:207
	void cv_line_descriptor_BinaryDescriptor_Params_read_const_FileNodeR(cv::line_descriptor::BinaryDescriptor::Params* instance, const cv::FileNode* fn, Result_void* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/line_descriptor/descriptor.hpp:210
	void cv_line_descriptor_BinaryDescriptor_Params_write_const_FileStorageR(const cv::line_descriptor::BinaryDescriptor::Params* instance, cv::FileStorage* fs, Result_void* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::Algorithm* cv_BinaryDescriptorMatcher_to_Algorithm(cv::line_descriptor::BinaryDescriptorMatcher* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_BinaryDescriptorMatcher_delete(cv::line_descriptor::BinaryDescriptorMatcher* instance) {
		delete instance;
	}
	// match(const cv::Mat &, const cv::Mat &, std::vector<DMatch> &, const cv::Mat &) /usr/include/opencv2/line_descriptor/descriptor.hpp:1031
	void cv_line_descriptor_BinaryDescriptorMatcher_match_const_const_MatR_const_MatR_vector_DMatch_R_const_MatR(const cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, const cv::Mat* trainDescriptors, std::vector<cv::DMatch>* matches, const cv::Mat* mask, Result_void* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *trainDescriptors, *matches, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// match(const cv::Mat &, std::vector<DMatch> &, const std::vector<Mat> &) /usr/include/opencv2/line_descriptor/descriptor.hpp:1040
	void cv_line_descriptor_BinaryDescriptorMatcher_match_const_MatR_vector_DMatch_R_const_vector_Mat_R(cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, std::vector<cv::DMatch>* matches, const std::vector<cv::Mat>* masks, Result_void* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *matches, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// knnMatch(const cv::Mat &, const cv::Mat &, std::vector<std::vector<DMatch>> &, int, const cv::Mat &, bool) /usr/include/opencv2/line_descriptor/descriptor.hpp:1053
	void cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_const_MatR_const_MatR_vector_vector_DMatch__R_int_const_MatR_bool(const cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, const cv::Mat* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, const cv::Mat* mask, bool compactResult, Result_void* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *trainDescriptors, *matches, k, *mask, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// knnMatch(const cv::Mat &, std::vector<std::vector<DMatch>> &, int, const std::vector<Mat> &, bool) /usr/include/opencv2/line_descriptor/descriptor.hpp:1066
	void cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_MatR_vector_vector_DMatch__R_int_const_vector_Mat_R_bool(cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, const std::vector<cv::Mat>* masks, bool compactResult, Result_void* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *matches, k, *masks, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// radiusMatch(const cv::Mat &, const cv::Mat &, std::vector<std::vector<DMatch>> &, float, const cv::Mat &, bool) /usr/include/opencv2/line_descriptor/descriptor.hpp:1080
	void cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_const_MatR_const_MatR_vector_vector_DMatch__R_float_const_MatR_bool(const cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, const cv::Mat* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, const cv::Mat* mask, bool compactResult, Result_void* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *trainDescriptors, *matches, maxDistance, *mask, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// radiusMatch(const cv::Mat &, std::vector<std::vector<DMatch>> &, float, const std::vector<Mat> &, bool) /usr/include/opencv2/line_descriptor/descriptor.hpp:1093
	void cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_MatR_vector_vector_DMatch__R_float_const_vector_Mat_R_bool(cv::line_descriptor::BinaryDescriptorMatcher* instance, const cv::Mat* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, const std::vector<cv::Mat>* masks, bool compactResult, Result_void* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *matches, maxDistance, *masks, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// add(const std::vector<Mat> &) /usr/include/opencv2/line_descriptor/descriptor.hpp:1104
	void cv_line_descriptor_BinaryDescriptorMatcher_add_const_vector_Mat_R(cv::line_descriptor::BinaryDescriptorMatcher* instance, const std::vector<cv::Mat>* descriptors, Result_void* ocvrs_return) {
		try {
			instance->add(*descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// train() /usr/include/opencv2/line_descriptor/descriptor.hpp:1111
	void cv_line_descriptor_BinaryDescriptorMatcher_train(cv::line_descriptor::BinaryDescriptorMatcher* instance, Result_void* ocvrs_return) {
		try {
			instance->train();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// createBinaryDescriptorMatcher() /usr/include/opencv2/line_descriptor/descriptor.hpp:1115
	void cv_line_descriptor_BinaryDescriptorMatcher_createBinaryDescriptorMatcher(Result<cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher> ret = cv::line_descriptor::BinaryDescriptorMatcher::createBinaryDescriptorMatcher();
			Ok(new cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>*>))
	}
	
	// clear() /usr/include/opencv2/line_descriptor/descriptor.hpp:1119
	void cv_line_descriptor_BinaryDescriptorMatcher_clear(cv::line_descriptor::BinaryDescriptorMatcher* instance, Result_void* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// BinaryDescriptorMatcher() /usr/include/opencv2/line_descriptor/descriptor.hpp:1125
	void cv_line_descriptor_BinaryDescriptorMatcher_BinaryDescriptorMatcher(Result<cv::line_descriptor::BinaryDescriptorMatcher*>* ocvrs_return) {
		try {
			cv::line_descriptor::BinaryDescriptorMatcher* ret = new cv::line_descriptor::BinaryDescriptorMatcher();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::line_descriptor::BinaryDescriptorMatcher*>))
	}
	
	// getStartPoint() /usr/include/opencv2/line_descriptor/descriptor.hpp:146
	void cv_line_descriptor_KeyLine_getStartPoint_const(const cv::line_descriptor::KeyLine instance, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance.getStartPoint();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	// getEndPoint() /usr/include/opencv2/line_descriptor/descriptor.hpp:152
	void cv_line_descriptor_KeyLine_getEndPoint_const(const cv::line_descriptor::KeyLine instance, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance.getEndPoint();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	// getStartPointInOctave() /usr/include/opencv2/line_descriptor/descriptor.hpp:158
	void cv_line_descriptor_KeyLine_getStartPointInOctave_const(const cv::line_descriptor::KeyLine instance, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance.getStartPointInOctave();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	// getEndPointInOctave() /usr/include/opencv2/line_descriptor/descriptor.hpp:164
	void cv_line_descriptor_KeyLine_getEndPointInOctave_const(const cv::line_descriptor::KeyLine instance, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance.getEndPointInOctave();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	// KeyLine() /usr/include/opencv2/line_descriptor/descriptor.hpp:170
	void cv_line_descriptor_KeyLine_KeyLine(Result<cv::line_descriptor::KeyLine>* ocvrs_return) {
		try {
			cv::line_descriptor::KeyLine ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::line_descriptor::KeyLine>))
	}
	
	cv::Algorithm* cv_LSDDetector_to_Algorithm(cv::line_descriptor::LSDDetector* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_LSDDetector_delete(cv::line_descriptor::LSDDetector* instance) {
		delete instance;
	}
	// LSDDetector() /usr/include/opencv2/line_descriptor/descriptor.hpp:929
	void cv_line_descriptor_LSDDetector_LSDDetector(Result<cv::line_descriptor::LSDDetector*>* ocvrs_return) {
		try {
			cv::line_descriptor::LSDDetector* ret = new cv::line_descriptor::LSDDetector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::line_descriptor::LSDDetector*>))
	}
	
	// LSDDetector(cv::line_descriptor::LSDParam) /usr/include/opencv2/line_descriptor/descriptor.hpp:934
	void cv_line_descriptor_LSDDetector_LSDDetector_LSDParam(cv::line_descriptor::LSDParam* _params, Result<cv::line_descriptor::LSDDetector*>* ocvrs_return) {
		try {
			cv::line_descriptor::LSDDetector* ret = new cv::line_descriptor::LSDDetector(*_params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::line_descriptor::LSDDetector*>))
	}
	
	// createLSDDetector() /usr/include/opencv2/line_descriptor/descriptor.hpp:941
	void cv_line_descriptor_LSDDetector_createLSDDetector(Result<cv::Ptr<cv::line_descriptor::LSDDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::line_descriptor::LSDDetector> ret = cv::line_descriptor::LSDDetector::createLSDDetector();
			Ok(new cv::Ptr<cv::line_descriptor::LSDDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::line_descriptor::LSDDetector>*>))
	}
	
	// createLSDDetector(cv::line_descriptor::LSDParam) /usr/include/opencv2/line_descriptor/descriptor.hpp:944
	void cv_line_descriptor_LSDDetector_createLSDDetector_LSDParam(cv::line_descriptor::LSDParam* params, Result<cv::Ptr<cv::line_descriptor::LSDDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::line_descriptor::LSDDetector> ret = cv::line_descriptor::LSDDetector::createLSDDetector(*params);
			Ok(new cv::Ptr<cv::line_descriptor::LSDDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::line_descriptor::LSDDetector>*>))
	}
	
	// detect(const cv::Mat &, std::vector<KeyLine> &, int, int, const cv::Mat &) /usr/include/opencv2/line_descriptor/descriptor.hpp:955
	void cv_line_descriptor_LSDDetector_detect_const_MatR_vector_KeyLine_R_int_int_const_MatR(cv::line_descriptor::LSDDetector* instance, const cv::Mat* image, std::vector<cv::line_descriptor::KeyLine>* keypoints, int scale, int numOctaves, const cv::Mat* mask, Result_void* ocvrs_return) {
		try {
			instance->detect(*image, *keypoints, scale, numOctaves, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detect(const std::vector<Mat> &, std::vector<std::vector<KeyLine>> &, int, int, const std::vector<Mat> &) /usr/include/opencv2/line_descriptor/descriptor.hpp:964
	void cv_line_descriptor_LSDDetector_detect_const_const_vector_Mat_R_vector_vector_KeyLine__R_int_int_const_vector_Mat_R(const cv::line_descriptor::LSDDetector* instance, const std::vector<cv::Mat>* images, std::vector<std::vector<cv::line_descriptor::KeyLine>>* keylines, int scale, int numOctaves, const std::vector<cv::Mat>* masks, Result_void* ocvrs_return) {
		try {
			instance->detect(*images, *keylines, scale, numOctaves, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// LSDParam() /usr/include/opencv2/line_descriptor/descriptor.hpp:914
	void cv_line_descriptor_LSDParam_LSDParam(Result<cv::line_descriptor::LSDParam>* ocvrs_return) {
		try {
			cv::line_descriptor::LSDParam ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::line_descriptor::LSDParam>))
	}
	
}
