#include "ocvrs_common.hpp"
#include <opencv2/features2d.hpp>
#include "features2d_types.hpp"

extern "C" {
	// AGAST(cv::InputArray, std::vector<KeyPoint> &, int, bool) /usr/include/opencv2/features2d.hpp:611
	void cv_AGAST_const__InputArrayR_vector_KeyPoint_R_int_bool(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, Result_void* ocvrs_return) {
		try {
			cv::AGAST(*image, *keypoints, threshold, nonmaxSuppression);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// AGAST(cv::InputArray, std::vector<KeyPoint> &, int, bool, AgastFeatureDetector::DetectorType) /usr/include/opencv2/features2d.hpp:632
	void cv_AGAST_const__InputArrayR_vector_KeyPoint_R_int_bool_DetectorType(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, cv::AgastFeatureDetector::DetectorType type, Result_void* ocvrs_return) {
		try {
			cv::AGAST(*image, *keypoints, threshold, nonmaxSuppression, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// FAST(cv::InputArray, std::vector<KeyPoint> &, int, bool) /usr/include/opencv2/features2d.hpp:551
	void cv_FAST_const__InputArrayR_vector_KeyPoint_R_int_bool(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, Result_void* ocvrs_return) {
		try {
			cv::FAST(*image, *keypoints, threshold, nonmaxSuppression);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// FAST(cv::InputArray, std::vector<KeyPoint> &, int, bool, FastFeatureDetector::DetectorType) /usr/include/opencv2/features2d.hpp:572
	void cv_FAST_const__InputArrayR_vector_KeyPoint_R_int_bool_DetectorType(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, cv::FastFeatureDetector::DetectorType type, Result_void* ocvrs_return) {
		try {
			cv::FAST(*image, *keypoints, threshold, nonmaxSuppression, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// computeRecallPrecisionCurve(const std::vector<std::vector<DMatch>> &, const std::vector<std::vector<uchar>> &, std::vector<Point2f> &) /usr/include/opencv2/features2d.hpp:1364
	void cv_computeRecallPrecisionCurve_const_vector_vector_DMatch__R_const_vector_vector_unsigned_char__R_vector_Point2f_R(const std::vector<std::vector<cv::DMatch>>* matches1to2, const std::vector<std::vector<unsigned char>>* correctMatches1to2Mask, std::vector<cv::Point2f>* recallPrecisionCurve, Result_void* ocvrs_return) {
		try {
			cv::computeRecallPrecisionCurve(*matches1to2, *correctMatches1to2Mask, *recallPrecisionCurve);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// drawKeypoints(cv::InputArray, const std::vector<KeyPoint> &, cv::InputOutputArray, const cv::Scalar &, cv::DrawMatchesFlags) /usr/include/opencv2/features2d.hpp:1308
	void cv_drawKeypoints_const__InputArrayR_const_vector_KeyPoint_R_const__InputOutputArrayR_const_ScalarR_DrawMatchesFlags(const cv::_InputArray* image, const std::vector<cv::KeyPoint>* keypoints, const cv::_InputOutputArray* outImage, const cv::Scalar* color, cv::DrawMatchesFlags flags, Result_void* ocvrs_return) {
		try {
			cv::drawKeypoints(*image, *keypoints, *outImage, *color, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// drawMatches(cv::InputArray, const std::vector<KeyPoint> &, cv::InputArray, const std::vector<KeyPoint> &, const std::vector<DMatch> &, cv::InputOutputArray, const cv::Scalar &, const cv::Scalar &, const std::vector<char> &, cv::DrawMatchesFlags) /usr/include/opencv2/features2d.hpp:1333
	void cv_drawMatches_const__InputArrayR_const_vector_KeyPoint_R_const__InputArrayR_const_vector_KeyPoint_R_const_vector_DMatch_R_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vector_char_R_DrawMatchesFlags(const cv::_InputArray* img1, const std::vector<cv::KeyPoint>* keypoints1, const cv::_InputArray* img2, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<cv::DMatch>* matches1to2, const cv::_InputOutputArray* outImg, const cv::Scalar* matchColor, const cv::Scalar* singlePointColor, const std::vector<char>* matchesMask, cv::DrawMatchesFlags flags, Result_void* ocvrs_return) {
		try {
			cv::drawMatches(*img1, *keypoints1, *img2, *keypoints2, *matches1to2, *outImg, *matchColor, *singlePointColor, *matchesMask, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// drawMatches(cv::InputArray, const std::vector<KeyPoint> &, cv::InputArray, const std::vector<KeyPoint> &, const std::vector<DMatch> &, cv::InputOutputArray, const int, const cv::Scalar &, const cv::Scalar &, const std::vector<char> &, cv::DrawMatchesFlags) /usr/include/opencv2/features2d.hpp:1340
	void cv_drawMatches_const__InputArrayR_const_vector_KeyPoint_R_const__InputArrayR_const_vector_KeyPoint_R_const_vector_DMatch_R_const__InputOutputArrayR_const_int_const_ScalarR_const_ScalarR_const_vector_char_R_DrawMatchesFlags(const cv::_InputArray* img1, const std::vector<cv::KeyPoint>* keypoints1, const cv::_InputArray* img2, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<cv::DMatch>* matches1to2, const cv::_InputOutputArray* outImg, const int matchesThickness, const cv::Scalar* matchColor, const cv::Scalar* singlePointColor, const std::vector<char>* matchesMask, cv::DrawMatchesFlags flags, Result_void* ocvrs_return) {
		try {
			cv::drawMatches(*img1, *keypoints1, *img2, *keypoints2, *matches1to2, *outImg, matchesThickness, *matchColor, *singlePointColor, *matchesMask, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// drawMatches(cv::InputArray, const std::vector<KeyPoint> &, cv::InputArray, const std::vector<KeyPoint> &, const std::vector<std::vector<DMatch>> &, cv::InputOutputArray, const cv::Scalar &, const cv::Scalar &, const std::vector<std::vector<char>> &, cv::DrawMatchesFlags) /usr/include/opencv2/features2d.hpp:1347
	void cv_drawMatches_const__InputArrayR_const_vector_KeyPoint_R_const__InputArrayR_const_vector_KeyPoint_R_const_vector_vector_DMatch__R_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vector_vector_char__R_DrawMatchesFlags(const cv::_InputArray* img1, const std::vector<cv::KeyPoint>* keypoints1, const cv::_InputArray* img2, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<std::vector<cv::DMatch>>* matches1to2, const cv::_InputOutputArray* outImg, const cv::Scalar* matchColor, const cv::Scalar* singlePointColor, const std::vector<std::vector<char>>* matchesMask, cv::DrawMatchesFlags flags, Result_void* ocvrs_return) {
		try {
			cv::drawMatches(*img1, *keypoints1, *img2, *keypoints2, *matches1to2, *outImg, *matchColor, *singlePointColor, *matchesMask, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// evaluateFeatureDetector(const cv::Mat &, const cv::Mat &, const cv::Mat &, std::vector<KeyPoint> *, std::vector<KeyPoint> *, float &, int &, const Ptr<cv::FeatureDetector> &) /usr/include/opencv2/features2d.hpp:1359
	void cv_evaluateFeatureDetector_const_MatR_const_MatR_const_MatR_vector_KeyPoint_X_vector_KeyPoint_X_floatR_intR_const_Ptr_Feature2D_R(const cv::Mat* img1, const cv::Mat* img2, const cv::Mat* H1to2, std::vector<cv::KeyPoint>* keypoints1, std::vector<cv::KeyPoint>* keypoints2, float* repeatability, int* correspCount, const cv::Ptr<cv::Feature2D>* fdetector, Result_void* ocvrs_return) {
		try {
			cv::evaluateFeatureDetector(*img1, *img2, *H1to2, keypoints1, keypoints2, *repeatability, *correspCount, *fdetector);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNearestPoint(const std::vector<Point2f> &, float) /usr/include/opencv2/features2d.hpp:1369
	void cv_getNearestPoint_const_vector_Point2f_R_float(const std::vector<cv::Point2f>* recallPrecisionCurve, float l_precision, Result<int>* ocvrs_return) {
		try {
			int ret = cv::getNearestPoint(*recallPrecisionCurve, l_precision);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getRecall(const std::vector<Point2f> &, float) /usr/include/opencv2/features2d.hpp:1368
	void cv_getRecall_const_vector_Point2f_R_float(const std::vector<cv::Point2f>* recallPrecisionCurve, float l_precision, Result<float>* ocvrs_return) {
		try {
			float ret = cv::getRecall(*recallPrecisionCurve, l_precision);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// create(AKAZE::DescriptorType, int, int, float, int, int, KAZE::DiffusivityType) /usr/include/opencv2/features2d.hpp:828
	void cv_AKAZE_create_DescriptorType_int_int_float_int_int_DiffusivityType(cv::AKAZE::DescriptorType descriptor_type, int descriptor_size, int descriptor_channels, float threshold, int nOctaves, int nOctaveLayers, cv::KAZE::DiffusivityType diffusivity, Result<cv::Ptr<cv::AKAZE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::AKAZE> ret = cv::AKAZE::create(descriptor_type, descriptor_size, descriptor_channels, threshold, nOctaves, nOctaveLayers, diffusivity);
			Ok(new cv::Ptr<cv::AKAZE>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::AKAZE>*>))
	}
	
	// setDescriptorType(AKAZE::DescriptorType) /usr/include/opencv2/features2d.hpp:833
	void cv_AKAZE_setDescriptorType_DescriptorType(cv::AKAZE* instance, cv::AKAZE::DescriptorType dtype, Result_void* ocvrs_return) {
		try {
			instance->setDescriptorType(dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDescriptorType() /usr/include/opencv2/features2d.hpp:834
	void cv_AKAZE_getDescriptorType_const(const cv::AKAZE* instance, Result<cv::AKAZE::DescriptorType>* ocvrs_return) {
		try {
			cv::AKAZE::DescriptorType ret = instance->getDescriptorType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::AKAZE::DescriptorType>))
	}
	
	// setDescriptorSize(int) /usr/include/opencv2/features2d.hpp:836
	void cv_AKAZE_setDescriptorSize_int(cv::AKAZE* instance, int dsize, Result_void* ocvrs_return) {
		try {
			instance->setDescriptorSize(dsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDescriptorSize() /usr/include/opencv2/features2d.hpp:837
	void cv_AKAZE_getDescriptorSize_const(const cv::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDescriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setDescriptorChannels(int) /usr/include/opencv2/features2d.hpp:839
	void cv_AKAZE_setDescriptorChannels_int(cv::AKAZE* instance, int dch, Result_void* ocvrs_return) {
		try {
			instance->setDescriptorChannels(dch);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDescriptorChannels() /usr/include/opencv2/features2d.hpp:840
	void cv_AKAZE_getDescriptorChannels_const(const cv::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDescriptorChannels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setThreshold(double) /usr/include/opencv2/features2d.hpp:842
	void cv_AKAZE_setThreshold_double(cv::AKAZE* instance, double threshold, Result_void* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getThreshold() /usr/include/opencv2/features2d.hpp:843
	void cv_AKAZE_getThreshold_const(const cv::AKAZE* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setNOctaves(int) /usr/include/opencv2/features2d.hpp:845
	void cv_AKAZE_setNOctaves_int(cv::AKAZE* instance, int octaves, Result_void* ocvrs_return) {
		try {
			instance->setNOctaves(octaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNOctaves() /usr/include/opencv2/features2d.hpp:846
	void cv_AKAZE_getNOctaves_const(const cv::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setNOctaveLayers(int) /usr/include/opencv2/features2d.hpp:848
	void cv_AKAZE_setNOctaveLayers_int(cv::AKAZE* instance, int octaveLayers, Result_void* ocvrs_return) {
		try {
			instance->setNOctaveLayers(octaveLayers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNOctaveLayers() /usr/include/opencv2/features2d.hpp:849
	void cv_AKAZE_getNOctaveLayers_const(const cv::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaveLayers();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setDiffusivity(KAZE::DiffusivityType) /usr/include/opencv2/features2d.hpp:851
	void cv_AKAZE_setDiffusivity_DiffusivityType(cv::AKAZE* instance, cv::KAZE::DiffusivityType diff, Result_void* ocvrs_return) {
		try {
			instance->setDiffusivity(diff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDiffusivity() /usr/include/opencv2/features2d.hpp:852
	void cv_AKAZE_getDiffusivity_const(const cv::AKAZE* instance, Result<cv::KAZE::DiffusivityType>* ocvrs_return) {
		try {
			cv::KAZE::DiffusivityType ret = instance->getDiffusivity();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::KAZE::DiffusivityType>))
	}
	
	// getDefaultName() /usr/include/opencv2/features2d.hpp:853
	void cv_AKAZE_getDefaultName_const(const cv::AKAZE* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// create(const Ptr<cv::Feature2D> &, int, int, float, float) /usr/include/opencv2/features2d.hpp:244
	void cv_AffineFeature_create_const_Ptr_Feature2D_R_int_int_float_float(const cv::Ptr<cv::Feature2D>* backend, int maxTilt, int minTilt, float tiltStep, float rotateStepBase, Result<cv::Ptr<cv::AffineFeature>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::AffineFeature> ret = cv::AffineFeature::create(*backend, maxTilt, minTilt, tiltStep, rotateStepBase);
			Ok(new cv::Ptr<cv::AffineFeature>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::AffineFeature>*>))
	}
	
	// setViewParams(const std::vector<float> &, const std::vector<float> &) /usr/include/opencv2/features2d.hpp:247
	void cv_AffineFeature_setViewParams_const_vector_float_R_const_vector_float_R(cv::AffineFeature* instance, const std::vector<float>* tilts, const std::vector<float>* rolls, Result_void* ocvrs_return) {
		try {
			instance->setViewParams(*tilts, *rolls);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getViewParams(std::vector<float> &, std::vector<float> &) /usr/include/opencv2/features2d.hpp:248
	void cv_AffineFeature_getViewParams_const_vector_float_R_vector_float_R(const cv::AffineFeature* instance, std::vector<float>* tilts, std::vector<float>* rolls, Result_void* ocvrs_return) {
		try {
			instance->getViewParams(*tilts, *rolls);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDefaultName() /usr/include/opencv2/features2d.hpp:249
	void cv_AffineFeature_getDefaultName_const(const cv::AffineFeature* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// create(int, bool, AgastFeatureDetector::DetectorType) /usr/include/opencv2/features2d.hpp:595
	void cv_AgastFeatureDetector_create_int_bool_DetectorType(int threshold, bool nonmaxSuppression, cv::AgastFeatureDetector::DetectorType type, Result<cv::Ptr<cv::AgastFeatureDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::AgastFeatureDetector> ret = cv::AgastFeatureDetector::create(threshold, nonmaxSuppression, type);
			Ok(new cv::Ptr<cv::AgastFeatureDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::AgastFeatureDetector>*>))
	}
	
	// setThreshold(int) /usr/include/opencv2/features2d.hpp:599
	void cv_AgastFeatureDetector_setThreshold_int(cv::AgastFeatureDetector* instance, int threshold, Result_void* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getThreshold() /usr/include/opencv2/features2d.hpp:600
	void cv_AgastFeatureDetector_getThreshold_const(const cv::AgastFeatureDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setNonmaxSuppression(bool) /usr/include/opencv2/features2d.hpp:602
	void cv_AgastFeatureDetector_setNonmaxSuppression_bool(cv::AgastFeatureDetector* instance, bool f, Result_void* ocvrs_return) {
		try {
			instance->setNonmaxSuppression(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNonmaxSuppression() /usr/include/opencv2/features2d.hpp:603
	void cv_AgastFeatureDetector_getNonmaxSuppression_const(const cv::AgastFeatureDetector* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getNonmaxSuppression();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setType(AgastFeatureDetector::DetectorType) /usr/include/opencv2/features2d.hpp:605
	void cv_AgastFeatureDetector_setType_DetectorType(cv::AgastFeatureDetector* instance, cv::AgastFeatureDetector::DetectorType type, Result_void* ocvrs_return) {
		try {
			instance->setType(type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getType() /usr/include/opencv2/features2d.hpp:606
	void cv_AgastFeatureDetector_getType_const(const cv::AgastFeatureDetector* instance, Result<cv::AgastFeatureDetector::DetectorType>* ocvrs_return) {
		try {
			cv::AgastFeatureDetector::DetectorType ret = instance->getType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::AgastFeatureDetector::DetectorType>))
	}
	
	// getDefaultName() /usr/include/opencv2/features2d.hpp:607
	void cv_AgastFeatureDetector_getDefaultName_const(const cv::AgastFeatureDetector* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	cv::Algorithm* cv_BFMatcher_to_Algorithm(cv::BFMatcher* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_BFMatcher_delete(cv::BFMatcher* instance) {
		delete instance;
	}
	// BFMatcher(int, bool) /usr/include/opencv2/features2d.hpp:1189
	void cv_BFMatcher_BFMatcher_int_bool(int normType, bool crossCheck, Result<cv::BFMatcher*>* ocvrs_return) {
		try {
			cv::BFMatcher* ret = new cv::BFMatcher(normType, crossCheck);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::BFMatcher*>))
	}
	
	// isMaskSupported() /usr/include/opencv2/features2d.hpp:1193
	void cv_BFMatcher_isMaskSupported_const(const cv::BFMatcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMaskSupported();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// create(int, bool) /usr/include/opencv2/features2d.hpp:1207
	void cv_BFMatcher_create_int_bool(int normType, bool crossCheck, Result<cv::Ptr<cv::BFMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BFMatcher> ret = cv::BFMatcher::create(normType, crossCheck);
			Ok(new cv::Ptr<cv::BFMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::BFMatcher>*>))
	}
	
	// clone(bool) /usr/include/opencv2/features2d.hpp:1209
	void cv_BFMatcher_clone_const_bool(const cv::BFMatcher* instance, bool emptyTrainData, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->clone(emptyTrainData);
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DescriptorMatcher>*>))
	}
	
	void cv_BOWImgDescriptorExtractor_delete(cv::BOWImgDescriptorExtractor* instance) {
		delete instance;
	}
	// BOWImgDescriptorExtractor(const Ptr<cv::DescriptorExtractor> &, const Ptr<cv::DescriptorMatcher> &) /usr/include/opencv2/features2d.hpp:1472
	void cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_Ptr_Feature2D_R_const_Ptr_DescriptorMatcher_R(const cv::Ptr<cv::Feature2D>* dextractor, const cv::Ptr<cv::DescriptorMatcher>* dmatcher, Result<cv::BOWImgDescriptorExtractor*>* ocvrs_return) {
		try {
			cv::BOWImgDescriptorExtractor* ret = new cv::BOWImgDescriptorExtractor(*dextractor, *dmatcher);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::BOWImgDescriptorExtractor*>))
	}
	
	// BOWImgDescriptorExtractor(const Ptr<cv::DescriptorMatcher> &) /usr/include/opencv2/features2d.hpp:1475
	void cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_Ptr_DescriptorMatcher_R(const cv::Ptr<cv::DescriptorMatcher>* dmatcher, Result<cv::BOWImgDescriptorExtractor*>* ocvrs_return) {
		try {
			cv::BOWImgDescriptorExtractor* ret = new cv::BOWImgDescriptorExtractor(*dmatcher);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::BOWImgDescriptorExtractor*>))
	}
	
	// setVocabulary(const cv::Mat &) /usr/include/opencv2/features2d.hpp:1483
	void cv_BOWImgDescriptorExtractor_setVocabulary_const_MatR(cv::BOWImgDescriptorExtractor* instance, const cv::Mat* vocabulary, Result_void* ocvrs_return) {
		try {
			instance->setVocabulary(*vocabulary);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getVocabulary() /usr/include/opencv2/features2d.hpp:1487
	void cv_BOWImgDescriptorExtractor_getVocabulary_const(const cv::BOWImgDescriptorExtractor* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->getVocabulary();
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// compute(cv::InputArray, std::vector<KeyPoint> &, cv::OutputArray, std::vector<std::vector<int>> *, cv::Mat *) /usr/include/opencv2/features2d.hpp:1499
	void cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_vector_KeyPoint_R_const__OutputArrayR_vector_vector_int__X_MatX(cv::BOWImgDescriptorExtractor* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* imgDescriptor, std::vector<std::vector<int>>* pointIdxsOfClusters, cv::Mat* descriptors, Result_void* ocvrs_return) {
		try {
			instance->compute(*image, *keypoints, *imgDescriptor, pointIdxsOfClusters, descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// compute(cv::InputArray, cv::OutputArray, std::vector<std::vector<int>> *) /usr/include/opencv2/features2d.hpp:1508
	void cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_const__OutputArrayR_vector_vector_int__X(cv::BOWImgDescriptorExtractor* instance, const cv::_InputArray* keypointDescriptors, const cv::_OutputArray* imgDescriptor, std::vector<std::vector<int>>* pointIdxsOfClusters, Result_void* ocvrs_return) {
		try {
			instance->compute(*keypointDescriptors, *imgDescriptor, pointIdxsOfClusters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// compute2(const cv::Mat &, std::vector<KeyPoint> &, cv::Mat &) /usr/include/opencv2/features2d.hpp:1512
	void cv_BOWImgDescriptorExtractor_compute2_const_MatR_vector_KeyPoint_R_MatR(cv::BOWImgDescriptorExtractor* instance, const cv::Mat* image, std::vector<cv::KeyPoint>* keypoints, cv::Mat* imgDescriptor, Result_void* ocvrs_return) {
		try {
			instance->compute2(*image, *keypoints, *imgDescriptor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// descriptorSize() /usr/include/opencv2/features2d.hpp:1517
	void cv_BOWImgDescriptorExtractor_descriptorSize_const(const cv::BOWImgDescriptorExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// descriptorType() /usr/include/opencv2/features2d.hpp:1521
	void cv_BOWImgDescriptorExtractor_descriptorType_const(const cv::BOWImgDescriptorExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	void cv_BOWKMeansTrainer_delete(cv::BOWKMeansTrainer* instance) {
		delete instance;
	}
	// BOWKMeansTrainer(int, const cv::TermCriteria &, int, int) /usr/include/opencv2/features2d.hpp:1436
	void cv_BOWKMeansTrainer_BOWKMeansTrainer_int_const_TermCriteriaR_int_int(int clusterCount, const cv::TermCriteria* termcrit, int attempts, int flags, Result<cv::BOWKMeansTrainer*>* ocvrs_return) {
		try {
			cv::BOWKMeansTrainer* ret = new cv::BOWKMeansTrainer(clusterCount, *termcrit, attempts, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::BOWKMeansTrainer*>))
	}
	
	// cluster() /usr/include/opencv2/features2d.hpp:1441
	void cv_BOWKMeansTrainer_cluster_const(const cv::BOWKMeansTrainer* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cluster();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// cluster(const cv::Mat &) /usr/include/opencv2/features2d.hpp:1442
	void cv_BOWKMeansTrainer_cluster_const_const_MatR(const cv::BOWKMeansTrainer* instance, const cv::Mat* descriptors, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cluster(*descriptors);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// add(const cv::Mat &) /usr/include/opencv2/features2d.hpp:1396
	void cv_BOWTrainer_add_const_MatR(cv::BOWTrainer* instance, const cv::Mat* descriptors, Result_void* ocvrs_return) {
		try {
			instance->add(*descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDescriptors() /usr/include/opencv2/features2d.hpp:1400
	void cv_BOWTrainer_getDescriptors_const(const cv::BOWTrainer* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->getDescriptors();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	// descriptorsCount() /usr/include/opencv2/features2d.hpp:1404
	void cv_BOWTrainer_descriptorsCount_const(const cv::BOWTrainer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorsCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// clear() /usr/include/opencv2/features2d.hpp:1406
	void cv_BOWTrainer_clear(cv::BOWTrainer* instance, Result_void* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// cluster() /usr/include/opencv2/features2d.hpp:1409
	void cv_BOWTrainer_cluster_const(const cv::BOWTrainer* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cluster();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// cluster(const cv::Mat &) /usr/include/opencv2/features2d.hpp:1420
	void cv_BOWTrainer_cluster_const_const_MatR(const cv::BOWTrainer* instance, const cv::Mat* descriptors, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cluster(*descriptors);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	cv::Algorithm* cv_BRISK_to_Algorithm(cv::BRISK* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::Feature2D* cv_BRISK_to_Feature2D(cv::BRISK* instance) {
		return dynamic_cast<cv::Feature2D*>(instance);
	}
	
	void cv_BRISK_delete(cv::BRISK* instance) {
		delete instance;
	}
	// create(int, int, float) /usr/include/opencv2/features2d.hpp:333
	void cv_BRISK_create_int_int_float(int thresh, int octaves, float patternScale, Result<cv::Ptr<cv::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create(thresh, octaves, patternScale);
			Ok(new cv::Ptr<cv::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::BRISK>*>))
	}
	
	// create(const std::vector<float> &, const std::vector<int> &, float, float, const std::vector<int> &) /usr/include/opencv2/features2d.hpp:346
	void cv_BRISK_create_const_vector_float_R_const_vector_int_R_float_float_const_vector_int_R(const std::vector<float>* radiusList, const std::vector<int>* numberList, float dMax, float dMin, const std::vector<int>* indexChange, Result<cv::Ptr<cv::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create(*radiusList, *numberList, dMax, dMin, *indexChange);
			Ok(new cv::Ptr<cv::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::BRISK>*>))
	}
	
	// create(int, int, const std::vector<float> &, const std::vector<int> &, float, float, const std::vector<int> &) /usr/include/opencv2/features2d.hpp:362
	void cv_BRISK_create_int_int_const_vector_float_R_const_vector_int_R_float_float_const_vector_int_R(int thresh, int octaves, const std::vector<float>* radiusList, const std::vector<int>* numberList, float dMax, float dMin, const std::vector<int>* indexChange, Result<cv::Ptr<cv::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BRISK> ret = cv::BRISK::create(thresh, octaves, *radiusList, *numberList, dMax, dMin, *indexChange);
			Ok(new cv::Ptr<cv::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::BRISK>*>))
	}
	
	// getDefaultName() /usr/include/opencv2/features2d.hpp:365
	void cv_BRISK_getDefaultName_const(const cv::BRISK* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// setThreshold(int) /usr/include/opencv2/features2d.hpp:370
	void cv_BRISK_setThreshold_int(cv::BRISK* instance, int threshold, Result_void* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getThreshold() /usr/include/opencv2/features2d.hpp:371
	void cv_BRISK_getThreshold_const(const cv::BRISK* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setOctaves(int) /usr/include/opencv2/features2d.hpp:376
	void cv_BRISK_setOctaves_int(cv::BRISK* instance, int octaves, Result_void* ocvrs_return) {
		try {
			instance->setOctaves(octaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getOctaves() /usr/include/opencv2/features2d.hpp:377
	void cv_BRISK_getOctaves_const(const cv::BRISK* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// add(cv::InputArrayOfArrays) /usr/include/opencv2/features2d.hpp:956
	void cv_DescriptorMatcher_add_const__InputArrayR(cv::DescriptorMatcher* instance, const cv::_InputArray* descriptors, Result_void* ocvrs_return) {
		try {
			instance->add(*descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTrainDescriptors() /usr/include/opencv2/features2d.hpp:960
	void cv_DescriptorMatcher_getTrainDescriptors_const(const cv::DescriptorMatcher* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->getTrainDescriptors();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	// clear() /usr/include/opencv2/features2d.hpp:964
	void cv_DescriptorMatcher_clear(cv::DescriptorMatcher* instance, Result_void* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// empty() /usr/include/opencv2/features2d.hpp:968
	void cv_DescriptorMatcher_empty_const(const cv::DescriptorMatcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isMaskSupported() /usr/include/opencv2/features2d.hpp:972
	void cv_DescriptorMatcher_isMaskSupported_const(const cv::DescriptorMatcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMaskSupported();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// train() /usr/include/opencv2/features2d.hpp:981
	void cv_DescriptorMatcher_train(cv::DescriptorMatcher* instance, Result_void* ocvrs_return) {
		try {
			instance->train();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// match(cv::InputArray, cv::InputArray, std::vector<DMatch> &, cv::InputArray) /usr/include/opencv2/features2d.hpp:999
	void cv_DescriptorMatcher_match_const_const__InputArrayR_const__InputArrayR_vector_DMatch_R_const__InputArrayR(const cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<cv::DMatch>* matches, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *trainDescriptors, *matches, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// knnMatch(cv::InputArray, cv::InputArray, std::vector<std::vector<DMatch>> &, int, cv::InputArray, bool) /usr/include/opencv2/features2d.hpp:1020
	void cv_DescriptorMatcher_knnMatch_const_const__InputArrayR_const__InputArrayR_vector_vector_DMatch__R_int_const__InputArrayR_bool(const cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, const cv::_InputArray* mask, bool compactResult, Result_void* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *trainDescriptors, *matches, k, *mask, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// radiusMatch(cv::InputArray, cv::InputArray, std::vector<std::vector<DMatch>> &, float, cv::InputArray, bool) /usr/include/opencv2/features2d.hpp:1043
	void cv_DescriptorMatcher_radiusMatch_const_const__InputArrayR_const__InputArrayR_vector_vector_DMatch__R_float_const__InputArrayR_bool(const cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, const cv::_InputArray* mask, bool compactResult, Result_void* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *trainDescriptors, *matches, maxDistance, *mask, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// match(cv::InputArray, std::vector<DMatch> &, cv::InputArrayOfArrays) /usr/include/opencv2/features2d.hpp:1054
	void cv_DescriptorMatcher_match_const__InputArrayR_vector_DMatch_R_const__InputArrayR(cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<cv::DMatch>* matches, const cv::_InputArray* masks, Result_void* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *matches, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// knnMatch(cv::InputArray, std::vector<std::vector<DMatch>> &, int, cv::InputArrayOfArrays, bool) /usr/include/opencv2/features2d.hpp:1067
	void cv_DescriptorMatcher_knnMatch_const__InputArrayR_vector_vector_DMatch__R_int_const__InputArrayR_bool(cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, const cv::_InputArray* masks, bool compactResult, Result_void* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *matches, k, *masks, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// radiusMatch(cv::InputArray, std::vector<std::vector<DMatch>> &, float, cv::InputArrayOfArrays, bool) /usr/include/opencv2/features2d.hpp:1081
	void cv_DescriptorMatcher_radiusMatch_const__InputArrayR_vector_vector_DMatch__R_float_const__InputArrayR_bool(cv::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, const cv::_InputArray* masks, bool compactResult, Result_void* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *matches, maxDistance, *masks, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(const cv::String &) /usr/include/opencv2/features2d.hpp:1085
	void cv_DescriptorMatcher_write_const_const_StringR(const cv::DescriptorMatcher* instance, const char* fileName, Result_void* ocvrs_return) {
		try {
			instance->write(std::string(fileName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::String &) /usr/include/opencv2/features2d.hpp:1091
	void cv_DescriptorMatcher_read_const_StringR(cv::DescriptorMatcher* instance, const char* fileName, Result_void* ocvrs_return) {
		try {
			instance->read(std::string(fileName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/features2d.hpp:1098
	void cv_DescriptorMatcher_read_const_FileNodeR(cv::DescriptorMatcher* instance, const cv::FileNode* unnamed, Result_void* ocvrs_return) {
		try {
			instance->read(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/features2d.hpp:1100
	void cv_DescriptorMatcher_write_const_FileStorageR(const cv::DescriptorMatcher* instance, cv::FileStorage* unnamed, Result_void* ocvrs_return) {
		try {
			instance->write(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// clone(bool) /usr/include/opencv2/features2d.hpp:1108
	void cv_DescriptorMatcher_clone_const_bool(const cv::DescriptorMatcher* instance, bool emptyTrainData, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->clone(emptyTrainData);
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DescriptorMatcher>*>))
	}
	
	// create(const cv::String &) /usr/include/opencv2/features2d.hpp:1121
	void cv_DescriptorMatcher_create_const_StringR(const char* descriptorMatcherType, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = cv::DescriptorMatcher::create(std::string(descriptorMatcherType));
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DescriptorMatcher>*>))
	}
	
	// create(const DescriptorMatcher::MatcherType &) /usr/include/opencv2/features2d.hpp:1123
	void cv_DescriptorMatcher_create_const_MatcherTypeR(const cv::DescriptorMatcher::MatcherType* matcherType, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = cv::DescriptorMatcher::create(*matcherType);
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DescriptorMatcher>*>))
	}
	
	// write(const Ptr<cv::FileStorage> &, const cv::String &) /usr/include/opencv2/features2d.hpp:1127
	void cv_DescriptorMatcher_write_const_const_Ptr_FileStorage_R_const_StringR(const cv::DescriptorMatcher* instance, const cv::Ptr<cv::FileStorage>* fs, const char* name, Result_void* ocvrs_return) {
		try {
			instance->write(*fs, std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(int, bool, FastFeatureDetector::DetectorType) /usr/include/opencv2/features2d.hpp:535
	void cv_FastFeatureDetector_create_int_bool_DetectorType(int threshold, bool nonmaxSuppression, cv::FastFeatureDetector::DetectorType type, Result<cv::Ptr<cv::FastFeatureDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FastFeatureDetector> ret = cv::FastFeatureDetector::create(threshold, nonmaxSuppression, type);
			Ok(new cv::Ptr<cv::FastFeatureDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::FastFeatureDetector>*>))
	}
	
	// setThreshold(int) /usr/include/opencv2/features2d.hpp:539
	void cv_FastFeatureDetector_setThreshold_int(cv::FastFeatureDetector* instance, int threshold, Result_void* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getThreshold() /usr/include/opencv2/features2d.hpp:540
	void cv_FastFeatureDetector_getThreshold_const(const cv::FastFeatureDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setNonmaxSuppression(bool) /usr/include/opencv2/features2d.hpp:542
	void cv_FastFeatureDetector_setNonmaxSuppression_bool(cv::FastFeatureDetector* instance, bool f, Result_void* ocvrs_return) {
		try {
			instance->setNonmaxSuppression(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNonmaxSuppression() /usr/include/opencv2/features2d.hpp:543
	void cv_FastFeatureDetector_getNonmaxSuppression_const(const cv::FastFeatureDetector* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getNonmaxSuppression();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setType(FastFeatureDetector::DetectorType) /usr/include/opencv2/features2d.hpp:545
	void cv_FastFeatureDetector_setType_DetectorType(cv::FastFeatureDetector* instance, cv::FastFeatureDetector::DetectorType type, Result_void* ocvrs_return) {
		try {
			instance->setType(type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getType() /usr/include/opencv2/features2d.hpp:546
	void cv_FastFeatureDetector_getType_const(const cv::FastFeatureDetector* instance, Result<cv::FastFeatureDetector::DetectorType>* ocvrs_return) {
		try {
			cv::FastFeatureDetector::DetectorType ret = instance->getType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FastFeatureDetector::DetectorType>))
	}
	
	// getDefaultName() /usr/include/opencv2/features2d.hpp:547
	void cv_FastFeatureDetector_getDefaultName_const(const cv::FastFeatureDetector* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	cv::BRISK* cv_Feature2D_to_BRISK(cv::Feature2D* instance) {
		return dynamic_cast<cv::BRISK*>(instance);
	}
	
	cv::SIFT* cv_Feature2D_to_SIFT(cv::Feature2D* instance) {
		return dynamic_cast<cv::SIFT*>(instance);
	}
	
	cv::SimpleBlobDetector* cv_Feature2D_to_SimpleBlobDetector(cv::Feature2D* instance) {
		return dynamic_cast<cv::SimpleBlobDetector*>(instance);
	}
	
	cv::Algorithm* cv_Feature2D_to_Algorithm(cv::Feature2D* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_Feature2D_delete(cv::Feature2D* instance) {
		delete instance;
	}
	// detect(cv::InputArray, std::vector<KeyPoint> &, cv::InputArray) /usr/include/opencv2/features2d.hpp:147
	void cv_Feature2D_detect_const__InputArrayR_vector_KeyPoint_R_const__InputArrayR(cv::Feature2D* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->detect(*image, *keypoints, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detect(cv::InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, cv::InputArrayOfArrays) /usr/include/opencv2/features2d.hpp:158
	void cv_Feature2D_detect_const__InputArrayR_vector_vector_KeyPoint__R_const__InputArrayR(cv::Feature2D* instance, const cv::_InputArray* images, std::vector<std::vector<cv::KeyPoint>>* keypoints, const cv::_InputArray* masks, Result_void* ocvrs_return) {
		try {
			instance->detect(*images, *keypoints, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// compute(cv::InputArray, std::vector<KeyPoint> &, cv::OutputArray) /usr/include/opencv2/features2d.hpp:173
	void cv_Feature2D_compute_const__InputArrayR_vector_KeyPoint_R_const__OutputArrayR(cv::Feature2D* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* descriptors, Result_void* ocvrs_return) {
		try {
			instance->compute(*image, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// compute(cv::InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, cv::OutputArrayOfArrays) /usr/include/opencv2/features2d.hpp:187
	void cv_Feature2D_compute_const__InputArrayR_vector_vector_KeyPoint__R_const__OutputArrayR(cv::Feature2D* instance, const cv::_InputArray* images, std::vector<std::vector<cv::KeyPoint>>* keypoints, const cv::_OutputArray* descriptors, Result_void* ocvrs_return) {
		try {
			instance->compute(*images, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detectAndCompute(cv::InputArray, cv::InputArray, std::vector<KeyPoint> &, cv::OutputArray, bool) /usr/include/opencv2/features2d.hpp:192
	void cv_Feature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vector_KeyPoint_R_const__OutputArrayR_bool(cv::Feature2D* instance, const cv::_InputArray* image, const cv::_InputArray* mask, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* descriptors, bool useProvidedKeypoints, Result_void* ocvrs_return) {
		try {
			instance->detectAndCompute(*image, *mask, *keypoints, *descriptors, useProvidedKeypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// descriptorSize() /usr/include/opencv2/features2d.hpp:197
	void cv_Feature2D_descriptorSize_const(const cv::Feature2D* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// descriptorType() /usr/include/opencv2/features2d.hpp:198
	void cv_Feature2D_descriptorType_const(const cv::Feature2D* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// defaultNorm() /usr/include/opencv2/features2d.hpp:199
	void cv_Feature2D_defaultNorm_const(const cv::Feature2D* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->defaultNorm();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// write(const cv::String &) /usr/include/opencv2/features2d.hpp:201
	void cv_Feature2D_write_const_const_StringR(const cv::Feature2D* instance, const char* fileName, Result_void* ocvrs_return) {
		try {
			instance->write(std::string(fileName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::String &) /usr/include/opencv2/features2d.hpp:203
	void cv_Feature2D_read_const_StringR(cv::Feature2D* instance, const char* fileName, Result_void* ocvrs_return) {
		try {
			instance->read(std::string(fileName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/features2d.hpp:205
	void cv_Feature2D_write_const_FileStorageR(const cv::Feature2D* instance, cv::FileStorage* unnamed, Result_void* ocvrs_return) {
		try {
			instance->write(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/features2d.hpp:208
	void cv_Feature2D_read_const_FileNodeR(cv::Feature2D* instance, const cv::FileNode* unnamed, Result_void* ocvrs_return) {
		try {
			instance->read(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// empty() /usr/include/opencv2/features2d.hpp:211
	void cv_Feature2D_empty_const(const cv::Feature2D* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// getDefaultName() /usr/include/opencv2/features2d.hpp:212
	void cv_Feature2D_getDefaultName_const(const cv::Feature2D* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// write(const Ptr<cv::FileStorage> &, const cv::String &) /usr/include/opencv2/features2d.hpp:215
	void cv_Feature2D_write_const_const_Ptr_FileStorage_R_const_StringR(const cv::Feature2D* instance, const cv::Ptr<cv::FileStorage>* fs, const char* name, Result_void* ocvrs_return) {
		try {
			instance->write(*fs, std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::Algorithm* cv_FlannBasedMatcher_to_Algorithm(cv::FlannBasedMatcher* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_FlannBasedMatcher_delete(cv::FlannBasedMatcher* instance) {
		delete instance;
	}
	// FlannBasedMatcher(const Ptr<flann::IndexParams> &, const Ptr<flann::SearchParams> &) /usr/include/opencv2/features2d.hpp:1232
	void cv_FlannBasedMatcher_FlannBasedMatcher_const_Ptr_IndexParams_R_const_Ptr_SearchParams_R(const cv::Ptr<cv::flann::IndexParams>* indexParams, const cv::Ptr<cv::flann::SearchParams>* searchParams, Result<cv::FlannBasedMatcher*>* ocvrs_return) {
		try {
			cv::FlannBasedMatcher* ret = new cv::FlannBasedMatcher(*indexParams, *searchParams);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FlannBasedMatcher*>))
	}
	
	// add(cv::InputArrayOfArrays) /usr/include/opencv2/features2d.hpp:1235
	void cv_FlannBasedMatcher_add_const__InputArrayR(cv::FlannBasedMatcher* instance, const cv::_InputArray* descriptors, Result_void* ocvrs_return) {
		try {
			instance->add(*descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// clear() /usr/include/opencv2/features2d.hpp:1236
	void cv_FlannBasedMatcher_clear(cv::FlannBasedMatcher* instance, Result_void* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/features2d.hpp:1239
	void cv_FlannBasedMatcher_read_const_FileNodeR(cv::FlannBasedMatcher* instance, const cv::FileNode* unnamed, Result_void* ocvrs_return) {
		try {
			instance->read(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/features2d.hpp:1241
	void cv_FlannBasedMatcher_write_const_FileStorageR(const cv::FlannBasedMatcher* instance, cv::FileStorage* unnamed, Result_void* ocvrs_return) {
		try {
			instance->write(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// train() /usr/include/opencv2/features2d.hpp:1243
	void cv_FlannBasedMatcher_train(cv::FlannBasedMatcher* instance, Result_void* ocvrs_return) {
		try {
			instance->train();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// isMaskSupported() /usr/include/opencv2/features2d.hpp:1244
	void cv_FlannBasedMatcher_isMaskSupported_const(const cv::FlannBasedMatcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMaskSupported();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// create() /usr/include/opencv2/features2d.hpp:1246
	void cv_FlannBasedMatcher_create(Result<cv::Ptr<cv::FlannBasedMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FlannBasedMatcher> ret = cv::FlannBasedMatcher::create();
			Ok(new cv::Ptr<cv::FlannBasedMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::FlannBasedMatcher>*>))
	}
	
	// clone(bool) /usr/include/opencv2/features2d.hpp:1248
	void cv_FlannBasedMatcher_clone_const_bool(const cv::FlannBasedMatcher* instance, bool emptyTrainData, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->clone(emptyTrainData);
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DescriptorMatcher>*>))
	}
	
	// create(int, double, double, int, bool, double) /usr/include/opencv2/features2d.hpp:640
	void cv_GFTTDetector_create_int_double_double_int_bool_double(int maxCorners, double qualityLevel, double minDistance, int blockSize, bool useHarrisDetector, double k, Result<cv::Ptr<cv::GFTTDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::GFTTDetector> ret = cv::GFTTDetector::create(maxCorners, qualityLevel, minDistance, blockSize, useHarrisDetector, k);
			Ok(new cv::Ptr<cv::GFTTDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::GFTTDetector>*>))
	}
	
	// create(int, double, double, int, int, bool, double) /usr/include/opencv2/features2d.hpp:642
	void cv_GFTTDetector_create_int_double_double_int_int_bool_double(int maxCorners, double qualityLevel, double minDistance, int blockSize, int gradiantSize, bool useHarrisDetector, double k, Result<cv::Ptr<cv::GFTTDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::GFTTDetector> ret = cv::GFTTDetector::create(maxCorners, qualityLevel, minDistance, blockSize, gradiantSize, useHarrisDetector, k);
			Ok(new cv::Ptr<cv::GFTTDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::GFTTDetector>*>))
	}
	
	// setMaxFeatures(int) /usr/include/opencv2/features2d.hpp:644
	void cv_GFTTDetector_setMaxFeatures_int(cv::GFTTDetector* instance, int maxFeatures, Result_void* ocvrs_return) {
		try {
			instance->setMaxFeatures(maxFeatures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxFeatures() /usr/include/opencv2/features2d.hpp:645
	void cv_GFTTDetector_getMaxFeatures_const(const cv::GFTTDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxFeatures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setQualityLevel(double) /usr/include/opencv2/features2d.hpp:647
	void cv_GFTTDetector_setQualityLevel_double(cv::GFTTDetector* instance, double qlevel, Result_void* ocvrs_return) {
		try {
			instance->setQualityLevel(qlevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getQualityLevel() /usr/include/opencv2/features2d.hpp:648
	void cv_GFTTDetector_getQualityLevel_const(const cv::GFTTDetector* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getQualityLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMinDistance(double) /usr/include/opencv2/features2d.hpp:650
	void cv_GFTTDetector_setMinDistance_double(cv::GFTTDetector* instance, double minDistance, Result_void* ocvrs_return) {
		try {
			instance->setMinDistance(minDistance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinDistance() /usr/include/opencv2/features2d.hpp:651
	void cv_GFTTDetector_getMinDistance_const(const cv::GFTTDetector* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinDistance();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setBlockSize(int) /usr/include/opencv2/features2d.hpp:653
	void cv_GFTTDetector_setBlockSize_int(cv::GFTTDetector* instance, int blockSize, Result_void* ocvrs_return) {
		try {
			instance->setBlockSize(blockSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getBlockSize() /usr/include/opencv2/features2d.hpp:654
	void cv_GFTTDetector_getBlockSize_const(const cv::GFTTDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBlockSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setHarrisDetector(bool) /usr/include/opencv2/features2d.hpp:656
	void cv_GFTTDetector_setHarrisDetector_bool(cv::GFTTDetector* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setHarrisDetector(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getHarrisDetector() /usr/include/opencv2/features2d.hpp:657
	void cv_GFTTDetector_getHarrisDetector_const(const cv::GFTTDetector* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getHarrisDetector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setK(double) /usr/include/opencv2/features2d.hpp:659
	void cv_GFTTDetector_setK_double(cv::GFTTDetector* instance, double k, Result_void* ocvrs_return) {
		try {
			instance->setK(k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getK() /usr/include/opencv2/features2d.hpp:660
	void cv_GFTTDetector_getK_const(const cv::GFTTDetector* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// getDefaultName() /usr/include/opencv2/features2d.hpp:661
	void cv_GFTTDetector_getDefaultName_const(const cv::GFTTDetector* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// create(bool, bool, float, int, int, KAZE::DiffusivityType) /usr/include/opencv2/features2d.hpp:763
	void cv_KAZE_create_bool_bool_float_int_int_DiffusivityType(bool extended, bool upright, float threshold, int nOctaves, int nOctaveLayers, cv::KAZE::DiffusivityType diffusivity, Result<cv::Ptr<cv::KAZE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::KAZE> ret = cv::KAZE::create(extended, upright, threshold, nOctaves, nOctaveLayers, diffusivity);
			Ok(new cv::Ptr<cv::KAZE>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::KAZE>*>))
	}
	
	// setExtended(bool) /usr/include/opencv2/features2d.hpp:768
	void cv_KAZE_setExtended_bool(cv::KAZE* instance, bool extended, Result_void* ocvrs_return) {
		try {
			instance->setExtended(extended);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getExtended() /usr/include/opencv2/features2d.hpp:769
	void cv_KAZE_getExtended_const(const cv::KAZE* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getExtended();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setUpright(bool) /usr/include/opencv2/features2d.hpp:771
	void cv_KAZE_setUpright_bool(cv::KAZE* instance, bool upright, Result_void* ocvrs_return) {
		try {
			instance->setUpright(upright);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUpright() /usr/include/opencv2/features2d.hpp:772
	void cv_KAZE_getUpright_const(const cv::KAZE* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUpright();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setThreshold(double) /usr/include/opencv2/features2d.hpp:774
	void cv_KAZE_setThreshold_double(cv::KAZE* instance, double threshold, Result_void* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getThreshold() /usr/include/opencv2/features2d.hpp:775
	void cv_KAZE_getThreshold_const(const cv::KAZE* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setNOctaves(int) /usr/include/opencv2/features2d.hpp:777
	void cv_KAZE_setNOctaves_int(cv::KAZE* instance, int octaves, Result_void* ocvrs_return) {
		try {
			instance->setNOctaves(octaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNOctaves() /usr/include/opencv2/features2d.hpp:778
	void cv_KAZE_getNOctaves_const(const cv::KAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setNOctaveLayers(int) /usr/include/opencv2/features2d.hpp:780
	void cv_KAZE_setNOctaveLayers_int(cv::KAZE* instance, int octaveLayers, Result_void* ocvrs_return) {
		try {
			instance->setNOctaveLayers(octaveLayers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNOctaveLayers() /usr/include/opencv2/features2d.hpp:781
	void cv_KAZE_getNOctaveLayers_const(const cv::KAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaveLayers();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setDiffusivity(KAZE::DiffusivityType) /usr/include/opencv2/features2d.hpp:783
	void cv_KAZE_setDiffusivity_DiffusivityType(cv::KAZE* instance, cv::KAZE::DiffusivityType diff, Result_void* ocvrs_return) {
		try {
			instance->setDiffusivity(diff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDiffusivity() /usr/include/opencv2/features2d.hpp:784
	void cv_KAZE_getDiffusivity_const(const cv::KAZE* instance, Result<cv::KAZE::DiffusivityType>* ocvrs_return) {
		try {
			cv::KAZE::DiffusivityType ret = instance->getDiffusivity();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::KAZE::DiffusivityType>))
	}
	
	// getDefaultName() /usr/include/opencv2/features2d.hpp:785
	void cv_KAZE_getDefaultName_const(const cv::KAZE* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	void cv_KeyPointsFilter_delete(cv::KeyPointsFilter* instance) {
		delete instance;
	}
	// KeyPointsFilter() /usr/include/opencv2/features2d.hpp:95
	void cv_KeyPointsFilter_KeyPointsFilter(Result<cv::KeyPointsFilter*>* ocvrs_return) {
		try {
			cv::KeyPointsFilter* ret = new cv::KeyPointsFilter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::KeyPointsFilter*>))
	}
	
	// runByImageBorder(std::vector<KeyPoint> &, cv::Size, int) /usr/include/opencv2/features2d.hpp:100
	void cv_KeyPointsFilter_runByImageBorder_vector_KeyPoint_R_Size_int(std::vector<cv::KeyPoint>* keypoints, cv::Size* imageSize, int borderSize, Result_void* ocvrs_return) {
		try {
			cv::KeyPointsFilter::runByImageBorder(*keypoints, *imageSize, borderSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// runByKeypointSize(std::vector<KeyPoint> &, float, float) /usr/include/opencv2/features2d.hpp:104
	void cv_KeyPointsFilter_runByKeypointSize_vector_KeyPoint_R_float_float(std::vector<cv::KeyPoint>* keypoints, float minSize, float maxSize, Result_void* ocvrs_return) {
		try {
			cv::KeyPointsFilter::runByKeypointSize(*keypoints, minSize, maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// runByPixelsMask(std::vector<KeyPoint> &, const cv::Mat &) /usr/include/opencv2/features2d.hpp:109
	void cv_KeyPointsFilter_runByPixelsMask_vector_KeyPoint_R_const_MatR(std::vector<cv::KeyPoint>* keypoints, const cv::Mat* mask, Result_void* ocvrs_return) {
		try {
			cv::KeyPointsFilter::runByPixelsMask(*keypoints, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// removeDuplicated(std::vector<KeyPoint> &) /usr/include/opencv2/features2d.hpp:113
	void cv_KeyPointsFilter_removeDuplicated_vector_KeyPoint_R(std::vector<cv::KeyPoint>* keypoints, Result_void* ocvrs_return) {
		try {
			cv::KeyPointsFilter::removeDuplicated(*keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// removeDuplicatedSorted(std::vector<KeyPoint> &) /usr/include/opencv2/features2d.hpp:117
	void cv_KeyPointsFilter_removeDuplicatedSorted_vector_KeyPoint_R(std::vector<cv::KeyPoint>* keypoints, Result_void* ocvrs_return) {
		try {
			cv::KeyPointsFilter::removeDuplicatedSorted(*keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// retainBest(std::vector<KeyPoint> &, int) /usr/include/opencv2/features2d.hpp:122
	void cv_KeyPointsFilter_retainBest_vector_KeyPoint_R_int(std::vector<cv::KeyPoint>* keypoints, int npoints, Result_void* ocvrs_return) {
		try {
			cv::KeyPointsFilter::retainBest(*keypoints, npoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(int, int, int, double, double, int, double, double, int) /usr/include/opencv2/features2d.hpp:486
	void cv_MSER_create_int_int_int_double_double_int_double_double_int(int delta, int min_area, int max_area, double max_variation, double min_diversity, int max_evolution, double area_threshold, double min_margin, int edge_blur_size, Result<cv::Ptr<cv::MSER>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::MSER> ret = cv::MSER::create(delta, min_area, max_area, max_variation, min_diversity, max_evolution, area_threshold, min_margin, edge_blur_size);
			Ok(new cv::Ptr<cv::MSER>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::MSER>*>))
	}
	
	// detectRegions(cv::InputArray, std::vector<std::vector<Point>> &, std::vector<Rect> &) /usr/include/opencv2/features2d.hpp:497
	void cv_MSER_detectRegions_const__InputArrayR_vector_vector_Point__R_vector_Rect_R(cv::MSER* instance, const cv::_InputArray* image, std::vector<std::vector<cv::Point>>* msers, std::vector<cv::Rect>* bboxes, Result_void* ocvrs_return) {
		try {
			instance->detectRegions(*image, *msers, *bboxes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setDelta(int) /usr/include/opencv2/features2d.hpp:501
	void cv_MSER_setDelta_int(cv::MSER* instance, int delta, Result_void* ocvrs_return) {
		try {
			instance->setDelta(delta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDelta() /usr/include/opencv2/features2d.hpp:502
	void cv_MSER_getDelta_const(const cv::MSER* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDelta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMinArea(int) /usr/include/opencv2/features2d.hpp:504
	void cv_MSER_setMinArea_int(cv::MSER* instance, int minArea, Result_void* ocvrs_return) {
		try {
			instance->setMinArea(minArea);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinArea() /usr/include/opencv2/features2d.hpp:505
	void cv_MSER_getMinArea_const(const cv::MSER* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinArea();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMaxArea(int) /usr/include/opencv2/features2d.hpp:507
	void cv_MSER_setMaxArea_int(cv::MSER* instance, int maxArea, Result_void* ocvrs_return) {
		try {
			instance->setMaxArea(maxArea);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxArea() /usr/include/opencv2/features2d.hpp:508
	void cv_MSER_getMaxArea_const(const cv::MSER* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxArea();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setPass2Only(bool) /usr/include/opencv2/features2d.hpp:510
	void cv_MSER_setPass2Only_bool(cv::MSER* instance, bool f, Result_void* ocvrs_return) {
		try {
			instance->setPass2Only(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPass2Only() /usr/include/opencv2/features2d.hpp:511
	void cv_MSER_getPass2Only_const(const cv::MSER* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getPass2Only();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// getDefaultName() /usr/include/opencv2/features2d.hpp:512
	void cv_MSER_getDefaultName_const(const cv::MSER* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// create(int, float, int, int, int, int, ORB::ScoreType, int, int) /usr/include/opencv2/features2d.hpp:424
	void cv_ORB_create_int_float_int_int_int_int_ScoreType_int_int(int nfeatures, float scaleFactor, int nlevels, int edgeThreshold, int firstLevel, int WTA_K, cv::ORB::ScoreType scoreType, int patchSize, int fastThreshold, Result<cv::Ptr<cv::ORB>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ORB> ret = cv::ORB::create(nfeatures, scaleFactor, nlevels, edgeThreshold, firstLevel, WTA_K, scoreType, patchSize, fastThreshold);
			Ok(new cv::Ptr<cv::ORB>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ORB>*>))
	}
	
	// setMaxFeatures(int) /usr/include/opencv2/features2d.hpp:427
	void cv_ORB_setMaxFeatures_int(cv::ORB* instance, int maxFeatures, Result_void* ocvrs_return) {
		try {
			instance->setMaxFeatures(maxFeatures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxFeatures() /usr/include/opencv2/features2d.hpp:428
	void cv_ORB_getMaxFeatures_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxFeatures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setScaleFactor(double) /usr/include/opencv2/features2d.hpp:430
	void cv_ORB_setScaleFactor_double(cv::ORB* instance, double scaleFactor, Result_void* ocvrs_return) {
		try {
			instance->setScaleFactor(scaleFactor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getScaleFactor() /usr/include/opencv2/features2d.hpp:431
	void cv_ORB_getScaleFactor_const(const cv::ORB* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setNLevels(int) /usr/include/opencv2/features2d.hpp:433
	void cv_ORB_setNLevels_int(cv::ORB* instance, int nlevels, Result_void* ocvrs_return) {
		try {
			instance->setNLevels(nlevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNLevels() /usr/include/opencv2/features2d.hpp:434
	void cv_ORB_getNLevels_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setEdgeThreshold(int) /usr/include/opencv2/features2d.hpp:436
	void cv_ORB_setEdgeThreshold_int(cv::ORB* instance, int edgeThreshold, Result_void* ocvrs_return) {
		try {
			instance->setEdgeThreshold(edgeThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getEdgeThreshold() /usr/include/opencv2/features2d.hpp:437
	void cv_ORB_getEdgeThreshold_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getEdgeThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setFirstLevel(int) /usr/include/opencv2/features2d.hpp:439
	void cv_ORB_setFirstLevel_int(cv::ORB* instance, int firstLevel, Result_void* ocvrs_return) {
		try {
			instance->setFirstLevel(firstLevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFirstLevel() /usr/include/opencv2/features2d.hpp:440
	void cv_ORB_getFirstLevel_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFirstLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setWTA_K(int) /usr/include/opencv2/features2d.hpp:442
	void cv_ORB_setWTA_K_int(cv::ORB* instance, int wta_k, Result_void* ocvrs_return) {
		try {
			instance->setWTA_K(wta_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWTA_K() /usr/include/opencv2/features2d.hpp:443
	void cv_ORB_getWTA_K_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWTA_K();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setScoreType(ORB::ScoreType) /usr/include/opencv2/features2d.hpp:445
	void cv_ORB_setScoreType_ScoreType(cv::ORB* instance, cv::ORB::ScoreType scoreType, Result_void* ocvrs_return) {
		try {
			instance->setScoreType(scoreType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getScoreType() /usr/include/opencv2/features2d.hpp:446
	void cv_ORB_getScoreType_const(const cv::ORB* instance, Result<cv::ORB::ScoreType>* ocvrs_return) {
		try {
			cv::ORB::ScoreType ret = instance->getScoreType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ORB::ScoreType>))
	}
	
	// setPatchSize(int) /usr/include/opencv2/features2d.hpp:448
	void cv_ORB_setPatchSize_int(cv::ORB* instance, int patchSize, Result_void* ocvrs_return) {
		try {
			instance->setPatchSize(patchSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPatchSize() /usr/include/opencv2/features2d.hpp:449
	void cv_ORB_getPatchSize_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPatchSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setFastThreshold(int) /usr/include/opencv2/features2d.hpp:451
	void cv_ORB_setFastThreshold_int(cv::ORB* instance, int fastThreshold, Result_void* ocvrs_return) {
		try {
			instance->setFastThreshold(fastThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFastThreshold() /usr/include/opencv2/features2d.hpp:452
	void cv_ORB_getFastThreshold_const(const cv::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFastThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getDefaultName() /usr/include/opencv2/features2d.hpp:453
	void cv_ORB_getDefaultName_const(const cv::ORB* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	cv::Algorithm* cv_SIFT_to_Algorithm(cv::SIFT* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::Feature2D* cv_SIFT_to_Feature2D(cv::SIFT* instance) {
		return dynamic_cast<cv::Feature2D*>(instance);
	}
	
	void cv_SIFT_delete(cv::SIFT* instance) {
		delete instance;
	}
	// create(int, int, double, double, double) /usr/include/opencv2/features2d.hpp:283
	void cv_SIFT_create_int_int_double_double_double(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma, Result<cv::Ptr<cv::SIFT>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SIFT> ret = cv::SIFT::create(nfeatures, nOctaveLayers, contrastThreshold, edgeThreshold, sigma);
			Ok(new cv::Ptr<cv::SIFT>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::SIFT>*>))
	}
	
	// create(int, int, double, double, double, int) /usr/include/opencv2/features2d.hpp:310
	void cv_SIFT_create_int_int_double_double_double_int(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma, int descriptorType, Result<cv::Ptr<cv::SIFT>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SIFT> ret = cv::SIFT::create(nfeatures, nOctaveLayers, contrastThreshold, edgeThreshold, sigma, descriptorType);
			Ok(new cv::Ptr<cv::SIFT>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::SIFT>*>))
	}
	
	// getDefaultName() /usr/include/opencv2/features2d.hpp:314
	void cv_SIFT_getDefaultName_const(const cv::SIFT* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	cv::Algorithm* cv_SimpleBlobDetector_to_Algorithm(cv::SimpleBlobDetector* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::Feature2D* cv_SimpleBlobDetector_to_Feature2D(cv::SimpleBlobDetector* instance) {
		return dynamic_cast<cv::Feature2D*>(instance);
	}
	
	void cv_SimpleBlobDetector_delete(cv::SimpleBlobDetector* instance) {
		delete instance;
	}
	// create(const SimpleBlobDetector::Params &) /usr/include/opencv2/features2d.hpp:727
	void cv_SimpleBlobDetector_create_const_ParamsR(const cv::SimpleBlobDetector::Params* parameters, Result<cv::Ptr<cv::SimpleBlobDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SimpleBlobDetector> ret = cv::SimpleBlobDetector::create(*parameters);
			Ok(new cv::Ptr<cv::SimpleBlobDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::SimpleBlobDetector>*>))
	}
	
	// getDefaultName() /usr/include/opencv2/features2d.hpp:728
	void cv_SimpleBlobDetector_getDefaultName_const(const cv::SimpleBlobDetector* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// Params() /usr/include/opencv2/features2d.hpp:700
	void cv_SimpleBlobDetector_Params_Params(Result<cv::SimpleBlobDetector::Params>* ocvrs_return) {
		try {
			cv::SimpleBlobDetector::Params ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SimpleBlobDetector::Params>))
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/features2d.hpp:722
	void cv_SimpleBlobDetector_Params_read_const_FileNodeR(cv::SimpleBlobDetector::Params instance, const cv::FileNode* fn, Result_void* ocvrs_return) {
		try {
			instance.read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/features2d.hpp:723
	void cv_SimpleBlobDetector_Params_write_const_FileStorageR(const cv::SimpleBlobDetector::Params instance, cv::FileStorage* fs, Result_void* ocvrs_return) {
		try {
			instance.write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
