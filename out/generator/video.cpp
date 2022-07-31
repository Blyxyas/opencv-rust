#include "ocvrs_common.hpp"
#include <opencv2/video.hpp>
#include "video_types.hpp"

extern "C" {
	// CamShift(cv::InputArray, cv::Rect &, cv::TermCriteria) /usr/include/opencv2/video/tracking.hpp:79
	void cv_CamShift_const__InputArrayR_RectR_TermCriteria(const cv::_InputArray* probImage, cv::Rect* window, cv::TermCriteria* criteria, Result<cv::RotatedRect*>* ocvrs_return) {
		try {
			cv::RotatedRect ret = cv::CamShift(*probImage, *window, *criteria);
			Ok(new cv::RotatedRect(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RotatedRect*>))
	}
	
	// buildOpticalFlowPyramid(cv::InputArray, cv::OutputArrayOfArrays, cv::Size, int, bool, int, int, bool) /usr/include/opencv2/video/tracking.hpp:121
	void cv_buildOpticalFlowPyramid_const__InputArrayR_const__OutputArrayR_Size_int_bool_int_int_bool(const cv::_InputArray* img, const cv::_OutputArray* pyramid, cv::Size* winSize, int maxLevel, bool withDerivatives, int pyrBorder, int derivBorder, bool tryReuseInputImage, Result<int>* ocvrs_return) {
		try {
			int ret = cv::buildOpticalFlowPyramid(*img, *pyramid, *winSize, maxLevel, withDerivatives, pyrBorder, derivBorder, tryReuseInputImage);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// calcOpticalFlowFarneback(cv::InputArray, cv::InputArray, cv::InputOutputArray, double, int, int, int, int, double, int) /usr/include/opencv2/video/tracking.hpp:223
	void cv_calcOpticalFlowFarneback_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_double_int_int_int_int_double_int(const cv::_InputArray* prev, const cv::_InputArray* next, const cv::_InputOutputArray* flow, double pyr_scale, int levels, int winsize, int iterations, int poly_n, double poly_sigma, int flags, Result_void* ocvrs_return) {
		try {
			cv::calcOpticalFlowFarneback(*prev, *next, *flow, pyr_scale, levels, winsize, iterations, poly_n, poly_sigma, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// calcOpticalFlowPyrLK(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray, cv::Size, int, cv::TermCriteria, int, double) /usr/include/opencv2/video/tracking.hpp:178
	void cv_calcOpticalFlowPyrLK_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_Size_int_TermCriteria_int_double(const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, const cv::_OutputArray* err, cv::Size* winSize, int maxLevel, cv::TermCriteria* criteria, int flags, double minEigThreshold, Result_void* ocvrs_return) {
		try {
			cv::calcOpticalFlowPyrLK(*prevImg, *nextImg, *prevPts, *nextPts, *status, *err, *winSize, maxLevel, *criteria, flags, minEigThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// computeECC(cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/video/tracking.hpp:279
	void cv_computeECC_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* templateImage, const cv::_InputArray* inputImage, const cv::_InputArray* inputMask, Result<double>* ocvrs_return) {
		try {
			double ret = cv::computeECC(*templateImage, *inputImage, *inputMask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// createBackgroundSubtractorKNN(int, double, bool) /usr/include/opencv2/video/background_segm.hpp:310
	void cv_createBackgroundSubtractorKNN_int_double_bool(int history, double dist2Threshold, bool detectShadows, Result<cv::Ptr<cv::BackgroundSubtractorKNN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BackgroundSubtractorKNN> ret = cv::createBackgroundSubtractorKNN(history, dist2Threshold, detectShadows);
			Ok(new cv::Ptr<cv::BackgroundSubtractorKNN>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::BackgroundSubtractorKNN>*>))
	}
	
	// createBackgroundSubtractorMOG2(int, double, bool) /usr/include/opencv2/video/background_segm.hpp:221
	void cv_createBackgroundSubtractorMOG2_int_double_bool(int history, double varThreshold, bool detectShadows, Result<cv::Ptr<cv::BackgroundSubtractorMOG2>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BackgroundSubtractorMOG2> ret = cv::createBackgroundSubtractorMOG2(history, varThreshold, detectShadows);
			Ok(new cv::Ptr<cv::BackgroundSubtractorMOG2>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::BackgroundSubtractorMOG2>*>))
	}
	
	// estimateRigidTransform(cv::InputArray, cv::InputArray, bool) /usr/include/opencv2/video/tracking.hpp:258
	void cv_estimateRigidTransform_const__InputArrayR_const__InputArrayR_bool(const cv::_InputArray* src, const cv::_InputArray* dst, bool fullAffine, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::estimateRigidTransform(*src, *dst, fullAffine);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// findTransformECC(cv::InputArray, cv::InputArray, cv::InputOutputArray, int, cv::TermCriteria, cv::InputArray) /usr/include/opencv2/video/tracking.hpp:343
	void cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR(const cv::_InputArray* templateImage, const cv::_InputArray* inputImage, const cv::_InputOutputArray* warpMatrix, int motionType, cv::TermCriteria* criteria, const cv::_InputArray* inputMask, Result<double>* ocvrs_return) {
		try {
			double ret = cv::findTransformECC(*templateImage, *inputImage, *warpMatrix, motionType, *criteria, *inputMask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// findTransformECC(cv::InputArray, cv::InputArray, cv::InputOutputArray, int, cv::TermCriteria, cv::InputArray, int) /usr/include/opencv2/video/tracking.hpp:336
	void cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR_int(const cv::_InputArray* templateImage, const cv::_InputArray* inputImage, const cv::_InputOutputArray* warpMatrix, int motionType, cv::TermCriteria* criteria, const cv::_InputArray* inputMask, int gaussFiltSize, Result<double>* ocvrs_return) {
		try {
			double ret = cv::findTransformECC(*templateImage, *inputImage, *warpMatrix, motionType, *criteria, *inputMask, gaussFiltSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// meanShift(cv::InputArray, cv::Rect &, cv::TermCriteria) /usr/include/opencv2/video/tracking.hpp:104
	void cv_meanShift_const__InputArrayR_RectR_TermCriteria(const cv::_InputArray* probImage, cv::Rect* window, cv::TermCriteria* criteria, Result<int>* ocvrs_return) {
		try {
			int ret = cv::meanShift(*probImage, *window, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// readOpticalFlow(const cv::String &) /usr/include/opencv2/video/tracking.hpp:421
	void cv_readOpticalFlow_const_StringR(const char* path, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::readOpticalFlow(std::string(path));
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// writeOpticalFlow(const cv::String &, cv::InputArray) /usr/include/opencv2/video/tracking.hpp:431
	void cv_writeOpticalFlow_const_StringR_const__InputArrayR(const char* path, const cv::_InputArray* flow, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::writeOpticalFlow(std::string(path), *flow);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// apply(cv::InputArray, cv::OutputArray, double) /usr/include/opencv2/video/background_segm.hpp:72
	void cv_BackgroundSubtractor_apply_const__InputArrayR_const__OutputArrayR_double(cv::BackgroundSubtractor* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, Result_void* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask, learningRate);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getBackgroundImage(cv::OutputArray) /usr/include/opencv2/video/background_segm.hpp:81
	void cv_BackgroundSubtractor_getBackgroundImage_const_const__OutputArrayR(const cv::BackgroundSubtractor* instance, const cv::_OutputArray* backgroundImage, Result_void* ocvrs_return) {
		try {
			instance->getBackgroundImage(*backgroundImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getHistory() /usr/include/opencv2/video/background_segm.hpp:234
	void cv_BackgroundSubtractorKNN_getHistory_const(const cv::BackgroundSubtractorKNN* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getHistory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setHistory(int) /usr/include/opencv2/video/background_segm.hpp:237
	void cv_BackgroundSubtractorKNN_setHistory_int(cv::BackgroundSubtractorKNN* instance, int history, Result_void* ocvrs_return) {
		try {
			instance->setHistory(history);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNSamples() /usr/include/opencv2/video/background_segm.hpp:241
	void cv_BackgroundSubtractorKNN_getNSamples_const(const cv::BackgroundSubtractorKNN* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNSamples();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setNSamples(int) /usr/include/opencv2/video/background_segm.hpp:246
	void cv_BackgroundSubtractorKNN_setNSamples_int(cv::BackgroundSubtractorKNN* instance, int _nN, Result_void* ocvrs_return) {
		try {
			instance->setNSamples(_nN);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDist2Threshold() /usr/include/opencv2/video/background_segm.hpp:253
	void cv_BackgroundSubtractorKNN_getDist2Threshold_const(const cv::BackgroundSubtractorKNN* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDist2Threshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setDist2Threshold(double) /usr/include/opencv2/video/background_segm.hpp:256
	void cv_BackgroundSubtractorKNN_setDist2Threshold_double(cv::BackgroundSubtractorKNN* instance, double _dist2Threshold, Result_void* ocvrs_return) {
		try {
			instance->setDist2Threshold(_dist2Threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getkNNSamples() /usr/include/opencv2/video/background_segm.hpp:263
	void cv_BackgroundSubtractorKNN_getkNNSamples_const(const cv::BackgroundSubtractorKNN* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getkNNSamples();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setkNNSamples(int) /usr/include/opencv2/video/background_segm.hpp:266
	void cv_BackgroundSubtractorKNN_setkNNSamples_int(cv::BackgroundSubtractorKNN* instance, int _nkNN, Result_void* ocvrs_return) {
		try {
			instance->setkNNSamples(_nkNN);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDetectShadows() /usr/include/opencv2/video/background_segm.hpp:273
	void cv_BackgroundSubtractorKNN_getDetectShadows_const(const cv::BackgroundSubtractorKNN* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getDetectShadows();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setDetectShadows(bool) /usr/include/opencv2/video/background_segm.hpp:276
	void cv_BackgroundSubtractorKNN_setDetectShadows_bool(cv::BackgroundSubtractorKNN* instance, bool detectShadows, Result_void* ocvrs_return) {
		try {
			instance->setDetectShadows(detectShadows);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getShadowValue() /usr/include/opencv2/video/background_segm.hpp:283
	void cv_BackgroundSubtractorKNN_getShadowValue_const(const cv::BackgroundSubtractorKNN* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getShadowValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setShadowValue(int) /usr/include/opencv2/video/background_segm.hpp:286
	void cv_BackgroundSubtractorKNN_setShadowValue_int(cv::BackgroundSubtractorKNN* instance, int value, Result_void* ocvrs_return) {
		try {
			instance->setShadowValue(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getShadowThreshold() /usr/include/opencv2/video/background_segm.hpp:295
	void cv_BackgroundSubtractorKNN_getShadowThreshold_const(const cv::BackgroundSubtractorKNN* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getShadowThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setShadowThreshold(double) /usr/include/opencv2/video/background_segm.hpp:298
	void cv_BackgroundSubtractorKNN_setShadowThreshold_double(cv::BackgroundSubtractorKNN* instance, double threshold, Result_void* ocvrs_return) {
		try {
			instance->setShadowThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getHistory() /usr/include/opencv2/video/background_segm.hpp:95
	void cv_BackgroundSubtractorMOG2_getHistory_const(const cv::BackgroundSubtractorMOG2* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getHistory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setHistory(int) /usr/include/opencv2/video/background_segm.hpp:98
	void cv_BackgroundSubtractorMOG2_setHistory_int(cv::BackgroundSubtractorMOG2* instance, int history, Result_void* ocvrs_return) {
		try {
			instance->setHistory(history);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNMixtures() /usr/include/opencv2/video/background_segm.hpp:102
	void cv_BackgroundSubtractorMOG2_getNMixtures_const(const cv::BackgroundSubtractorMOG2* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNMixtures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setNMixtures(int) /usr/include/opencv2/video/background_segm.hpp:107
	void cv_BackgroundSubtractorMOG2_setNMixtures_int(cv::BackgroundSubtractorMOG2* instance, int nmixtures, Result_void* ocvrs_return) {
		try {
			instance->setNMixtures(nmixtures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getBackgroundRatio() /usr/include/opencv2/video/background_segm.hpp:115
	void cv_BackgroundSubtractorMOG2_getBackgroundRatio_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getBackgroundRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setBackgroundRatio(double) /usr/include/opencv2/video/background_segm.hpp:118
	void cv_BackgroundSubtractorMOG2_setBackgroundRatio_double(cv::BackgroundSubtractorMOG2* instance, double ratio, Result_void* ocvrs_return) {
		try {
			instance->setBackgroundRatio(ratio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getVarThreshold() /usr/include/opencv2/video/background_segm.hpp:125
	void cv_BackgroundSubtractorMOG2_getVarThreshold_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getVarThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setVarThreshold(double) /usr/include/opencv2/video/background_segm.hpp:128
	void cv_BackgroundSubtractorMOG2_setVarThreshold_double(cv::BackgroundSubtractorMOG2* instance, double varThreshold, Result_void* ocvrs_return) {
		try {
			instance->setVarThreshold(varThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getVarThresholdGen() /usr/include/opencv2/video/background_segm.hpp:138
	void cv_BackgroundSubtractorMOG2_getVarThresholdGen_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getVarThresholdGen();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setVarThresholdGen(double) /usr/include/opencv2/video/background_segm.hpp:141
	void cv_BackgroundSubtractorMOG2_setVarThresholdGen_double(cv::BackgroundSubtractorMOG2* instance, double varThresholdGen, Result_void* ocvrs_return) {
		try {
			instance->setVarThresholdGen(varThresholdGen);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getVarInit() /usr/include/opencv2/video/background_segm.hpp:145
	void cv_BackgroundSubtractorMOG2_getVarInit_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getVarInit();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setVarInit(double) /usr/include/opencv2/video/background_segm.hpp:148
	void cv_BackgroundSubtractorMOG2_setVarInit_double(cv::BackgroundSubtractorMOG2* instance, double varInit, Result_void* ocvrs_return) {
		try {
			instance->setVarInit(varInit);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getVarMin() /usr/include/opencv2/video/background_segm.hpp:150
	void cv_BackgroundSubtractorMOG2_getVarMin_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getVarMin();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setVarMin(double) /usr/include/opencv2/video/background_segm.hpp:151
	void cv_BackgroundSubtractorMOG2_setVarMin_double(cv::BackgroundSubtractorMOG2* instance, double varMin, Result_void* ocvrs_return) {
		try {
			instance->setVarMin(varMin);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getVarMax() /usr/include/opencv2/video/background_segm.hpp:153
	void cv_BackgroundSubtractorMOG2_getVarMax_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getVarMax();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setVarMax(double) /usr/include/opencv2/video/background_segm.hpp:154
	void cv_BackgroundSubtractorMOG2_setVarMax_double(cv::BackgroundSubtractorMOG2* instance, double varMax, Result_void* ocvrs_return) {
		try {
			instance->setVarMax(varMax);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getComplexityReductionThreshold() /usr/include/opencv2/video/background_segm.hpp:162
	void cv_BackgroundSubtractorMOG2_getComplexityReductionThreshold_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getComplexityReductionThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setComplexityReductionThreshold(double) /usr/include/opencv2/video/background_segm.hpp:165
	void cv_BackgroundSubtractorMOG2_setComplexityReductionThreshold_double(cv::BackgroundSubtractorMOG2* instance, double ct, Result_void* ocvrs_return) {
		try {
			instance->setComplexityReductionThreshold(ct);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDetectShadows() /usr/include/opencv2/video/background_segm.hpp:172
	void cv_BackgroundSubtractorMOG2_getDetectShadows_const(const cv::BackgroundSubtractorMOG2* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getDetectShadows();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setDetectShadows(bool) /usr/include/opencv2/video/background_segm.hpp:175
	void cv_BackgroundSubtractorMOG2_setDetectShadows_bool(cv::BackgroundSubtractorMOG2* instance, bool detectShadows, Result_void* ocvrs_return) {
		try {
			instance->setDetectShadows(detectShadows);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getShadowValue() /usr/include/opencv2/video/background_segm.hpp:182
	void cv_BackgroundSubtractorMOG2_getShadowValue_const(const cv::BackgroundSubtractorMOG2* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getShadowValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setShadowValue(int) /usr/include/opencv2/video/background_segm.hpp:185
	void cv_BackgroundSubtractorMOG2_setShadowValue_int(cv::BackgroundSubtractorMOG2* instance, int value, Result_void* ocvrs_return) {
		try {
			instance->setShadowValue(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getShadowThreshold() /usr/include/opencv2/video/background_segm.hpp:194
	void cv_BackgroundSubtractorMOG2_getShadowThreshold_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getShadowThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setShadowThreshold(double) /usr/include/opencv2/video/background_segm.hpp:197
	void cv_BackgroundSubtractorMOG2_setShadowThreshold_double(cv::BackgroundSubtractorMOG2* instance, double threshold, Result_void* ocvrs_return) {
		try {
			instance->setShadowThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// apply(cv::InputArray, cv::OutputArray, double) /usr/include/opencv2/video/background_segm.hpp:208
	void cv_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR_double(cv::BackgroundSubtractorMOG2* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, Result_void* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask, learningRate);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFinestScale() /usr/include/opencv2/video/tracking.hpp:597
	void cv_DISOpticalFlow_getFinestScale_const(const cv::DISOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFinestScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setFinestScale(int) /usr/include/opencv2/video/tracking.hpp:599
	void cv_DISOpticalFlow_setFinestScale_int(cv::DISOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setFinestScale(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPatchSize() /usr/include/opencv2/video/tracking.hpp:604
	void cv_DISOpticalFlow_getPatchSize_const(const cv::DISOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPatchSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setPatchSize(int) /usr/include/opencv2/video/tracking.hpp:606
	void cv_DISOpticalFlow_setPatchSize_int(cv::DISOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setPatchSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPatchStride() /usr/include/opencv2/video/tracking.hpp:611
	void cv_DISOpticalFlow_getPatchStride_const(const cv::DISOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPatchStride();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setPatchStride(int) /usr/include/opencv2/video/tracking.hpp:613
	void cv_DISOpticalFlow_setPatchStride_int(cv::DISOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setPatchStride(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getGradientDescentIterations() /usr/include/opencv2/video/tracking.hpp:618
	void cv_DISOpticalFlow_getGradientDescentIterations_const(const cv::DISOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getGradientDescentIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setGradientDescentIterations(int) /usr/include/opencv2/video/tracking.hpp:620
	void cv_DISOpticalFlow_setGradientDescentIterations_int(cv::DISOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setGradientDescentIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getVariationalRefinementIterations() /usr/include/opencv2/video/tracking.hpp:626
	void cv_DISOpticalFlow_getVariationalRefinementIterations_const(const cv::DISOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getVariationalRefinementIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setVariationalRefinementIterations(int) /usr/include/opencv2/video/tracking.hpp:628
	void cv_DISOpticalFlow_setVariationalRefinementIterations_int(cv::DISOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setVariationalRefinementIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getVariationalRefinementAlpha() /usr/include/opencv2/video/tracking.hpp:632
	void cv_DISOpticalFlow_getVariationalRefinementAlpha_const(const cv::DISOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getVariationalRefinementAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setVariationalRefinementAlpha(float) /usr/include/opencv2/video/tracking.hpp:634
	void cv_DISOpticalFlow_setVariationalRefinementAlpha_float(cv::DISOpticalFlow* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setVariationalRefinementAlpha(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getVariationalRefinementDelta() /usr/include/opencv2/video/tracking.hpp:638
	void cv_DISOpticalFlow_getVariationalRefinementDelta_const(const cv::DISOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getVariationalRefinementDelta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setVariationalRefinementDelta(float) /usr/include/opencv2/video/tracking.hpp:640
	void cv_DISOpticalFlow_setVariationalRefinementDelta_float(cv::DISOpticalFlow* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setVariationalRefinementDelta(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getVariationalRefinementGamma() /usr/include/opencv2/video/tracking.hpp:644
	void cv_DISOpticalFlow_getVariationalRefinementGamma_const(const cv::DISOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getVariationalRefinementGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setVariationalRefinementGamma(float) /usr/include/opencv2/video/tracking.hpp:646
	void cv_DISOpticalFlow_setVariationalRefinementGamma_float(cv::DISOpticalFlow* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setVariationalRefinementGamma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUseMeanNormalization() /usr/include/opencv2/video/tracking.hpp:654
	void cv_DISOpticalFlow_getUseMeanNormalization_const(const cv::DISOpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseMeanNormalization();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setUseMeanNormalization(bool) /usr/include/opencv2/video/tracking.hpp:656
	void cv_DISOpticalFlow_setUseMeanNormalization_bool(cv::DISOpticalFlow* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setUseMeanNormalization(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUseSpatialPropagation() /usr/include/opencv2/video/tracking.hpp:663
	void cv_DISOpticalFlow_getUseSpatialPropagation_const(const cv::DISOpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseSpatialPropagation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setUseSpatialPropagation(bool) /usr/include/opencv2/video/tracking.hpp:665
	void cv_DISOpticalFlow_setUseSpatialPropagation_bool(cv::DISOpticalFlow* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setUseSpatialPropagation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(int) /usr/include/opencv2/video/tracking.hpp:671
	void cv_DISOpticalFlow_create_int(int preset, Result<cv::Ptr<cv::DISOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DISOpticalFlow> ret = cv::DISOpticalFlow::create(preset);
			Ok(new cv::Ptr<cv::DISOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DISOpticalFlow>*>))
	}
	
	// calc(cv::InputArray, cv::InputArray, cv::InputOutputArray) /usr/include/opencv2/video/tracking.hpp:445
	void cv_DenseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(cv::DenseOpticalFlow* instance, const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow, Result_void* ocvrs_return) {
		try {
			instance->calc(*I0, *I1, *flow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// collectGarbage() /usr/include/opencv2/video/tracking.hpp:448
	void cv_DenseOpticalFlow_collectGarbage(cv::DenseOpticalFlow* instance, Result_void* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNumLevels() /usr/include/opencv2/video/tracking.hpp:478
	void cv_FarnebackOpticalFlow_getNumLevels_const(const cv::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setNumLevels(int) /usr/include/opencv2/video/tracking.hpp:479
	void cv_FarnebackOpticalFlow_setNumLevels_int(cv::FarnebackOpticalFlow* instance, int numLevels, Result_void* ocvrs_return) {
		try {
			instance->setNumLevels(numLevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPyrScale() /usr/include/opencv2/video/tracking.hpp:481
	void cv_FarnebackOpticalFlow_getPyrScale_const(const cv::FarnebackOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getPyrScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setPyrScale(double) /usr/include/opencv2/video/tracking.hpp:482
	void cv_FarnebackOpticalFlow_setPyrScale_double(cv::FarnebackOpticalFlow* instance, double pyrScale, Result_void* ocvrs_return) {
		try {
			instance->setPyrScale(pyrScale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFastPyramids() /usr/include/opencv2/video/tracking.hpp:484
	void cv_FarnebackOpticalFlow_getFastPyramids_const(const cv::FarnebackOpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getFastPyramids();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setFastPyramids(bool) /usr/include/opencv2/video/tracking.hpp:485
	void cv_FarnebackOpticalFlow_setFastPyramids_bool(cv::FarnebackOpticalFlow* instance, bool fastPyramids, Result_void* ocvrs_return) {
		try {
			instance->setFastPyramids(fastPyramids);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWinSize() /usr/include/opencv2/video/tracking.hpp:487
	void cv_FarnebackOpticalFlow_getWinSize_const(const cv::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setWinSize(int) /usr/include/opencv2/video/tracking.hpp:488
	void cv_FarnebackOpticalFlow_setWinSize_int(cv::FarnebackOpticalFlow* instance, int winSize, Result_void* ocvrs_return) {
		try {
			instance->setWinSize(winSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNumIters() /usr/include/opencv2/video/tracking.hpp:490
	void cv_FarnebackOpticalFlow_getNumIters_const(const cv::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumIters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setNumIters(int) /usr/include/opencv2/video/tracking.hpp:491
	void cv_FarnebackOpticalFlow_setNumIters_int(cv::FarnebackOpticalFlow* instance, int numIters, Result_void* ocvrs_return) {
		try {
			instance->setNumIters(numIters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPolyN() /usr/include/opencv2/video/tracking.hpp:493
	void cv_FarnebackOpticalFlow_getPolyN_const(const cv::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPolyN();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setPolyN(int) /usr/include/opencv2/video/tracking.hpp:494
	void cv_FarnebackOpticalFlow_setPolyN_int(cv::FarnebackOpticalFlow* instance, int polyN, Result_void* ocvrs_return) {
		try {
			instance->setPolyN(polyN);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPolySigma() /usr/include/opencv2/video/tracking.hpp:496
	void cv_FarnebackOpticalFlow_getPolySigma_const(const cv::FarnebackOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getPolySigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setPolySigma(double) /usr/include/opencv2/video/tracking.hpp:497
	void cv_FarnebackOpticalFlow_setPolySigma_double(cv::FarnebackOpticalFlow* instance, double polySigma, Result_void* ocvrs_return) {
		try {
			instance->setPolySigma(polySigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFlags() /usr/include/opencv2/video/tracking.hpp:499
	void cv_FarnebackOpticalFlow_getFlags_const(const cv::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFlags();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setFlags(int) /usr/include/opencv2/video/tracking.hpp:500
	void cv_FarnebackOpticalFlow_setFlags_int(cv::FarnebackOpticalFlow* instance, int flags, Result_void* ocvrs_return) {
		try {
			instance->setFlags(flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(int, double, bool, int, int, int, double, int) /usr/include/opencv2/video/tracking.hpp:502
	void cv_FarnebackOpticalFlow_create_int_double_bool_int_int_int_double_int(int numLevels, double pyrScale, bool fastPyramids, int winSize, int numIters, int polyN, double polySigma, int flags, Result<cv::Ptr<cv::FarnebackOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FarnebackOpticalFlow> ret = cv::FarnebackOpticalFlow::create(numLevels, pyrScale, fastPyramids, winSize, numIters, polyN, polySigma, flags);
			Ok(new cv::Ptr<cv::FarnebackOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::FarnebackOpticalFlow>*>))
	}
	
	// statePre /usr/include/opencv2/video/tracking.hpp:393
	cv::Mat* cv_KalmanFilter_getPropStatePre_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->statePre;
			return new cv::Mat(ret);
	}
	
	// statePre /usr/include/opencv2/video/tracking.hpp:393
	void cv_KalmanFilter_setPropStatePre_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
			instance->statePre = *val;
	}
	
	// statePost /usr/include/opencv2/video/tracking.hpp:394
	cv::Mat* cv_KalmanFilter_getPropStatePost_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->statePost;
			return new cv::Mat(ret);
	}
	
	// statePost /usr/include/opencv2/video/tracking.hpp:394
	void cv_KalmanFilter_setPropStatePost_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
			instance->statePost = *val;
	}
	
	// transitionMatrix /usr/include/opencv2/video/tracking.hpp:395
	cv::Mat* cv_KalmanFilter_getPropTransitionMatrix_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->transitionMatrix;
			return new cv::Mat(ret);
	}
	
	// transitionMatrix /usr/include/opencv2/video/tracking.hpp:395
	void cv_KalmanFilter_setPropTransitionMatrix_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
			instance->transitionMatrix = *val;
	}
	
	// controlMatrix /usr/include/opencv2/video/tracking.hpp:396
	cv::Mat* cv_KalmanFilter_getPropControlMatrix_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->controlMatrix;
			return new cv::Mat(ret);
	}
	
	// controlMatrix /usr/include/opencv2/video/tracking.hpp:396
	void cv_KalmanFilter_setPropControlMatrix_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
			instance->controlMatrix = *val;
	}
	
	// measurementMatrix /usr/include/opencv2/video/tracking.hpp:397
	cv::Mat* cv_KalmanFilter_getPropMeasurementMatrix_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->measurementMatrix;
			return new cv::Mat(ret);
	}
	
	// measurementMatrix /usr/include/opencv2/video/tracking.hpp:397
	void cv_KalmanFilter_setPropMeasurementMatrix_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
			instance->measurementMatrix = *val;
	}
	
	// processNoiseCov /usr/include/opencv2/video/tracking.hpp:398
	cv::Mat* cv_KalmanFilter_getPropProcessNoiseCov_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->processNoiseCov;
			return new cv::Mat(ret);
	}
	
	// processNoiseCov /usr/include/opencv2/video/tracking.hpp:398
	void cv_KalmanFilter_setPropProcessNoiseCov_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
			instance->processNoiseCov = *val;
	}
	
	// measurementNoiseCov /usr/include/opencv2/video/tracking.hpp:399
	cv::Mat* cv_KalmanFilter_getPropMeasurementNoiseCov_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->measurementNoiseCov;
			return new cv::Mat(ret);
	}
	
	// measurementNoiseCov /usr/include/opencv2/video/tracking.hpp:399
	void cv_KalmanFilter_setPropMeasurementNoiseCov_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
			instance->measurementNoiseCov = *val;
	}
	
	// errorCovPre /usr/include/opencv2/video/tracking.hpp:400
	cv::Mat* cv_KalmanFilter_getPropErrorCovPre_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->errorCovPre;
			return new cv::Mat(ret);
	}
	
	// errorCovPre /usr/include/opencv2/video/tracking.hpp:400
	void cv_KalmanFilter_setPropErrorCovPre_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
			instance->errorCovPre = *val;
	}
	
	// gain /usr/include/opencv2/video/tracking.hpp:401
	cv::Mat* cv_KalmanFilter_getPropGain_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->gain;
			return new cv::Mat(ret);
	}
	
	// gain /usr/include/opencv2/video/tracking.hpp:401
	void cv_KalmanFilter_setPropGain_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
			instance->gain = *val;
	}
	
	// errorCovPost /usr/include/opencv2/video/tracking.hpp:402
	cv::Mat* cv_KalmanFilter_getPropErrorCovPost_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->errorCovPost;
			return new cv::Mat(ret);
	}
	
	// errorCovPost /usr/include/opencv2/video/tracking.hpp:402
	void cv_KalmanFilter_setPropErrorCovPost_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
			instance->errorCovPost = *val;
	}
	
	// temp1 /usr/include/opencv2/video/tracking.hpp:405
	cv::Mat* cv_KalmanFilter_getPropTemp1_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->temp1;
			return new cv::Mat(ret);
	}
	
	// temp1 /usr/include/opencv2/video/tracking.hpp:405
	void cv_KalmanFilter_setPropTemp1_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
			instance->temp1 = *val;
	}
	
	// temp2 /usr/include/opencv2/video/tracking.hpp:406
	cv::Mat* cv_KalmanFilter_getPropTemp2_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->temp2;
			return new cv::Mat(ret);
	}
	
	// temp2 /usr/include/opencv2/video/tracking.hpp:406
	void cv_KalmanFilter_setPropTemp2_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
			instance->temp2 = *val;
	}
	
	// temp3 /usr/include/opencv2/video/tracking.hpp:407
	cv::Mat* cv_KalmanFilter_getPropTemp3_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->temp3;
			return new cv::Mat(ret);
	}
	
	// temp3 /usr/include/opencv2/video/tracking.hpp:407
	void cv_KalmanFilter_setPropTemp3_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
			instance->temp3 = *val;
	}
	
	// temp4 /usr/include/opencv2/video/tracking.hpp:408
	cv::Mat* cv_KalmanFilter_getPropTemp4_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->temp4;
			return new cv::Mat(ret);
	}
	
	// temp4 /usr/include/opencv2/video/tracking.hpp:408
	void cv_KalmanFilter_setPropTemp4_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
			instance->temp4 = *val;
	}
	
	// temp5 /usr/include/opencv2/video/tracking.hpp:409
	cv::Mat* cv_KalmanFilter_getPropTemp5_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->temp5;
			return new cv::Mat(ret);
	}
	
	// temp5 /usr/include/opencv2/video/tracking.hpp:409
	void cv_KalmanFilter_setPropTemp5_Mat(cv::KalmanFilter* instance, cv::Mat* val) {
			instance->temp5 = *val;
	}
	
	void cv_KalmanFilter_delete(cv::KalmanFilter* instance) {
		delete instance;
	}
	// KalmanFilter() /usr/include/opencv2/video/tracking.hpp:363
	void cv_KalmanFilter_KalmanFilter(Result<cv::KalmanFilter*>* ocvrs_return) {
		try {
			cv::KalmanFilter* ret = new cv::KalmanFilter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::KalmanFilter*>))
	}
	
	// KalmanFilter(int, int, int, int) /usr/include/opencv2/video/tracking.hpp:370
	void cv_KalmanFilter_KalmanFilter_int_int_int_int(int dynamParams, int measureParams, int controlParams, int type, Result<cv::KalmanFilter*>* ocvrs_return) {
		try {
			cv::KalmanFilter* ret = new cv::KalmanFilter(dynamParams, measureParams, controlParams, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::KalmanFilter*>))
	}
	
	// init(int, int, int, int) /usr/include/opencv2/video/tracking.hpp:379
	void cv_KalmanFilter_init_int_int_int_int(cv::KalmanFilter* instance, int dynamParams, int measureParams, int controlParams, int type, Result_void* ocvrs_return) {
		try {
			instance->init(dynamParams, measureParams, controlParams, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// predict(const cv::Mat &) /usr/include/opencv2/video/tracking.hpp:385
	void cv_KalmanFilter_predict_const_MatR(cv::KalmanFilter* instance, const cv::Mat* control, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->predict(*control);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// correct(const cv::Mat &) /usr/include/opencv2/video/tracking.hpp:391
	void cv_KalmanFilter_correct_const_MatR(cv::KalmanFilter* instance, const cv::Mat* measurement, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->correct(*measurement);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// calc(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/video/tracking.hpp:466
	void cv_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::SparseOpticalFlow* instance, const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, const cv::_OutputArray* err, Result_void* ocvrs_return) {
		try {
			instance->calc(*prevImg, *nextImg, *prevPts, *nextPts, *status, *err);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWinSize() /usr/include/opencv2/video/tracking.hpp:685
	void cv_SparsePyrLKOpticalFlow_getWinSize_const(const cv::SparsePyrLKOpticalFlow* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getWinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// setWinSize(cv::Size) /usr/include/opencv2/video/tracking.hpp:686
	void cv_SparsePyrLKOpticalFlow_setWinSize_Size(cv::SparsePyrLKOpticalFlow* instance, cv::Size* winSize, Result_void* ocvrs_return) {
		try {
			instance->setWinSize(*winSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxLevel() /usr/include/opencv2/video/tracking.hpp:688
	void cv_SparsePyrLKOpticalFlow_getMaxLevel_const(const cv::SparsePyrLKOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMaxLevel(int) /usr/include/opencv2/video/tracking.hpp:689
	void cv_SparsePyrLKOpticalFlow_setMaxLevel_int(cv::SparsePyrLKOpticalFlow* instance, int maxLevel, Result_void* ocvrs_return) {
		try {
			instance->setMaxLevel(maxLevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTermCriteria() /usr/include/opencv2/video/tracking.hpp:691
	void cv_SparsePyrLKOpticalFlow_getTermCriteria_const(const cv::SparsePyrLKOpticalFlow* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TermCriteria>))
	}
	
	// setTermCriteria(cv::TermCriteria &) /usr/include/opencv2/video/tracking.hpp:692
	void cv_SparsePyrLKOpticalFlow_setTermCriteria_TermCriteriaR(cv::SparsePyrLKOpticalFlow* instance, cv::TermCriteria* crit, Result_void* ocvrs_return) {
		try {
			instance->setTermCriteria(*crit);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFlags() /usr/include/opencv2/video/tracking.hpp:694
	void cv_SparsePyrLKOpticalFlow_getFlags_const(const cv::SparsePyrLKOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFlags();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setFlags(int) /usr/include/opencv2/video/tracking.hpp:695
	void cv_SparsePyrLKOpticalFlow_setFlags_int(cv::SparsePyrLKOpticalFlow* instance, int flags, Result_void* ocvrs_return) {
		try {
			instance->setFlags(flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinEigThreshold() /usr/include/opencv2/video/tracking.hpp:697
	void cv_SparsePyrLKOpticalFlow_getMinEigThreshold_const(const cv::SparsePyrLKOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinEigThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMinEigThreshold(double) /usr/include/opencv2/video/tracking.hpp:698
	void cv_SparsePyrLKOpticalFlow_setMinEigThreshold_double(cv::SparsePyrLKOpticalFlow* instance, double minEigThreshold, Result_void* ocvrs_return) {
		try {
			instance->setMinEigThreshold(minEigThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(cv::Size, int, cv::TermCriteria, int, double) /usr/include/opencv2/video/tracking.hpp:700
	void cv_SparsePyrLKOpticalFlow_create_Size_int_TermCriteria_int_double(cv::Size* winSize, int maxLevel, cv::TermCriteria* crit, int flags, double minEigThreshold, Result<cv::Ptr<cv::SparsePyrLKOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SparsePyrLKOpticalFlow> ret = cv::SparsePyrLKOpticalFlow::create(*winSize, maxLevel, *crit, flags, minEigThreshold);
			Ok(new cv::Ptr<cv::SparsePyrLKOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::SparsePyrLKOpticalFlow>*>))
	}
	
	// init(cv::InputArray, const cv::Rect &) /usr/include/opencv2/video/tracking.hpp:725
	void cv_Tracker_init_const__InputArrayR_const_RectR(cv::Tracker* instance, const cv::_InputArray* image, const cv::Rect* boundingBox, Result_void* ocvrs_return) {
		try {
			instance->init(*image, *boundingBox);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// update(cv::InputArray, cv::Rect &) /usr/include/opencv2/video/tracking.hpp:737
	void cv_Tracker_update_const__InputArrayR_RectR(cv::Tracker* instance, const cv::_InputArray* image, cv::Rect* boundingBox, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->update(*image, *boundingBox);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// create(const TrackerDaSiamRPN::Params &) /usr/include/opencv2/video/tracking.hpp:842
	void cv_TrackerDaSiamRPN_create_const_ParamsR(const cv::TrackerDaSiamRPN::Params* parameters, Result<cv::Ptr<cv::TrackerDaSiamRPN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerDaSiamRPN> ret = cv::TrackerDaSiamRPN::create(*parameters);
			Ok(new cv::Ptr<cv::TrackerDaSiamRPN>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerDaSiamRPN>*>))
	}
	
	// getTrackingScore() /usr/include/opencv2/video/tracking.hpp:846
	void cv_TrackerDaSiamRPN_getTrackingScore(cv::TrackerDaSiamRPN* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getTrackingScore();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// model /usr/include/opencv2/video/tracking.hpp:831
	void* cv_TrackerDaSiamRPN_Params_getPropModel_const(const cv::TrackerDaSiamRPN::Params* instance) {
			std::string ret = instance->model;
			return ocvrs_create_string(ret.c_str());
	}
	
	// model /usr/include/opencv2/video/tracking.hpp:831
	void cv_TrackerDaSiamRPN_Params_setPropModel_string(cv::TrackerDaSiamRPN::Params* instance, char* val) {
			instance->model = std::string(val);
	}
	
	// kernel_cls1 /usr/include/opencv2/video/tracking.hpp:832
	void* cv_TrackerDaSiamRPN_Params_getPropKernel_cls1_const(const cv::TrackerDaSiamRPN::Params* instance) {
			std::string ret = instance->kernel_cls1;
			return ocvrs_create_string(ret.c_str());
	}
	
	// kernel_cls1 /usr/include/opencv2/video/tracking.hpp:832
	void cv_TrackerDaSiamRPN_Params_setPropKernel_cls1_string(cv::TrackerDaSiamRPN::Params* instance, char* val) {
			instance->kernel_cls1 = std::string(val);
	}
	
	// kernel_r1 /usr/include/opencv2/video/tracking.hpp:833
	void* cv_TrackerDaSiamRPN_Params_getPropKernel_r1_const(const cv::TrackerDaSiamRPN::Params* instance) {
			std::string ret = instance->kernel_r1;
			return ocvrs_create_string(ret.c_str());
	}
	
	// kernel_r1 /usr/include/opencv2/video/tracking.hpp:833
	void cv_TrackerDaSiamRPN_Params_setPropKernel_r1_string(cv::TrackerDaSiamRPN::Params* instance, char* val) {
			instance->kernel_r1 = std::string(val);
	}
	
	// backend /usr/include/opencv2/video/tracking.hpp:834
	int cv_TrackerDaSiamRPN_Params_getPropBackend_const(const cv::TrackerDaSiamRPN::Params* instance) {
			int ret = instance->backend;
			return ret;
	}
	
	// backend /usr/include/opencv2/video/tracking.hpp:834
	void cv_TrackerDaSiamRPN_Params_setPropBackend_int(cv::TrackerDaSiamRPN::Params* instance, int val) {
			instance->backend = val;
	}
	
	// target /usr/include/opencv2/video/tracking.hpp:835
	int cv_TrackerDaSiamRPN_Params_getPropTarget_const(const cv::TrackerDaSiamRPN::Params* instance) {
			int ret = instance->target;
			return ret;
	}
	
	// target /usr/include/opencv2/video/tracking.hpp:835
	void cv_TrackerDaSiamRPN_Params_setPropTarget_int(cv::TrackerDaSiamRPN::Params* instance, int val) {
			instance->target = val;
	}
	
	void cv_TrackerDaSiamRPN_Params_delete(cv::TrackerDaSiamRPN::Params* instance) {
		delete instance;
	}
	// Params() /usr/include/opencv2/video/tracking.hpp:830
	void cv_TrackerDaSiamRPN_Params_Params(Result<cv::TrackerDaSiamRPN::Params*>* ocvrs_return) {
		try {
			cv::TrackerDaSiamRPN::Params* ret = new cv::TrackerDaSiamRPN::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerDaSiamRPN::Params*>))
	}
	
	// create(const TrackerGOTURN::Params &) /usr/include/opencv2/video/tracking.hpp:815
	void cv_TrackerGOTURN_create_const_ParamsR(const cv::TrackerGOTURN::Params* parameters, Result<cv::Ptr<cv::TrackerGOTURN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerGOTURN> ret = cv::TrackerGOTURN::create(*parameters);
			Ok(new cv::Ptr<cv::TrackerGOTURN>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerGOTURN>*>))
	}
	
	// modelTxt /usr/include/opencv2/video/tracking.hpp:807
	void* cv_TrackerGOTURN_Params_getPropModelTxt_const(const cv::TrackerGOTURN::Params* instance) {
			std::string ret = instance->modelTxt;
			return ocvrs_create_string(ret.c_str());
	}
	
	// modelTxt /usr/include/opencv2/video/tracking.hpp:807
	void cv_TrackerGOTURN_Params_setPropModelTxt_string(cv::TrackerGOTURN::Params* instance, char* val) {
			instance->modelTxt = std::string(val);
	}
	
	// modelBin /usr/include/opencv2/video/tracking.hpp:808
	void* cv_TrackerGOTURN_Params_getPropModelBin_const(const cv::TrackerGOTURN::Params* instance) {
			std::string ret = instance->modelBin;
			return ocvrs_create_string(ret.c_str());
	}
	
	// modelBin /usr/include/opencv2/video/tracking.hpp:808
	void cv_TrackerGOTURN_Params_setPropModelBin_string(cv::TrackerGOTURN::Params* instance, char* val) {
			instance->modelBin = std::string(val);
	}
	
	void cv_TrackerGOTURN_Params_delete(cv::TrackerGOTURN::Params* instance) {
		delete instance;
	}
	// Params() /usr/include/opencv2/video/tracking.hpp:806
	void cv_TrackerGOTURN_Params_Params(Result<cv::TrackerGOTURN::Params*>* ocvrs_return) {
		try {
			cv::TrackerGOTURN::Params* ret = new cv::TrackerGOTURN::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerGOTURN::Params*>))
	}
	
	// create(const TrackerMIL::Params &) /usr/include/opencv2/video/tracking.hpp:774
	void cv_TrackerMIL_create_const_ParamsR(const cv::TrackerMIL::Params* parameters, Result<cv::Ptr<cv::TrackerMIL>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerMIL> ret = cv::TrackerMIL::create(*parameters);
			Ok(new cv::Ptr<cv::TrackerMIL>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerMIL>*>))
	}
	
	// Params() /usr/include/opencv2/video/tracking.hpp:759
	void cv_TrackerMIL_Params_Params(Result<cv::TrackerMIL::Params>* ocvrs_return) {
		try {
			cv::TrackerMIL::Params ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerMIL::Params>))
	}
	
	// calcUV(cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray) /usr/include/opencv2/video/tracking.hpp:528
	void cv_VariationalRefinement_calcUV_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(cv::VariationalRefinement* instance, const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow_u, const cv::_InputOutputArray* flow_v, Result_void* ocvrs_return) {
		try {
			instance->calcUV(*I0, *I1, *flow_u, *flow_v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFixedPointIterations() /usr/include/opencv2/video/tracking.hpp:532
	void cv_VariationalRefinement_getFixedPointIterations_const(const cv::VariationalRefinement* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFixedPointIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setFixedPointIterations(int) /usr/include/opencv2/video/tracking.hpp:534
	void cv_VariationalRefinement_setFixedPointIterations_int(cv::VariationalRefinement* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setFixedPointIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSorIterations() /usr/include/opencv2/video/tracking.hpp:539
	void cv_VariationalRefinement_getSorIterations_const(const cv::VariationalRefinement* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSorIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setSorIterations(int) /usr/include/opencv2/video/tracking.hpp:541
	void cv_VariationalRefinement_setSorIterations_int(cv::VariationalRefinement* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setSorIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getOmega() /usr/include/opencv2/video/tracking.hpp:545
	void cv_VariationalRefinement_getOmega_const(const cv::VariationalRefinement* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getOmega();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setOmega(float) /usr/include/opencv2/video/tracking.hpp:547
	void cv_VariationalRefinement_setOmega_float(cv::VariationalRefinement* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setOmega(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getAlpha() /usr/include/opencv2/video/tracking.hpp:551
	void cv_VariationalRefinement_getAlpha_const(const cv::VariationalRefinement* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setAlpha(float) /usr/include/opencv2/video/tracking.hpp:553
	void cv_VariationalRefinement_setAlpha_float(cv::VariationalRefinement* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setAlpha(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDelta() /usr/include/opencv2/video/tracking.hpp:557
	void cv_VariationalRefinement_getDelta_const(const cv::VariationalRefinement* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getDelta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setDelta(float) /usr/include/opencv2/video/tracking.hpp:559
	void cv_VariationalRefinement_setDelta_float(cv::VariationalRefinement* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setDelta(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getGamma() /usr/include/opencv2/video/tracking.hpp:563
	void cv_VariationalRefinement_getGamma_const(const cv::VariationalRefinement* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setGamma(float) /usr/include/opencv2/video/tracking.hpp:565
	void cv_VariationalRefinement_setGamma_float(cv::VariationalRefinement* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setGamma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create() /usr/include/opencv2/video/tracking.hpp:569
	void cv_VariationalRefinement_create(Result<cv::Ptr<cv::VariationalRefinement>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::VariationalRefinement> ret = cv::VariationalRefinement::create();
			Ok(new cv::Ptr<cv::VariationalRefinement>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::VariationalRefinement>*>))
	}
	
}
