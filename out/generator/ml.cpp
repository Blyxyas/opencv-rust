#include "ocvrs_common.hpp"
#include <opencv2/ml.hpp>
#include "ml_types.hpp"

extern "C" {
	// createConcentricSpheresTestSet(int, int, int, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ml.hpp:1902
	void cv_ml_createConcentricSpheresTestSet_int_int_int_const__OutputArrayR_const__OutputArrayR(int nsamples, int nfeatures, int nclasses, const cv::_OutputArray* samples, const cv::_OutputArray* responses, Result_void* ocvrs_return) {
		try {
			cv::ml::createConcentricSpheresTestSet(nsamples, nfeatures, nclasses, *samples, *responses);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// randMVNormal(cv::InputArray, cv::InputArray, int, cv::OutputArray) /usr/include/opencv2/ml.hpp:1899
	void cv_ml_randMVNormal_const__InputArrayR_const__InputArrayR_int_const__OutputArrayR(const cv::_InputArray* mean, const cv::_InputArray* cov, int nsamples, const cv::_OutputArray* samples, Result_void* ocvrs_return) {
		try {
			cv::ml::randMVNormal(*mean, *cov, nsamples, *samples);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setTrainMethod(int, double, double) /usr/include/opencv2/ml.hpp:1446
	void cv_ml_ANN_MLP_setTrainMethod_int_double_double(cv::ml::ANN_MLP* instance, int method, double param1, double param2, Result_void* ocvrs_return) {
		try {
			instance->setTrainMethod(method, param1, param2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTrainMethod() /usr/include/opencv2/ml.hpp:1449
	void cv_ml_ANN_MLP_getTrainMethod_const(const cv::ml::ANN_MLP* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTrainMethod();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setActivationFunction(int, double, double) /usr/include/opencv2/ml.hpp:1457
	void cv_ml_ANN_MLP_setActivationFunction_int_double_double(cv::ml::ANN_MLP* instance, int type, double param1, double param2, Result_void* ocvrs_return) {
		try {
			instance->setActivationFunction(type, param1, param2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setLayerSizes(cv::InputArray) /usr/include/opencv2/ml.hpp:1463
	void cv_ml_ANN_MLP_setLayerSizes_const__InputArrayR(cv::ml::ANN_MLP* instance, const cv::_InputArray* _layer_sizes, Result_void* ocvrs_return) {
		try {
			instance->setLayerSizes(*_layer_sizes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getLayerSizes() /usr/include/opencv2/ml.hpp:1469
	void cv_ml_ANN_MLP_getLayerSizes_const(const cv::ml::ANN_MLP* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getLayerSizes();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getTermCriteria() /usr/include/opencv2/ml.hpp:1476
	void cv_ml_ANN_MLP_getTermCriteria_const(const cv::ml::ANN_MLP* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TermCriteria>))
	}
	
	// setTermCriteria(cv::TermCriteria) /usr/include/opencv2/ml.hpp:1478
	void cv_ml_ANN_MLP_setTermCriteria_TermCriteria(cv::ml::ANN_MLP* instance, cv::TermCriteria* val, Result_void* ocvrs_return) {
		try {
			instance->setTermCriteria(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getBackpropWeightScale() /usr/include/opencv2/ml.hpp:1483
	void cv_ml_ANN_MLP_getBackpropWeightScale_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getBackpropWeightScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setBackpropWeightScale(double) /usr/include/opencv2/ml.hpp:1485
	void cv_ml_ANN_MLP_setBackpropWeightScale_double(cv::ml::ANN_MLP* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setBackpropWeightScale(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getBackpropMomentumScale() /usr/include/opencv2/ml.hpp:1492
	void cv_ml_ANN_MLP_getBackpropMomentumScale_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getBackpropMomentumScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setBackpropMomentumScale(double) /usr/include/opencv2/ml.hpp:1494
	void cv_ml_ANN_MLP_setBackpropMomentumScale_double(cv::ml::ANN_MLP* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setBackpropMomentumScale(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRpropDW0() /usr/include/opencv2/ml.hpp:1499
	void cv_ml_ANN_MLP_getRpropDW0_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRpropDW0();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setRpropDW0(double) /usr/include/opencv2/ml.hpp:1501
	void cv_ml_ANN_MLP_setRpropDW0_double(cv::ml::ANN_MLP* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setRpropDW0(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRpropDWPlus() /usr/include/opencv2/ml.hpp:1506
	void cv_ml_ANN_MLP_getRpropDWPlus_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRpropDWPlus();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setRpropDWPlus(double) /usr/include/opencv2/ml.hpp:1508
	void cv_ml_ANN_MLP_setRpropDWPlus_double(cv::ml::ANN_MLP* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setRpropDWPlus(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRpropDWMinus() /usr/include/opencv2/ml.hpp:1513
	void cv_ml_ANN_MLP_getRpropDWMinus_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRpropDWMinus();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setRpropDWMinus(double) /usr/include/opencv2/ml.hpp:1515
	void cv_ml_ANN_MLP_setRpropDWMinus_double(cv::ml::ANN_MLP* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setRpropDWMinus(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRpropDWMin() /usr/include/opencv2/ml.hpp:1520
	void cv_ml_ANN_MLP_getRpropDWMin_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRpropDWMin();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setRpropDWMin(double) /usr/include/opencv2/ml.hpp:1522
	void cv_ml_ANN_MLP_setRpropDWMin_double(cv::ml::ANN_MLP* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setRpropDWMin(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRpropDWMax() /usr/include/opencv2/ml.hpp:1527
	void cv_ml_ANN_MLP_getRpropDWMax_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRpropDWMax();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setRpropDWMax(double) /usr/include/opencv2/ml.hpp:1529
	void cv_ml_ANN_MLP_setRpropDWMax_double(cv::ml::ANN_MLP* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setRpropDWMax(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getAnnealInitialT() /usr/include/opencv2/ml.hpp:1534
	void cv_ml_ANN_MLP_getAnnealInitialT_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAnnealInitialT();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setAnnealInitialT(double) /usr/include/opencv2/ml.hpp:1536
	void cv_ml_ANN_MLP_setAnnealInitialT_double(cv::ml::ANN_MLP* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setAnnealInitialT(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getAnnealFinalT() /usr/include/opencv2/ml.hpp:1541
	void cv_ml_ANN_MLP_getAnnealFinalT_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAnnealFinalT();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setAnnealFinalT(double) /usr/include/opencv2/ml.hpp:1543
	void cv_ml_ANN_MLP_setAnnealFinalT_double(cv::ml::ANN_MLP* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setAnnealFinalT(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getAnnealCoolingRatio() /usr/include/opencv2/ml.hpp:1548
	void cv_ml_ANN_MLP_getAnnealCoolingRatio_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAnnealCoolingRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setAnnealCoolingRatio(double) /usr/include/opencv2/ml.hpp:1550
	void cv_ml_ANN_MLP_setAnnealCoolingRatio_double(cv::ml::ANN_MLP* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setAnnealCoolingRatio(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getAnnealItePerStep() /usr/include/opencv2/ml.hpp:1555
	void cv_ml_ANN_MLP_getAnnealItePerStep_const(const cv::ml::ANN_MLP* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getAnnealItePerStep();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setAnnealItePerStep(int) /usr/include/opencv2/ml.hpp:1557
	void cv_ml_ANN_MLP_setAnnealItePerStep_int(cv::ml::ANN_MLP* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setAnnealItePerStep(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setAnnealEnergyRNG(const cv::RNG &) /usr/include/opencv2/ml.hpp:1560
	void cv_ml_ANN_MLP_setAnnealEnergyRNG_const_RNGR(cv::ml::ANN_MLP* instance, const cv::RNG* rng, Result_void* ocvrs_return) {
		try {
			instance->setAnnealEnergyRNG(*rng);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWeights(int) /usr/include/opencv2/ml.hpp:1597
	void cv_ml_ANN_MLP_getWeights_const_int(const cv::ml::ANN_MLP* instance, int layerIdx, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getWeights(layerIdx);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// create() /usr/include/opencv2/ml.hpp:1604
	void cv_ml_ANN_MLP_create(Result<cv::Ptr<cv::ml::ANN_MLP>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::ANN_MLP> ret = cv::ml::ANN_MLP::create();
			Ok(new cv::Ptr<cv::ml::ANN_MLP>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::ANN_MLP>*>))
	}
	
	// load(const cv::String &) /usr/include/opencv2/ml.hpp:1613
	void cv_ml_ANN_MLP_load_const_StringR(const char* filepath, Result<cv::Ptr<cv::ml::ANN_MLP>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::ANN_MLP> ret = cv::ml::ANN_MLP::load(std::string(filepath));
			Ok(new cv::Ptr<cv::ml::ANN_MLP>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::ANN_MLP>*>))
	}
	
	// getBoostType() /usr/include/opencv2/ml.hpp:1338
	void cv_ml_Boost_getBoostType_const(const cv::ml::Boost* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBoostType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setBoostType(int) /usr/include/opencv2/ml.hpp:1340
	void cv_ml_Boost_setBoostType_int(cv::ml::Boost* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setBoostType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWeakCount() /usr/include/opencv2/ml.hpp:1345
	void cv_ml_Boost_getWeakCount_const(const cv::ml::Boost* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWeakCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setWeakCount(int) /usr/include/opencv2/ml.hpp:1347
	void cv_ml_Boost_setWeakCount_int(cv::ml::Boost* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setWeakCount(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWeightTrimRate() /usr/include/opencv2/ml.hpp:1353
	void cv_ml_Boost_getWeightTrimRate_const(const cv::ml::Boost* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getWeightTrimRate();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setWeightTrimRate(double) /usr/include/opencv2/ml.hpp:1355
	void cv_ml_Boost_setWeightTrimRate_double(cv::ml::Boost* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setWeightTrimRate(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create() /usr/include/opencv2/ml.hpp:1370
	void cv_ml_Boost_create(Result<cv::Ptr<cv::ml::Boost>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::Boost> ret = cv::ml::Boost::create();
			Ok(new cv::Ptr<cv::ml::Boost>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::Boost>*>))
	}
	
	// load(const cv::String &, const cv::String &) /usr/include/opencv2/ml.hpp:1381
	void cv_ml_Boost_load_const_StringR_const_StringR(const char* filepath, const char* nodeName, Result<cv::Ptr<cv::ml::Boost>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::Boost> ret = cv::ml::Boost::load(std::string(filepath), std::string(nodeName));
			Ok(new cv::Ptr<cv::ml::Boost>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::Boost>*>))
	}
	
	// getMaxCategories() /usr/include/opencv2/ml.hpp:1071
	void cv_ml_DTrees_getMaxCategories_const(const cv::ml::DTrees* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxCategories();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMaxCategories(int) /usr/include/opencv2/ml.hpp:1073
	void cv_ml_DTrees_setMaxCategories_int(cv::ml::DTrees* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setMaxCategories(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxDepth() /usr/include/opencv2/ml.hpp:1081
	void cv_ml_DTrees_getMaxDepth_const(const cv::ml::DTrees* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMaxDepth(int) /usr/include/opencv2/ml.hpp:1083
	void cv_ml_DTrees_setMaxDepth_int(cv::ml::DTrees* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setMaxDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinSampleCount() /usr/include/opencv2/ml.hpp:1089
	void cv_ml_DTrees_getMinSampleCount_const(const cv::ml::DTrees* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinSampleCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMinSampleCount(int) /usr/include/opencv2/ml.hpp:1091
	void cv_ml_DTrees_setMinSampleCount_int(cv::ml::DTrees* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setMinSampleCount(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getCVFolds() /usr/include/opencv2/ml.hpp:1097
	void cv_ml_DTrees_getCVFolds_const(const cv::ml::DTrees* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCVFolds();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setCVFolds(int) /usr/include/opencv2/ml.hpp:1099
	void cv_ml_DTrees_setCVFolds_int(cv::ml::DTrees* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setCVFolds(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUseSurrogates() /usr/include/opencv2/ml.hpp:1106
	void cv_ml_DTrees_getUseSurrogates_const(const cv::ml::DTrees* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseSurrogates();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setUseSurrogates(bool) /usr/include/opencv2/ml.hpp:1108
	void cv_ml_DTrees_setUseSurrogates_bool(cv::ml::DTrees* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setUseSurrogates(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUse1SERule() /usr/include/opencv2/ml.hpp:1114
	void cv_ml_DTrees_getUse1SERule_const(const cv::ml::DTrees* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUse1SERule();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setUse1SERule(bool) /usr/include/opencv2/ml.hpp:1116
	void cv_ml_DTrees_setUse1SERule_bool(cv::ml::DTrees* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setUse1SERule(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTruncatePrunedTree() /usr/include/opencv2/ml.hpp:1122
	void cv_ml_DTrees_getTruncatePrunedTree_const(const cv::ml::DTrees* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getTruncatePrunedTree();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setTruncatePrunedTree(bool) /usr/include/opencv2/ml.hpp:1124
	void cv_ml_DTrees_setTruncatePrunedTree_bool(cv::ml::DTrees* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setTruncatePrunedTree(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRegressionAccuracy() /usr/include/opencv2/ml.hpp:1131
	void cv_ml_DTrees_getRegressionAccuracy_const(const cv::ml::DTrees* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getRegressionAccuracy();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setRegressionAccuracy(float) /usr/include/opencv2/ml.hpp:1133
	void cv_ml_DTrees_setRegressionAccuracy_float(cv::ml::DTrees* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setRegressionAccuracy(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPriors() /usr/include/opencv2/ml.hpp:1151
	void cv_ml_DTrees_getPriors_const(const cv::ml::DTrees* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getPriors();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// setPriors(const cv::Mat &) /usr/include/opencv2/ml.hpp:1153
	void cv_ml_DTrees_setPriors_const_MatR(cv::ml::DTrees* instance, const cv::Mat* val, Result_void* ocvrs_return) {
		try {
			instance->setPriors(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRoots() /usr/include/opencv2/ml.hpp:1202
	void cv_ml_DTrees_getRoots_const(const cv::ml::DTrees* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			const std::vector<int> ret = instance->getRoots();
			Ok(new const std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	// getNodes() /usr/include/opencv2/ml.hpp:1207
	void cv_ml_DTrees_getNodes_const(const cv::ml::DTrees* instance, Result<std::vector<cv::ml::DTrees::Node>*>* ocvrs_return) {
		try {
			const std::vector<cv::ml::DTrees::Node> ret = instance->getNodes();
			Ok(new const std::vector<cv::ml::DTrees::Node>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::ml::DTrees::Node>*>))
	}
	
	// getSplits() /usr/include/opencv2/ml.hpp:1212
	void cv_ml_DTrees_getSplits_const(const cv::ml::DTrees* instance, Result<std::vector<cv::ml::DTrees::Split>*>* ocvrs_return) {
		try {
			const std::vector<cv::ml::DTrees::Split> ret = instance->getSplits();
			Ok(new const std::vector<cv::ml::DTrees::Split>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::ml::DTrees::Split>*>))
	}
	
	// getSubsets() /usr/include/opencv2/ml.hpp:1217
	void cv_ml_DTrees_getSubsets_const(const cv::ml::DTrees* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			const std::vector<int> ret = instance->getSubsets();
			Ok(new const std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	// create() /usr/include/opencv2/ml.hpp:1225
	void cv_ml_DTrees_create(Result<cv::Ptr<cv::ml::DTrees>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::DTrees> ret = cv::ml::DTrees::create();
			Ok(new cv::Ptr<cv::ml::DTrees>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::DTrees>*>))
	}
	
	// load(const cv::String &, const cv::String &) /usr/include/opencv2/ml.hpp:1236
	void cv_ml_DTrees_load_const_StringR_const_StringR(const char* filepath, const char* nodeName, Result<cv::Ptr<cv::ml::DTrees>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::DTrees> ret = cv::ml::DTrees::load(std::string(filepath), std::string(nodeName));
			Ok(new cv::Ptr<cv::ml::DTrees>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::DTrees>*>))
	}
	
	// value /usr/include/opencv2/ml.hpp:1161
	double cv_ml_DTrees_Node_getPropValue_const(const cv::ml::DTrees::Node* instance) {
			double ret = instance->value;
			return ret;
	}
	
	// value /usr/include/opencv2/ml.hpp:1161
	void cv_ml_DTrees_Node_setPropValue_double(cv::ml::DTrees::Node* instance, double val) {
			instance->value = val;
	}
	
	// classIdx /usr/include/opencv2/ml.hpp:1163
	int cv_ml_DTrees_Node_getPropClassIdx_const(const cv::ml::DTrees::Node* instance) {
			int ret = instance->classIdx;
			return ret;
	}
	
	// classIdx /usr/include/opencv2/ml.hpp:1163
	void cv_ml_DTrees_Node_setPropClassIdx_int(cv::ml::DTrees::Node* instance, int val) {
			instance->classIdx = val;
	}
	
	// parent /usr/include/opencv2/ml.hpp:1165
	int cv_ml_DTrees_Node_getPropParent_const(const cv::ml::DTrees::Node* instance) {
			int ret = instance->parent;
			return ret;
	}
	
	// parent /usr/include/opencv2/ml.hpp:1165
	void cv_ml_DTrees_Node_setPropParent_int(cv::ml::DTrees::Node* instance, int val) {
			instance->parent = val;
	}
	
	// left /usr/include/opencv2/ml.hpp:1166
	int cv_ml_DTrees_Node_getPropLeft_const(const cv::ml::DTrees::Node* instance) {
			int ret = instance->left;
			return ret;
	}
	
	// left /usr/include/opencv2/ml.hpp:1166
	void cv_ml_DTrees_Node_setPropLeft_int(cv::ml::DTrees::Node* instance, int val) {
			instance->left = val;
	}
	
	// right /usr/include/opencv2/ml.hpp:1167
	int cv_ml_DTrees_Node_getPropRight_const(const cv::ml::DTrees::Node* instance) {
			int ret = instance->right;
			return ret;
	}
	
	// right /usr/include/opencv2/ml.hpp:1167
	void cv_ml_DTrees_Node_setPropRight_int(cv::ml::DTrees::Node* instance, int val) {
			instance->right = val;
	}
	
	// defaultDir /usr/include/opencv2/ml.hpp:1168
	int cv_ml_DTrees_Node_getPropDefaultDir_const(const cv::ml::DTrees::Node* instance) {
			int ret = instance->defaultDir;
			return ret;
	}
	
	// defaultDir /usr/include/opencv2/ml.hpp:1168
	void cv_ml_DTrees_Node_setPropDefaultDir_int(cv::ml::DTrees::Node* instance, int val) {
			instance->defaultDir = val;
	}
	
	// split /usr/include/opencv2/ml.hpp:1170
	int cv_ml_DTrees_Node_getPropSplit_const(const cv::ml::DTrees::Node* instance) {
			int ret = instance->split;
			return ret;
	}
	
	// split /usr/include/opencv2/ml.hpp:1170
	void cv_ml_DTrees_Node_setPropSplit_int(cv::ml::DTrees::Node* instance, int val) {
			instance->split = val;
	}
	
	void cv_DTrees_Node_delete(cv::ml::DTrees::Node* instance) {
		delete instance;
	}
	// Node() /usr/include/opencv2/ml.hpp:1160
	void cv_ml_DTrees_Node_Node(Result<cv::ml::DTrees::Node*>* ocvrs_return) {
		try {
			cv::ml::DTrees::Node* ret = new cv::ml::DTrees::Node();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ml::DTrees::Node*>))
	}
	
	// varIdx /usr/include/opencv2/ml.hpp:1179
	int cv_ml_DTrees_Split_getPropVarIdx_const(const cv::ml::DTrees::Split* instance) {
			int ret = instance->varIdx;
			return ret;
	}
	
	// varIdx /usr/include/opencv2/ml.hpp:1179
	void cv_ml_DTrees_Split_setPropVarIdx_int(cv::ml::DTrees::Split* instance, int val) {
			instance->varIdx = val;
	}
	
	// inversed /usr/include/opencv2/ml.hpp:1180
	bool cv_ml_DTrees_Split_getPropInversed_const(const cv::ml::DTrees::Split* instance) {
			bool ret = instance->inversed;
			return ret;
	}
	
	// inversed /usr/include/opencv2/ml.hpp:1180
	void cv_ml_DTrees_Split_setPropInversed_bool(cv::ml::DTrees::Split* instance, bool val) {
			instance->inversed = val;
	}
	
	// quality /usr/include/opencv2/ml.hpp:1182
	float cv_ml_DTrees_Split_getPropQuality_const(const cv::ml::DTrees::Split* instance) {
			float ret = instance->quality;
			return ret;
	}
	
	// quality /usr/include/opencv2/ml.hpp:1182
	void cv_ml_DTrees_Split_setPropQuality_float(cv::ml::DTrees::Split* instance, float val) {
			instance->quality = val;
	}
	
	// next /usr/include/opencv2/ml.hpp:1183
	int cv_ml_DTrees_Split_getPropNext_const(const cv::ml::DTrees::Split* instance) {
			int ret = instance->next;
			return ret;
	}
	
	// next /usr/include/opencv2/ml.hpp:1183
	void cv_ml_DTrees_Split_setPropNext_int(cv::ml::DTrees::Split* instance, int val) {
			instance->next = val;
	}
	
	// c /usr/include/opencv2/ml.hpp:1184
	float cv_ml_DTrees_Split_getPropC_const(const cv::ml::DTrees::Split* instance) {
			float ret = instance->c;
			return ret;
	}
	
	// c /usr/include/opencv2/ml.hpp:1184
	void cv_ml_DTrees_Split_setPropC_float(cv::ml::DTrees::Split* instance, float val) {
			instance->c = val;
	}
	
	// subsetOfs /usr/include/opencv2/ml.hpp:1191
	int cv_ml_DTrees_Split_getPropSubsetOfs_const(const cv::ml::DTrees::Split* instance) {
			int ret = instance->subsetOfs;
			return ret;
	}
	
	// subsetOfs /usr/include/opencv2/ml.hpp:1191
	void cv_ml_DTrees_Split_setPropSubsetOfs_int(cv::ml::DTrees::Split* instance, int val) {
			instance->subsetOfs = val;
	}
	
	void cv_DTrees_Split_delete(cv::ml::DTrees::Split* instance) {
		delete instance;
	}
	// Split() /usr/include/opencv2/ml.hpp:1178
	void cv_ml_DTrees_Split_Split(Result<cv::ml::DTrees::Split*>* ocvrs_return) {
		try {
			cv::ml::DTrees::Split* ret = new cv::ml::DTrees::Split();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ml::DTrees::Split*>))
	}
	
	// getClustersNumber() /usr/include/opencv2/ml.hpp:871
	void cv_ml_EM_getClustersNumber_const(const cv::ml::EM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getClustersNumber();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setClustersNumber(int) /usr/include/opencv2/ml.hpp:873
	void cv_ml_EM_setClustersNumber_int(cv::ml::EM* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setClustersNumber(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getCovarianceMatrixType() /usr/include/opencv2/ml.hpp:878
	void cv_ml_EM_getCovarianceMatrixType_const(const cv::ml::EM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCovarianceMatrixType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setCovarianceMatrixType(int) /usr/include/opencv2/ml.hpp:880
	void cv_ml_EM_setCovarianceMatrixType_int(cv::ml::EM* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setCovarianceMatrixType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTermCriteria() /usr/include/opencv2/ml.hpp:887
	void cv_ml_EM_getTermCriteria_const(const cv::ml::EM* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TermCriteria>))
	}
	
	// setTermCriteria(const cv::TermCriteria &) /usr/include/opencv2/ml.hpp:889
	void cv_ml_EM_setTermCriteria_const_TermCriteriaR(cv::ml::EM* instance, const cv::TermCriteria* val, Result_void* ocvrs_return) {
		try {
			instance->setTermCriteria(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWeights() /usr/include/opencv2/ml.hpp:895
	void cv_ml_EM_getWeights_const(const cv::ml::EM* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getWeights();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getMeans() /usr/include/opencv2/ml.hpp:901
	void cv_ml_EM_getMeans_const(const cv::ml::EM* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMeans();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getCovs(std::vector<Mat> &) /usr/include/opencv2/ml.hpp:907
	void cv_ml_EM_getCovs_const_vector_Mat_R(const cv::ml::EM* instance, std::vector<cv::Mat>* covs, Result_void* ocvrs_return) {
		try {
			instance->getCovs(*covs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// predict(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ml.hpp:916
	void cv_ml_EM_predict_const_const__InputArrayR_const__OutputArrayR_int(const cv::ml::EM* instance, const cv::_InputArray* samples, const cv::_OutputArray* results, int flags, Result<float>* ocvrs_return) {
		try {
			float ret = instance->predict(*samples, *results, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// predict2(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ml.hpp:930
	void cv_ml_EM_predict2_const_const__InputArrayR_const__OutputArrayR(const cv::ml::EM* instance, const cv::_InputArray* sample, const cv::_OutputArray* probs, Result<cv::Vec2d>* ocvrs_return) {
		try {
			cv::Vec2d ret = instance->predict2(*sample, *probs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2d>))
	}
	
	// trainEM(cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ml.hpp:960
	void cv_ml_EM_trainEM_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::ml::EM* instance, const cv::_InputArray* samples, const cv::_OutputArray* logLikelihoods, const cv::_OutputArray* labels, const cv::_OutputArray* probs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->trainEM(*samples, *logLikelihoods, *labels, *probs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// trainE(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ml.hpp:992
	void cv_ml_EM_trainE_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::ml::EM* instance, const cv::_InputArray* samples, const cv::_InputArray* means0, const cv::_InputArray* covs0, const cv::_InputArray* weights0, const cv::_OutputArray* logLikelihoods, const cv::_OutputArray* labels, const cv::_OutputArray* probs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->trainE(*samples, *means0, *covs0, *weights0, *logLikelihoods, *labels, *probs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// trainM(cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ml.hpp:1017
	void cv_ml_EM_trainM_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::ml::EM* instance, const cv::_InputArray* samples, const cv::_InputArray* probs0, const cv::_OutputArray* logLikelihoods, const cv::_OutputArray* labels, const cv::_OutputArray* probs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->trainM(*samples, *probs0, *logLikelihoods, *labels, *probs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// create() /usr/include/opencv2/ml.hpp:1026
	void cv_ml_EM_create(Result<cv::Ptr<cv::ml::EM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::EM> ret = cv::ml::EM::create();
			Ok(new cv::Ptr<cv::ml::EM>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::EM>*>))
	}
	
	// load(const cv::String &, const cv::String &) /usr/include/opencv2/ml.hpp:1037
	void cv_ml_EM_load_const_StringR_const_StringR(const char* filepath, const char* nodeName, Result<cv::Ptr<cv::ml::EM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::EM> ret = cv::ml::EM::load(std::string(filepath), std::string(nodeName));
			Ok(new cv::Ptr<cv::ml::EM>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::EM>*>))
	}
	
	// getDefaultK() /usr/include/opencv2/ml.hpp:442
	void cv_ml_KNearest_getDefaultK_const(const cv::ml::KNearest* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDefaultK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setDefaultK(int) /usr/include/opencv2/ml.hpp:444
	void cv_ml_KNearest_setDefaultK_int(cv::ml::KNearest* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setDefaultK(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getIsClassifier() /usr/include/opencv2/ml.hpp:448
	void cv_ml_KNearest_getIsClassifier_const(const cv::ml::KNearest* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getIsClassifier();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setIsClassifier(bool) /usr/include/opencv2/ml.hpp:450
	void cv_ml_KNearest_setIsClassifier_bool(cv::ml::KNearest* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setIsClassifier(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getEmax() /usr/include/opencv2/ml.hpp:454
	void cv_ml_KNearest_getEmax_const(const cv::ml::KNearest* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getEmax();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setEmax(int) /usr/include/opencv2/ml.hpp:456
	void cv_ml_KNearest_setEmax_int(cv::ml::KNearest* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setEmax(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getAlgorithmType() /usr/include/opencv2/ml.hpp:460
	void cv_ml_KNearest_getAlgorithmType_const(const cv::ml::KNearest* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getAlgorithmType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setAlgorithmType(int) /usr/include/opencv2/ml.hpp:462
	void cv_ml_KNearest_setAlgorithmType_int(cv::ml::KNearest* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setAlgorithmType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// findNearest(cv::InputArray, int, cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ml.hpp:490
	void cv_ml_KNearest_findNearest_const_const__InputArrayR_int_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::ml::KNearest* instance, const cv::_InputArray* samples, int k, const cv::_OutputArray* results, const cv::_OutputArray* neighborResponses, const cv::_OutputArray* dist, Result<float>* ocvrs_return) {
		try {
			float ret = instance->findNearest(*samples, k, *results, *neighborResponses, *dist);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// create() /usr/include/opencv2/ml.hpp:507
	void cv_ml_KNearest_create(Result<cv::Ptr<cv::ml::KNearest>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::KNearest> ret = cv::ml::KNearest::create();
			Ok(new cv::Ptr<cv::ml::KNearest>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::KNearest>*>))
	}
	
	// load(const cv::String &) /usr/include/opencv2/ml.hpp:515
	void cv_ml_KNearest_load_const_StringR(const char* filepath, Result<cv::Ptr<cv::ml::KNearest>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::KNearest> ret = cv::ml::KNearest::load(std::string(filepath));
			Ok(new cv::Ptr<cv::ml::KNearest>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::KNearest>*>))
	}
	
	// getLearningRate() /usr/include/opencv2/ml.hpp:1635
	void cv_ml_LogisticRegression_getLearningRate_const(const cv::ml::LogisticRegression* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getLearningRate();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setLearningRate(double) /usr/include/opencv2/ml.hpp:1637
	void cv_ml_LogisticRegression_setLearningRate_double(cv::ml::LogisticRegression* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setLearningRate(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getIterations() /usr/include/opencv2/ml.hpp:1641
	void cv_ml_LogisticRegression_getIterations_const(const cv::ml::LogisticRegression* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setIterations(int) /usr/include/opencv2/ml.hpp:1643
	void cv_ml_LogisticRegression_setIterations_int(cv::ml::LogisticRegression* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRegularization() /usr/include/opencv2/ml.hpp:1647
	void cv_ml_LogisticRegression_getRegularization_const(const cv::ml::LogisticRegression* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRegularization();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setRegularization(int) /usr/include/opencv2/ml.hpp:1649
	void cv_ml_LogisticRegression_setRegularization_int(cv::ml::LogisticRegression* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setRegularization(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTrainMethod() /usr/include/opencv2/ml.hpp:1653
	void cv_ml_LogisticRegression_getTrainMethod_const(const cv::ml::LogisticRegression* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTrainMethod();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setTrainMethod(int) /usr/include/opencv2/ml.hpp:1655
	void cv_ml_LogisticRegression_setTrainMethod_int(cv::ml::LogisticRegression* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setTrainMethod(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMiniBatchSize() /usr/include/opencv2/ml.hpp:1661
	void cv_ml_LogisticRegression_getMiniBatchSize_const(const cv::ml::LogisticRegression* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMiniBatchSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMiniBatchSize(int) /usr/include/opencv2/ml.hpp:1663
	void cv_ml_LogisticRegression_setMiniBatchSize_int(cv::ml::LogisticRegression* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setMiniBatchSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTermCriteria() /usr/include/opencv2/ml.hpp:1667
	void cv_ml_LogisticRegression_getTermCriteria_const(const cv::ml::LogisticRegression* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TermCriteria>))
	}
	
	// setTermCriteria(cv::TermCriteria) /usr/include/opencv2/ml.hpp:1669
	void cv_ml_LogisticRegression_setTermCriteria_TermCriteria(cv::ml::LogisticRegression* instance, cv::TermCriteria* val, Result_void* ocvrs_return) {
		try {
			instance->setTermCriteria(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// predict(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ml.hpp:1691
	void cv_ml_LogisticRegression_predict_const_const__InputArrayR_const__OutputArrayR_int(const cv::ml::LogisticRegression* instance, const cv::_InputArray* samples, const cv::_OutputArray* results, int flags, Result<float>* ocvrs_return) {
		try {
			float ret = instance->predict(*samples, *results, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// get_learnt_thetas() /usr/include/opencv2/ml.hpp:1698
	void cv_ml_LogisticRegression_get_learnt_thetas_const(const cv::ml::LogisticRegression* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->get_learnt_thetas();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// create() /usr/include/opencv2/ml.hpp:1704
	void cv_ml_LogisticRegression_create(Result<cv::Ptr<cv::ml::LogisticRegression>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::LogisticRegression> ret = cv::ml::LogisticRegression::create();
			Ok(new cv::Ptr<cv::ml::LogisticRegression>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::LogisticRegression>*>))
	}
	
	// load(const cv::String &, const cv::String &) /usr/include/opencv2/ml.hpp:1715
	void cv_ml_LogisticRegression_load_const_StringR_const_StringR(const char* filepath, const char* nodeName, Result<cv::Ptr<cv::ml::LogisticRegression>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::LogisticRegression> ret = cv::ml::LogisticRegression::load(std::string(filepath), std::string(nodeName));
			Ok(new cv::Ptr<cv::ml::LogisticRegression>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::LogisticRegression>*>))
	}
	
	// predictProb(cv::InputArray, cv::OutputArray, cv::OutputArray, int) /usr/include/opencv2/ml.hpp:409
	void cv_ml_NormalBayesClassifier_predictProb_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int(const cv::ml::NormalBayesClassifier* instance, const cv::_InputArray* inputs, const cv::_OutputArray* outputs, const cv::_OutputArray* outputProbs, int flags, Result<float>* ocvrs_return) {
		try {
			float ret = instance->predictProb(*inputs, *outputs, *outputProbs, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// create() /usr/include/opencv2/ml.hpp:414
	void cv_ml_NormalBayesClassifier_create(Result<cv::Ptr<cv::ml::NormalBayesClassifier>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::NormalBayesClassifier> ret = cv::ml::NormalBayesClassifier::create();
			Ok(new cv::Ptr<cv::ml::NormalBayesClassifier>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::NormalBayesClassifier>*>))
	}
	
	// load(const cv::String &, const cv::String &) /usr/include/opencv2/ml.hpp:425
	void cv_ml_NormalBayesClassifier_load_const_StringR_const_StringR(const char* filepath, const char* nodeName, Result<cv::Ptr<cv::ml::NormalBayesClassifier>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::NormalBayesClassifier> ret = cv::ml::NormalBayesClassifier::load(std::string(filepath), std::string(nodeName));
			Ok(new cv::Ptr<cv::ml::NormalBayesClassifier>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::NormalBayesClassifier>*>))
	}
	
	// minVal /usr/include/opencv2/ml.hpp:115
	double cv_ml_ParamGrid_getPropMinVal_const(const cv::ml::ParamGrid* instance) {
			double ret = instance->minVal;
			return ret;
	}
	
	// minVal /usr/include/opencv2/ml.hpp:115
	void cv_ml_ParamGrid_setPropMinVal_double(cv::ml::ParamGrid* instance, double val) {
			instance->minVal = val;
	}
	
	// maxVal /usr/include/opencv2/ml.hpp:116
	double cv_ml_ParamGrid_getPropMaxVal_const(const cv::ml::ParamGrid* instance) {
			double ret = instance->maxVal;
			return ret;
	}
	
	// maxVal /usr/include/opencv2/ml.hpp:116
	void cv_ml_ParamGrid_setPropMaxVal_double(cv::ml::ParamGrid* instance, double val) {
			instance->maxVal = val;
	}
	
	// logStep /usr/include/opencv2/ml.hpp:125
	double cv_ml_ParamGrid_getPropLogStep_const(const cv::ml::ParamGrid* instance) {
			double ret = instance->logStep;
			return ret;
	}
	
	// logStep /usr/include/opencv2/ml.hpp:125
	void cv_ml_ParamGrid_setPropLogStep_double(cv::ml::ParamGrid* instance, double val) {
			instance->logStep = val;
	}
	
	void cv_ParamGrid_delete(cv::ml::ParamGrid* instance) {
		delete instance;
	}
	// ParamGrid() /usr/include/opencv2/ml.hpp:111
	void cv_ml_ParamGrid_ParamGrid(Result<cv::ml::ParamGrid*>* ocvrs_return) {
		try {
			cv::ml::ParamGrid* ret = new cv::ml::ParamGrid();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ml::ParamGrid*>))
	}
	
	// ParamGrid(double, double, double) /usr/include/opencv2/ml.hpp:113
	void cv_ml_ParamGrid_ParamGrid_double_double_double(double _minVal, double _maxVal, double _logStep, Result<cv::ml::ParamGrid*>* ocvrs_return) {
		try {
			cv::ml::ParamGrid* ret = new cv::ml::ParamGrid(_minVal, _maxVal, _logStep);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ml::ParamGrid*>))
	}
	
	// create(double, double, double) /usr/include/opencv2/ml.hpp:133
	void cv_ml_ParamGrid_create_double_double_double(double minVal, double maxVal, double logstep, Result<cv::Ptr<cv::ml::ParamGrid>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::ParamGrid> ret = cv::ml::ParamGrid::create(minVal, maxVal, logstep);
			Ok(new cv::Ptr<cv::ml::ParamGrid>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::ParamGrid>*>))
	}
	
	// getCalculateVarImportance() /usr/include/opencv2/ml.hpp:1254
	void cv_ml_RTrees_getCalculateVarImportance_const(const cv::ml::RTrees* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getCalculateVarImportance();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setCalculateVarImportance(bool) /usr/include/opencv2/ml.hpp:1256
	void cv_ml_RTrees_setCalculateVarImportance_bool(cv::ml::RTrees* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setCalculateVarImportance(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getActiveVarCount() /usr/include/opencv2/ml.hpp:1263
	void cv_ml_RTrees_getActiveVarCount_const(const cv::ml::RTrees* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getActiveVarCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setActiveVarCount(int) /usr/include/opencv2/ml.hpp:1265
	void cv_ml_RTrees_setActiveVarCount_int(cv::ml::RTrees* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setActiveVarCount(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTermCriteria() /usr/include/opencv2/ml.hpp:1275
	void cv_ml_RTrees_getTermCriteria_const(const cv::ml::RTrees* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TermCriteria>))
	}
	
	// setTermCriteria(const cv::TermCriteria &) /usr/include/opencv2/ml.hpp:1277
	void cv_ml_RTrees_setTermCriteria_const_TermCriteriaR(cv::ml::RTrees* instance, const cv::TermCriteria* val, Result_void* ocvrs_return) {
		try {
			instance->setTermCriteria(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getVarImportance() /usr/include/opencv2/ml.hpp:1284
	void cv_ml_RTrees_getVarImportance_const(const cv::ml::RTrees* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getVarImportance();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getVotes(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ml.hpp:1295
	void cv_ml_RTrees_getVotes_const_const__InputArrayR_const__OutputArrayR_int(const cv::ml::RTrees* instance, const cv::_InputArray* samples, const cv::_OutputArray* results, int flags, Result_void* ocvrs_return) {
		try {
			instance->getVotes(*samples, *results, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getOOBError() /usr/include/opencv2/ml.hpp:1301
	void cv_ml_RTrees_getOOBError_const(const cv::ml::RTrees* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getOOBError();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// create() /usr/include/opencv2/ml.hpp:1310
	void cv_ml_RTrees_create(Result<cv::Ptr<cv::ml::RTrees>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::RTrees> ret = cv::ml::RTrees::create();
			Ok(new cv::Ptr<cv::ml::RTrees>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::RTrees>*>))
	}
	
	// load(const cv::String &, const cv::String &) /usr/include/opencv2/ml.hpp:1321
	void cv_ml_RTrees_load_const_StringR_const_StringR(const char* filepath, const char* nodeName, Result<cv::Ptr<cv::ml::RTrees>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::RTrees> ret = cv::ml::RTrees::load(std::string(filepath), std::string(nodeName));
			Ok(new cv::Ptr<cv::ml::RTrees>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::RTrees>*>))
	}
	
	// getType() /usr/include/opencv2/ml.hpp:540
	void cv_ml_SVM_getType_const(const cv::ml::SVM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setType(int) /usr/include/opencv2/ml.hpp:542
	void cv_ml_SVM_setType_int(cv::ml::SVM* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getGamma() /usr/include/opencv2/ml.hpp:547
	void cv_ml_SVM_getGamma_const(const cv::ml::SVM* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setGamma(double) /usr/include/opencv2/ml.hpp:549
	void cv_ml_SVM_setGamma_double(cv::ml::SVM* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setGamma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getCoef0() /usr/include/opencv2/ml.hpp:554
	void cv_ml_SVM_getCoef0_const(const cv::ml::SVM* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getCoef0();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setCoef0(double) /usr/include/opencv2/ml.hpp:556
	void cv_ml_SVM_setCoef0_double(cv::ml::SVM* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setCoef0(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDegree() /usr/include/opencv2/ml.hpp:561
	void cv_ml_SVM_getDegree_const(const cv::ml::SVM* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDegree();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setDegree(double) /usr/include/opencv2/ml.hpp:563
	void cv_ml_SVM_setDegree_double(cv::ml::SVM* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setDegree(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getC() /usr/include/opencv2/ml.hpp:568
	void cv_ml_SVM_getC_const(const cv::ml::SVM* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getC();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setC(double) /usr/include/opencv2/ml.hpp:570
	void cv_ml_SVM_setC_double(cv::ml::SVM* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setC(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNu() /usr/include/opencv2/ml.hpp:575
	void cv_ml_SVM_getNu_const(const cv::ml::SVM* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getNu();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setNu(double) /usr/include/opencv2/ml.hpp:577
	void cv_ml_SVM_setNu_double(cv::ml::SVM* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setNu(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getP() /usr/include/opencv2/ml.hpp:582
	void cv_ml_SVM_getP_const(const cv::ml::SVM* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getP();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setP(double) /usr/include/opencv2/ml.hpp:584
	void cv_ml_SVM_setP_double(cv::ml::SVM* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setP(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getClassWeights() /usr/include/opencv2/ml.hpp:592
	void cv_ml_SVM_getClassWeights_const(const cv::ml::SVM* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getClassWeights();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// setClassWeights(const cv::Mat &) /usr/include/opencv2/ml.hpp:594
	void cv_ml_SVM_setClassWeights_const_MatR(cv::ml::SVM* instance, const cv::Mat* val, Result_void* ocvrs_return) {
		try {
			instance->setClassWeights(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTermCriteria() /usr/include/opencv2/ml.hpp:601
	void cv_ml_SVM_getTermCriteria_const(const cv::ml::SVM* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TermCriteria>))
	}
	
	// setTermCriteria(const cv::TermCriteria &) /usr/include/opencv2/ml.hpp:603
	void cv_ml_SVM_setTermCriteria_const_TermCriteriaR(cv::ml::SVM* instance, const cv::TermCriteria* val, Result_void* ocvrs_return) {
		try {
			instance->setTermCriteria(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getKernelType() /usr/include/opencv2/ml.hpp:607
	void cv_ml_SVM_getKernelType_const(const cv::ml::SVM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getKernelType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setKernel(int) /usr/include/opencv2/ml.hpp:611
	void cv_ml_SVM_setKernel_int(cv::ml::SVM* instance, int kernelType, Result_void* ocvrs_return) {
		try {
			instance->setKernel(kernelType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setCustomKernel(const Ptr<cv::ml::SVM::Kernel> &) /usr/include/opencv2/ml.hpp:615
	void cv_ml_SVM_setCustomKernel_const_Ptr_Kernel_R(cv::ml::SVM* instance, const cv::Ptr<cv::ml::SVM::Kernel>* _kernel, Result_void* ocvrs_return) {
		try {
			instance->setCustomKernel(*_kernel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// trainAuto(const Ptr<cv::ml::TrainData> &, int, cv::ml::ParamGrid, cv::ml::ParamGrid, cv::ml::ParamGrid, cv::ml::ParamGrid, cv::ml::ParamGrid, cv::ml::ParamGrid, bool) /usr/include/opencv2/ml.hpp:712
	void cv_ml_SVM_trainAuto_const_Ptr_TrainData_R_int_ParamGrid_ParamGrid_ParamGrid_ParamGrid_ParamGrid_ParamGrid_bool(cv::ml::SVM* instance, const cv::Ptr<cv::ml::TrainData>* data, int kFold, cv::ml::ParamGrid* Cgrid, cv::ml::ParamGrid* gammaGrid, cv::ml::ParamGrid* pGrid, cv::ml::ParamGrid* nuGrid, cv::ml::ParamGrid* coeffGrid, cv::ml::ParamGrid* degreeGrid, bool balanced, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->trainAuto(*data, kFold, *Cgrid, *gammaGrid, *pGrid, *nuGrid, *coeffGrid, *degreeGrid, balanced);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// trainAuto(cv::InputArray, int, cv::InputArray, int, Ptr<cv::ml::ParamGrid>, Ptr<cv::ml::ParamGrid>, Ptr<cv::ml::ParamGrid>, Ptr<cv::ml::ParamGrid>, Ptr<cv::ml::ParamGrid>, Ptr<cv::ml::ParamGrid>, bool) /usr/include/opencv2/ml.hpp:749
	void cv_ml_SVM_trainAuto_const__InputArrayR_int_const__InputArrayR_int_Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__bool(cv::ml::SVM* instance, const cv::_InputArray* samples, int layout, const cv::_InputArray* responses, int kFold, cv::Ptr<cv::ml::ParamGrid>* Cgrid, cv::Ptr<cv::ml::ParamGrid>* gammaGrid, cv::Ptr<cv::ml::ParamGrid>* pGrid, cv::Ptr<cv::ml::ParamGrid>* nuGrid, cv::Ptr<cv::ml::ParamGrid>* coeffGrid, cv::Ptr<cv::ml::ParamGrid>* degreeGrid, bool balanced, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->trainAuto(*samples, layout, *responses, kFold, *Cgrid, *gammaGrid, *pGrid, *nuGrid, *coeffGrid, *degreeGrid, balanced);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// getSupportVectors() /usr/include/opencv2/ml.hpp:766
	void cv_ml_SVM_getSupportVectors_const(const cv::ml::SVM* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getSupportVectors();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getUncompressedSupportVectors() /usr/include/opencv2/ml.hpp:774
	void cv_ml_SVM_getUncompressedSupportVectors_const(const cv::ml::SVM* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getUncompressedSupportVectors();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getDecisionFunction(int, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ml.hpp:791
	void cv_ml_SVM_getDecisionFunction_const_int_const__OutputArrayR_const__OutputArrayR(const cv::ml::SVM* instance, int i, const cv::_OutputArray* alpha, const cv::_OutputArray* svidx, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDecisionFunction(i, *alpha, *svidx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// getDefaultGrid(int) /usr/include/opencv2/ml.hpp:801
	void cv_ml_SVM_getDefaultGrid_int(int param_id, Result<cv::ml::ParamGrid*>* ocvrs_return) {
		try {
			cv::ml::ParamGrid ret = cv::ml::SVM::getDefaultGrid(param_id);
			Ok(new cv::ml::ParamGrid(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ml::ParamGrid*>))
	}
	
	// getDefaultGridPtr(int) /usr/include/opencv2/ml.hpp:811
	void cv_ml_SVM_getDefaultGridPtr_int(int param_id, Result<cv::Ptr<cv::ml::ParamGrid>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::ParamGrid> ret = cv::ml::SVM::getDefaultGridPtr(param_id);
			Ok(new cv::Ptr<cv::ml::ParamGrid>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::ParamGrid>*>))
	}
	
	// create() /usr/include/opencv2/ml.hpp:816
	void cv_ml_SVM_create(Result<cv::Ptr<cv::ml::SVM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::SVM> ret = cv::ml::SVM::create();
			Ok(new cv::Ptr<cv::ml::SVM>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::SVM>*>))
	}
	
	// load(const cv::String &) /usr/include/opencv2/ml.hpp:825
	void cv_ml_SVM_load_const_StringR(const char* filepath, Result<cv::Ptr<cv::ml::SVM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::SVM> ret = cv::ml::SVM::load(std::string(filepath));
			Ok(new cv::Ptr<cv::ml::SVM>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::SVM>*>))
	}
	
	// getType() /usr/include/opencv2/ml.hpp:533
	void cv_ml_SVM_Kernel_getType_const(const cv::ml::SVM::Kernel* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// calc(int, int, const float *, const float *, float *) /usr/include/opencv2/ml.hpp:534
	void cv_ml_SVM_Kernel_calc_int_int_const_floatX_const_floatX_floatX(cv::ml::SVM::Kernel* instance, int vcount, int n, const float* vecs, const float* another, float* results, Result_void* ocvrs_return) {
		try {
			instance->calc(vcount, n, vecs, another, results);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWeights() /usr/include/opencv2/ml.hpp:1818
	void cv_ml_SVMSGD_getWeights(cv::ml::SVMSGD* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getWeights();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getShift() /usr/include/opencv2/ml.hpp:1823
	void cv_ml_SVMSGD_getShift(cv::ml::SVMSGD* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getShift();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// create() /usr/include/opencv2/ml.hpp:1829
	void cv_ml_SVMSGD_create(Result<cv::Ptr<cv::ml::SVMSGD>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::SVMSGD> ret = cv::ml::SVMSGD::create();
			Ok(new cv::Ptr<cv::ml::SVMSGD>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::SVMSGD>*>))
	}
	
	// load(const cv::String &, const cv::String &) /usr/include/opencv2/ml.hpp:1840
	void cv_ml_SVMSGD_load_const_StringR_const_StringR(const char* filepath, const char* nodeName, Result<cv::Ptr<cv::ml::SVMSGD>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::SVMSGD> ret = cv::ml::SVMSGD::load(std::string(filepath), std::string(nodeName));
			Ok(new cv::Ptr<cv::ml::SVMSGD>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::SVMSGD>*>))
	}
	
	// setOptimalParameters(int, int) /usr/include/opencv2/ml.hpp:1846
	void cv_ml_SVMSGD_setOptimalParameters_int_int(cv::ml::SVMSGD* instance, int svmsgdType, int marginType, Result_void* ocvrs_return) {
		try {
			instance->setOptimalParameters(svmsgdType, marginType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSvmsgdType() /usr/include/opencv2/ml.hpp:1850
	void cv_ml_SVMSGD_getSvmsgdType_const(const cv::ml::SVMSGD* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSvmsgdType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setSvmsgdType(int) /usr/include/opencv2/ml.hpp:1852
	void cv_ml_SVMSGD_setSvmsgdType_int(cv::ml::SVMSGD* instance, int svmsgdType, Result_void* ocvrs_return) {
		try {
			instance->setSvmsgdType(svmsgdType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMarginType() /usr/include/opencv2/ml.hpp:1856
	void cv_ml_SVMSGD_getMarginType_const(const cv::ml::SVMSGD* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMarginType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMarginType(int) /usr/include/opencv2/ml.hpp:1858
	void cv_ml_SVMSGD_setMarginType_int(cv::ml::SVMSGD* instance, int marginType, Result_void* ocvrs_return) {
		try {
			instance->setMarginType(marginType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMarginRegularization() /usr/include/opencv2/ml.hpp:1862
	void cv_ml_SVMSGD_getMarginRegularization_const(const cv::ml::SVMSGD* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMarginRegularization();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setMarginRegularization(float) /usr/include/opencv2/ml.hpp:1864
	void cv_ml_SVMSGD_setMarginRegularization_float(cv::ml::SVMSGD* instance, float marginRegularization, Result_void* ocvrs_return) {
		try {
			instance->setMarginRegularization(marginRegularization);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getInitialStepSize() /usr/include/opencv2/ml.hpp:1868
	void cv_ml_SVMSGD_getInitialStepSize_const(const cv::ml::SVMSGD* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getInitialStepSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setInitialStepSize(float) /usr/include/opencv2/ml.hpp:1870
	void cv_ml_SVMSGD_setInitialStepSize_float(cv::ml::SVMSGD* instance, float InitialStepSize, Result_void* ocvrs_return) {
		try {
			instance->setInitialStepSize(InitialStepSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getStepDecreasingPower() /usr/include/opencv2/ml.hpp:1874
	void cv_ml_SVMSGD_getStepDecreasingPower_const(const cv::ml::SVMSGD* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getStepDecreasingPower();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setStepDecreasingPower(float) /usr/include/opencv2/ml.hpp:1876
	void cv_ml_SVMSGD_setStepDecreasingPower_float(cv::ml::SVMSGD* instance, float stepDecreasingPower, Result_void* ocvrs_return) {
		try {
			instance->setStepDecreasingPower(stepDecreasingPower);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTermCriteria() /usr/include/opencv2/ml.hpp:1882
	void cv_ml_SVMSGD_getTermCriteria_const(const cv::ml::SVMSGD* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TermCriteria>))
	}
	
	// setTermCriteria(const cv::TermCriteria &) /usr/include/opencv2/ml.hpp:1884
	void cv_ml_SVMSGD_setTermCriteria_const_TermCriteriaR(cv::ml::SVMSGD* instance, const cv::TermCriteria* val, Result_void* ocvrs_return) {
		try {
			instance->setTermCriteria(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getVarCount() /usr/include/opencv2/ml.hpp:330
	void cv_ml_StatModel_getVarCount_const(const cv::ml::StatModel* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getVarCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// empty() /usr/include/opencv2/ml.hpp:332
	void cv_ml_StatModel_empty_const(const cv::ml::StatModel* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isTrained() /usr/include/opencv2/ml.hpp:335
	void cv_ml_StatModel_isTrained_const(const cv::ml::StatModel* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isTrained();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isClassifier() /usr/include/opencv2/ml.hpp:337
	void cv_ml_StatModel_isClassifier_const(const cv::ml::StatModel* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isClassifier();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// train(const Ptr<cv::ml::TrainData> &, int) /usr/include/opencv2/ml.hpp:346
	void cv_ml_StatModel_train_const_Ptr_TrainData_R_int(cv::ml::StatModel* instance, const cv::Ptr<cv::ml::TrainData>* trainData, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->train(*trainData, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// train(cv::InputArray, int, cv::InputArray) /usr/include/opencv2/ml.hpp:354
	void cv_ml_StatModel_train_const__InputArrayR_int_const__InputArrayR(cv::ml::StatModel* instance, const cv::_InputArray* samples, int layout, const cv::_InputArray* responses, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->train(*samples, layout, *responses);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// calcError(const Ptr<cv::ml::TrainData> &, bool, cv::OutputArray) /usr/include/opencv2/ml.hpp:369
	void cv_ml_StatModel_calcError_const_const_Ptr_TrainData_R_bool_const__OutputArrayR(const cv::ml::StatModel* instance, const cv::Ptr<cv::ml::TrainData>* data, bool test, const cv::_OutputArray* resp, Result<float>* ocvrs_return) {
		try {
			float ret = instance->calcError(*data, test, *resp);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// predict(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ml.hpp:377
	void cv_ml_StatModel_predict_const_const__InputArrayR_const__OutputArrayR_int(const cv::ml::StatModel* instance, const cv::_InputArray* samples, const cv::_OutputArray* results, int flags, Result<float>* ocvrs_return) {
		try {
			float ret = instance->predict(*samples, *results, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// missingValue() /usr/include/opencv2/ml.hpp:148
	void cv_ml_TrainData_missingValue(Result<float>* ocvrs_return) {
		try {
			float ret = cv::ml::TrainData::missingValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// getLayout() /usr/include/opencv2/ml.hpp:151
	void cv_ml_TrainData_getLayout_const(const cv::ml::TrainData* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLayout();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getNTrainSamples() /usr/include/opencv2/ml.hpp:152
	void cv_ml_TrainData_getNTrainSamples_const(const cv::ml::TrainData* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNTrainSamples();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getNTestSamples() /usr/include/opencv2/ml.hpp:153
	void cv_ml_TrainData_getNTestSamples_const(const cv::ml::TrainData* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNTestSamples();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getNSamples() /usr/include/opencv2/ml.hpp:154
	void cv_ml_TrainData_getNSamples_const(const cv::ml::TrainData* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNSamples();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getNVars() /usr/include/opencv2/ml.hpp:155
	void cv_ml_TrainData_getNVars_const(const cv::ml::TrainData* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNVars();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getNAllVars() /usr/include/opencv2/ml.hpp:156
	void cv_ml_TrainData_getNAllVars_const(const cv::ml::TrainData* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNAllVars();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getSample(cv::InputArray, int, float *) /usr/include/opencv2/ml.hpp:158
	void cv_ml_TrainData_getSample_const_const__InputArrayR_int_floatX(const cv::ml::TrainData* instance, const cv::_InputArray* varIdx, int sidx, float* buf, Result_void* ocvrs_return) {
		try {
			instance->getSample(*varIdx, sidx, buf);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSamples() /usr/include/opencv2/ml.hpp:159
	void cv_ml_TrainData_getSamples_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getSamples();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getMissing() /usr/include/opencv2/ml.hpp:160
	void cv_ml_TrainData_getMissing_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMissing();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getTrainSamples(int, bool, bool) /usr/include/opencv2/ml.hpp:174
	void cv_ml_TrainData_getTrainSamples_const_int_bool_bool(const cv::ml::TrainData* instance, int layout, bool compressSamples, bool compressVars, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTrainSamples(layout, compressSamples, compressVars);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getTrainResponses() /usr/include/opencv2/ml.hpp:183
	void cv_ml_TrainData_getTrainResponses_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTrainResponses();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getTrainNormCatResponses() /usr/include/opencv2/ml.hpp:191
	void cv_ml_TrainData_getTrainNormCatResponses_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTrainNormCatResponses();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getTestResponses() /usr/include/opencv2/ml.hpp:192
	void cv_ml_TrainData_getTestResponses_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTestResponses();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getTestNormCatResponses() /usr/include/opencv2/ml.hpp:193
	void cv_ml_TrainData_getTestNormCatResponses_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTestNormCatResponses();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getResponses() /usr/include/opencv2/ml.hpp:194
	void cv_ml_TrainData_getResponses_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getResponses();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getNormCatResponses() /usr/include/opencv2/ml.hpp:195
	void cv_ml_TrainData_getNormCatResponses_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getNormCatResponses();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getSampleWeights() /usr/include/opencv2/ml.hpp:196
	void cv_ml_TrainData_getSampleWeights_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getSampleWeights();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getTrainSampleWeights() /usr/include/opencv2/ml.hpp:197
	void cv_ml_TrainData_getTrainSampleWeights_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTrainSampleWeights();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getTestSampleWeights() /usr/include/opencv2/ml.hpp:198
	void cv_ml_TrainData_getTestSampleWeights_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTestSampleWeights();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getVarIdx() /usr/include/opencv2/ml.hpp:199
	void cv_ml_TrainData_getVarIdx_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getVarIdx();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getVarType() /usr/include/opencv2/ml.hpp:200
	void cv_ml_TrainData_getVarType_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getVarType();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getVarSymbolFlags() /usr/include/opencv2/ml.hpp:201
	void cv_ml_TrainData_getVarSymbolFlags_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getVarSymbolFlags();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getResponseType() /usr/include/opencv2/ml.hpp:202
	void cv_ml_TrainData_getResponseType_const(const cv::ml::TrainData* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getResponseType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getTrainSampleIdx() /usr/include/opencv2/ml.hpp:203
	void cv_ml_TrainData_getTrainSampleIdx_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTrainSampleIdx();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getTestSampleIdx() /usr/include/opencv2/ml.hpp:204
	void cv_ml_TrainData_getTestSampleIdx_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTestSampleIdx();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getValues(int, cv::InputArray, float *) /usr/include/opencv2/ml.hpp:205
	void cv_ml_TrainData_getValues_const_int_const__InputArrayR_floatX(const cv::ml::TrainData* instance, int vi, const cv::_InputArray* sidx, float* values, Result_void* ocvrs_return) {
		try {
			instance->getValues(vi, *sidx, values);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNormCatValues(int, cv::InputArray, int *) /usr/include/opencv2/ml.hpp:206
	void cv_ml_TrainData_getNormCatValues_const_int_const__InputArrayR_intX(const cv::ml::TrainData* instance, int vi, const cv::_InputArray* sidx, int* values, Result_void* ocvrs_return) {
		try {
			instance->getNormCatValues(vi, *sidx, values);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDefaultSubstValues() /usr/include/opencv2/ml.hpp:207
	void cv_ml_TrainData_getDefaultSubstValues_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getDefaultSubstValues();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getCatCount(int) /usr/include/opencv2/ml.hpp:209
	void cv_ml_TrainData_getCatCount_const_int(const cv::ml::TrainData* instance, int vi, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCatCount(vi);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getClassLabels() /usr/include/opencv2/ml.hpp:215
	void cv_ml_TrainData_getClassLabels_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getClassLabels();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getCatOfs() /usr/include/opencv2/ml.hpp:217
	void cv_ml_TrainData_getCatOfs_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCatOfs();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getCatMap() /usr/include/opencv2/ml.hpp:218
	void cv_ml_TrainData_getCatMap_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCatMap();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// setTrainTestSplit(int, bool) /usr/include/opencv2/ml.hpp:223
	void cv_ml_TrainData_setTrainTestSplit_int_bool(cv::ml::TrainData* instance, int count, bool shuffle, Result_void* ocvrs_return) {
		try {
			instance->setTrainTestSplit(count, shuffle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setTrainTestSplitRatio(double, bool) /usr/include/opencv2/ml.hpp:233
	void cv_ml_TrainData_setTrainTestSplitRatio_double_bool(cv::ml::TrainData* instance, double ratio, bool shuffle, Result_void* ocvrs_return) {
		try {
			instance->setTrainTestSplitRatio(ratio, shuffle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// shuffleTrainTest() /usr/include/opencv2/ml.hpp:234
	void cv_ml_TrainData_shuffleTrainTest(cv::ml::TrainData* instance, Result_void* ocvrs_return) {
		try {
			instance->shuffleTrainTest();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTestSamples() /usr/include/opencv2/ml.hpp:237
	void cv_ml_TrainData_getTestSamples_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTestSamples();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getNames(std::vector<String> &) /usr/include/opencv2/ml.hpp:240
	void cv_ml_TrainData_getNames_const_vector_String_R(const cv::ml::TrainData* instance, std::vector<cv::String>* names, Result_void* ocvrs_return) {
		try {
			instance->getNames(*names);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSubVector(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/ml.hpp:246
	void cv_ml_TrainData_getSubVector_const_MatR_const_MatR(const cv::Mat* vec, const cv::Mat* idx, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::ml::TrainData::getSubVector(*vec, *idx);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getSubMatrix(const cv::Mat &, const cv::Mat &, int) /usr/include/opencv2/ml.hpp:253
	void cv_ml_TrainData_getSubMatrix_const_MatR_const_MatR_int(const cv::Mat* matrix, const cv::Mat* idx, int layout, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::ml::TrainData::getSubMatrix(*matrix, *idx, layout);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// loadFromCSV(const cv::String &, int, int, int, const cv::String &, char, char) /usr/include/opencv2/ml.hpp:284
	void cv_ml_TrainData_loadFromCSV_const_StringR_int_int_int_const_StringR_char_char(const char* filename, int headerLineCount, int responseStartIdx, int responseEndIdx, const char* varTypeSpec, char delimiter, char missch, Result<cv::Ptr<cv::ml::TrainData>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::TrainData> ret = cv::ml::TrainData::loadFromCSV(std::string(filename), headerLineCount, responseStartIdx, responseEndIdx, std::string(varTypeSpec), delimiter, missch);
			Ok(new cv::Ptr<cv::ml::TrainData>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::TrainData>*>))
	}
	
	// create(cv::InputArray, int, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/ml.hpp:311
	void cv_ml_TrainData_create_const__InputArrayR_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* samples, int layout, const cv::_InputArray* responses, const cv::_InputArray* varIdx, const cv::_InputArray* sampleIdx, const cv::_InputArray* sampleWeights, const cv::_InputArray* varType, Result<cv::Ptr<cv::ml::TrainData>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::TrainData> ret = cv::ml::TrainData::create(*samples, layout, *responses, *varIdx, *sampleIdx, *sampleWeights, *varType);
			Ok(new cv::Ptr<cv::ml::TrainData>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ml::TrainData>*>))
	}
	
}
