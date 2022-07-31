extern "C" {
	// createConcentricSpheresTestSet(int, int, int, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ml.hpp:1902
	pub fn cv_ml_createConcentricSpheresTestSet_int_int_int_const__OutputArrayR_const__OutputArrayR(nsamples: i32, nfeatures: i32, nclasses: i32, samples: *const c_void, responses: *const c_void, ocvrs_return: *mut Result_void);
	// randMVNormal(cv::InputArray, cv::InputArray, int, cv::OutputArray) /usr/include/opencv2/ml.hpp:1899
	pub fn cv_ml_randMVNormal_const__InputArrayR_const__InputArrayR_int_const__OutputArrayR(mean: *const c_void, cov: *const c_void, nsamples: i32, samples: *const c_void, ocvrs_return: *mut Result_void);
	// setTrainMethod(int, double, double) /usr/include/opencv2/ml.hpp:1446
	pub fn cv_ml_ANN_MLP_setTrainMethod_int_double_double(instance: *mut c_void, method: i32, param1: f64, param2: f64, ocvrs_return: *mut Result_void);
	// getTrainMethod() /usr/include/opencv2/ml.hpp:1449
	pub fn cv_ml_ANN_MLP_getTrainMethod_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setActivationFunction(int, double, double) /usr/include/opencv2/ml.hpp:1457
	pub fn cv_ml_ANN_MLP_setActivationFunction_int_double_double(instance: *mut c_void, typ: i32, param1: f64, param2: f64, ocvrs_return: *mut Result_void);
	// setLayerSizes(cv::InputArray) /usr/include/opencv2/ml.hpp:1463
	pub fn cv_ml_ANN_MLP_setLayerSizes_const__InputArrayR(instance: *mut c_void, _layer_sizes: *const c_void, ocvrs_return: *mut Result_void);
	// getLayerSizes() /usr/include/opencv2/ml.hpp:1469
	pub fn cv_ml_ANN_MLP_getLayerSizes_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getTermCriteria() /usr/include/opencv2/ml.hpp:1476
	pub fn cv_ml_ANN_MLP_getTermCriteria_const(instance: *const c_void, ocvrs_return: *mut Result<core::TermCriteria>);
	// setTermCriteria(cv::TermCriteria) /usr/include/opencv2/ml.hpp:1478
	pub fn cv_ml_ANN_MLP_setTermCriteria_TermCriteria(instance: *mut c_void, val: *const core::TermCriteria, ocvrs_return: *mut Result_void);
	// getBackpropWeightScale() /usr/include/opencv2/ml.hpp:1483
	pub fn cv_ml_ANN_MLP_getBackpropWeightScale_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setBackpropWeightScale(double) /usr/include/opencv2/ml.hpp:1485
	pub fn cv_ml_ANN_MLP_setBackpropWeightScale_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getBackpropMomentumScale() /usr/include/opencv2/ml.hpp:1492
	pub fn cv_ml_ANN_MLP_getBackpropMomentumScale_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setBackpropMomentumScale(double) /usr/include/opencv2/ml.hpp:1494
	pub fn cv_ml_ANN_MLP_setBackpropMomentumScale_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getRpropDW0() /usr/include/opencv2/ml.hpp:1499
	pub fn cv_ml_ANN_MLP_getRpropDW0_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setRpropDW0(double) /usr/include/opencv2/ml.hpp:1501
	pub fn cv_ml_ANN_MLP_setRpropDW0_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getRpropDWPlus() /usr/include/opencv2/ml.hpp:1506
	pub fn cv_ml_ANN_MLP_getRpropDWPlus_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setRpropDWPlus(double) /usr/include/opencv2/ml.hpp:1508
	pub fn cv_ml_ANN_MLP_setRpropDWPlus_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getRpropDWMinus() /usr/include/opencv2/ml.hpp:1513
	pub fn cv_ml_ANN_MLP_getRpropDWMinus_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setRpropDWMinus(double) /usr/include/opencv2/ml.hpp:1515
	pub fn cv_ml_ANN_MLP_setRpropDWMinus_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getRpropDWMin() /usr/include/opencv2/ml.hpp:1520
	pub fn cv_ml_ANN_MLP_getRpropDWMin_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setRpropDWMin(double) /usr/include/opencv2/ml.hpp:1522
	pub fn cv_ml_ANN_MLP_setRpropDWMin_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getRpropDWMax() /usr/include/opencv2/ml.hpp:1527
	pub fn cv_ml_ANN_MLP_getRpropDWMax_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setRpropDWMax(double) /usr/include/opencv2/ml.hpp:1529
	pub fn cv_ml_ANN_MLP_setRpropDWMax_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getAnnealInitialT() /usr/include/opencv2/ml.hpp:1534
	pub fn cv_ml_ANN_MLP_getAnnealInitialT_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setAnnealInitialT(double) /usr/include/opencv2/ml.hpp:1536
	pub fn cv_ml_ANN_MLP_setAnnealInitialT_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getAnnealFinalT() /usr/include/opencv2/ml.hpp:1541
	pub fn cv_ml_ANN_MLP_getAnnealFinalT_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setAnnealFinalT(double) /usr/include/opencv2/ml.hpp:1543
	pub fn cv_ml_ANN_MLP_setAnnealFinalT_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getAnnealCoolingRatio() /usr/include/opencv2/ml.hpp:1548
	pub fn cv_ml_ANN_MLP_getAnnealCoolingRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setAnnealCoolingRatio(double) /usr/include/opencv2/ml.hpp:1550
	pub fn cv_ml_ANN_MLP_setAnnealCoolingRatio_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getAnnealItePerStep() /usr/include/opencv2/ml.hpp:1555
	pub fn cv_ml_ANN_MLP_getAnnealItePerStep_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setAnnealItePerStep(int) /usr/include/opencv2/ml.hpp:1557
	pub fn cv_ml_ANN_MLP_setAnnealItePerStep_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// setAnnealEnergyRNG(const cv::RNG &) /usr/include/opencv2/ml.hpp:1560
	pub fn cv_ml_ANN_MLP_setAnnealEnergyRNG_const_RNGR(instance: *mut c_void, rng: *const c_void, ocvrs_return: *mut Result_void);
	// getWeights(int) /usr/include/opencv2/ml.hpp:1597
	pub fn cv_ml_ANN_MLP_getWeights_const_int(instance: *const c_void, layer_idx: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create() /usr/include/opencv2/ml.hpp:1604
	pub fn cv_ml_ANN_MLP_create(ocvrs_return: *mut Result<*mut c_void>);
	// load(const cv::String &) /usr/include/opencv2/ml.hpp:1613
	pub fn cv_ml_ANN_MLP_load_const_StringR(filepath: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// getBoostType() /usr/include/opencv2/ml.hpp:1338
	pub fn cv_ml_Boost_getBoostType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setBoostType(int) /usr/include/opencv2/ml.hpp:1340
	pub fn cv_ml_Boost_setBoostType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getWeakCount() /usr/include/opencv2/ml.hpp:1345
	pub fn cv_ml_Boost_getWeakCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setWeakCount(int) /usr/include/opencv2/ml.hpp:1347
	pub fn cv_ml_Boost_setWeakCount_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getWeightTrimRate() /usr/include/opencv2/ml.hpp:1353
	pub fn cv_ml_Boost_getWeightTrimRate_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setWeightTrimRate(double) /usr/include/opencv2/ml.hpp:1355
	pub fn cv_ml_Boost_setWeightTrimRate_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// create() /usr/include/opencv2/ml.hpp:1370
	pub fn cv_ml_Boost_create(ocvrs_return: *mut Result<*mut c_void>);
	// load(const cv::String &, const cv::String &) /usr/include/opencv2/ml.hpp:1381
	pub fn cv_ml_Boost_load_const_StringR_const_StringR(filepath: *const c_char, node_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// getMaxCategories() /usr/include/opencv2/ml.hpp:1071
	pub fn cv_ml_DTrees_getMaxCategories_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMaxCategories(int) /usr/include/opencv2/ml.hpp:1073
	pub fn cv_ml_DTrees_setMaxCategories_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getMaxDepth() /usr/include/opencv2/ml.hpp:1081
	pub fn cv_ml_DTrees_getMaxDepth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMaxDepth(int) /usr/include/opencv2/ml.hpp:1083
	pub fn cv_ml_DTrees_setMaxDepth_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getMinSampleCount() /usr/include/opencv2/ml.hpp:1089
	pub fn cv_ml_DTrees_getMinSampleCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMinSampleCount(int) /usr/include/opencv2/ml.hpp:1091
	pub fn cv_ml_DTrees_setMinSampleCount_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getCVFolds() /usr/include/opencv2/ml.hpp:1097
	pub fn cv_ml_DTrees_getCVFolds_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setCVFolds(int) /usr/include/opencv2/ml.hpp:1099
	pub fn cv_ml_DTrees_setCVFolds_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getUseSurrogates() /usr/include/opencv2/ml.hpp:1106
	pub fn cv_ml_DTrees_getUseSurrogates_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUseSurrogates(bool) /usr/include/opencv2/ml.hpp:1108
	pub fn cv_ml_DTrees_setUseSurrogates_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// getUse1SERule() /usr/include/opencv2/ml.hpp:1114
	pub fn cv_ml_DTrees_getUse1SERule_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUse1SERule(bool) /usr/include/opencv2/ml.hpp:1116
	pub fn cv_ml_DTrees_setUse1SERule_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// getTruncatePrunedTree() /usr/include/opencv2/ml.hpp:1122
	pub fn cv_ml_DTrees_getTruncatePrunedTree_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setTruncatePrunedTree(bool) /usr/include/opencv2/ml.hpp:1124
	pub fn cv_ml_DTrees_setTruncatePrunedTree_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// getRegressionAccuracy() /usr/include/opencv2/ml.hpp:1131
	pub fn cv_ml_DTrees_getRegressionAccuracy_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setRegressionAccuracy(float) /usr/include/opencv2/ml.hpp:1133
	pub fn cv_ml_DTrees_setRegressionAccuracy_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getPriors() /usr/include/opencv2/ml.hpp:1151
	pub fn cv_ml_DTrees_getPriors_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setPriors(const cv::Mat &) /usr/include/opencv2/ml.hpp:1153
	pub fn cv_ml_DTrees_setPriors_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// getRoots() /usr/include/opencv2/ml.hpp:1202
	pub fn cv_ml_DTrees_getRoots_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getNodes() /usr/include/opencv2/ml.hpp:1207
	pub fn cv_ml_DTrees_getNodes_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getSplits() /usr/include/opencv2/ml.hpp:1212
	pub fn cv_ml_DTrees_getSplits_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getSubsets() /usr/include/opencv2/ml.hpp:1217
	pub fn cv_ml_DTrees_getSubsets_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create() /usr/include/opencv2/ml.hpp:1225
	pub fn cv_ml_DTrees_create(ocvrs_return: *mut Result<*mut c_void>);
	// load(const cv::String &, const cv::String &) /usr/include/opencv2/ml.hpp:1236
	pub fn cv_ml_DTrees_load_const_StringR_const_StringR(filepath: *const c_char, node_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// value /usr/include/opencv2/ml.hpp:1161
	pub fn cv_ml_DTrees_Node_getPropValue_const(instance: *const c_void) -> f64;
	// value /usr/include/opencv2/ml.hpp:1161
	pub fn cv_ml_DTrees_Node_setPropValue_double(instance: *mut c_void, val: f64);
	// classIdx /usr/include/opencv2/ml.hpp:1163
	pub fn cv_ml_DTrees_Node_getPropClassIdx_const(instance: *const c_void) -> i32;
	// classIdx /usr/include/opencv2/ml.hpp:1163
	pub fn cv_ml_DTrees_Node_setPropClassIdx_int(instance: *mut c_void, val: i32);
	// parent /usr/include/opencv2/ml.hpp:1165
	pub fn cv_ml_DTrees_Node_getPropParent_const(instance: *const c_void) -> i32;
	// parent /usr/include/opencv2/ml.hpp:1165
	pub fn cv_ml_DTrees_Node_setPropParent_int(instance: *mut c_void, val: i32);
	// left /usr/include/opencv2/ml.hpp:1166
	pub fn cv_ml_DTrees_Node_getPropLeft_const(instance: *const c_void) -> i32;
	// left /usr/include/opencv2/ml.hpp:1166
	pub fn cv_ml_DTrees_Node_setPropLeft_int(instance: *mut c_void, val: i32);
	// right /usr/include/opencv2/ml.hpp:1167
	pub fn cv_ml_DTrees_Node_getPropRight_const(instance: *const c_void) -> i32;
	// right /usr/include/opencv2/ml.hpp:1167
	pub fn cv_ml_DTrees_Node_setPropRight_int(instance: *mut c_void, val: i32);
	// defaultDir /usr/include/opencv2/ml.hpp:1168
	pub fn cv_ml_DTrees_Node_getPropDefaultDir_const(instance: *const c_void) -> i32;
	// defaultDir /usr/include/opencv2/ml.hpp:1168
	pub fn cv_ml_DTrees_Node_setPropDefaultDir_int(instance: *mut c_void, val: i32);
	// split /usr/include/opencv2/ml.hpp:1170
	pub fn cv_ml_DTrees_Node_getPropSplit_const(instance: *const c_void) -> i32;
	// split /usr/include/opencv2/ml.hpp:1170
	pub fn cv_ml_DTrees_Node_setPropSplit_int(instance: *mut c_void, val: i32);
	// Node() /usr/include/opencv2/ml.hpp:1160
	pub fn cv_ml_DTrees_Node_Node(ocvrs_return: *mut Result<*mut c_void>);
	// varIdx /usr/include/opencv2/ml.hpp:1179
	pub fn cv_ml_DTrees_Split_getPropVarIdx_const(instance: *const c_void) -> i32;
	// varIdx /usr/include/opencv2/ml.hpp:1179
	pub fn cv_ml_DTrees_Split_setPropVarIdx_int(instance: *mut c_void, val: i32);
	// inversed /usr/include/opencv2/ml.hpp:1180
	pub fn cv_ml_DTrees_Split_getPropInversed_const(instance: *const c_void) -> bool;
	// inversed /usr/include/opencv2/ml.hpp:1180
	pub fn cv_ml_DTrees_Split_setPropInversed_bool(instance: *mut c_void, val: bool);
	// quality /usr/include/opencv2/ml.hpp:1182
	pub fn cv_ml_DTrees_Split_getPropQuality_const(instance: *const c_void) -> f32;
	// quality /usr/include/opencv2/ml.hpp:1182
	pub fn cv_ml_DTrees_Split_setPropQuality_float(instance: *mut c_void, val: f32);
	// next /usr/include/opencv2/ml.hpp:1183
	pub fn cv_ml_DTrees_Split_getPropNext_const(instance: *const c_void) -> i32;
	// next /usr/include/opencv2/ml.hpp:1183
	pub fn cv_ml_DTrees_Split_setPropNext_int(instance: *mut c_void, val: i32);
	// c /usr/include/opencv2/ml.hpp:1184
	pub fn cv_ml_DTrees_Split_getPropC_const(instance: *const c_void) -> f32;
	// c /usr/include/opencv2/ml.hpp:1184
	pub fn cv_ml_DTrees_Split_setPropC_float(instance: *mut c_void, val: f32);
	// subsetOfs /usr/include/opencv2/ml.hpp:1191
	pub fn cv_ml_DTrees_Split_getPropSubsetOfs_const(instance: *const c_void) -> i32;
	// subsetOfs /usr/include/opencv2/ml.hpp:1191
	pub fn cv_ml_DTrees_Split_setPropSubsetOfs_int(instance: *mut c_void, val: i32);
	// Split() /usr/include/opencv2/ml.hpp:1178
	pub fn cv_ml_DTrees_Split_Split(ocvrs_return: *mut Result<*mut c_void>);
	// getClustersNumber() /usr/include/opencv2/ml.hpp:871
	pub fn cv_ml_EM_getClustersNumber_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setClustersNumber(int) /usr/include/opencv2/ml.hpp:873
	pub fn cv_ml_EM_setClustersNumber_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getCovarianceMatrixType() /usr/include/opencv2/ml.hpp:878
	pub fn cv_ml_EM_getCovarianceMatrixType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setCovarianceMatrixType(int) /usr/include/opencv2/ml.hpp:880
	pub fn cv_ml_EM_setCovarianceMatrixType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getTermCriteria() /usr/include/opencv2/ml.hpp:887
	pub fn cv_ml_EM_getTermCriteria_const(instance: *const c_void, ocvrs_return: *mut Result<core::TermCriteria>);
	// setTermCriteria(const cv::TermCriteria &) /usr/include/opencv2/ml.hpp:889
	pub fn cv_ml_EM_setTermCriteria_const_TermCriteriaR(instance: *mut c_void, val: *const core::TermCriteria, ocvrs_return: *mut Result_void);
	// getWeights() /usr/include/opencv2/ml.hpp:895
	pub fn cv_ml_EM_getWeights_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getMeans() /usr/include/opencv2/ml.hpp:901
	pub fn cv_ml_EM_getMeans_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getCovs(std::vector<Mat> &) /usr/include/opencv2/ml.hpp:907
	pub fn cv_ml_EM_getCovs_const_vector_Mat_R(instance: *const c_void, covs: *mut c_void, ocvrs_return: *mut Result_void);
	// predict(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ml.hpp:916
	pub fn cv_ml_EM_predict_const_const__InputArrayR_const__OutputArrayR_int(instance: *const c_void, samples: *const c_void, results: *const c_void, flags: i32, ocvrs_return: *mut Result<f32>);
	// predict2(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ml.hpp:930
	pub fn cv_ml_EM_predict2_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, sample: *const c_void, probs: *const c_void, ocvrs_return: *mut Result<core::Vec2d>);
	// trainEM(cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ml.hpp:960
	pub fn cv_ml_EM_trainEM_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, samples: *const c_void, log_likelihoods: *const c_void, labels: *const c_void, probs: *const c_void, ocvrs_return: *mut Result<bool>);
	// trainE(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ml.hpp:992
	pub fn cv_ml_EM_trainE_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, samples: *const c_void, means0: *const c_void, covs0: *const c_void, weights0: *const c_void, log_likelihoods: *const c_void, labels: *const c_void, probs: *const c_void, ocvrs_return: *mut Result<bool>);
	// trainM(cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ml.hpp:1017
	pub fn cv_ml_EM_trainM_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, samples: *const c_void, probs0: *const c_void, log_likelihoods: *const c_void, labels: *const c_void, probs: *const c_void, ocvrs_return: *mut Result<bool>);
	// create() /usr/include/opencv2/ml.hpp:1026
	pub fn cv_ml_EM_create(ocvrs_return: *mut Result<*mut c_void>);
	// load(const cv::String &, const cv::String &) /usr/include/opencv2/ml.hpp:1037
	pub fn cv_ml_EM_load_const_StringR_const_StringR(filepath: *const c_char, node_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// getDefaultK() /usr/include/opencv2/ml.hpp:442
	pub fn cv_ml_KNearest_getDefaultK_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setDefaultK(int) /usr/include/opencv2/ml.hpp:444
	pub fn cv_ml_KNearest_setDefaultK_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getIsClassifier() /usr/include/opencv2/ml.hpp:448
	pub fn cv_ml_KNearest_getIsClassifier_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setIsClassifier(bool) /usr/include/opencv2/ml.hpp:450
	pub fn cv_ml_KNearest_setIsClassifier_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// getEmax() /usr/include/opencv2/ml.hpp:454
	pub fn cv_ml_KNearest_getEmax_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setEmax(int) /usr/include/opencv2/ml.hpp:456
	pub fn cv_ml_KNearest_setEmax_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getAlgorithmType() /usr/include/opencv2/ml.hpp:460
	pub fn cv_ml_KNearest_getAlgorithmType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setAlgorithmType(int) /usr/include/opencv2/ml.hpp:462
	pub fn cv_ml_KNearest_setAlgorithmType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// findNearest(cv::InputArray, int, cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ml.hpp:490
	pub fn cv_ml_KNearest_findNearest_const_const__InputArrayR_int_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, samples: *const c_void, k: i32, results: *const c_void, neighbor_responses: *const c_void, dist: *const c_void, ocvrs_return: *mut Result<f32>);
	// create() /usr/include/opencv2/ml.hpp:507
	pub fn cv_ml_KNearest_create(ocvrs_return: *mut Result<*mut c_void>);
	// load(const cv::String &) /usr/include/opencv2/ml.hpp:515
	pub fn cv_ml_KNearest_load_const_StringR(filepath: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// getLearningRate() /usr/include/opencv2/ml.hpp:1635
	pub fn cv_ml_LogisticRegression_getLearningRate_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setLearningRate(double) /usr/include/opencv2/ml.hpp:1637
	pub fn cv_ml_LogisticRegression_setLearningRate_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getIterations() /usr/include/opencv2/ml.hpp:1641
	pub fn cv_ml_LogisticRegression_getIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setIterations(int) /usr/include/opencv2/ml.hpp:1643
	pub fn cv_ml_LogisticRegression_setIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getRegularization() /usr/include/opencv2/ml.hpp:1647
	pub fn cv_ml_LogisticRegression_getRegularization_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setRegularization(int) /usr/include/opencv2/ml.hpp:1649
	pub fn cv_ml_LogisticRegression_setRegularization_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getTrainMethod() /usr/include/opencv2/ml.hpp:1653
	pub fn cv_ml_LogisticRegression_getTrainMethod_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setTrainMethod(int) /usr/include/opencv2/ml.hpp:1655
	pub fn cv_ml_LogisticRegression_setTrainMethod_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getMiniBatchSize() /usr/include/opencv2/ml.hpp:1661
	pub fn cv_ml_LogisticRegression_getMiniBatchSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMiniBatchSize(int) /usr/include/opencv2/ml.hpp:1663
	pub fn cv_ml_LogisticRegression_setMiniBatchSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getTermCriteria() /usr/include/opencv2/ml.hpp:1667
	pub fn cv_ml_LogisticRegression_getTermCriteria_const(instance: *const c_void, ocvrs_return: *mut Result<core::TermCriteria>);
	// setTermCriteria(cv::TermCriteria) /usr/include/opencv2/ml.hpp:1669
	pub fn cv_ml_LogisticRegression_setTermCriteria_TermCriteria(instance: *mut c_void, val: *const core::TermCriteria, ocvrs_return: *mut Result_void);
	// predict(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ml.hpp:1691
	pub fn cv_ml_LogisticRegression_predict_const_const__InputArrayR_const__OutputArrayR_int(instance: *const c_void, samples: *const c_void, results: *const c_void, flags: i32, ocvrs_return: *mut Result<f32>);
	// get_learnt_thetas() /usr/include/opencv2/ml.hpp:1698
	pub fn cv_ml_LogisticRegression_get_learnt_thetas_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create() /usr/include/opencv2/ml.hpp:1704
	pub fn cv_ml_LogisticRegression_create(ocvrs_return: *mut Result<*mut c_void>);
	// load(const cv::String &, const cv::String &) /usr/include/opencv2/ml.hpp:1715
	pub fn cv_ml_LogisticRegression_load_const_StringR_const_StringR(filepath: *const c_char, node_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// predictProb(cv::InputArray, cv::OutputArray, cv::OutputArray, int) /usr/include/opencv2/ml.hpp:409
	pub fn cv_ml_NormalBayesClassifier_predictProb_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int(instance: *const c_void, inputs: *const c_void, outputs: *const c_void, output_probs: *const c_void, flags: i32, ocvrs_return: *mut Result<f32>);
	// create() /usr/include/opencv2/ml.hpp:414
	pub fn cv_ml_NormalBayesClassifier_create(ocvrs_return: *mut Result<*mut c_void>);
	// load(const cv::String &, const cv::String &) /usr/include/opencv2/ml.hpp:425
	pub fn cv_ml_NormalBayesClassifier_load_const_StringR_const_StringR(filepath: *const c_char, node_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// minVal /usr/include/opencv2/ml.hpp:115
	pub fn cv_ml_ParamGrid_getPropMinVal_const(instance: *const c_void) -> f64;
	// minVal /usr/include/opencv2/ml.hpp:115
	pub fn cv_ml_ParamGrid_setPropMinVal_double(instance: *mut c_void, val: f64);
	// maxVal /usr/include/opencv2/ml.hpp:116
	pub fn cv_ml_ParamGrid_getPropMaxVal_const(instance: *const c_void) -> f64;
	// maxVal /usr/include/opencv2/ml.hpp:116
	pub fn cv_ml_ParamGrid_setPropMaxVal_double(instance: *mut c_void, val: f64);
	// logStep /usr/include/opencv2/ml.hpp:125
	pub fn cv_ml_ParamGrid_getPropLogStep_const(instance: *const c_void) -> f64;
	// logStep /usr/include/opencv2/ml.hpp:125
	pub fn cv_ml_ParamGrid_setPropLogStep_double(instance: *mut c_void, val: f64);
	// ParamGrid() /usr/include/opencv2/ml.hpp:111
	pub fn cv_ml_ParamGrid_ParamGrid(ocvrs_return: *mut Result<*mut c_void>);
	// ParamGrid(double, double, double) /usr/include/opencv2/ml.hpp:113
	pub fn cv_ml_ParamGrid_ParamGrid_double_double_double(_min_val: f64, _max_val: f64, _log_step: f64, ocvrs_return: *mut Result<*mut c_void>);
	// create(double, double, double) /usr/include/opencv2/ml.hpp:133
	pub fn cv_ml_ParamGrid_create_double_double_double(min_val: f64, max_val: f64, logstep: f64, ocvrs_return: *mut Result<*mut c_void>);
	// getCalculateVarImportance() /usr/include/opencv2/ml.hpp:1254
	pub fn cv_ml_RTrees_getCalculateVarImportance_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setCalculateVarImportance(bool) /usr/include/opencv2/ml.hpp:1256
	pub fn cv_ml_RTrees_setCalculateVarImportance_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// getActiveVarCount() /usr/include/opencv2/ml.hpp:1263
	pub fn cv_ml_RTrees_getActiveVarCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setActiveVarCount(int) /usr/include/opencv2/ml.hpp:1265
	pub fn cv_ml_RTrees_setActiveVarCount_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getTermCriteria() /usr/include/opencv2/ml.hpp:1275
	pub fn cv_ml_RTrees_getTermCriteria_const(instance: *const c_void, ocvrs_return: *mut Result<core::TermCriteria>);
	// setTermCriteria(const cv::TermCriteria &) /usr/include/opencv2/ml.hpp:1277
	pub fn cv_ml_RTrees_setTermCriteria_const_TermCriteriaR(instance: *mut c_void, val: *const core::TermCriteria, ocvrs_return: *mut Result_void);
	// getVarImportance() /usr/include/opencv2/ml.hpp:1284
	pub fn cv_ml_RTrees_getVarImportance_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getVotes(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ml.hpp:1295
	pub fn cv_ml_RTrees_getVotes_const_const__InputArrayR_const__OutputArrayR_int(instance: *const c_void, samples: *const c_void, results: *const c_void, flags: i32, ocvrs_return: *mut Result_void);
	// getOOBError() /usr/include/opencv2/ml.hpp:1301
	pub fn cv_ml_RTrees_getOOBError_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// create() /usr/include/opencv2/ml.hpp:1310
	pub fn cv_ml_RTrees_create(ocvrs_return: *mut Result<*mut c_void>);
	// load(const cv::String &, const cv::String &) /usr/include/opencv2/ml.hpp:1321
	pub fn cv_ml_RTrees_load_const_StringR_const_StringR(filepath: *const c_char, node_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// getType() /usr/include/opencv2/ml.hpp:540
	pub fn cv_ml_SVM_getType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setType(int) /usr/include/opencv2/ml.hpp:542
	pub fn cv_ml_SVM_setType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getGamma() /usr/include/opencv2/ml.hpp:547
	pub fn cv_ml_SVM_getGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setGamma(double) /usr/include/opencv2/ml.hpp:549
	pub fn cv_ml_SVM_setGamma_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getCoef0() /usr/include/opencv2/ml.hpp:554
	pub fn cv_ml_SVM_getCoef0_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setCoef0(double) /usr/include/opencv2/ml.hpp:556
	pub fn cv_ml_SVM_setCoef0_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getDegree() /usr/include/opencv2/ml.hpp:561
	pub fn cv_ml_SVM_getDegree_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setDegree(double) /usr/include/opencv2/ml.hpp:563
	pub fn cv_ml_SVM_setDegree_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getC() /usr/include/opencv2/ml.hpp:568
	pub fn cv_ml_SVM_getC_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setC(double) /usr/include/opencv2/ml.hpp:570
	pub fn cv_ml_SVM_setC_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getNu() /usr/include/opencv2/ml.hpp:575
	pub fn cv_ml_SVM_getNu_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setNu(double) /usr/include/opencv2/ml.hpp:577
	pub fn cv_ml_SVM_setNu_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getP() /usr/include/opencv2/ml.hpp:582
	pub fn cv_ml_SVM_getP_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setP(double) /usr/include/opencv2/ml.hpp:584
	pub fn cv_ml_SVM_setP_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getClassWeights() /usr/include/opencv2/ml.hpp:592
	pub fn cv_ml_SVM_getClassWeights_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setClassWeights(const cv::Mat &) /usr/include/opencv2/ml.hpp:594
	pub fn cv_ml_SVM_setClassWeights_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// getTermCriteria() /usr/include/opencv2/ml.hpp:601
	pub fn cv_ml_SVM_getTermCriteria_const(instance: *const c_void, ocvrs_return: *mut Result<core::TermCriteria>);
	// setTermCriteria(const cv::TermCriteria &) /usr/include/opencv2/ml.hpp:603
	pub fn cv_ml_SVM_setTermCriteria_const_TermCriteriaR(instance: *mut c_void, val: *const core::TermCriteria, ocvrs_return: *mut Result_void);
	// getKernelType() /usr/include/opencv2/ml.hpp:607
	pub fn cv_ml_SVM_getKernelType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setKernel(int) /usr/include/opencv2/ml.hpp:611
	pub fn cv_ml_SVM_setKernel_int(instance: *mut c_void, kernel_type: i32, ocvrs_return: *mut Result_void);
	// setCustomKernel(const Ptr<cv::ml::SVM::Kernel> &) /usr/include/opencv2/ml.hpp:615
	pub fn cv_ml_SVM_setCustomKernel_const_Ptr_Kernel_R(instance: *mut c_void, _kernel: *const c_void, ocvrs_return: *mut Result_void);
	// trainAuto(const Ptr<cv::ml::TrainData> &, int, cv::ml::ParamGrid, cv::ml::ParamGrid, cv::ml::ParamGrid, cv::ml::ParamGrid, cv::ml::ParamGrid, cv::ml::ParamGrid, bool) /usr/include/opencv2/ml.hpp:712
	pub fn cv_ml_SVM_trainAuto_const_Ptr_TrainData_R_int_ParamGrid_ParamGrid_ParamGrid_ParamGrid_ParamGrid_ParamGrid_bool(instance: *mut c_void, data: *const c_void, k_fold: i32, cgrid: *mut c_void, gamma_grid: *mut c_void, p_grid: *mut c_void, nu_grid: *mut c_void, coeff_grid: *mut c_void, degree_grid: *mut c_void, balanced: bool, ocvrs_return: *mut Result<bool>);
	// trainAuto(cv::InputArray, int, cv::InputArray, int, Ptr<cv::ml::ParamGrid>, Ptr<cv::ml::ParamGrid>, Ptr<cv::ml::ParamGrid>, Ptr<cv::ml::ParamGrid>, Ptr<cv::ml::ParamGrid>, Ptr<cv::ml::ParamGrid>, bool) /usr/include/opencv2/ml.hpp:749
	pub fn cv_ml_SVM_trainAuto_const__InputArrayR_int_const__InputArrayR_int_Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__bool(instance: *mut c_void, samples: *const c_void, layout: i32, responses: *const c_void, k_fold: i32, cgrid: *mut c_void, gamma_grid: *mut c_void, p_grid: *mut c_void, nu_grid: *mut c_void, coeff_grid: *mut c_void, degree_grid: *mut c_void, balanced: bool, ocvrs_return: *mut Result<bool>);
	// getSupportVectors() /usr/include/opencv2/ml.hpp:766
	pub fn cv_ml_SVM_getSupportVectors_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getUncompressedSupportVectors() /usr/include/opencv2/ml.hpp:774
	pub fn cv_ml_SVM_getUncompressedSupportVectors_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getDecisionFunction(int, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ml.hpp:791
	pub fn cv_ml_SVM_getDecisionFunction_const_int_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, i: i32, alpha: *const c_void, svidx: *const c_void, ocvrs_return: *mut Result<f64>);
	// getDefaultGrid(int) /usr/include/opencv2/ml.hpp:801
	pub fn cv_ml_SVM_getDefaultGrid_int(param_id: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getDefaultGridPtr(int) /usr/include/opencv2/ml.hpp:811
	pub fn cv_ml_SVM_getDefaultGridPtr_int(param_id: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create() /usr/include/opencv2/ml.hpp:816
	pub fn cv_ml_SVM_create(ocvrs_return: *mut Result<*mut c_void>);
	// load(const cv::String &) /usr/include/opencv2/ml.hpp:825
	pub fn cv_ml_SVM_load_const_StringR(filepath: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// getType() /usr/include/opencv2/ml.hpp:533
	pub fn cv_ml_SVM_Kernel_getType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// calc(int, int, const float *, const float *, float *) /usr/include/opencv2/ml.hpp:534
	pub fn cv_ml_SVM_Kernel_calc_int_int_const_floatX_const_floatX_floatX(instance: *mut c_void, vcount: i32, n: i32, vecs: *const f32, another: *const f32, results: *mut f32, ocvrs_return: *mut Result_void);
	// getWeights() /usr/include/opencv2/ml.hpp:1818
	pub fn cv_ml_SVMSGD_getWeights(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getShift() /usr/include/opencv2/ml.hpp:1823
	pub fn cv_ml_SVMSGD_getShift(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
	// create() /usr/include/opencv2/ml.hpp:1829
	pub fn cv_ml_SVMSGD_create(ocvrs_return: *mut Result<*mut c_void>);
	// load(const cv::String &, const cv::String &) /usr/include/opencv2/ml.hpp:1840
	pub fn cv_ml_SVMSGD_load_const_StringR_const_StringR(filepath: *const c_char, node_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// setOptimalParameters(int, int) /usr/include/opencv2/ml.hpp:1846
	pub fn cv_ml_SVMSGD_setOptimalParameters_int_int(instance: *mut c_void, svmsgd_type: i32, margin_type: i32, ocvrs_return: *mut Result_void);
	// getSvmsgdType() /usr/include/opencv2/ml.hpp:1850
	pub fn cv_ml_SVMSGD_getSvmsgdType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setSvmsgdType(int) /usr/include/opencv2/ml.hpp:1852
	pub fn cv_ml_SVMSGD_setSvmsgdType_int(instance: *mut c_void, svmsgd_type: i32, ocvrs_return: *mut Result_void);
	// getMarginType() /usr/include/opencv2/ml.hpp:1856
	pub fn cv_ml_SVMSGD_getMarginType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMarginType(int) /usr/include/opencv2/ml.hpp:1858
	pub fn cv_ml_SVMSGD_setMarginType_int(instance: *mut c_void, margin_type: i32, ocvrs_return: *mut Result_void);
	// getMarginRegularization() /usr/include/opencv2/ml.hpp:1862
	pub fn cv_ml_SVMSGD_getMarginRegularization_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setMarginRegularization(float) /usr/include/opencv2/ml.hpp:1864
	pub fn cv_ml_SVMSGD_setMarginRegularization_float(instance: *mut c_void, margin_regularization: f32, ocvrs_return: *mut Result_void);
	// getInitialStepSize() /usr/include/opencv2/ml.hpp:1868
	pub fn cv_ml_SVMSGD_getInitialStepSize_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setInitialStepSize(float) /usr/include/opencv2/ml.hpp:1870
	pub fn cv_ml_SVMSGD_setInitialStepSize_float(instance: *mut c_void, initial_step_size: f32, ocvrs_return: *mut Result_void);
	// getStepDecreasingPower() /usr/include/opencv2/ml.hpp:1874
	pub fn cv_ml_SVMSGD_getStepDecreasingPower_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setStepDecreasingPower(float) /usr/include/opencv2/ml.hpp:1876
	pub fn cv_ml_SVMSGD_setStepDecreasingPower_float(instance: *mut c_void, step_decreasing_power: f32, ocvrs_return: *mut Result_void);
	// getTermCriteria() /usr/include/opencv2/ml.hpp:1882
	pub fn cv_ml_SVMSGD_getTermCriteria_const(instance: *const c_void, ocvrs_return: *mut Result<core::TermCriteria>);
	// setTermCriteria(const cv::TermCriteria &) /usr/include/opencv2/ml.hpp:1884
	pub fn cv_ml_SVMSGD_setTermCriteria_const_TermCriteriaR(instance: *mut c_void, val: *const core::TermCriteria, ocvrs_return: *mut Result_void);
	// getVarCount() /usr/include/opencv2/ml.hpp:330
	pub fn cv_ml_StatModel_getVarCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// empty() /usr/include/opencv2/ml.hpp:332
	pub fn cv_ml_StatModel_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isTrained() /usr/include/opencv2/ml.hpp:335
	pub fn cv_ml_StatModel_isTrained_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isClassifier() /usr/include/opencv2/ml.hpp:337
	pub fn cv_ml_StatModel_isClassifier_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// train(const Ptr<cv::ml::TrainData> &, int) /usr/include/opencv2/ml.hpp:346
	pub fn cv_ml_StatModel_train_const_Ptr_TrainData_R_int(instance: *mut c_void, train_data: *const c_void, flags: i32, ocvrs_return: *mut Result<bool>);
	// train(cv::InputArray, int, cv::InputArray) /usr/include/opencv2/ml.hpp:354
	pub fn cv_ml_StatModel_train_const__InputArrayR_int_const__InputArrayR(instance: *mut c_void, samples: *const c_void, layout: i32, responses: *const c_void, ocvrs_return: *mut Result<bool>);
	// calcError(const Ptr<cv::ml::TrainData> &, bool, cv::OutputArray) /usr/include/opencv2/ml.hpp:369
	pub fn cv_ml_StatModel_calcError_const_const_Ptr_TrainData_R_bool_const__OutputArrayR(instance: *const c_void, data: *const c_void, test: bool, resp: *const c_void, ocvrs_return: *mut Result<f32>);
	// predict(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ml.hpp:377
	pub fn cv_ml_StatModel_predict_const_const__InputArrayR_const__OutputArrayR_int(instance: *const c_void, samples: *const c_void, results: *const c_void, flags: i32, ocvrs_return: *mut Result<f32>);
	// missingValue() /usr/include/opencv2/ml.hpp:148
	pub fn cv_ml_TrainData_missingValue(ocvrs_return: *mut Result<f32>);
	// getLayout() /usr/include/opencv2/ml.hpp:151
	pub fn cv_ml_TrainData_getLayout_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getNTrainSamples() /usr/include/opencv2/ml.hpp:152
	pub fn cv_ml_TrainData_getNTrainSamples_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getNTestSamples() /usr/include/opencv2/ml.hpp:153
	pub fn cv_ml_TrainData_getNTestSamples_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getNSamples() /usr/include/opencv2/ml.hpp:154
	pub fn cv_ml_TrainData_getNSamples_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getNVars() /usr/include/opencv2/ml.hpp:155
	pub fn cv_ml_TrainData_getNVars_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getNAllVars() /usr/include/opencv2/ml.hpp:156
	pub fn cv_ml_TrainData_getNAllVars_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getSample(cv::InputArray, int, float *) /usr/include/opencv2/ml.hpp:158
	pub fn cv_ml_TrainData_getSample_const_const__InputArrayR_int_floatX(instance: *const c_void, var_idx: *const c_void, sidx: i32, buf: *mut f32, ocvrs_return: *mut Result_void);
	// getSamples() /usr/include/opencv2/ml.hpp:159
	pub fn cv_ml_TrainData_getSamples_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getMissing() /usr/include/opencv2/ml.hpp:160
	pub fn cv_ml_TrainData_getMissing_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getTrainSamples(int, bool, bool) /usr/include/opencv2/ml.hpp:174
	pub fn cv_ml_TrainData_getTrainSamples_const_int_bool_bool(instance: *const c_void, layout: i32, compress_samples: bool, compress_vars: bool, ocvrs_return: *mut Result<*mut c_void>);
	// getTrainResponses() /usr/include/opencv2/ml.hpp:183
	pub fn cv_ml_TrainData_getTrainResponses_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getTrainNormCatResponses() /usr/include/opencv2/ml.hpp:191
	pub fn cv_ml_TrainData_getTrainNormCatResponses_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getTestResponses() /usr/include/opencv2/ml.hpp:192
	pub fn cv_ml_TrainData_getTestResponses_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getTestNormCatResponses() /usr/include/opencv2/ml.hpp:193
	pub fn cv_ml_TrainData_getTestNormCatResponses_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getResponses() /usr/include/opencv2/ml.hpp:194
	pub fn cv_ml_TrainData_getResponses_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getNormCatResponses() /usr/include/opencv2/ml.hpp:195
	pub fn cv_ml_TrainData_getNormCatResponses_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getSampleWeights() /usr/include/opencv2/ml.hpp:196
	pub fn cv_ml_TrainData_getSampleWeights_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getTrainSampleWeights() /usr/include/opencv2/ml.hpp:197
	pub fn cv_ml_TrainData_getTrainSampleWeights_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getTestSampleWeights() /usr/include/opencv2/ml.hpp:198
	pub fn cv_ml_TrainData_getTestSampleWeights_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getVarIdx() /usr/include/opencv2/ml.hpp:199
	pub fn cv_ml_TrainData_getVarIdx_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getVarType() /usr/include/opencv2/ml.hpp:200
	pub fn cv_ml_TrainData_getVarType_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getVarSymbolFlags() /usr/include/opencv2/ml.hpp:201
	pub fn cv_ml_TrainData_getVarSymbolFlags_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getResponseType() /usr/include/opencv2/ml.hpp:202
	pub fn cv_ml_TrainData_getResponseType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getTrainSampleIdx() /usr/include/opencv2/ml.hpp:203
	pub fn cv_ml_TrainData_getTrainSampleIdx_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getTestSampleIdx() /usr/include/opencv2/ml.hpp:204
	pub fn cv_ml_TrainData_getTestSampleIdx_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getValues(int, cv::InputArray, float *) /usr/include/opencv2/ml.hpp:205
	pub fn cv_ml_TrainData_getValues_const_int_const__InputArrayR_floatX(instance: *const c_void, vi: i32, sidx: *const c_void, values: *mut f32, ocvrs_return: *mut Result_void);
	// getNormCatValues(int, cv::InputArray, int *) /usr/include/opencv2/ml.hpp:206
	pub fn cv_ml_TrainData_getNormCatValues_const_int_const__InputArrayR_intX(instance: *const c_void, vi: i32, sidx: *const c_void, values: *mut i32, ocvrs_return: *mut Result_void);
	// getDefaultSubstValues() /usr/include/opencv2/ml.hpp:207
	pub fn cv_ml_TrainData_getDefaultSubstValues_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getCatCount(int) /usr/include/opencv2/ml.hpp:209
	pub fn cv_ml_TrainData_getCatCount_const_int(instance: *const c_void, vi: i32, ocvrs_return: *mut Result<i32>);
	// getClassLabels() /usr/include/opencv2/ml.hpp:215
	pub fn cv_ml_TrainData_getClassLabels_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getCatOfs() /usr/include/opencv2/ml.hpp:217
	pub fn cv_ml_TrainData_getCatOfs_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getCatMap() /usr/include/opencv2/ml.hpp:218
	pub fn cv_ml_TrainData_getCatMap_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setTrainTestSplit(int, bool) /usr/include/opencv2/ml.hpp:223
	pub fn cv_ml_TrainData_setTrainTestSplit_int_bool(instance: *mut c_void, count: i32, shuffle: bool, ocvrs_return: *mut Result_void);
	// setTrainTestSplitRatio(double, bool) /usr/include/opencv2/ml.hpp:233
	pub fn cv_ml_TrainData_setTrainTestSplitRatio_double_bool(instance: *mut c_void, ratio: f64, shuffle: bool, ocvrs_return: *mut Result_void);
	// shuffleTrainTest() /usr/include/opencv2/ml.hpp:234
	pub fn cv_ml_TrainData_shuffleTrainTest(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// getTestSamples() /usr/include/opencv2/ml.hpp:237
	pub fn cv_ml_TrainData_getTestSamples_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getNames(std::vector<String> &) /usr/include/opencv2/ml.hpp:240
	pub fn cv_ml_TrainData_getNames_const_vector_String_R(instance: *const c_void, names: *mut c_void, ocvrs_return: *mut Result_void);
	// getSubVector(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/ml.hpp:246
	pub fn cv_ml_TrainData_getSubVector_const_MatR_const_MatR(vec: *const c_void, idx: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getSubMatrix(const cv::Mat &, const cv::Mat &, int) /usr/include/opencv2/ml.hpp:253
	pub fn cv_ml_TrainData_getSubMatrix_const_MatR_const_MatR_int(matrix: *const c_void, idx: *const c_void, layout: i32, ocvrs_return: *mut Result<*mut c_void>);
	// loadFromCSV(const cv::String &, int, int, int, const cv::String &, char, char) /usr/include/opencv2/ml.hpp:284
	pub fn cv_ml_TrainData_loadFromCSV_const_StringR_int_int_int_const_StringR_char_char(filename: *const c_char, header_line_count: i32, response_start_idx: i32, response_end_idx: i32, var_type_spec: *const c_char, delimiter: i8, missch: i8, ocvrs_return: *mut Result<*mut c_void>);
	// create(cv::InputArray, int, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/ml.hpp:311
	pub fn cv_ml_TrainData_create_const__InputArrayR_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(samples: *const c_void, layout: i32, responses: *const c_void, var_idx: *const c_void, sample_idx: *const c_void, sample_weights: *const c_void, var_type: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
}
