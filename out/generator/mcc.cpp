#include "ocvrs_common.hpp"
#include <opencv2/mcc.hpp>
#include "mcc_types.hpp"

extern "C" {
	void cv_ColorCorrectionModel_delete(cv::ccm::ColorCorrectionModel* instance) {
		delete instance;
	}
	// ColorCorrectionModel(const cv::Mat &, cv::ccm::CONST_COLOR) /usr/include/opencv2/mcc/ccm.hpp:374
	void cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_CONST_COLOR(const cv::Mat* src, cv::ccm::CONST_COLOR constcolor, Result<cv::ccm::ColorCorrectionModel*>* ocvrs_return) {
		try {
			cv::ccm::ColorCorrectionModel* ret = new cv::ccm::ColorCorrectionModel(*src, constcolor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ccm::ColorCorrectionModel*>))
	}
	
	// ColorCorrectionModel(const cv::Mat &, cv::Mat, cv::ccm::COLOR_SPACE) /usr/include/opencv2/mcc/ccm.hpp:383
	void cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_Mat_COLOR_SPACE(const cv::Mat* src, cv::Mat* colors, cv::ccm::COLOR_SPACE ref_cs, Result<cv::ccm::ColorCorrectionModel*>* ocvrs_return) {
		try {
			cv::ccm::ColorCorrectionModel* ret = new cv::ccm::ColorCorrectionModel(*src, *colors, ref_cs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ccm::ColorCorrectionModel*>))
	}
	
	// ColorCorrectionModel(const cv::Mat &, cv::Mat, cv::ccm::COLOR_SPACE, cv::Mat) /usr/include/opencv2/mcc/ccm.hpp:393
	void cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_Mat_COLOR_SPACE_Mat(const cv::Mat* src, cv::Mat* colors, cv::ccm::COLOR_SPACE ref_cs, cv::Mat* colored, Result<cv::ccm::ColorCorrectionModel*>* ocvrs_return) {
		try {
			cv::ccm::ColorCorrectionModel* ret = new cv::ccm::ColorCorrectionModel(*src, *colors, ref_cs, *colored);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ccm::ColorCorrectionModel*>))
	}
	
	// setColorSpace(cv::ccm::COLOR_SPACE) /usr/include/opencv2/mcc/ccm.hpp:409
	void cv_ccm_ColorCorrectionModel_setColorSpace_COLOR_SPACE(cv::ccm::ColorCorrectionModel* instance, cv::ccm::COLOR_SPACE cs, Result_void* ocvrs_return) {
		try {
			instance->setColorSpace(cs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setCCM_TYPE(cv::ccm::CCM_TYPE) /usr/include/opencv2/mcc/ccm.hpp:415
	void cv_ccm_ColorCorrectionModel_setCCM_TYPE_CCM_TYPE(cv::ccm::ColorCorrectionModel* instance, cv::ccm::CCM_TYPE ccm_type, Result_void* ocvrs_return) {
		try {
			instance->setCCM_TYPE(ccm_type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setDistance(cv::ccm::DISTANCE_TYPE) /usr/include/opencv2/mcc/ccm.hpp:421
	void cv_ccm_ColorCorrectionModel_setDistance_DISTANCE_TYPE(cv::ccm::ColorCorrectionModel* instance, cv::ccm::DISTANCE_TYPE distance, Result_void* ocvrs_return) {
		try {
			instance->setDistance(distance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setLinear(cv::ccm::LINEAR_TYPE) /usr/include/opencv2/mcc/ccm.hpp:427
	void cv_ccm_ColorCorrectionModel_setLinear_LINEAR_TYPE(cv::ccm::ColorCorrectionModel* instance, cv::ccm::LINEAR_TYPE linear_type, Result_void* ocvrs_return) {
		try {
			instance->setLinear(linear_type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setLinearGamma(const double &) /usr/include/opencv2/mcc/ccm.hpp:436
	void cv_ccm_ColorCorrectionModel_setLinearGamma_const_doubleR(cv::ccm::ColorCorrectionModel* instance, const double* gamma, Result_void* ocvrs_return) {
		try {
			instance->setLinearGamma(*gamma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setLinearDegree(const int &) /usr/include/opencv2/mcc/ccm.hpp:449
	void cv_ccm_ColorCorrectionModel_setLinearDegree_const_intR(cv::ccm::ColorCorrectionModel* instance, const int* deg, Result_void* ocvrs_return) {
		try {
			instance->setLinearDegree(*deg);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setSaturatedThreshold(const double &, const double &) /usr/include/opencv2/mcc/ccm.hpp:459
	void cv_ccm_ColorCorrectionModel_setSaturatedThreshold_const_doubleR_const_doubleR(cv::ccm::ColorCorrectionModel* instance, const double* lower, const double* upper, Result_void* ocvrs_return) {
		try {
			instance->setSaturatedThreshold(*lower, *upper);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setWeightsList(const cv::Mat &) /usr/include/opencv2/mcc/ccm.hpp:465
	void cv_ccm_ColorCorrectionModel_setWeightsList_const_MatR(cv::ccm::ColorCorrectionModel* instance, const cv::Mat* weights_list, Result_void* ocvrs_return) {
		try {
			instance->setWeightsList(*weights_list);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setWeightCoeff(const double &) /usr/include/opencv2/mcc/ccm.hpp:471
	void cv_ccm_ColorCorrectionModel_setWeightCoeff_const_doubleR(cv::ccm::ColorCorrectionModel* instance, const double* weights_coeff, Result_void* ocvrs_return) {
		try {
			instance->setWeightCoeff(*weights_coeff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setInitialMethod(cv::ccm::INITIAL_METHOD_TYPE) /usr/include/opencv2/mcc/ccm.hpp:477
	void cv_ccm_ColorCorrectionModel_setInitialMethod_INITIAL_METHOD_TYPE(cv::ccm::ColorCorrectionModel* instance, cv::ccm::INITIAL_METHOD_TYPE initial_method_type, Result_void* ocvrs_return) {
		try {
			instance->setInitialMethod(initial_method_type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setMaxCount(const int &) /usr/include/opencv2/mcc/ccm.hpp:484
	void cv_ccm_ColorCorrectionModel_setMaxCount_const_intR(cv::ccm::ColorCorrectionModel* instance, const int* max_count, Result_void* ocvrs_return) {
		try {
			instance->setMaxCount(*max_count);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setEpsilon(const double &) /usr/include/opencv2/mcc/ccm.hpp:491
	void cv_ccm_ColorCorrectionModel_setEpsilon_const_doubleR(cv::ccm::ColorCorrectionModel* instance, const double* epsilon, Result_void* ocvrs_return) {
		try {
			instance->setEpsilon(*epsilon);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// run() /usr/include/opencv2/mcc/ccm.hpp:494
	void cv_ccm_ColorCorrectionModel_run(cv::ccm::ColorCorrectionModel* instance, Result_void* ocvrs_return) {
		try {
			instance->run();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getCCM() /usr/include/opencv2/mcc/ccm.hpp:496
	void cv_ccm_ColorCorrectionModel_getCCM_const(const cv::ccm::ColorCorrectionModel* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCCM();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getLoss() /usr/include/opencv2/mcc/ccm.hpp:497
	void cv_ccm_ColorCorrectionModel_getLoss_const(const cv::ccm::ColorCorrectionModel* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getLoss();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// get_src_rgbl() /usr/include/opencv2/mcc/ccm.hpp:498
	void cv_ccm_ColorCorrectionModel_get_src_rgbl_const(const cv::ccm::ColorCorrectionModel* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->get_src_rgbl();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// get_dst_rgbl() /usr/include/opencv2/mcc/ccm.hpp:499
	void cv_ccm_ColorCorrectionModel_get_dst_rgbl_const(const cv::ccm::ColorCorrectionModel* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->get_dst_rgbl();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getMask() /usr/include/opencv2/mcc/ccm.hpp:500
	void cv_ccm_ColorCorrectionModel_getMask_const(const cv::ccm::ColorCorrectionModel* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMask();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getWeights() /usr/include/opencv2/mcc/ccm.hpp:501
	void cv_ccm_ColorCorrectionModel_getWeights_const(const cv::ccm::ColorCorrectionModel* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getWeights();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// infer(const cv::Mat &, bool) /usr/include/opencv2/mcc/ccm.hpp:508
	void cv_ccm_ColorCorrectionModel_infer_const_MatR_bool(cv::ccm::ColorCorrectionModel* instance, const cv::Mat* img, bool islinear, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->infer(*img, islinear);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// create() /usr/include/opencv2/mcc/checker_model.hpp:73
	void cv_mcc_CChecker_create(Result<cv::Ptr<cv::mcc::CChecker>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::mcc::CChecker> ret = cv::mcc::CChecker::create();
			Ok(new cv::Ptr<cv::mcc::CChecker>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::mcc::CChecker>*>))
	}
	
	// setTarget(cv::mcc::TYPECHART) /usr/include/opencv2/mcc/checker_model.hpp:83
	void cv_mcc_CChecker_setTarget_TYPECHART(cv::mcc::CChecker* instance, cv::mcc::TYPECHART _target, Result_void* ocvrs_return) {
		try {
			instance->setTarget(_target);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setBox(std::vector<Point2f>) /usr/include/opencv2/mcc/checker_model.hpp:84
	void cv_mcc_CChecker_setBox_vector_Point2f_(cv::mcc::CChecker* instance, std::vector<cv::Point2f>* _box, Result_void* ocvrs_return) {
		try {
			instance->setBox(*_box);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setChartsRGB(cv::Mat) /usr/include/opencv2/mcc/checker_model.hpp:85
	void cv_mcc_CChecker_setChartsRGB_Mat(cv::mcc::CChecker* instance, cv::Mat* _chartsRGB, Result_void* ocvrs_return) {
		try {
			instance->setChartsRGB(*_chartsRGB);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setChartsYCbCr(cv::Mat) /usr/include/opencv2/mcc/checker_model.hpp:86
	void cv_mcc_CChecker_setChartsYCbCr_Mat(cv::mcc::CChecker* instance, cv::Mat* _chartsYCbCr, Result_void* ocvrs_return) {
		try {
			instance->setChartsYCbCr(*_chartsYCbCr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setCost(float) /usr/include/opencv2/mcc/checker_model.hpp:87
	void cv_mcc_CChecker_setCost_float(cv::mcc::CChecker* instance, float _cost, Result_void* ocvrs_return) {
		try {
			instance->setCost(_cost);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setCenter(cv::Point2f) /usr/include/opencv2/mcc/checker_model.hpp:88
	void cv_mcc_CChecker_setCenter_Point2f(cv::mcc::CChecker* instance, cv::Point2f* _center, Result_void* ocvrs_return) {
		try {
			instance->setCenter(*_center);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTarget() /usr/include/opencv2/mcc/checker_model.hpp:90
	void cv_mcc_CChecker_getTarget(cv::mcc::CChecker* instance, Result<cv::mcc::TYPECHART>* ocvrs_return) {
		try {
			cv::mcc::TYPECHART ret = instance->getTarget();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::mcc::TYPECHART>))
	}
	
	// getBox() /usr/include/opencv2/mcc/checker_model.hpp:91
	void cv_mcc_CChecker_getBox(cv::mcc::CChecker* instance, Result<std::vector<cv::Point2f>*>* ocvrs_return) {
		try {
			std::vector<cv::Point2f> ret = instance->getBox();
			Ok(new std::vector<cv::Point2f>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Point2f>*>))
	}
	
	// getChartsRGB() /usr/include/opencv2/mcc/checker_model.hpp:92
	void cv_mcc_CChecker_getChartsRGB(cv::mcc::CChecker* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getChartsRGB();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getChartsYCbCr() /usr/include/opencv2/mcc/checker_model.hpp:93
	void cv_mcc_CChecker_getChartsYCbCr(cv::mcc::CChecker* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getChartsYCbCr();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getCost() /usr/include/opencv2/mcc/checker_model.hpp:94
	void cv_mcc_CChecker_getCost(cv::mcc::CChecker* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getCost();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// getCenter() /usr/include/opencv2/mcc/checker_model.hpp:95
	void cv_mcc_CChecker_getCenter(cv::mcc::CChecker* instance, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->getCenter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	// setNet(dnn::Net) /usr/include/opencv2/mcc/checker_detector.hpp:144
	void cv_mcc_CCheckerDetector_setNet_Net(cv::mcc::CCheckerDetector* instance, cv::dnn::Net* net, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setNet(*net);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// process(cv::InputArray, const cv::mcc::TYPECHART, const std::vector<Rect> &, const int, bool, const Ptr<cv::mcc::DetectorParameters> &) /usr/include/opencv2/mcc/checker_detector.hpp:167
	void cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART_const_vector_Rect_R_const_int_bool_const_Ptr_DetectorParameters_R(cv::mcc::CCheckerDetector* instance, const cv::_InputArray* image, const cv::mcc::TYPECHART chartType, const std::vector<cv::Rect>* regionsOfInterest, const int nc, bool useNet, const cv::Ptr<cv::mcc::DetectorParameters>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->process(*image, chartType, *regionsOfInterest, nc, useNet, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// process(cv::InputArray, const cv::mcc::TYPECHART, const int, bool, const Ptr<cv::mcc::DetectorParameters> &) /usr/include/opencv2/mcc/checker_detector.hpp:195
	void cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART_const_int_bool_const_Ptr_DetectorParameters_R(cv::mcc::CCheckerDetector* instance, const cv::_InputArray* image, const cv::mcc::TYPECHART chartType, const int nc, bool useNet, const cv::Ptr<cv::mcc::DetectorParameters>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->process(*image, chartType, nc, useNet, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// getBestColorChecker() /usr/include/opencv2/mcc/checker_detector.hpp:204
	void cv_mcc_CCheckerDetector_getBestColorChecker(cv::mcc::CCheckerDetector* instance, Result<cv::Ptr<cv::mcc::CChecker>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::mcc::CChecker> ret = instance->getBestColorChecker();
			Ok(new cv::Ptr<cv::mcc::CChecker>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::mcc::CChecker>*>))
	}
	
	// getListColorChecker() /usr/include/opencv2/mcc/checker_detector.hpp:209
	void cv_mcc_CCheckerDetector_getListColorChecker(cv::mcc::CCheckerDetector* instance, Result<std::vector<cv::Ptr<cv::mcc::CChecker>>*>* ocvrs_return) {
		try {
			std::vector<cv::Ptr<cv::mcc::CChecker>> ret = instance->getListColorChecker();
			Ok(new std::vector<cv::Ptr<cv::mcc::CChecker>>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Ptr<cv::mcc::CChecker>>*>))
	}
	
	// create() /usr/include/opencv2/mcc/checker_detector.hpp:214
	void cv_mcc_CCheckerDetector_create(Result<cv::Ptr<cv::mcc::CCheckerDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::mcc::CCheckerDetector> ret = cv::mcc::CCheckerDetector::create();
			Ok(new cv::Ptr<cv::mcc::CCheckerDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::mcc::CCheckerDetector>*>))
	}
	
	// draw(cv::InputOutputArray) /usr/include/opencv2/mcc/checker_model.hpp:121
	void cv_mcc_CCheckerDraw_draw_const__InputOutputArrayR(cv::mcc::CCheckerDraw* instance, const cv::_InputOutputArray* img, Result_void* ocvrs_return) {
		try {
			instance->draw(*img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(Ptr<cv::mcc::CChecker>, cv::Scalar, int) /usr/include/opencv2/mcc/checker_model.hpp:130
	void cv_mcc_CCheckerDraw_create_Ptr_CChecker__Scalar_int(cv::Ptr<cv::mcc::CChecker>* pChecker, cv::Scalar* color, int thickness, Result<cv::Ptr<cv::mcc::CCheckerDraw>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::mcc::CCheckerDraw> ret = cv::mcc::CCheckerDraw::create(*pChecker, *color, thickness);
			Ok(new cv::Ptr<cv::mcc::CCheckerDraw>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::mcc::CCheckerDraw>*>))
	}
	
	// adaptiveThreshWinSizeMin /usr/include/opencv2/mcc/checker_detector.hpp:106
	int cv_mcc_DetectorParameters_getPropAdaptiveThreshWinSizeMin_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeMin;
			return ret;
	}
	
	// adaptiveThreshWinSizeMin /usr/include/opencv2/mcc/checker_detector.hpp:106
	void cv_mcc_DetectorParameters_setPropAdaptiveThreshWinSizeMin_int(cv::mcc::DetectorParameters* instance, int val) {
			instance->adaptiveThreshWinSizeMin = val;
	}
	
	// adaptiveThreshWinSizeMax /usr/include/opencv2/mcc/checker_detector.hpp:107
	int cv_mcc_DetectorParameters_getPropAdaptiveThreshWinSizeMax_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeMax;
			return ret;
	}
	
	// adaptiveThreshWinSizeMax /usr/include/opencv2/mcc/checker_detector.hpp:107
	void cv_mcc_DetectorParameters_setPropAdaptiveThreshWinSizeMax_int(cv::mcc::DetectorParameters* instance, int val) {
			instance->adaptiveThreshWinSizeMax = val;
	}
	
	// adaptiveThreshWinSizeStep /usr/include/opencv2/mcc/checker_detector.hpp:108
	int cv_mcc_DetectorParameters_getPropAdaptiveThreshWinSizeStep_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeStep;
			return ret;
	}
	
	// adaptiveThreshWinSizeStep /usr/include/opencv2/mcc/checker_detector.hpp:108
	void cv_mcc_DetectorParameters_setPropAdaptiveThreshWinSizeStep_int(cv::mcc::DetectorParameters* instance, int val) {
			instance->adaptiveThreshWinSizeStep = val;
	}
	
	// adaptiveThreshConstant /usr/include/opencv2/mcc/checker_detector.hpp:109
	double cv_mcc_DetectorParameters_getPropAdaptiveThreshConstant_const(const cv::mcc::DetectorParameters* instance) {
			double ret = instance->adaptiveThreshConstant;
			return ret;
	}
	
	// adaptiveThreshConstant /usr/include/opencv2/mcc/checker_detector.hpp:109
	void cv_mcc_DetectorParameters_setPropAdaptiveThreshConstant_double(cv::mcc::DetectorParameters* instance, double val) {
			instance->adaptiveThreshConstant = val;
	}
	
	// minContoursAreaRate /usr/include/opencv2/mcc/checker_detector.hpp:110
	double cv_mcc_DetectorParameters_getPropMinContoursAreaRate_const(const cv::mcc::DetectorParameters* instance) {
			double ret = instance->minContoursAreaRate;
			return ret;
	}
	
	// minContoursAreaRate /usr/include/opencv2/mcc/checker_detector.hpp:110
	void cv_mcc_DetectorParameters_setPropMinContoursAreaRate_double(cv::mcc::DetectorParameters* instance, double val) {
			instance->minContoursAreaRate = val;
	}
	
	// minContoursArea /usr/include/opencv2/mcc/checker_detector.hpp:111
	double cv_mcc_DetectorParameters_getPropMinContoursArea_const(const cv::mcc::DetectorParameters* instance) {
			double ret = instance->minContoursArea;
			return ret;
	}
	
	// minContoursArea /usr/include/opencv2/mcc/checker_detector.hpp:111
	void cv_mcc_DetectorParameters_setPropMinContoursArea_double(cv::mcc::DetectorParameters* instance, double val) {
			instance->minContoursArea = val;
	}
	
	// confidenceThreshold /usr/include/opencv2/mcc/checker_detector.hpp:112
	double cv_mcc_DetectorParameters_getPropConfidenceThreshold_const(const cv::mcc::DetectorParameters* instance) {
			double ret = instance->confidenceThreshold;
			return ret;
	}
	
	// confidenceThreshold /usr/include/opencv2/mcc/checker_detector.hpp:112
	void cv_mcc_DetectorParameters_setPropConfidenceThreshold_double(cv::mcc::DetectorParameters* instance, double val) {
			instance->confidenceThreshold = val;
	}
	
	// minContourSolidity /usr/include/opencv2/mcc/checker_detector.hpp:113
	double cv_mcc_DetectorParameters_getPropMinContourSolidity_const(const cv::mcc::DetectorParameters* instance) {
			double ret = instance->minContourSolidity;
			return ret;
	}
	
	// minContourSolidity /usr/include/opencv2/mcc/checker_detector.hpp:113
	void cv_mcc_DetectorParameters_setPropMinContourSolidity_double(cv::mcc::DetectorParameters* instance, double val) {
			instance->minContourSolidity = val;
	}
	
	// findCandidatesApproxPolyDPEpsMultiplier /usr/include/opencv2/mcc/checker_detector.hpp:114
	double cv_mcc_DetectorParameters_getPropFindCandidatesApproxPolyDPEpsMultiplier_const(const cv::mcc::DetectorParameters* instance) {
			double ret = instance->findCandidatesApproxPolyDPEpsMultiplier;
			return ret;
	}
	
	// findCandidatesApproxPolyDPEpsMultiplier /usr/include/opencv2/mcc/checker_detector.hpp:114
	void cv_mcc_DetectorParameters_setPropFindCandidatesApproxPolyDPEpsMultiplier_double(cv::mcc::DetectorParameters* instance, double val) {
			instance->findCandidatesApproxPolyDPEpsMultiplier = val;
	}
	
	// borderWidth /usr/include/opencv2/mcc/checker_detector.hpp:115
	int cv_mcc_DetectorParameters_getPropBorderWidth_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->borderWidth;
			return ret;
	}
	
	// borderWidth /usr/include/opencv2/mcc/checker_detector.hpp:115
	void cv_mcc_DetectorParameters_setPropBorderWidth_int(cv::mcc::DetectorParameters* instance, int val) {
			instance->borderWidth = val;
	}
	
	// B0factor /usr/include/opencv2/mcc/checker_detector.hpp:116
	float cv_mcc_DetectorParameters_getPropB0factor_const(const cv::mcc::DetectorParameters* instance) {
			float ret = instance->B0factor;
			return ret;
	}
	
	// B0factor /usr/include/opencv2/mcc/checker_detector.hpp:116
	void cv_mcc_DetectorParameters_setPropB0factor_float(cv::mcc::DetectorParameters* instance, float val) {
			instance->B0factor = val;
	}
	
	// maxError /usr/include/opencv2/mcc/checker_detector.hpp:117
	float cv_mcc_DetectorParameters_getPropMaxError_const(const cv::mcc::DetectorParameters* instance) {
			float ret = instance->maxError;
			return ret;
	}
	
	// maxError /usr/include/opencv2/mcc/checker_detector.hpp:117
	void cv_mcc_DetectorParameters_setPropMaxError_float(cv::mcc::DetectorParameters* instance, float val) {
			instance->maxError = val;
	}
	
	// minContourPointsAllowed /usr/include/opencv2/mcc/checker_detector.hpp:118
	int cv_mcc_DetectorParameters_getPropMinContourPointsAllowed_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->minContourPointsAllowed;
			return ret;
	}
	
	// minContourPointsAllowed /usr/include/opencv2/mcc/checker_detector.hpp:118
	void cv_mcc_DetectorParameters_setPropMinContourPointsAllowed_int(cv::mcc::DetectorParameters* instance, int val) {
			instance->minContourPointsAllowed = val;
	}
	
	// minContourLengthAllowed /usr/include/opencv2/mcc/checker_detector.hpp:119
	int cv_mcc_DetectorParameters_getPropMinContourLengthAllowed_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->minContourLengthAllowed;
			return ret;
	}
	
	// minContourLengthAllowed /usr/include/opencv2/mcc/checker_detector.hpp:119
	void cv_mcc_DetectorParameters_setPropMinContourLengthAllowed_int(cv::mcc::DetectorParameters* instance, int val) {
			instance->minContourLengthAllowed = val;
	}
	
	// minInterContourDistance /usr/include/opencv2/mcc/checker_detector.hpp:120
	int cv_mcc_DetectorParameters_getPropMinInterContourDistance_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->minInterContourDistance;
			return ret;
	}
	
	// minInterContourDistance /usr/include/opencv2/mcc/checker_detector.hpp:120
	void cv_mcc_DetectorParameters_setPropMinInterContourDistance_int(cv::mcc::DetectorParameters* instance, int val) {
			instance->minInterContourDistance = val;
	}
	
	// minInterCheckerDistance /usr/include/opencv2/mcc/checker_detector.hpp:121
	int cv_mcc_DetectorParameters_getPropMinInterCheckerDistance_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->minInterCheckerDistance;
			return ret;
	}
	
	// minInterCheckerDistance /usr/include/opencv2/mcc/checker_detector.hpp:121
	void cv_mcc_DetectorParameters_setPropMinInterCheckerDistance_int(cv::mcc::DetectorParameters* instance, int val) {
			instance->minInterCheckerDistance = val;
	}
	
	// minImageSize /usr/include/opencv2/mcc/checker_detector.hpp:122
	int cv_mcc_DetectorParameters_getPropMinImageSize_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->minImageSize;
			return ret;
	}
	
	// minImageSize /usr/include/opencv2/mcc/checker_detector.hpp:122
	void cv_mcc_DetectorParameters_setPropMinImageSize_int(cv::mcc::DetectorParameters* instance, int val) {
			instance->minImageSize = val;
	}
	
	// minGroupSize /usr/include/opencv2/mcc/checker_detector.hpp:123
	unsigned int cv_mcc_DetectorParameters_getPropMinGroupSize_const(const cv::mcc::DetectorParameters* instance) {
			unsigned int ret = instance->minGroupSize;
			return ret;
	}
	
	// minGroupSize /usr/include/opencv2/mcc/checker_detector.hpp:123
	void cv_mcc_DetectorParameters_setPropMinGroupSize_unsigned_int(cv::mcc::DetectorParameters* instance, unsigned int val) {
			instance->minGroupSize = val;
	}
	
	void cv_MCC_DetectorParameters_delete(cv::mcc::DetectorParameters* instance) {
		delete instance;
	}
	// DetectorParameters() /usr/include/opencv2/mcc/checker_detector.hpp:102
	void cv_mcc_DetectorParameters_DetectorParameters(Result<cv::mcc::DetectorParameters*>* ocvrs_return) {
		try {
			cv::mcc::DetectorParameters* ret = new cv::mcc::DetectorParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::mcc::DetectorParameters*>))
	}
	
	// create() /usr/include/opencv2/mcc/checker_detector.hpp:104
	void cv_mcc_DetectorParameters_create(Result<cv::Ptr<cv::mcc::DetectorParameters>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::mcc::DetectorParameters> ret = cv::mcc::DetectorParameters::create();
			Ok(new cv::Ptr<cv::mcc::DetectorParameters>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::mcc::DetectorParameters>*>))
	}
	
}
