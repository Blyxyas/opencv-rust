extern "C" {
	// EMDL1(cv::InputArray, cv::InputArray) /usr/include/opencv2/shape/emdL1.hpp:66
	pub fn cv_EMDL1_const__InputArrayR_const__InputArrayR(signature1: *const c_void, signature2: *const c_void, ocvrs_return: *mut Result<f32>);
	// createAffineTransformer(bool) /usr/include/opencv2/shape/shape_transformer.hpp:127
	pub fn cv_createAffineTransformer_bool(full_affine: bool, ocvrs_return: *mut Result<*mut c_void>);
	// createChiHistogramCostExtractor(int, float) /usr/include/opencv2/shape/hist_cost.hpp:98
	pub fn cv_createChiHistogramCostExtractor_int_float(n_dummies: i32, default_cost: f32, ocvrs_return: *mut Result<*mut c_void>);
	// createEMDHistogramCostExtractor(int, int, float) /usr/include/opencv2/shape/hist_cost.hpp:91
	pub fn cv_createEMDHistogramCostExtractor_int_int_float(flag: i32, n_dummies: i32, default_cost: f32, ocvrs_return: *mut Result<*mut c_void>);
	// createEMDL1HistogramCostExtractor(int, float) /usr/include/opencv2/shape/hist_cost.hpp:106
	pub fn cv_createEMDL1HistogramCostExtractor_int_float(n_dummies: i32, default_cost: f32, ocvrs_return: *mut Result<*mut c_void>);
	// createHausdorffDistanceExtractor(int, float) /usr/include/opencv2/shape/shape_distance.hpp:222
	pub fn cv_createHausdorffDistanceExtractor_int_float(distance_flag: i32, rank_prop: f32, ocvrs_return: *mut Result<*mut c_void>);
	// createNormHistogramCostExtractor(int, int, float) /usr/include/opencv2/shape/hist_cost.hpp:79
	pub fn cv_createNormHistogramCostExtractor_int_int_float(flag: i32, n_dummies: i32, default_cost: f32, ocvrs_return: *mut Result<*mut c_void>);
	// createShapeContextDistanceExtractor(int, int, float, float, int, const Ptr<cv::HistogramCostExtractor> &, const Ptr<cv::ShapeTransformer> &) /usr/include/opencv2/shape/shape_distance.hpp:187
	pub fn cv_createShapeContextDistanceExtractor_int_int_float_float_int_const_Ptr_HistogramCostExtractor_R_const_Ptr_ShapeTransformer_R(n_angular_bins: i32, n_radial_bins: i32, inner_radius: f32, outer_radius: f32, iterations: i32, comparer: *const c_void, transformer: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// createThinPlateSplineShapeTransformer(double) /usr/include/opencv2/shape/shape_transformer.hpp:112
	pub fn cv_createThinPlateSplineShapeTransformer_double(regularization_parameter: f64, ocvrs_return: *mut Result<*mut c_void>);
	// setFullAffine(bool) /usr/include/opencv2/shape/shape_transformer.hpp:122
	pub fn cv_AffineTransformer_setFullAffine_bool(instance: *mut c_void, full_affine: bool, ocvrs_return: *mut Result_void);
	// getFullAffine() /usr/include/opencv2/shape/shape_transformer.hpp:123
	pub fn cv_AffineTransformer_getFullAffine_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setNormFlag(int) /usr/include/opencv2/shape/hist_cost.hpp:86
	pub fn cv_EMDHistogramCostExtractor_setNormFlag_int(instance: *mut c_void, flag: i32, ocvrs_return: *mut Result_void);
	// getNormFlag() /usr/include/opencv2/shape/hist_cost.hpp:87
	pub fn cv_EMDHistogramCostExtractor_getNormFlag_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setDistanceFlag(int) /usr/include/opencv2/shape/shape_distance.hpp:208
	pub fn cv_HausdorffDistanceExtractor_setDistanceFlag_int(instance: *mut c_void, distance_flag: i32, ocvrs_return: *mut Result_void);
	// getDistanceFlag() /usr/include/opencv2/shape/shape_distance.hpp:209
	pub fn cv_HausdorffDistanceExtractor_getDistanceFlag_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setRankProportion(float) /usr/include/opencv2/shape/shape_distance.hpp:217
	pub fn cv_HausdorffDistanceExtractor_setRankProportion_float(instance: *mut c_void, rank_proportion: f32, ocvrs_return: *mut Result_void);
	// getRankProportion() /usr/include/opencv2/shape/shape_distance.hpp:218
	pub fn cv_HausdorffDistanceExtractor_getRankProportion_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// buildCostMatrix(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/shape/hist_cost.hpp:60
	pub fn cv_HistogramCostExtractor_buildCostMatrix_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, descriptors1: *const c_void, descriptors2: *const c_void, cost_matrix: *const c_void, ocvrs_return: *mut Result_void);
	// setNDummies(int) /usr/include/opencv2/shape/hist_cost.hpp:62
	pub fn cv_HistogramCostExtractor_setNDummies_int(instance: *mut c_void, n_dummies: i32, ocvrs_return: *mut Result_void);
	// getNDummies() /usr/include/opencv2/shape/hist_cost.hpp:63
	pub fn cv_HistogramCostExtractor_getNDummies_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setDefaultCost(float) /usr/include/opencv2/shape/hist_cost.hpp:65
	pub fn cv_HistogramCostExtractor_setDefaultCost_float(instance: *mut c_void, default_cost: f32, ocvrs_return: *mut Result_void);
	// getDefaultCost() /usr/include/opencv2/shape/hist_cost.hpp:66
	pub fn cv_HistogramCostExtractor_getDefaultCost_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setNormFlag(int) /usr/include/opencv2/shape/hist_cost.hpp:74
	pub fn cv_NormHistogramCostExtractor_setNormFlag_int(instance: *mut c_void, flag: i32, ocvrs_return: *mut Result_void);
	// getNormFlag() /usr/include/opencv2/shape/hist_cost.hpp:75
	pub fn cv_NormHistogramCostExtractor_getNormFlag_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setAngularBins(int) /usr/include/opencv2/shape/shape_distance.hpp:89
	pub fn cv_ShapeContextDistanceExtractor_setAngularBins_int(instance: *mut c_void, n_angular_bins: i32, ocvrs_return: *mut Result_void);
	// getAngularBins() /usr/include/opencv2/shape/shape_distance.hpp:90
	pub fn cv_ShapeContextDistanceExtractor_getAngularBins_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setRadialBins(int) /usr/include/opencv2/shape/shape_distance.hpp:97
	pub fn cv_ShapeContextDistanceExtractor_setRadialBins_int(instance: *mut c_void, n_radial_bins: i32, ocvrs_return: *mut Result_void);
	// getRadialBins() /usr/include/opencv2/shape/shape_distance.hpp:98
	pub fn cv_ShapeContextDistanceExtractor_getRadialBins_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setInnerRadius(float) /usr/include/opencv2/shape/shape_distance.hpp:104
	pub fn cv_ShapeContextDistanceExtractor_setInnerRadius_float(instance: *mut c_void, inner_radius: f32, ocvrs_return: *mut Result_void);
	// getInnerRadius() /usr/include/opencv2/shape/shape_distance.hpp:105
	pub fn cv_ShapeContextDistanceExtractor_getInnerRadius_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setOuterRadius(float) /usr/include/opencv2/shape/shape_distance.hpp:111
	pub fn cv_ShapeContextDistanceExtractor_setOuterRadius_float(instance: *mut c_void, outer_radius: f32, ocvrs_return: *mut Result_void);
	// getOuterRadius() /usr/include/opencv2/shape/shape_distance.hpp:112
	pub fn cv_ShapeContextDistanceExtractor_getOuterRadius_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setRotationInvariant(bool) /usr/include/opencv2/shape/shape_distance.hpp:114
	pub fn cv_ShapeContextDistanceExtractor_setRotationInvariant_bool(instance: *mut c_void, rotation_invariant: bool, ocvrs_return: *mut Result_void);
	// getRotationInvariant() /usr/include/opencv2/shape/shape_distance.hpp:115
	pub fn cv_ShapeContextDistanceExtractor_getRotationInvariant_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setShapeContextWeight(float) /usr/include/opencv2/shape/shape_distance.hpp:124
	pub fn cv_ShapeContextDistanceExtractor_setShapeContextWeight_float(instance: *mut c_void, shape_context_weight: f32, ocvrs_return: *mut Result_void);
	// getShapeContextWeight() /usr/include/opencv2/shape/shape_distance.hpp:125
	pub fn cv_ShapeContextDistanceExtractor_getShapeContextWeight_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setImageAppearanceWeight(float) /usr/include/opencv2/shape/shape_distance.hpp:136
	pub fn cv_ShapeContextDistanceExtractor_setImageAppearanceWeight_float(instance: *mut c_void, image_appearance_weight: f32, ocvrs_return: *mut Result_void);
	// getImageAppearanceWeight() /usr/include/opencv2/shape/shape_distance.hpp:137
	pub fn cv_ShapeContextDistanceExtractor_getImageAppearanceWeight_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setBendingEnergyWeight(float) /usr/include/opencv2/shape/shape_distance.hpp:146
	pub fn cv_ShapeContextDistanceExtractor_setBendingEnergyWeight_float(instance: *mut c_void, bending_energy_weight: f32, ocvrs_return: *mut Result_void);
	// getBendingEnergyWeight() /usr/include/opencv2/shape/shape_distance.hpp:147
	pub fn cv_ShapeContextDistanceExtractor_getBendingEnergyWeight_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setImages(cv::InputArray, cv::InputArray) /usr/include/opencv2/shape/shape_distance.hpp:155
	pub fn cv_ShapeContextDistanceExtractor_setImages_const__InputArrayR_const__InputArrayR(instance: *mut c_void, image1: *const c_void, image2: *const c_void, ocvrs_return: *mut Result_void);
	// getImages(cv::OutputArray, cv::OutputArray) /usr/include/opencv2/shape/shape_distance.hpp:156
	pub fn cv_ShapeContextDistanceExtractor_getImages_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, image1: *const c_void, image2: *const c_void, ocvrs_return: *mut Result_void);
	// setIterations(int) /usr/include/opencv2/shape/shape_distance.hpp:158
	pub fn cv_ShapeContextDistanceExtractor_setIterations_int(instance: *mut c_void, iterations: i32, ocvrs_return: *mut Result_void);
	// getIterations() /usr/include/opencv2/shape/shape_distance.hpp:159
	pub fn cv_ShapeContextDistanceExtractor_getIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setCostExtractor(Ptr<cv::HistogramCostExtractor>) /usr/include/opencv2/shape/shape_distance.hpp:166
	pub fn cv_ShapeContextDistanceExtractor_setCostExtractor_Ptr_HistogramCostExtractor_(instance: *mut c_void, comparer: *mut c_void, ocvrs_return: *mut Result_void);
	// getCostExtractor() /usr/include/opencv2/shape/shape_distance.hpp:167
	pub fn cv_ShapeContextDistanceExtractor_getCostExtractor_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setStdDev(float) /usr/include/opencv2/shape/shape_distance.hpp:173
	pub fn cv_ShapeContextDistanceExtractor_setStdDev_float(instance: *mut c_void, sigma: f32, ocvrs_return: *mut Result_void);
	// getStdDev() /usr/include/opencv2/shape/shape_distance.hpp:174
	pub fn cv_ShapeContextDistanceExtractor_getStdDev_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setTransformAlgorithm(Ptr<cv::ShapeTransformer>) /usr/include/opencv2/shape/shape_distance.hpp:181
	pub fn cv_ShapeContextDistanceExtractor_setTransformAlgorithm_Ptr_ShapeTransformer_(instance: *mut c_void, transformer: *mut c_void, ocvrs_return: *mut Result_void);
	// getTransformAlgorithm() /usr/include/opencv2/shape/shape_distance.hpp:182
	pub fn cv_ShapeContextDistanceExtractor_getTransformAlgorithm_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// computeDistance(cv::InputArray, cv::InputArray) /usr/include/opencv2/shape/shape_distance.hpp:69
	pub fn cv_ShapeDistanceExtractor_computeDistance_const__InputArrayR_const__InputArrayR(instance: *mut c_void, contour1: *const c_void, contour2: *const c_void, ocvrs_return: *mut Result<f32>);
	// estimateTransformation(cv::InputArray, cv::InputArray, std::vector<DMatch> &) /usr/include/opencv2/shape/shape_transformer.hpp:67
	pub fn cv_ShapeTransformer_estimateTransformation_const__InputArrayR_const__InputArrayR_vector_DMatch_R(instance: *mut c_void, transforming_shape: *const c_void, target_shape: *const c_void, matches: *mut c_void, ocvrs_return: *mut Result_void);
	// applyTransformation(cv::InputArray, cv::OutputArray) /usr/include/opencv2/shape/shape_transformer.hpp:75
	pub fn cv_ShapeTransformer_applyTransformation_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, input: *const c_void, output: *const c_void, ocvrs_return: *mut Result<f32>);
	// warpImage(cv::InputArray, cv::OutputArray, int, int, const cv::Scalar &) /usr/include/opencv2/shape/shape_transformer.hpp:85
	pub fn cv_ShapeTransformer_warpImage_const_const__InputArrayR_const__OutputArrayR_int_int_const_ScalarR(instance: *const c_void, transforming_image: *const c_void, output: *const c_void, flags: i32, border_mode: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result_void);
	// setRegularizationParameter(double) /usr/include/opencv2/shape/shape_transformer.hpp:106
	pub fn cv_ThinPlateSplineShapeTransformer_setRegularizationParameter_double(instance: *mut c_void, beta: f64, ocvrs_return: *mut Result_void);
	// getRegularizationParameter() /usr/include/opencv2/shape/shape_transformer.hpp:107
	pub fn cv_ThinPlateSplineShapeTransformer_getRegularizationParameter_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
}
