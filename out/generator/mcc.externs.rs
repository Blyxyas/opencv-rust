extern "C" {
	// ColorCorrectionModel(const cv::Mat &, cv::ccm::CONST_COLOR) /usr/include/opencv2/mcc/ccm.hpp:374
	pub fn cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_CONST_COLOR(src: *const c_void, constcolor: crate::mcc::CONST_COLOR, ocvrs_return: *mut Result<*mut c_void>);
	// ColorCorrectionModel(const cv::Mat &, cv::Mat, cv::ccm::COLOR_SPACE) /usr/include/opencv2/mcc/ccm.hpp:383
	pub fn cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_Mat_COLOR_SPACE(src: *const c_void, colors: *mut c_void, ref_cs: crate::mcc::COLOR_SPACE, ocvrs_return: *mut Result<*mut c_void>);
	// ColorCorrectionModel(const cv::Mat &, cv::Mat, cv::ccm::COLOR_SPACE, cv::Mat) /usr/include/opencv2/mcc/ccm.hpp:393
	pub fn cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_Mat_COLOR_SPACE_Mat(src: *const c_void, colors: *mut c_void, ref_cs: crate::mcc::COLOR_SPACE, colored: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setColorSpace(cv::ccm::COLOR_SPACE) /usr/include/opencv2/mcc/ccm.hpp:409
	pub fn cv_ccm_ColorCorrectionModel_setColorSpace_COLOR_SPACE(instance: *mut c_void, cs: crate::mcc::COLOR_SPACE, ocvrs_return: *mut Result_void);
	// setCCM_TYPE(cv::ccm::CCM_TYPE) /usr/include/opencv2/mcc/ccm.hpp:415
	pub fn cv_ccm_ColorCorrectionModel_setCCM_TYPE_CCM_TYPE(instance: *mut c_void, ccm_type: crate::mcc::CCM_TYPE, ocvrs_return: *mut Result_void);
	// setDistance(cv::ccm::DISTANCE_TYPE) /usr/include/opencv2/mcc/ccm.hpp:421
	pub fn cv_ccm_ColorCorrectionModel_setDistance_DISTANCE_TYPE(instance: *mut c_void, distance: crate::mcc::DISTANCE_TYPE, ocvrs_return: *mut Result_void);
	// setLinear(cv::ccm::LINEAR_TYPE) /usr/include/opencv2/mcc/ccm.hpp:427
	pub fn cv_ccm_ColorCorrectionModel_setLinear_LINEAR_TYPE(instance: *mut c_void, linear_type: crate::mcc::LINEAR_TYPE, ocvrs_return: *mut Result_void);
	// setLinearGamma(const double &) /usr/include/opencv2/mcc/ccm.hpp:436
	pub fn cv_ccm_ColorCorrectionModel_setLinearGamma_const_doubleR(instance: *mut c_void, gamma: *const f64, ocvrs_return: *mut Result_void);
	// setLinearDegree(const int &) /usr/include/opencv2/mcc/ccm.hpp:449
	pub fn cv_ccm_ColorCorrectionModel_setLinearDegree_const_intR(instance: *mut c_void, deg: *const i32, ocvrs_return: *mut Result_void);
	// setSaturatedThreshold(const double &, const double &) /usr/include/opencv2/mcc/ccm.hpp:459
	pub fn cv_ccm_ColorCorrectionModel_setSaturatedThreshold_const_doubleR_const_doubleR(instance: *mut c_void, lower: *const f64, upper: *const f64, ocvrs_return: *mut Result_void);
	// setWeightsList(const cv::Mat &) /usr/include/opencv2/mcc/ccm.hpp:465
	pub fn cv_ccm_ColorCorrectionModel_setWeightsList_const_MatR(instance: *mut c_void, weights_list: *const c_void, ocvrs_return: *mut Result_void);
	// setWeightCoeff(const double &) /usr/include/opencv2/mcc/ccm.hpp:471
	pub fn cv_ccm_ColorCorrectionModel_setWeightCoeff_const_doubleR(instance: *mut c_void, weights_coeff: *const f64, ocvrs_return: *mut Result_void);
	// setInitialMethod(cv::ccm::INITIAL_METHOD_TYPE) /usr/include/opencv2/mcc/ccm.hpp:477
	pub fn cv_ccm_ColorCorrectionModel_setInitialMethod_INITIAL_METHOD_TYPE(instance: *mut c_void, initial_method_type: crate::mcc::INITIAL_METHOD_TYPE, ocvrs_return: *mut Result_void);
	// setMaxCount(const int &) /usr/include/opencv2/mcc/ccm.hpp:484
	pub fn cv_ccm_ColorCorrectionModel_setMaxCount_const_intR(instance: *mut c_void, max_count: *const i32, ocvrs_return: *mut Result_void);
	// setEpsilon(const double &) /usr/include/opencv2/mcc/ccm.hpp:491
	pub fn cv_ccm_ColorCorrectionModel_setEpsilon_const_doubleR(instance: *mut c_void, epsilon: *const f64, ocvrs_return: *mut Result_void);
	// run() /usr/include/opencv2/mcc/ccm.hpp:494
	pub fn cv_ccm_ColorCorrectionModel_run(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// getCCM() /usr/include/opencv2/mcc/ccm.hpp:496
	pub fn cv_ccm_ColorCorrectionModel_getCCM_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getLoss() /usr/include/opencv2/mcc/ccm.hpp:497
	pub fn cv_ccm_ColorCorrectionModel_getLoss_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// get_src_rgbl() /usr/include/opencv2/mcc/ccm.hpp:498
	pub fn cv_ccm_ColorCorrectionModel_get_src_rgbl_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// get_dst_rgbl() /usr/include/opencv2/mcc/ccm.hpp:499
	pub fn cv_ccm_ColorCorrectionModel_get_dst_rgbl_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getMask() /usr/include/opencv2/mcc/ccm.hpp:500
	pub fn cv_ccm_ColorCorrectionModel_getMask_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getWeights() /usr/include/opencv2/mcc/ccm.hpp:501
	pub fn cv_ccm_ColorCorrectionModel_getWeights_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// infer(const cv::Mat &, bool) /usr/include/opencv2/mcc/ccm.hpp:508
	pub fn cv_ccm_ColorCorrectionModel_infer_const_MatR_bool(instance: *mut c_void, img: *const c_void, islinear: bool, ocvrs_return: *mut Result<*mut c_void>);
	// create() /usr/include/opencv2/mcc/checker_model.hpp:73
	pub fn cv_mcc_CChecker_create(ocvrs_return: *mut Result<*mut c_void>);
	// setTarget(cv::mcc::TYPECHART) /usr/include/opencv2/mcc/checker_model.hpp:83
	pub fn cv_mcc_CChecker_setTarget_TYPECHART(instance: *mut c_void, _target: crate::mcc::MCC_TYPECHART, ocvrs_return: *mut Result_void);
	// setBox(std::vector<Point2f>) /usr/include/opencv2/mcc/checker_model.hpp:84
	pub fn cv_mcc_CChecker_setBox_vector_Point2f_(instance: *mut c_void, _box: *mut c_void, ocvrs_return: *mut Result_void);
	// setChartsRGB(cv::Mat) /usr/include/opencv2/mcc/checker_model.hpp:85
	pub fn cv_mcc_CChecker_setChartsRGB_Mat(instance: *mut c_void, _charts_rgb: *mut c_void, ocvrs_return: *mut Result_void);
	// setChartsYCbCr(cv::Mat) /usr/include/opencv2/mcc/checker_model.hpp:86
	pub fn cv_mcc_CChecker_setChartsYCbCr_Mat(instance: *mut c_void, _charts_y_cb_cr: *mut c_void, ocvrs_return: *mut Result_void);
	// setCost(float) /usr/include/opencv2/mcc/checker_model.hpp:87
	pub fn cv_mcc_CChecker_setCost_float(instance: *mut c_void, _cost: f32, ocvrs_return: *mut Result_void);
	// setCenter(cv::Point2f) /usr/include/opencv2/mcc/checker_model.hpp:88
	pub fn cv_mcc_CChecker_setCenter_Point2f(instance: *mut c_void, _center: *const core::Point2f, ocvrs_return: *mut Result_void);
	// getTarget() /usr/include/opencv2/mcc/checker_model.hpp:90
	pub fn cv_mcc_CChecker_getTarget(instance: *mut c_void, ocvrs_return: *mut Result<crate::mcc::MCC_TYPECHART>);
	// getBox() /usr/include/opencv2/mcc/checker_model.hpp:91
	pub fn cv_mcc_CChecker_getBox(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getChartsRGB() /usr/include/opencv2/mcc/checker_model.hpp:92
	pub fn cv_mcc_CChecker_getChartsRGB(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getChartsYCbCr() /usr/include/opencv2/mcc/checker_model.hpp:93
	pub fn cv_mcc_CChecker_getChartsYCbCr(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getCost() /usr/include/opencv2/mcc/checker_model.hpp:94
	pub fn cv_mcc_CChecker_getCost(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
	// getCenter() /usr/include/opencv2/mcc/checker_model.hpp:95
	pub fn cv_mcc_CChecker_getCenter(instance: *mut c_void, ocvrs_return: *mut Result<core::Point2f>);
	// setNet(dnn::Net) /usr/include/opencv2/mcc/checker_detector.hpp:144
	pub fn cv_mcc_CCheckerDetector_setNet_Net(instance: *mut c_void, net: *mut c_void, ocvrs_return: *mut Result<bool>);
	// process(cv::InputArray, const cv::mcc::TYPECHART, const std::vector<Rect> &, const int, bool, const Ptr<cv::mcc::DetectorParameters> &) /usr/include/opencv2/mcc/checker_detector.hpp:167
	pub fn cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART_const_vector_Rect_R_const_int_bool_const_Ptr_DetectorParameters_R(instance: *mut c_void, image: *const c_void, chart_type: crate::mcc::MCC_TYPECHART, regions_of_interest: *const c_void, nc: i32, use_net: bool, params: *const c_void, ocvrs_return: *mut Result<bool>);
	// process(cv::InputArray, const cv::mcc::TYPECHART, const int, bool, const Ptr<cv::mcc::DetectorParameters> &) /usr/include/opencv2/mcc/checker_detector.hpp:195
	pub fn cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART_const_int_bool_const_Ptr_DetectorParameters_R(instance: *mut c_void, image: *const c_void, chart_type: crate::mcc::MCC_TYPECHART, nc: i32, use_net: bool, params: *const c_void, ocvrs_return: *mut Result<bool>);
	// getBestColorChecker() /usr/include/opencv2/mcc/checker_detector.hpp:204
	pub fn cv_mcc_CCheckerDetector_getBestColorChecker(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getListColorChecker() /usr/include/opencv2/mcc/checker_detector.hpp:209
	pub fn cv_mcc_CCheckerDetector_getListColorChecker(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create() /usr/include/opencv2/mcc/checker_detector.hpp:214
	pub fn cv_mcc_CCheckerDetector_create(ocvrs_return: *mut Result<*mut c_void>);
	// draw(cv::InputOutputArray) /usr/include/opencv2/mcc/checker_model.hpp:121
	pub fn cv_mcc_CCheckerDraw_draw_const__InputOutputArrayR(instance: *mut c_void, img: *const c_void, ocvrs_return: *mut Result_void);
	// create(Ptr<cv::mcc::CChecker>, cv::Scalar, int) /usr/include/opencv2/mcc/checker_model.hpp:130
	pub fn cv_mcc_CCheckerDraw_create_Ptr_CChecker__Scalar_int(p_checker: *mut c_void, color: *const core::Scalar, thickness: i32, ocvrs_return: *mut Result<*mut c_void>);
	// adaptiveThreshWinSizeMin /usr/include/opencv2/mcc/checker_detector.hpp:106
	pub fn cv_mcc_DetectorParameters_getPropAdaptiveThreshWinSizeMin_const(instance: *const c_void) -> i32;
	// adaptiveThreshWinSizeMin /usr/include/opencv2/mcc/checker_detector.hpp:106
	pub fn cv_mcc_DetectorParameters_setPropAdaptiveThreshWinSizeMin_int(instance: *mut c_void, val: i32);
	// adaptiveThreshWinSizeMax /usr/include/opencv2/mcc/checker_detector.hpp:107
	pub fn cv_mcc_DetectorParameters_getPropAdaptiveThreshWinSizeMax_const(instance: *const c_void) -> i32;
	// adaptiveThreshWinSizeMax /usr/include/opencv2/mcc/checker_detector.hpp:107
	pub fn cv_mcc_DetectorParameters_setPropAdaptiveThreshWinSizeMax_int(instance: *mut c_void, val: i32);
	// adaptiveThreshWinSizeStep /usr/include/opencv2/mcc/checker_detector.hpp:108
	pub fn cv_mcc_DetectorParameters_getPropAdaptiveThreshWinSizeStep_const(instance: *const c_void) -> i32;
	// adaptiveThreshWinSizeStep /usr/include/opencv2/mcc/checker_detector.hpp:108
	pub fn cv_mcc_DetectorParameters_setPropAdaptiveThreshWinSizeStep_int(instance: *mut c_void, val: i32);
	// adaptiveThreshConstant /usr/include/opencv2/mcc/checker_detector.hpp:109
	pub fn cv_mcc_DetectorParameters_getPropAdaptiveThreshConstant_const(instance: *const c_void) -> f64;
	// adaptiveThreshConstant /usr/include/opencv2/mcc/checker_detector.hpp:109
	pub fn cv_mcc_DetectorParameters_setPropAdaptiveThreshConstant_double(instance: *mut c_void, val: f64);
	// minContoursAreaRate /usr/include/opencv2/mcc/checker_detector.hpp:110
	pub fn cv_mcc_DetectorParameters_getPropMinContoursAreaRate_const(instance: *const c_void) -> f64;
	// minContoursAreaRate /usr/include/opencv2/mcc/checker_detector.hpp:110
	pub fn cv_mcc_DetectorParameters_setPropMinContoursAreaRate_double(instance: *mut c_void, val: f64);
	// minContoursArea /usr/include/opencv2/mcc/checker_detector.hpp:111
	pub fn cv_mcc_DetectorParameters_getPropMinContoursArea_const(instance: *const c_void) -> f64;
	// minContoursArea /usr/include/opencv2/mcc/checker_detector.hpp:111
	pub fn cv_mcc_DetectorParameters_setPropMinContoursArea_double(instance: *mut c_void, val: f64);
	// confidenceThreshold /usr/include/opencv2/mcc/checker_detector.hpp:112
	pub fn cv_mcc_DetectorParameters_getPropConfidenceThreshold_const(instance: *const c_void) -> f64;
	// confidenceThreshold /usr/include/opencv2/mcc/checker_detector.hpp:112
	pub fn cv_mcc_DetectorParameters_setPropConfidenceThreshold_double(instance: *mut c_void, val: f64);
	// minContourSolidity /usr/include/opencv2/mcc/checker_detector.hpp:113
	pub fn cv_mcc_DetectorParameters_getPropMinContourSolidity_const(instance: *const c_void) -> f64;
	// minContourSolidity /usr/include/opencv2/mcc/checker_detector.hpp:113
	pub fn cv_mcc_DetectorParameters_setPropMinContourSolidity_double(instance: *mut c_void, val: f64);
	// findCandidatesApproxPolyDPEpsMultiplier /usr/include/opencv2/mcc/checker_detector.hpp:114
	pub fn cv_mcc_DetectorParameters_getPropFindCandidatesApproxPolyDPEpsMultiplier_const(instance: *const c_void) -> f64;
	// findCandidatesApproxPolyDPEpsMultiplier /usr/include/opencv2/mcc/checker_detector.hpp:114
	pub fn cv_mcc_DetectorParameters_setPropFindCandidatesApproxPolyDPEpsMultiplier_double(instance: *mut c_void, val: f64);
	// borderWidth /usr/include/opencv2/mcc/checker_detector.hpp:115
	pub fn cv_mcc_DetectorParameters_getPropBorderWidth_const(instance: *const c_void) -> i32;
	// borderWidth /usr/include/opencv2/mcc/checker_detector.hpp:115
	pub fn cv_mcc_DetectorParameters_setPropBorderWidth_int(instance: *mut c_void, val: i32);
	// B0factor /usr/include/opencv2/mcc/checker_detector.hpp:116
	pub fn cv_mcc_DetectorParameters_getPropB0factor_const(instance: *const c_void) -> f32;
	// B0factor /usr/include/opencv2/mcc/checker_detector.hpp:116
	pub fn cv_mcc_DetectorParameters_setPropB0factor_float(instance: *mut c_void, val: f32);
	// maxError /usr/include/opencv2/mcc/checker_detector.hpp:117
	pub fn cv_mcc_DetectorParameters_getPropMaxError_const(instance: *const c_void) -> f32;
	// maxError /usr/include/opencv2/mcc/checker_detector.hpp:117
	pub fn cv_mcc_DetectorParameters_setPropMaxError_float(instance: *mut c_void, val: f32);
	// minContourPointsAllowed /usr/include/opencv2/mcc/checker_detector.hpp:118
	pub fn cv_mcc_DetectorParameters_getPropMinContourPointsAllowed_const(instance: *const c_void) -> i32;
	// minContourPointsAllowed /usr/include/opencv2/mcc/checker_detector.hpp:118
	pub fn cv_mcc_DetectorParameters_setPropMinContourPointsAllowed_int(instance: *mut c_void, val: i32);
	// minContourLengthAllowed /usr/include/opencv2/mcc/checker_detector.hpp:119
	pub fn cv_mcc_DetectorParameters_getPropMinContourLengthAllowed_const(instance: *const c_void) -> i32;
	// minContourLengthAllowed /usr/include/opencv2/mcc/checker_detector.hpp:119
	pub fn cv_mcc_DetectorParameters_setPropMinContourLengthAllowed_int(instance: *mut c_void, val: i32);
	// minInterContourDistance /usr/include/opencv2/mcc/checker_detector.hpp:120
	pub fn cv_mcc_DetectorParameters_getPropMinInterContourDistance_const(instance: *const c_void) -> i32;
	// minInterContourDistance /usr/include/opencv2/mcc/checker_detector.hpp:120
	pub fn cv_mcc_DetectorParameters_setPropMinInterContourDistance_int(instance: *mut c_void, val: i32);
	// minInterCheckerDistance /usr/include/opencv2/mcc/checker_detector.hpp:121
	pub fn cv_mcc_DetectorParameters_getPropMinInterCheckerDistance_const(instance: *const c_void) -> i32;
	// minInterCheckerDistance /usr/include/opencv2/mcc/checker_detector.hpp:121
	pub fn cv_mcc_DetectorParameters_setPropMinInterCheckerDistance_int(instance: *mut c_void, val: i32);
	// minImageSize /usr/include/opencv2/mcc/checker_detector.hpp:122
	pub fn cv_mcc_DetectorParameters_getPropMinImageSize_const(instance: *const c_void) -> i32;
	// minImageSize /usr/include/opencv2/mcc/checker_detector.hpp:122
	pub fn cv_mcc_DetectorParameters_setPropMinImageSize_int(instance: *mut c_void, val: i32);
	// minGroupSize /usr/include/opencv2/mcc/checker_detector.hpp:123
	pub fn cv_mcc_DetectorParameters_getPropMinGroupSize_const(instance: *const c_void) -> u32;
	// minGroupSize /usr/include/opencv2/mcc/checker_detector.hpp:123
	pub fn cv_mcc_DetectorParameters_setPropMinGroupSize_unsigned_int(instance: *mut c_void, val: u32);
	// DetectorParameters() /usr/include/opencv2/mcc/checker_detector.hpp:102
	pub fn cv_mcc_DetectorParameters_DetectorParameters(ocvrs_return: *mut Result<*mut c_void>);
	// create() /usr/include/opencv2/mcc/checker_detector.hpp:104
	pub fn cv_mcc_DetectorParameters_create(ocvrs_return: *mut Result<*mut c_void>);
}
