extern "C" {
	// AGAST(cv::InputArray, std::vector<KeyPoint> &, int, bool) /usr/include/opencv2/features2d.hpp:611
	pub fn cv_AGAST_const__InputArrayR_vector_KeyPoint_R_int_bool(image: *const c_void, keypoints: *mut c_void, threshold: i32, nonmax_suppression: bool, ocvrs_return: *mut Result_void);
	// AGAST(cv::InputArray, std::vector<KeyPoint> &, int, bool, AgastFeatureDetector::DetectorType) /usr/include/opencv2/features2d.hpp:632
	pub fn cv_AGAST_const__InputArrayR_vector_KeyPoint_R_int_bool_DetectorType(image: *const c_void, keypoints: *mut c_void, threshold: i32, nonmax_suppression: bool, typ: crate::features2d::AgastFeatureDetector_DetectorType, ocvrs_return: *mut Result_void);
	// FAST(cv::InputArray, std::vector<KeyPoint> &, int, bool) /usr/include/opencv2/features2d.hpp:551
	pub fn cv_FAST_const__InputArrayR_vector_KeyPoint_R_int_bool(image: *const c_void, keypoints: *mut c_void, threshold: i32, nonmax_suppression: bool, ocvrs_return: *mut Result_void);
	// FAST(cv::InputArray, std::vector<KeyPoint> &, int, bool, FastFeatureDetector::DetectorType) /usr/include/opencv2/features2d.hpp:572
	pub fn cv_FAST_const__InputArrayR_vector_KeyPoint_R_int_bool_DetectorType(image: *const c_void, keypoints: *mut c_void, threshold: i32, nonmax_suppression: bool, typ: crate::features2d::FastFeatureDetector_DetectorType, ocvrs_return: *mut Result_void);
	// computeRecallPrecisionCurve(const std::vector<std::vector<DMatch>> &, const std::vector<std::vector<uchar>> &, std::vector<Point2f> &) /usr/include/opencv2/features2d.hpp:1364
	pub fn cv_computeRecallPrecisionCurve_const_vector_vector_DMatch__R_const_vector_vector_unsigned_char__R_vector_Point2f_R(matches1to2: *const c_void, correct_matches1to2_mask: *const c_void, recall_precision_curve: *mut c_void, ocvrs_return: *mut Result_void);
	// drawKeypoints(cv::InputArray, const std::vector<KeyPoint> &, cv::InputOutputArray, const cv::Scalar &, cv::DrawMatchesFlags) /usr/include/opencv2/features2d.hpp:1308
	pub fn cv_drawKeypoints_const__InputArrayR_const_vector_KeyPoint_R_const__InputOutputArrayR_const_ScalarR_DrawMatchesFlags(image: *const c_void, keypoints: *const c_void, out_image: *const c_void, color: *const core::Scalar, flags: crate::features2d::DrawMatchesFlags, ocvrs_return: *mut Result_void);
	// drawMatches(cv::InputArray, const std::vector<KeyPoint> &, cv::InputArray, const std::vector<KeyPoint> &, const std::vector<DMatch> &, cv::InputOutputArray, const cv::Scalar &, const cv::Scalar &, const std::vector<char> &, cv::DrawMatchesFlags) /usr/include/opencv2/features2d.hpp:1333
	pub fn cv_drawMatches_const__InputArrayR_const_vector_KeyPoint_R_const__InputArrayR_const_vector_KeyPoint_R_const_vector_DMatch_R_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vector_char_R_DrawMatchesFlags(img1: *const c_void, keypoints1: *const c_void, img2: *const c_void, keypoints2: *const c_void, matches1to2: *const c_void, out_img: *const c_void, match_color: *const core::Scalar, single_point_color: *const core::Scalar, matches_mask: *const c_void, flags: crate::features2d::DrawMatchesFlags, ocvrs_return: *mut Result_void);
	// drawMatches(cv::InputArray, const std::vector<KeyPoint> &, cv::InputArray, const std::vector<KeyPoint> &, const std::vector<DMatch> &, cv::InputOutputArray, const int, const cv::Scalar &, const cv::Scalar &, const std::vector<char> &, cv::DrawMatchesFlags) /usr/include/opencv2/features2d.hpp:1340
	pub fn cv_drawMatches_const__InputArrayR_const_vector_KeyPoint_R_const__InputArrayR_const_vector_KeyPoint_R_const_vector_DMatch_R_const__InputOutputArrayR_const_int_const_ScalarR_const_ScalarR_const_vector_char_R_DrawMatchesFlags(img1: *const c_void, keypoints1: *const c_void, img2: *const c_void, keypoints2: *const c_void, matches1to2: *const c_void, out_img: *const c_void, matches_thickness: i32, match_color: *const core::Scalar, single_point_color: *const core::Scalar, matches_mask: *const c_void, flags: crate::features2d::DrawMatchesFlags, ocvrs_return: *mut Result_void);
	// drawMatches(cv::InputArray, const std::vector<KeyPoint> &, cv::InputArray, const std::vector<KeyPoint> &, const std::vector<std::vector<DMatch>> &, cv::InputOutputArray, const cv::Scalar &, const cv::Scalar &, const std::vector<std::vector<char>> &, cv::DrawMatchesFlags) /usr/include/opencv2/features2d.hpp:1347
	pub fn cv_drawMatches_const__InputArrayR_const_vector_KeyPoint_R_const__InputArrayR_const_vector_KeyPoint_R_const_vector_vector_DMatch__R_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vector_vector_char__R_DrawMatchesFlags(img1: *const c_void, keypoints1: *const c_void, img2: *const c_void, keypoints2: *const c_void, matches1to2: *const c_void, out_img: *const c_void, match_color: *const core::Scalar, single_point_color: *const core::Scalar, matches_mask: *const c_void, flags: crate::features2d::DrawMatchesFlags, ocvrs_return: *mut Result_void);
	// evaluateFeatureDetector(const cv::Mat &, const cv::Mat &, const cv::Mat &, std::vector<KeyPoint> *, std::vector<KeyPoint> *, float &, int &, const Ptr<cv::FeatureDetector> &) /usr/include/opencv2/features2d.hpp:1359
	pub fn cv_evaluateFeatureDetector_const_MatR_const_MatR_const_MatR_vector_KeyPoint_X_vector_KeyPoint_X_floatR_intR_const_Ptr_Feature2D_R(img1: *const c_void, img2: *const c_void, h1to2: *const c_void, keypoints1: *mut c_void, keypoints2: *mut c_void, repeatability: *mut f32, corresp_count: *mut i32, fdetector: *const c_void, ocvrs_return: *mut Result_void);
	// getNearestPoint(const std::vector<Point2f> &, float) /usr/include/opencv2/features2d.hpp:1369
	pub fn cv_getNearestPoint_const_vector_Point2f_R_float(recall_precision_curve: *const c_void, l_precision: f32, ocvrs_return: *mut Result<i32>);
	// getRecall(const std::vector<Point2f> &, float) /usr/include/opencv2/features2d.hpp:1368
	pub fn cv_getRecall_const_vector_Point2f_R_float(recall_precision_curve: *const c_void, l_precision: f32, ocvrs_return: *mut Result<f32>);
	// create(AKAZE::DescriptorType, int, int, float, int, int, KAZE::DiffusivityType) /usr/include/opencv2/features2d.hpp:828
	pub fn cv_AKAZE_create_DescriptorType_int_int_float_int_int_DiffusivityType(descriptor_type: crate::features2d::AKAZE_DescriptorType, descriptor_size: i32, descriptor_channels: i32, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: crate::features2d::KAZE_DiffusivityType, ocvrs_return: *mut Result<*mut c_void>);
	// setDescriptorType(AKAZE::DescriptorType) /usr/include/opencv2/features2d.hpp:833
	pub fn cv_AKAZE_setDescriptorType_DescriptorType(instance: *mut c_void, dtype: crate::features2d::AKAZE_DescriptorType, ocvrs_return: *mut Result_void);
	// getDescriptorType() /usr/include/opencv2/features2d.hpp:834
	pub fn cv_AKAZE_getDescriptorType_const(instance: *const c_void, ocvrs_return: *mut Result<crate::features2d::AKAZE_DescriptorType>);
	// setDescriptorSize(int) /usr/include/opencv2/features2d.hpp:836
	pub fn cv_AKAZE_setDescriptorSize_int(instance: *mut c_void, dsize: i32, ocvrs_return: *mut Result_void);
	// getDescriptorSize() /usr/include/opencv2/features2d.hpp:837
	pub fn cv_AKAZE_getDescriptorSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setDescriptorChannels(int) /usr/include/opencv2/features2d.hpp:839
	pub fn cv_AKAZE_setDescriptorChannels_int(instance: *mut c_void, dch: i32, ocvrs_return: *mut Result_void);
	// getDescriptorChannels() /usr/include/opencv2/features2d.hpp:840
	pub fn cv_AKAZE_getDescriptorChannels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setThreshold(double) /usr/include/opencv2/features2d.hpp:842
	pub fn cv_AKAZE_setThreshold_double(instance: *mut c_void, threshold: f64, ocvrs_return: *mut Result_void);
	// getThreshold() /usr/include/opencv2/features2d.hpp:843
	pub fn cv_AKAZE_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setNOctaves(int) /usr/include/opencv2/features2d.hpp:845
	pub fn cv_AKAZE_setNOctaves_int(instance: *mut c_void, octaves: i32, ocvrs_return: *mut Result_void);
	// getNOctaves() /usr/include/opencv2/features2d.hpp:846
	pub fn cv_AKAZE_getNOctaves_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setNOctaveLayers(int) /usr/include/opencv2/features2d.hpp:848
	pub fn cv_AKAZE_setNOctaveLayers_int(instance: *mut c_void, octave_layers: i32, ocvrs_return: *mut Result_void);
	// getNOctaveLayers() /usr/include/opencv2/features2d.hpp:849
	pub fn cv_AKAZE_getNOctaveLayers_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setDiffusivity(KAZE::DiffusivityType) /usr/include/opencv2/features2d.hpp:851
	pub fn cv_AKAZE_setDiffusivity_DiffusivityType(instance: *mut c_void, diff: crate::features2d::KAZE_DiffusivityType, ocvrs_return: *mut Result_void);
	// getDiffusivity() /usr/include/opencv2/features2d.hpp:852
	pub fn cv_AKAZE_getDiffusivity_const(instance: *const c_void, ocvrs_return: *mut Result<crate::features2d::KAZE_DiffusivityType>);
	// getDefaultName() /usr/include/opencv2/features2d.hpp:853
	pub fn cv_AKAZE_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const Ptr<cv::Feature2D> &, int, int, float, float) /usr/include/opencv2/features2d.hpp:244
	pub fn cv_AffineFeature_create_const_Ptr_Feature2D_R_int_int_float_float(backend: *const c_void, max_tilt: i32, min_tilt: i32, tilt_step: f32, rotate_step_base: f32, ocvrs_return: *mut Result<*mut c_void>);
	// setViewParams(const std::vector<float> &, const std::vector<float> &) /usr/include/opencv2/features2d.hpp:247
	pub fn cv_AffineFeature_setViewParams_const_vector_float_R_const_vector_float_R(instance: *mut c_void, tilts: *const c_void, rolls: *const c_void, ocvrs_return: *mut Result_void);
	// getViewParams(std::vector<float> &, std::vector<float> &) /usr/include/opencv2/features2d.hpp:248
	pub fn cv_AffineFeature_getViewParams_const_vector_float_R_vector_float_R(instance: *const c_void, tilts: *mut c_void, rolls: *mut c_void, ocvrs_return: *mut Result_void);
	// getDefaultName() /usr/include/opencv2/features2d.hpp:249
	pub fn cv_AffineFeature_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, bool, AgastFeatureDetector::DetectorType) /usr/include/opencv2/features2d.hpp:595
	pub fn cv_AgastFeatureDetector_create_int_bool_DetectorType(threshold: i32, nonmax_suppression: bool, typ: crate::features2d::AgastFeatureDetector_DetectorType, ocvrs_return: *mut Result<*mut c_void>);
	// setThreshold(int) /usr/include/opencv2/features2d.hpp:599
	pub fn cv_AgastFeatureDetector_setThreshold_int(instance: *mut c_void, threshold: i32, ocvrs_return: *mut Result_void);
	// getThreshold() /usr/include/opencv2/features2d.hpp:600
	pub fn cv_AgastFeatureDetector_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setNonmaxSuppression(bool) /usr/include/opencv2/features2d.hpp:602
	pub fn cv_AgastFeatureDetector_setNonmaxSuppression_bool(instance: *mut c_void, f: bool, ocvrs_return: *mut Result_void);
	// getNonmaxSuppression() /usr/include/opencv2/features2d.hpp:603
	pub fn cv_AgastFeatureDetector_getNonmaxSuppression_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setType(AgastFeatureDetector::DetectorType) /usr/include/opencv2/features2d.hpp:605
	pub fn cv_AgastFeatureDetector_setType_DetectorType(instance: *mut c_void, typ: crate::features2d::AgastFeatureDetector_DetectorType, ocvrs_return: *mut Result_void);
	// getType() /usr/include/opencv2/features2d.hpp:606
	pub fn cv_AgastFeatureDetector_getType_const(instance: *const c_void, ocvrs_return: *mut Result<crate::features2d::AgastFeatureDetector_DetectorType>);
	// getDefaultName() /usr/include/opencv2/features2d.hpp:607
	pub fn cv_AgastFeatureDetector_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// BFMatcher(int, bool) /usr/include/opencv2/features2d.hpp:1189
	pub fn cv_BFMatcher_BFMatcher_int_bool(norm_type: i32, cross_check: bool, ocvrs_return: *mut Result<*mut c_void>);
	// isMaskSupported() /usr/include/opencv2/features2d.hpp:1193
	pub fn cv_BFMatcher_isMaskSupported_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// create(int, bool) /usr/include/opencv2/features2d.hpp:1207
	pub fn cv_BFMatcher_create_int_bool(norm_type: i32, cross_check: bool, ocvrs_return: *mut Result<*mut c_void>);
	// clone(bool) /usr/include/opencv2/features2d.hpp:1209
	pub fn cv_BFMatcher_clone_const_bool(instance: *const c_void, empty_train_data: bool, ocvrs_return: *mut Result<*mut c_void>);
	// BOWImgDescriptorExtractor(const Ptr<cv::DescriptorExtractor> &, const Ptr<cv::DescriptorMatcher> &) /usr/include/opencv2/features2d.hpp:1472
	pub fn cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_Ptr_Feature2D_R_const_Ptr_DescriptorMatcher_R(dextractor: *const c_void, dmatcher: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// BOWImgDescriptorExtractor(const Ptr<cv::DescriptorMatcher> &) /usr/include/opencv2/features2d.hpp:1475
	pub fn cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_Ptr_DescriptorMatcher_R(dmatcher: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setVocabulary(const cv::Mat &) /usr/include/opencv2/features2d.hpp:1483
	pub fn cv_BOWImgDescriptorExtractor_setVocabulary_const_MatR(instance: *mut c_void, vocabulary: *const c_void, ocvrs_return: *mut Result_void);
	// getVocabulary() /usr/include/opencv2/features2d.hpp:1487
	pub fn cv_BOWImgDescriptorExtractor_getVocabulary_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// compute(cv::InputArray, std::vector<KeyPoint> &, cv::OutputArray, std::vector<std::vector<int>> *, cv::Mat *) /usr/include/opencv2/features2d.hpp:1499
	pub fn cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_vector_KeyPoint_R_const__OutputArrayR_vector_vector_int__X_MatX(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, img_descriptor: *const c_void, point_idxs_of_clusters: *mut c_void, descriptors: *mut c_void, ocvrs_return: *mut Result_void);
	// compute(cv::InputArray, cv::OutputArray, std::vector<std::vector<int>> *) /usr/include/opencv2/features2d.hpp:1508
	pub fn cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_const__OutputArrayR_vector_vector_int__X(instance: *mut c_void, keypoint_descriptors: *const c_void, img_descriptor: *const c_void, point_idxs_of_clusters: *mut c_void, ocvrs_return: *mut Result_void);
	// compute2(const cv::Mat &, std::vector<KeyPoint> &, cv::Mat &) /usr/include/opencv2/features2d.hpp:1512
	pub fn cv_BOWImgDescriptorExtractor_compute2_const_MatR_vector_KeyPoint_R_MatR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, img_descriptor: *mut c_void, ocvrs_return: *mut Result_void);
	// descriptorSize() /usr/include/opencv2/features2d.hpp:1517
	pub fn cv_BOWImgDescriptorExtractor_descriptorSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// descriptorType() /usr/include/opencv2/features2d.hpp:1521
	pub fn cv_BOWImgDescriptorExtractor_descriptorType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// BOWKMeansTrainer(int, const cv::TermCriteria &, int, int) /usr/include/opencv2/features2d.hpp:1436
	pub fn cv_BOWKMeansTrainer_BOWKMeansTrainer_int_const_TermCriteriaR_int_int(cluster_count: i32, termcrit: *const core::TermCriteria, attempts: i32, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
	// cluster() /usr/include/opencv2/features2d.hpp:1441
	pub fn cv_BOWKMeansTrainer_cluster_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// cluster(const cv::Mat &) /usr/include/opencv2/features2d.hpp:1442
	pub fn cv_BOWKMeansTrainer_cluster_const_const_MatR(instance: *const c_void, descriptors: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// add(const cv::Mat &) /usr/include/opencv2/features2d.hpp:1396
	pub fn cv_BOWTrainer_add_const_MatR(instance: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result_void);
	// getDescriptors() /usr/include/opencv2/features2d.hpp:1400
	pub fn cv_BOWTrainer_getDescriptors_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// descriptorsCount() /usr/include/opencv2/features2d.hpp:1404
	pub fn cv_BOWTrainer_descriptorsCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// clear() /usr/include/opencv2/features2d.hpp:1406
	pub fn cv_BOWTrainer_clear(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// cluster() /usr/include/opencv2/features2d.hpp:1409
	pub fn cv_BOWTrainer_cluster_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// cluster(const cv::Mat &) /usr/include/opencv2/features2d.hpp:1420
	pub fn cv_BOWTrainer_cluster_const_const_MatR(instance: *const c_void, descriptors: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, float) /usr/include/opencv2/features2d.hpp:333
	pub fn cv_BRISK_create_int_int_float(thresh: i32, octaves: i32, pattern_scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// create(const std::vector<float> &, const std::vector<int> &, float, float, const std::vector<int> &) /usr/include/opencv2/features2d.hpp:346
	pub fn cv_BRISK_create_const_vector_float_R_const_vector_int_R_float_float_const_vector_int_R(radius_list: *const c_void, number_list: *const c_void, d_max: f32, d_min: f32, index_change: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, const std::vector<float> &, const std::vector<int> &, float, float, const std::vector<int> &) /usr/include/opencv2/features2d.hpp:362
	pub fn cv_BRISK_create_int_int_const_vector_float_R_const_vector_int_R_float_float_const_vector_int_R(thresh: i32, octaves: i32, radius_list: *const c_void, number_list: *const c_void, d_max: f32, d_min: f32, index_change: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getDefaultName() /usr/include/opencv2/features2d.hpp:365
	pub fn cv_BRISK_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setThreshold(int) /usr/include/opencv2/features2d.hpp:370
	pub fn cv_BRISK_setThreshold_int(instance: *mut c_void, threshold: i32, ocvrs_return: *mut Result_void);
	// getThreshold() /usr/include/opencv2/features2d.hpp:371
	pub fn cv_BRISK_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setOctaves(int) /usr/include/opencv2/features2d.hpp:376
	pub fn cv_BRISK_setOctaves_int(instance: *mut c_void, octaves: i32, ocvrs_return: *mut Result_void);
	// getOctaves() /usr/include/opencv2/features2d.hpp:377
	pub fn cv_BRISK_getOctaves_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// add(cv::InputArrayOfArrays) /usr/include/opencv2/features2d.hpp:956
	pub fn cv_DescriptorMatcher_add_const__InputArrayR(instance: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result_void);
	// getTrainDescriptors() /usr/include/opencv2/features2d.hpp:960
	pub fn cv_DescriptorMatcher_getTrainDescriptors_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// clear() /usr/include/opencv2/features2d.hpp:964
	pub fn cv_DescriptorMatcher_clear(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// empty() /usr/include/opencv2/features2d.hpp:968
	pub fn cv_DescriptorMatcher_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isMaskSupported() /usr/include/opencv2/features2d.hpp:972
	pub fn cv_DescriptorMatcher_isMaskSupported_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// train() /usr/include/opencv2/features2d.hpp:981
	pub fn cv_DescriptorMatcher_train(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// match(cv::InputArray, cv::InputArray, std::vector<DMatch> &, cv::InputArray) /usr/include/opencv2/features2d.hpp:999
	pub fn cv_DescriptorMatcher_match_const_const__InputArrayR_const__InputArrayR_vector_DMatch_R_const__InputArrayR(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// knnMatch(cv::InputArray, cv::InputArray, std::vector<std::vector<DMatch>> &, int, cv::InputArray, bool) /usr/include/opencv2/features2d.hpp:1020
	pub fn cv_DescriptorMatcher_knnMatch_const_const__InputArrayR_const__InputArrayR_vector_vector_DMatch__R_int_const__InputArrayR_bool(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, k: i32, mask: *const c_void, compact_result: bool, ocvrs_return: *mut Result_void);
	// radiusMatch(cv::InputArray, cv::InputArray, std::vector<std::vector<DMatch>> &, float, cv::InputArray, bool) /usr/include/opencv2/features2d.hpp:1043
	pub fn cv_DescriptorMatcher_radiusMatch_const_const__InputArrayR_const__InputArrayR_vector_vector_DMatch__R_float_const__InputArrayR_bool(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, max_distance: f32, mask: *const c_void, compact_result: bool, ocvrs_return: *mut Result_void);
	// match(cv::InputArray, std::vector<DMatch> &, cv::InputArrayOfArrays) /usr/include/opencv2/features2d.hpp:1054
	pub fn cv_DescriptorMatcher_match_const__InputArrayR_vector_DMatch_R_const__InputArrayR(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, masks: *const c_void, ocvrs_return: *mut Result_void);
	// knnMatch(cv::InputArray, std::vector<std::vector<DMatch>> &, int, cv::InputArrayOfArrays, bool) /usr/include/opencv2/features2d.hpp:1067
	pub fn cv_DescriptorMatcher_knnMatch_const__InputArrayR_vector_vector_DMatch__R_int_const__InputArrayR_bool(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, k: i32, masks: *const c_void, compact_result: bool, ocvrs_return: *mut Result_void);
	// radiusMatch(cv::InputArray, std::vector<std::vector<DMatch>> &, float, cv::InputArrayOfArrays, bool) /usr/include/opencv2/features2d.hpp:1081
	pub fn cv_DescriptorMatcher_radiusMatch_const__InputArrayR_vector_vector_DMatch__R_float_const__InputArrayR_bool(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, max_distance: f32, masks: *const c_void, compact_result: bool, ocvrs_return: *mut Result_void);
	// write(const cv::String &) /usr/include/opencv2/features2d.hpp:1085
	pub fn cv_DescriptorMatcher_write_const_const_StringR(instance: *const c_void, file_name: *const c_char, ocvrs_return: *mut Result_void);
	// read(const cv::String &) /usr/include/opencv2/features2d.hpp:1091
	pub fn cv_DescriptorMatcher_read_const_StringR(instance: *mut c_void, file_name: *const c_char, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &) /usr/include/opencv2/features2d.hpp:1098
	pub fn cv_DescriptorMatcher_read_const_FileNodeR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/features2d.hpp:1100
	pub fn cv_DescriptorMatcher_write_const_FileStorageR(instance: *const c_void, unnamed: *mut c_void, ocvrs_return: *mut Result_void);
	// clone(bool) /usr/include/opencv2/features2d.hpp:1108
	pub fn cv_DescriptorMatcher_clone_const_bool(instance: *const c_void, empty_train_data: bool, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::String &) /usr/include/opencv2/features2d.hpp:1121
	pub fn cv_DescriptorMatcher_create_const_StringR(descriptor_matcher_type: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// create(const DescriptorMatcher::MatcherType &) /usr/include/opencv2/features2d.hpp:1123
	pub fn cv_DescriptorMatcher_create_const_MatcherTypeR(matcher_type: *const crate::features2d::DescriptorMatcher_MatcherType, ocvrs_return: *mut Result<*mut c_void>);
	// write(const Ptr<cv::FileStorage> &, const cv::String &) /usr/include/opencv2/features2d.hpp:1127
	pub fn cv_DescriptorMatcher_write_const_const_Ptr_FileStorage_R_const_StringR(instance: *const c_void, fs: *const c_void, name: *const c_char, ocvrs_return: *mut Result_void);
	// create(int, bool, FastFeatureDetector::DetectorType) /usr/include/opencv2/features2d.hpp:535
	pub fn cv_FastFeatureDetector_create_int_bool_DetectorType(threshold: i32, nonmax_suppression: bool, typ: crate::features2d::FastFeatureDetector_DetectorType, ocvrs_return: *mut Result<*mut c_void>);
	// setThreshold(int) /usr/include/opencv2/features2d.hpp:539
	pub fn cv_FastFeatureDetector_setThreshold_int(instance: *mut c_void, threshold: i32, ocvrs_return: *mut Result_void);
	// getThreshold() /usr/include/opencv2/features2d.hpp:540
	pub fn cv_FastFeatureDetector_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setNonmaxSuppression(bool) /usr/include/opencv2/features2d.hpp:542
	pub fn cv_FastFeatureDetector_setNonmaxSuppression_bool(instance: *mut c_void, f: bool, ocvrs_return: *mut Result_void);
	// getNonmaxSuppression() /usr/include/opencv2/features2d.hpp:543
	pub fn cv_FastFeatureDetector_getNonmaxSuppression_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setType(FastFeatureDetector::DetectorType) /usr/include/opencv2/features2d.hpp:545
	pub fn cv_FastFeatureDetector_setType_DetectorType(instance: *mut c_void, typ: crate::features2d::FastFeatureDetector_DetectorType, ocvrs_return: *mut Result_void);
	// getType() /usr/include/opencv2/features2d.hpp:546
	pub fn cv_FastFeatureDetector_getType_const(instance: *const c_void, ocvrs_return: *mut Result<crate::features2d::FastFeatureDetector_DetectorType>);
	// getDefaultName() /usr/include/opencv2/features2d.hpp:547
	pub fn cv_FastFeatureDetector_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// detect(cv::InputArray, std::vector<KeyPoint> &, cv::InputArray) /usr/include/opencv2/features2d.hpp:147
	pub fn cv_Feature2D_detect_const__InputArrayR_vector_KeyPoint_R_const__InputArrayR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// detect(cv::InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, cv::InputArrayOfArrays) /usr/include/opencv2/features2d.hpp:158
	pub fn cv_Feature2D_detect_const__InputArrayR_vector_vector_KeyPoint__R_const__InputArrayR(instance: *mut c_void, images: *const c_void, keypoints: *mut c_void, masks: *const c_void, ocvrs_return: *mut Result_void);
	// compute(cv::InputArray, std::vector<KeyPoint> &, cv::OutputArray) /usr/include/opencv2/features2d.hpp:173
	pub fn cv_Feature2D_compute_const__InputArrayR_vector_KeyPoint_R_const__OutputArrayR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result_void);
	// compute(cv::InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, cv::OutputArrayOfArrays) /usr/include/opencv2/features2d.hpp:187
	pub fn cv_Feature2D_compute_const__InputArrayR_vector_vector_KeyPoint__R_const__OutputArrayR(instance: *mut c_void, images: *const c_void, keypoints: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result_void);
	// detectAndCompute(cv::InputArray, cv::InputArray, std::vector<KeyPoint> &, cv::OutputArray, bool) /usr/include/opencv2/features2d.hpp:192
	pub fn cv_Feature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vector_KeyPoint_R_const__OutputArrayR_bool(instance: *mut c_void, image: *const c_void, mask: *const c_void, keypoints: *mut c_void, descriptors: *const c_void, use_provided_keypoints: bool, ocvrs_return: *mut Result_void);
	// descriptorSize() /usr/include/opencv2/features2d.hpp:197
	pub fn cv_Feature2D_descriptorSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// descriptorType() /usr/include/opencv2/features2d.hpp:198
	pub fn cv_Feature2D_descriptorType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// defaultNorm() /usr/include/opencv2/features2d.hpp:199
	pub fn cv_Feature2D_defaultNorm_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// write(const cv::String &) /usr/include/opencv2/features2d.hpp:201
	pub fn cv_Feature2D_write_const_const_StringR(instance: *const c_void, file_name: *const c_char, ocvrs_return: *mut Result_void);
	// read(const cv::String &) /usr/include/opencv2/features2d.hpp:203
	pub fn cv_Feature2D_read_const_StringR(instance: *mut c_void, file_name: *const c_char, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/features2d.hpp:205
	pub fn cv_Feature2D_write_const_FileStorageR(instance: *const c_void, unnamed: *mut c_void, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &) /usr/include/opencv2/features2d.hpp:208
	pub fn cv_Feature2D_read_const_FileNodeR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result_void);
	// empty() /usr/include/opencv2/features2d.hpp:211
	pub fn cv_Feature2D_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// getDefaultName() /usr/include/opencv2/features2d.hpp:212
	pub fn cv_Feature2D_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// write(const Ptr<cv::FileStorage> &, const cv::String &) /usr/include/opencv2/features2d.hpp:215
	pub fn cv_Feature2D_write_const_const_Ptr_FileStorage_R_const_StringR(instance: *const c_void, fs: *const c_void, name: *const c_char, ocvrs_return: *mut Result_void);
	// FlannBasedMatcher(const Ptr<flann::IndexParams> &, const Ptr<flann::SearchParams> &) /usr/include/opencv2/features2d.hpp:1232
	pub fn cv_FlannBasedMatcher_FlannBasedMatcher_const_Ptr_IndexParams_R_const_Ptr_SearchParams_R(index_params: *const c_void, search_params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// add(cv::InputArrayOfArrays) /usr/include/opencv2/features2d.hpp:1235
	pub fn cv_FlannBasedMatcher_add_const__InputArrayR(instance: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result_void);
	// clear() /usr/include/opencv2/features2d.hpp:1236
	pub fn cv_FlannBasedMatcher_clear(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &) /usr/include/opencv2/features2d.hpp:1239
	pub fn cv_FlannBasedMatcher_read_const_FileNodeR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/features2d.hpp:1241
	pub fn cv_FlannBasedMatcher_write_const_FileStorageR(instance: *const c_void, unnamed: *mut c_void, ocvrs_return: *mut Result_void);
	// train() /usr/include/opencv2/features2d.hpp:1243
	pub fn cv_FlannBasedMatcher_train(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// isMaskSupported() /usr/include/opencv2/features2d.hpp:1244
	pub fn cv_FlannBasedMatcher_isMaskSupported_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// create() /usr/include/opencv2/features2d.hpp:1246
	pub fn cv_FlannBasedMatcher_create(ocvrs_return: *mut Result<*mut c_void>);
	// clone(bool) /usr/include/opencv2/features2d.hpp:1248
	pub fn cv_FlannBasedMatcher_clone_const_bool(instance: *const c_void, empty_train_data: bool, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, double, double, int, bool, double) /usr/include/opencv2/features2d.hpp:640
	pub fn cv_GFTTDetector_create_int_double_double_int_bool_double(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, use_harris_detector: bool, k: f64, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, double, double, int, int, bool, double) /usr/include/opencv2/features2d.hpp:642
	pub fn cv_GFTTDetector_create_int_double_double_int_int_bool_double(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, gradiant_size: i32, use_harris_detector: bool, k: f64, ocvrs_return: *mut Result<*mut c_void>);
	// setMaxFeatures(int) /usr/include/opencv2/features2d.hpp:644
	pub fn cv_GFTTDetector_setMaxFeatures_int(instance: *mut c_void, max_features: i32, ocvrs_return: *mut Result_void);
	// getMaxFeatures() /usr/include/opencv2/features2d.hpp:645
	pub fn cv_GFTTDetector_getMaxFeatures_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setQualityLevel(double) /usr/include/opencv2/features2d.hpp:647
	pub fn cv_GFTTDetector_setQualityLevel_double(instance: *mut c_void, qlevel: f64, ocvrs_return: *mut Result_void);
	// getQualityLevel() /usr/include/opencv2/features2d.hpp:648
	pub fn cv_GFTTDetector_getQualityLevel_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMinDistance(double) /usr/include/opencv2/features2d.hpp:650
	pub fn cv_GFTTDetector_setMinDistance_double(instance: *mut c_void, min_distance: f64, ocvrs_return: *mut Result_void);
	// getMinDistance() /usr/include/opencv2/features2d.hpp:651
	pub fn cv_GFTTDetector_getMinDistance_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setBlockSize(int) /usr/include/opencv2/features2d.hpp:653
	pub fn cv_GFTTDetector_setBlockSize_int(instance: *mut c_void, block_size: i32, ocvrs_return: *mut Result_void);
	// getBlockSize() /usr/include/opencv2/features2d.hpp:654
	pub fn cv_GFTTDetector_getBlockSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setHarrisDetector(bool) /usr/include/opencv2/features2d.hpp:656
	pub fn cv_GFTTDetector_setHarrisDetector_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// getHarrisDetector() /usr/include/opencv2/features2d.hpp:657
	pub fn cv_GFTTDetector_getHarrisDetector_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setK(double) /usr/include/opencv2/features2d.hpp:659
	pub fn cv_GFTTDetector_setK_double(instance: *mut c_void, k: f64, ocvrs_return: *mut Result_void);
	// getK() /usr/include/opencv2/features2d.hpp:660
	pub fn cv_GFTTDetector_getK_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// getDefaultName() /usr/include/opencv2/features2d.hpp:661
	pub fn cv_GFTTDetector_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(bool, bool, float, int, int, KAZE::DiffusivityType) /usr/include/opencv2/features2d.hpp:763
	pub fn cv_KAZE_create_bool_bool_float_int_int_DiffusivityType(extended: bool, upright: bool, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: crate::features2d::KAZE_DiffusivityType, ocvrs_return: *mut Result<*mut c_void>);
	// setExtended(bool) /usr/include/opencv2/features2d.hpp:768
	pub fn cv_KAZE_setExtended_bool(instance: *mut c_void, extended: bool, ocvrs_return: *mut Result_void);
	// getExtended() /usr/include/opencv2/features2d.hpp:769
	pub fn cv_KAZE_getExtended_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUpright(bool) /usr/include/opencv2/features2d.hpp:771
	pub fn cv_KAZE_setUpright_bool(instance: *mut c_void, upright: bool, ocvrs_return: *mut Result_void);
	// getUpright() /usr/include/opencv2/features2d.hpp:772
	pub fn cv_KAZE_getUpright_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setThreshold(double) /usr/include/opencv2/features2d.hpp:774
	pub fn cv_KAZE_setThreshold_double(instance: *mut c_void, threshold: f64, ocvrs_return: *mut Result_void);
	// getThreshold() /usr/include/opencv2/features2d.hpp:775
	pub fn cv_KAZE_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setNOctaves(int) /usr/include/opencv2/features2d.hpp:777
	pub fn cv_KAZE_setNOctaves_int(instance: *mut c_void, octaves: i32, ocvrs_return: *mut Result_void);
	// getNOctaves() /usr/include/opencv2/features2d.hpp:778
	pub fn cv_KAZE_getNOctaves_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setNOctaveLayers(int) /usr/include/opencv2/features2d.hpp:780
	pub fn cv_KAZE_setNOctaveLayers_int(instance: *mut c_void, octave_layers: i32, ocvrs_return: *mut Result_void);
	// getNOctaveLayers() /usr/include/opencv2/features2d.hpp:781
	pub fn cv_KAZE_getNOctaveLayers_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setDiffusivity(KAZE::DiffusivityType) /usr/include/opencv2/features2d.hpp:783
	pub fn cv_KAZE_setDiffusivity_DiffusivityType(instance: *mut c_void, diff: crate::features2d::KAZE_DiffusivityType, ocvrs_return: *mut Result_void);
	// getDiffusivity() /usr/include/opencv2/features2d.hpp:784
	pub fn cv_KAZE_getDiffusivity_const(instance: *const c_void, ocvrs_return: *mut Result<crate::features2d::KAZE_DiffusivityType>);
	// getDefaultName() /usr/include/opencv2/features2d.hpp:785
	pub fn cv_KAZE_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// KeyPointsFilter() /usr/include/opencv2/features2d.hpp:95
	pub fn cv_KeyPointsFilter_KeyPointsFilter(ocvrs_return: *mut Result<*mut c_void>);
	// runByImageBorder(std::vector<KeyPoint> &, cv::Size, int) /usr/include/opencv2/features2d.hpp:100
	pub fn cv_KeyPointsFilter_runByImageBorder_vector_KeyPoint_R_Size_int(keypoints: *mut c_void, image_size: *const core::Size, border_size: i32, ocvrs_return: *mut Result_void);
	// runByKeypointSize(std::vector<KeyPoint> &, float, float) /usr/include/opencv2/features2d.hpp:104
	pub fn cv_KeyPointsFilter_runByKeypointSize_vector_KeyPoint_R_float_float(keypoints: *mut c_void, min_size: f32, max_size: f32, ocvrs_return: *mut Result_void);
	// runByPixelsMask(std::vector<KeyPoint> &, const cv::Mat &) /usr/include/opencv2/features2d.hpp:109
	pub fn cv_KeyPointsFilter_runByPixelsMask_vector_KeyPoint_R_const_MatR(keypoints: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// removeDuplicated(std::vector<KeyPoint> &) /usr/include/opencv2/features2d.hpp:113
	pub fn cv_KeyPointsFilter_removeDuplicated_vector_KeyPoint_R(keypoints: *mut c_void, ocvrs_return: *mut Result_void);
	// removeDuplicatedSorted(std::vector<KeyPoint> &) /usr/include/opencv2/features2d.hpp:117
	pub fn cv_KeyPointsFilter_removeDuplicatedSorted_vector_KeyPoint_R(keypoints: *mut c_void, ocvrs_return: *mut Result_void);
	// retainBest(std::vector<KeyPoint> &, int) /usr/include/opencv2/features2d.hpp:122
	pub fn cv_KeyPointsFilter_retainBest_vector_KeyPoint_R_int(keypoints: *mut c_void, npoints: i32, ocvrs_return: *mut Result_void);
	// create(int, int, int, double, double, int, double, double, int) /usr/include/opencv2/features2d.hpp:486
	pub fn cv_MSER_create_int_int_int_double_double_int_double_double_int(delta: i32, min_area: i32, max_area: i32, max_variation: f64, min_diversity: f64, max_evolution: i32, area_threshold: f64, min_margin: f64, edge_blur_size: i32, ocvrs_return: *mut Result<*mut c_void>);
	// detectRegions(cv::InputArray, std::vector<std::vector<Point>> &, std::vector<Rect> &) /usr/include/opencv2/features2d.hpp:497
	pub fn cv_MSER_detectRegions_const__InputArrayR_vector_vector_Point__R_vector_Rect_R(instance: *mut c_void, image: *const c_void, msers: *mut c_void, bboxes: *mut c_void, ocvrs_return: *mut Result_void);
	// setDelta(int) /usr/include/opencv2/features2d.hpp:501
	pub fn cv_MSER_setDelta_int(instance: *mut c_void, delta: i32, ocvrs_return: *mut Result_void);
	// getDelta() /usr/include/opencv2/features2d.hpp:502
	pub fn cv_MSER_getDelta_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMinArea(int) /usr/include/opencv2/features2d.hpp:504
	pub fn cv_MSER_setMinArea_int(instance: *mut c_void, min_area: i32, ocvrs_return: *mut Result_void);
	// getMinArea() /usr/include/opencv2/features2d.hpp:505
	pub fn cv_MSER_getMinArea_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMaxArea(int) /usr/include/opencv2/features2d.hpp:507
	pub fn cv_MSER_setMaxArea_int(instance: *mut c_void, max_area: i32, ocvrs_return: *mut Result_void);
	// getMaxArea() /usr/include/opencv2/features2d.hpp:508
	pub fn cv_MSER_getMaxArea_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setPass2Only(bool) /usr/include/opencv2/features2d.hpp:510
	pub fn cv_MSER_setPass2Only_bool(instance: *mut c_void, f: bool, ocvrs_return: *mut Result_void);
	// getPass2Only() /usr/include/opencv2/features2d.hpp:511
	pub fn cv_MSER_getPass2Only_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// getDefaultName() /usr/include/opencv2/features2d.hpp:512
	pub fn cv_MSER_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, float, int, int, int, int, ORB::ScoreType, int, int) /usr/include/opencv2/features2d.hpp:424
	pub fn cv_ORB_create_int_float_int_int_int_int_ScoreType_int_int(nfeatures: i32, scale_factor: f32, nlevels: i32, edge_threshold: i32, first_level: i32, wta_k: i32, score_type: crate::features2d::ORB_ScoreType, patch_size: i32, fast_threshold: i32, ocvrs_return: *mut Result<*mut c_void>);
	// setMaxFeatures(int) /usr/include/opencv2/features2d.hpp:427
	pub fn cv_ORB_setMaxFeatures_int(instance: *mut c_void, max_features: i32, ocvrs_return: *mut Result_void);
	// getMaxFeatures() /usr/include/opencv2/features2d.hpp:428
	pub fn cv_ORB_getMaxFeatures_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setScaleFactor(double) /usr/include/opencv2/features2d.hpp:430
	pub fn cv_ORB_setScaleFactor_double(instance: *mut c_void, scale_factor: f64, ocvrs_return: *mut Result_void);
	// getScaleFactor() /usr/include/opencv2/features2d.hpp:431
	pub fn cv_ORB_getScaleFactor_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setNLevels(int) /usr/include/opencv2/features2d.hpp:433
	pub fn cv_ORB_setNLevels_int(instance: *mut c_void, nlevels: i32, ocvrs_return: *mut Result_void);
	// getNLevels() /usr/include/opencv2/features2d.hpp:434
	pub fn cv_ORB_getNLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setEdgeThreshold(int) /usr/include/opencv2/features2d.hpp:436
	pub fn cv_ORB_setEdgeThreshold_int(instance: *mut c_void, edge_threshold: i32, ocvrs_return: *mut Result_void);
	// getEdgeThreshold() /usr/include/opencv2/features2d.hpp:437
	pub fn cv_ORB_getEdgeThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setFirstLevel(int) /usr/include/opencv2/features2d.hpp:439
	pub fn cv_ORB_setFirstLevel_int(instance: *mut c_void, first_level: i32, ocvrs_return: *mut Result_void);
	// getFirstLevel() /usr/include/opencv2/features2d.hpp:440
	pub fn cv_ORB_getFirstLevel_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setWTA_K(int) /usr/include/opencv2/features2d.hpp:442
	pub fn cv_ORB_setWTA_K_int(instance: *mut c_void, wta_k: i32, ocvrs_return: *mut Result_void);
	// getWTA_K() /usr/include/opencv2/features2d.hpp:443
	pub fn cv_ORB_getWTA_K_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setScoreType(ORB::ScoreType) /usr/include/opencv2/features2d.hpp:445
	pub fn cv_ORB_setScoreType_ScoreType(instance: *mut c_void, score_type: crate::features2d::ORB_ScoreType, ocvrs_return: *mut Result_void);
	// getScoreType() /usr/include/opencv2/features2d.hpp:446
	pub fn cv_ORB_getScoreType_const(instance: *const c_void, ocvrs_return: *mut Result<crate::features2d::ORB_ScoreType>);
	// setPatchSize(int) /usr/include/opencv2/features2d.hpp:448
	pub fn cv_ORB_setPatchSize_int(instance: *mut c_void, patch_size: i32, ocvrs_return: *mut Result_void);
	// getPatchSize() /usr/include/opencv2/features2d.hpp:449
	pub fn cv_ORB_getPatchSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setFastThreshold(int) /usr/include/opencv2/features2d.hpp:451
	pub fn cv_ORB_setFastThreshold_int(instance: *mut c_void, fast_threshold: i32, ocvrs_return: *mut Result_void);
	// getFastThreshold() /usr/include/opencv2/features2d.hpp:452
	pub fn cv_ORB_getFastThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getDefaultName() /usr/include/opencv2/features2d.hpp:453
	pub fn cv_ORB_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, double, double, double) /usr/include/opencv2/features2d.hpp:283
	pub fn cv_SIFT_create_int_int_double_double_double(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, double, double, double, int) /usr/include/opencv2/features2d.hpp:310
	pub fn cv_SIFT_create_int_int_double_double_double_int(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64, descriptor_type: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getDefaultName() /usr/include/opencv2/features2d.hpp:314
	pub fn cv_SIFT_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const SimpleBlobDetector::Params &) /usr/include/opencv2/features2d.hpp:727
	pub fn cv_SimpleBlobDetector_create_const_ParamsR(parameters: *const crate::features2d::SimpleBlobDetector_Params, ocvrs_return: *mut Result<*mut c_void>);
	// getDefaultName() /usr/include/opencv2/features2d.hpp:728
	pub fn cv_SimpleBlobDetector_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Params() /usr/include/opencv2/features2d.hpp:700
	pub fn cv_SimpleBlobDetector_Params_Params(ocvrs_return: *mut Result<crate::features2d::SimpleBlobDetector_Params>);
	// read(const cv::FileNode &) /usr/include/opencv2/features2d.hpp:722
	pub fn cv_SimpleBlobDetector_Params_read_const_FileNodeR(instance: *const crate::features2d::SimpleBlobDetector_Params, fn_: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/features2d.hpp:723
	pub fn cv_SimpleBlobDetector_Params_write_const_FileStorageR(instance: *const crate::features2d::SimpleBlobDetector_Params, fs: *mut c_void, ocvrs_return: *mut Result_void);
}
