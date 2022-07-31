#include "ocvrs_common.hpp"
#include <opencv2/bgsegm.hpp>
#include "bgsegm_types.hpp"

extern "C" {
	// createBackgroundSubtractorCNT(int, bool, int, bool) /usr/include/opencv2/bgsegm.hpp:240
	void cv_bgsegm_createBackgroundSubtractorCNT_int_bool_int_bool(int minPixelStability, bool useHistory, int maxPixelStability, bool isParallel, Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT> ret = cv::bgsegm::createBackgroundSubtractorCNT(minPixelStability, useHistory, maxPixelStability, isParallel);
			Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>*>))
	}
	
	// createBackgroundSubtractorGMG(int, double) /usr/include/opencv2/bgsegm.hpp:185
	void cv_bgsegm_createBackgroundSubtractorGMG_int_double(int initializationFrames, double decisionThreshold, Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG> ret = cv::bgsegm::createBackgroundSubtractorGMG(initializationFrames, decisionThreshold);
			Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>*>))
	}
	
	// createBackgroundSubtractorGSOC(int, int, float, float, int, float, float, float, float, float, float) /usr/include/opencv2/bgsegm.hpp:302
	void cv_bgsegm_createBackgroundSubtractorGSOC_int_int_float_float_int_float_float_float_float_float_float(int mc, int nSamples, float replaceRate, float propagationRate, int hitsThreshold, float alpha, float beta, float blinkingSupressionDecay, float blinkingSupressionMultiplier, float noiseRemovalThresholdFacBG, float noiseRemovalThresholdFacFG, Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC> ret = cv::bgsegm::createBackgroundSubtractorGSOC(mc, nSamples, replaceRate, propagationRate, hitsThreshold, alpha, beta, blinkingSupressionDecay, blinkingSupressionMultiplier, noiseRemovalThresholdFacBG, noiseRemovalThresholdFacFG);
			Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>*>))
	}
	
	// createBackgroundSubtractorLSBP(int, int, int, float, float, float, float, float, float, float, float, int, int) /usr/include/opencv2/bgsegm.hpp:322
	void cv_bgsegm_createBackgroundSubtractorLSBP_int_int_int_float_float_float_float_float_float_float_float_int_int(int mc, int nSamples, int LSBPRadius, float Tlower, float Tupper, float Tinc, float Tdec, float Rscale, float Rincdec, float noiseRemovalThresholdFacBG, float noiseRemovalThresholdFacFG, int LSBPthreshold, int minCount, Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP> ret = cv::bgsegm::createBackgroundSubtractorLSBP(mc, nSamples, LSBPRadius, Tlower, Tupper, Tinc, Tdec, Rscale, Rincdec, noiseRemovalThresholdFacBG, noiseRemovalThresholdFacFG, LSBPthreshold, minCount);
			Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>*>))
	}
	
	// createBackgroundSubtractorMOG(int, int, double, double) /usr/include/opencv2/bgsegm.hpp:87
	void cv_bgsegm_createBackgroundSubtractorMOG_int_int_double_double(int history, int nmixtures, double backgroundRatio, double noiseSigma, Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG> ret = cv::bgsegm::createBackgroundSubtractorMOG(history, nmixtures, backgroundRatio, noiseSigma);
			Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>*>))
	}
	
	// createSyntheticSequenceGenerator(cv::InputArray, cv::InputArray, double, double, double, double) /usr/include/opencv2/bgsegm.hpp:372
	void cv_bgsegm_createSyntheticSequenceGenerator_const__InputArrayR_const__InputArrayR_double_double_double_double(const cv::_InputArray* background, const cv::_InputArray* object, double amplitude, double wavelength, double wavespeed, double objspeed, Result<cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator> ret = cv::bgsegm::createSyntheticSequenceGenerator(*background, *object, amplitude, wavelength, wavespeed, objspeed);
			Ok(new cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>*>))
	}
	
	// apply(cv::InputArray, cv::OutputArray, double) /usr/include/opencv2/bgsegm.hpp:199
	void cv_bgsegm_BackgroundSubtractorCNT_apply_const__InputArrayR_const__OutputArrayR_double(cv::bgsegm::BackgroundSubtractorCNT* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, Result_void* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask, learningRate);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getBackgroundImage(cv::OutputArray) /usr/include/opencv2/bgsegm.hpp:200
	void cv_bgsegm_BackgroundSubtractorCNT_getBackgroundImage_const_const__OutputArrayR(const cv::bgsegm::BackgroundSubtractorCNT* instance, const cv::_OutputArray* backgroundImage, Result_void* ocvrs_return) {
		try {
			instance->getBackgroundImage(*backgroundImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinPixelStability() /usr/include/opencv2/bgsegm.hpp:204
	void cv_bgsegm_BackgroundSubtractorCNT_getMinPixelStability_const(const cv::bgsegm::BackgroundSubtractorCNT* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinPixelStability();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMinPixelStability(int) /usr/include/opencv2/bgsegm.hpp:207
	void cv_bgsegm_BackgroundSubtractorCNT_setMinPixelStability_int(cv::bgsegm::BackgroundSubtractorCNT* instance, int value, Result_void* ocvrs_return) {
		try {
			instance->setMinPixelStability(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxPixelStability() /usr/include/opencv2/bgsegm.hpp:211
	void cv_bgsegm_BackgroundSubtractorCNT_getMaxPixelStability_const(const cv::bgsegm::BackgroundSubtractorCNT* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxPixelStability();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMaxPixelStability(int) /usr/include/opencv2/bgsegm.hpp:214
	void cv_bgsegm_BackgroundSubtractorCNT_setMaxPixelStability_int(cv::bgsegm::BackgroundSubtractorCNT* instance, int value, Result_void* ocvrs_return) {
		try {
			instance->setMaxPixelStability(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUseHistory() /usr/include/opencv2/bgsegm.hpp:218
	void cv_bgsegm_BackgroundSubtractorCNT_getUseHistory_const(const cv::bgsegm::BackgroundSubtractorCNT* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseHistory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setUseHistory(bool) /usr/include/opencv2/bgsegm.hpp:221
	void cv_bgsegm_BackgroundSubtractorCNT_setUseHistory_bool(cv::bgsegm::BackgroundSubtractorCNT* instance, bool value, Result_void* ocvrs_return) {
		try {
			instance->setUseHistory(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getIsParallel() /usr/include/opencv2/bgsegm.hpp:225
	void cv_bgsegm_BackgroundSubtractorCNT_getIsParallel_const(const cv::bgsegm::BackgroundSubtractorCNT* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getIsParallel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setIsParallel(bool) /usr/include/opencv2/bgsegm.hpp:228
	void cv_bgsegm_BackgroundSubtractorCNT_setIsParallel_bool(cv::bgsegm::BackgroundSubtractorCNT* instance, bool value, Result_void* ocvrs_return) {
		try {
			instance->setIsParallel(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxFeatures() /usr/include/opencv2/bgsegm.hpp:104
	void cv_bgsegm_BackgroundSubtractorGMG_getMaxFeatures_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxFeatures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMaxFeatures(int) /usr/include/opencv2/bgsegm.hpp:107
	void cv_bgsegm_BackgroundSubtractorGMG_setMaxFeatures_int(cv::bgsegm::BackgroundSubtractorGMG* instance, int maxFeatures, Result_void* ocvrs_return) {
		try {
			instance->setMaxFeatures(maxFeatures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDefaultLearningRate() /usr/include/opencv2/bgsegm.hpp:114
	void cv_bgsegm_BackgroundSubtractorGMG_getDefaultLearningRate_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDefaultLearningRate();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setDefaultLearningRate(double) /usr/include/opencv2/bgsegm.hpp:117
	void cv_bgsegm_BackgroundSubtractorGMG_setDefaultLearningRate_double(cv::bgsegm::BackgroundSubtractorGMG* instance, double lr, Result_void* ocvrs_return) {
		try {
			instance->setDefaultLearningRate(lr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNumFrames() /usr/include/opencv2/bgsegm.hpp:121
	void cv_bgsegm_BackgroundSubtractorGMG_getNumFrames_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumFrames();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setNumFrames(int) /usr/include/opencv2/bgsegm.hpp:124
	void cv_bgsegm_BackgroundSubtractorGMG_setNumFrames_int(cv::bgsegm::BackgroundSubtractorGMG* instance, int nframes, Result_void* ocvrs_return) {
		try {
			instance->setNumFrames(nframes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getQuantizationLevels() /usr/include/opencv2/bgsegm.hpp:130
	void cv_bgsegm_BackgroundSubtractorGMG_getQuantizationLevels_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getQuantizationLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setQuantizationLevels(int) /usr/include/opencv2/bgsegm.hpp:133
	void cv_bgsegm_BackgroundSubtractorGMG_setQuantizationLevels_int(cv::bgsegm::BackgroundSubtractorGMG* instance, int nlevels, Result_void* ocvrs_return) {
		try {
			instance->setQuantizationLevels(nlevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getBackgroundPrior() /usr/include/opencv2/bgsegm.hpp:137
	void cv_bgsegm_BackgroundSubtractorGMG_getBackgroundPrior_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getBackgroundPrior();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setBackgroundPrior(double) /usr/include/opencv2/bgsegm.hpp:140
	void cv_bgsegm_BackgroundSubtractorGMG_setBackgroundPrior_double(cv::bgsegm::BackgroundSubtractorGMG* instance, double bgprior, Result_void* ocvrs_return) {
		try {
			instance->setBackgroundPrior(bgprior);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSmoothingRadius() /usr/include/opencv2/bgsegm.hpp:144
	void cv_bgsegm_BackgroundSubtractorGMG_getSmoothingRadius_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSmoothingRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setSmoothingRadius(int) /usr/include/opencv2/bgsegm.hpp:147
	void cv_bgsegm_BackgroundSubtractorGMG_setSmoothingRadius_int(cv::bgsegm::BackgroundSubtractorGMG* instance, int radius, Result_void* ocvrs_return) {
		try {
			instance->setSmoothingRadius(radius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDecisionThreshold() /usr/include/opencv2/bgsegm.hpp:153
	void cv_bgsegm_BackgroundSubtractorGMG_getDecisionThreshold_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDecisionThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setDecisionThreshold(double) /usr/include/opencv2/bgsegm.hpp:156
	void cv_bgsegm_BackgroundSubtractorGMG_setDecisionThreshold_double(cv::bgsegm::BackgroundSubtractorGMG* instance, double thresh, Result_void* ocvrs_return) {
		try {
			instance->setDecisionThreshold(thresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUpdateBackgroundModel() /usr/include/opencv2/bgsegm.hpp:160
	void cv_bgsegm_BackgroundSubtractorGMG_getUpdateBackgroundModel_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUpdateBackgroundModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setUpdateBackgroundModel(bool) /usr/include/opencv2/bgsegm.hpp:163
	void cv_bgsegm_BackgroundSubtractorGMG_setUpdateBackgroundModel_bool(cv::bgsegm::BackgroundSubtractorGMG* instance, bool update, Result_void* ocvrs_return) {
		try {
			instance->setUpdateBackgroundModel(update);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinVal() /usr/include/opencv2/bgsegm.hpp:167
	void cv_bgsegm_BackgroundSubtractorGMG_getMinVal_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinVal();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMinVal(double) /usr/include/opencv2/bgsegm.hpp:170
	void cv_bgsegm_BackgroundSubtractorGMG_setMinVal_double(cv::bgsegm::BackgroundSubtractorGMG* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMinVal(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxVal() /usr/include/opencv2/bgsegm.hpp:174
	void cv_bgsegm_BackgroundSubtractorGMG_getMaxVal_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxVal();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxVal(double) /usr/include/opencv2/bgsegm.hpp:177
	void cv_bgsegm_BackgroundSubtractorGMG_setMaxVal_double(cv::bgsegm::BackgroundSubtractorGMG* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMaxVal(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// apply(cv::InputArray, cv::OutputArray, double) /usr/include/opencv2/bgsegm.hpp:258
	void cv_bgsegm_BackgroundSubtractorGSOC_apply_const__InputArrayR_const__OutputArrayR_double(cv::bgsegm::BackgroundSubtractorGSOC* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, Result_void* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask, learningRate);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getBackgroundImage(cv::OutputArray) /usr/include/opencv2/bgsegm.hpp:260
	void cv_bgsegm_BackgroundSubtractorGSOC_getBackgroundImage_const_const__OutputArrayR(const cv::bgsegm::BackgroundSubtractorGSOC* instance, const cv::_OutputArray* backgroundImage, Result_void* ocvrs_return) {
		try {
			instance->getBackgroundImage(*backgroundImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// apply(cv::InputArray, cv::OutputArray, double) /usr/include/opencv2/bgsegm.hpp:269
	void cv_bgsegm_BackgroundSubtractorLSBP_apply_const__InputArrayR_const__OutputArrayR_double(cv::bgsegm::BackgroundSubtractorLSBP* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, Result_void* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask, learningRate);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getBackgroundImage(cv::OutputArray) /usr/include/opencv2/bgsegm.hpp:271
	void cv_bgsegm_BackgroundSubtractorLSBP_getBackgroundImage_const_const__OutputArrayR(const cv::bgsegm::BackgroundSubtractorLSBP* instance, const cv::_OutputArray* backgroundImage, Result_void* ocvrs_return) {
		try {
			instance->getBackgroundImage(*backgroundImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_BackgroundSubtractorLSBPDesc_delete(cv::bgsegm::BackgroundSubtractorLSBPDesc* instance) {
		delete instance;
	}
	// calcLocalSVDValues(cv::OutputArray, const cv::Mat &) /usr/include/opencv2/bgsegm.hpp:279
	void cv_bgsegm_BackgroundSubtractorLSBPDesc_calcLocalSVDValues_const__OutputArrayR_const_MatR(const cv::_OutputArray* localSVDValues, const cv::Mat* frame, Result_void* ocvrs_return) {
		try {
			cv::bgsegm::BackgroundSubtractorLSBPDesc::calcLocalSVDValues(*localSVDValues, *frame);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// computeFromLocalSVDValues(cv::OutputArray, const cv::Mat &, const cv::Point2i *) /usr/include/opencv2/bgsegm.hpp:281
	void cv_bgsegm_BackgroundSubtractorLSBPDesc_computeFromLocalSVDValues_const__OutputArrayR_const_MatR_const_Point2iX(const cv::_OutputArray* desc, const cv::Mat* localSVDValues, const cv::Point2i* LSBPSamplePoints, Result_void* ocvrs_return) {
		try {
			cv::bgsegm::BackgroundSubtractorLSBPDesc::computeFromLocalSVDValues(*desc, *localSVDValues, LSBPSamplePoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// compute(cv::OutputArray, const cv::Mat &, const cv::Point2i *) /usr/include/opencv2/bgsegm.hpp:283
	void cv_bgsegm_BackgroundSubtractorLSBPDesc_compute_const__OutputArrayR_const_MatR_const_Point2iX(const cv::_OutputArray* desc, const cv::Mat* frame, const cv::Point2i* LSBPSamplePoints, Result_void* ocvrs_return) {
		try {
			cv::bgsegm::BackgroundSubtractorLSBPDesc::compute(*desc, *frame, LSBPSamplePoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getHistory() /usr/include/opencv2/bgsegm.hpp:65
	void cv_bgsegm_BackgroundSubtractorMOG_getHistory_const(const cv::bgsegm::BackgroundSubtractorMOG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getHistory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setHistory(int) /usr/include/opencv2/bgsegm.hpp:66
	void cv_bgsegm_BackgroundSubtractorMOG_setHistory_int(cv::bgsegm::BackgroundSubtractorMOG* instance, int nframes, Result_void* ocvrs_return) {
		try {
			instance->setHistory(nframes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNMixtures() /usr/include/opencv2/bgsegm.hpp:68
	void cv_bgsegm_BackgroundSubtractorMOG_getNMixtures_const(const cv::bgsegm::BackgroundSubtractorMOG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNMixtures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setNMixtures(int) /usr/include/opencv2/bgsegm.hpp:69
	void cv_bgsegm_BackgroundSubtractorMOG_setNMixtures_int(cv::bgsegm::BackgroundSubtractorMOG* instance, int nmix, Result_void* ocvrs_return) {
		try {
			instance->setNMixtures(nmix);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getBackgroundRatio() /usr/include/opencv2/bgsegm.hpp:71
	void cv_bgsegm_BackgroundSubtractorMOG_getBackgroundRatio_const(const cv::bgsegm::BackgroundSubtractorMOG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getBackgroundRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setBackgroundRatio(double) /usr/include/opencv2/bgsegm.hpp:72
	void cv_bgsegm_BackgroundSubtractorMOG_setBackgroundRatio_double(cv::bgsegm::BackgroundSubtractorMOG* instance, double backgroundRatio, Result_void* ocvrs_return) {
		try {
			instance->setBackgroundRatio(backgroundRatio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNoiseSigma() /usr/include/opencv2/bgsegm.hpp:74
	void cv_bgsegm_BackgroundSubtractorMOG_getNoiseSigma_const(const cv::bgsegm::BackgroundSubtractorMOG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getNoiseSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setNoiseSigma(double) /usr/include/opencv2/bgsegm.hpp:75
	void cv_bgsegm_BackgroundSubtractorMOG_setNoiseSigma_double(cv::bgsegm::BackgroundSubtractorMOG* instance, double noiseSigma, Result_void* ocvrs_return) {
		try {
			instance->setNoiseSigma(noiseSigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::Algorithm* cv_SyntheticSequenceGenerator_to_Algorithm(cv::bgsegm::SyntheticSequenceGenerator* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_SyntheticSequenceGenerator_delete(cv::bgsegm::SyntheticSequenceGenerator* instance) {
		delete instance;
	}
	// SyntheticSequenceGenerator(cv::InputArray, cv::InputArray, double, double, double, double) /usr/include/opencv2/bgsegm.hpp:353
	void cv_bgsegm_SyntheticSequenceGenerator_SyntheticSequenceGenerator_const__InputArrayR_const__InputArrayR_double_double_double_double(const cv::_InputArray* background, const cv::_InputArray* object, double amplitude, double wavelength, double wavespeed, double objspeed, Result<cv::bgsegm::SyntheticSequenceGenerator*>* ocvrs_return) {
		try {
			cv::bgsegm::SyntheticSequenceGenerator* ret = new cv::bgsegm::SyntheticSequenceGenerator(*background, *object, amplitude, wavelength, wavespeed, objspeed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::bgsegm::SyntheticSequenceGenerator*>))
	}
	
	// getNextFrame(cv::OutputArray, cv::OutputArray) /usr/include/opencv2/bgsegm.hpp:360
	void cv_bgsegm_SyntheticSequenceGenerator_getNextFrame_const__OutputArrayR_const__OutputArrayR(cv::bgsegm::SyntheticSequenceGenerator* instance, const cv::_OutputArray* frame, const cv::_OutputArray* gtMask, Result_void* ocvrs_return) {
		try {
			instance->getNextFrame(*frame, *gtMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
