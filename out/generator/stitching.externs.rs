extern "C" {
	// autoDetectWaveCorrectKind(const std::vector<Mat> &) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:342
	pub fn cv_detail_autoDetectWaveCorrectKind_const_vector_Mat_R(rmats: *const c_void, ocvrs_return: *mut Result<crate::stitching::Detail_WaveCorrectKind>);
	// computeImageFeatures(const Ptr<cv::Feature2D> &, cv::InputArray, cv::detail::ImageFeatures &, cv::InputArray) /usr/include/opencv2/stitching/detail/matchers.hpp:86
	pub fn cv_detail_computeImageFeatures_const_Ptr_Feature2D_R_const__InputArrayR_ImageFeaturesR_const__InputArrayR(features_finder: *const c_void, image: *const c_void, features: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// computeImageFeatures(const Ptr<cv::Feature2D> &, cv::InputArrayOfArrays, std::vector<ImageFeatures> &, cv::InputArrayOfArrays) /usr/include/opencv2/stitching/detail/matchers.hpp:73
	pub fn cv_detail_computeImageFeatures_const_Ptr_Feature2D_R_const__InputArrayR_vector_ImageFeatures_R_const__InputArrayR(features_finder: *const c_void, images: *const c_void, features: *mut c_void, masks: *const c_void, ocvrs_return: *mut Result_void);
	// createLaplacePyrGpu(cv::InputArray, int, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/blenders.hpp:173
	pub fn cv_detail_createLaplacePyrGpu_const__InputArrayR_int_vector_UMat_R(img: *const c_void, num_levels: i32, pyr: *mut c_void, ocvrs_return: *mut Result_void);
	// createLaplacePyr(cv::InputArray, int, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/blenders.hpp:172
	pub fn cv_detail_createLaplacePyr_const__InputArrayR_int_vector_UMat_R(img: *const c_void, num_levels: i32, pyr: *mut c_void, ocvrs_return: *mut Result_void);
	// createWeightMap(cv::InputArray, float, cv::InputOutputArray) /usr/include/opencv2/stitching/detail/blenders.hpp:170
	pub fn cv_detail_createWeightMap_const__InputArrayR_float_const__InputOutputArrayR(mask: *const c_void, sharpness: f32, weight: *const c_void, ocvrs_return: *mut Result_void);
	// findMaxSpanningTree(int, const std::vector<MatchesInfo> &, cv::detail::Graph &, std::vector<int> &) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:364
	pub fn cv_detail_findMaxSpanningTree_int_const_vector_MatchesInfo_R_GraphR_vector_int_R(num_images: i32, pairwise_matches: *const c_void, span_tree: *mut c_void, centers: *mut c_void, ocvrs_return: *mut Result_void);
	// leaveBiggestComponent(std::vector<ImageFeatures> &, std::vector<MatchesInfo> &, float) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:359
	pub fn cv_detail_leaveBiggestComponent_vector_ImageFeatures_R_vector_MatchesInfo_R_float(features: *mut c_void, pairwise_matches: *mut c_void, conf_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
	// matchesGraphAsString(std::vector<String> &, std::vector<MatchesInfo> &, float) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:356
	pub fn cv_detail_matchesGraphAsString_vector_String_R_vector_MatchesInfo_R_float(pathes: *mut c_void, pairwise_matches: *mut c_void, conf_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
	// normalizeUsingWeightMap(cv::InputArray, cv::InputOutputArray) /usr/include/opencv2/stitching/detail/blenders.hpp:168
	pub fn cv_detail_normalizeUsingWeightMap_const__InputArrayR_const__InputOutputArrayR(weight: *const c_void, src: *const c_void, ocvrs_return: *mut Result_void);
	// overlapRoi(cv::Point, cv::Point, cv::Size, cv::Size, cv::Rect &) /usr/include/opencv2/stitching/detail/util.hpp:103
	pub fn cv_detail_overlapRoi_Point_Point_Size_Size_RectR(tl1: *const core::Point, tl2: *const core::Point, sz1: *const core::Size, sz2: *const core::Size, roi: *mut core::Rect, ocvrs_return: *mut Result<bool>);
	// restoreImageFromLaplacePyrGpu(std::vector<UMat> &) /usr/include/opencv2/stitching/detail/blenders.hpp:177
	pub fn cv_detail_restoreImageFromLaplacePyrGpu_vector_UMat_R(pyr: *mut c_void, ocvrs_return: *mut Result_void);
	// restoreImageFromLaplacePyr(std::vector<UMat> &) /usr/include/opencv2/stitching/detail/blenders.hpp:176
	pub fn cv_detail_restoreImageFromLaplacePyr_vector_UMat_R(pyr: *mut c_void, ocvrs_return: *mut Result_void);
	// resultRoiIntersection(const std::vector<Point> &, const std::vector<Size> &) /usr/include/opencv2/stitching/detail/util.hpp:106
	pub fn cv_detail_resultRoiIntersection_const_vector_Point_R_const_vector_Size_R(corners: *const c_void, sizes: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// resultRoi(const std::vector<Point> &, const std::vector<Size> &) /usr/include/opencv2/stitching/detail/util.hpp:105
	pub fn cv_detail_resultRoi_const_vector_Point_R_const_vector_Size_R(corners: *const c_void, sizes: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// resultRoi(const std::vector<Point> &, const std::vector<UMat> &) /usr/include/opencv2/stitching/detail/util.hpp:104
	pub fn cv_detail_resultRoi_const_vector_Point_R_const_vector_UMat_R(corners: *const c_void, images: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// resultTl(const std::vector<Point> &) /usr/include/opencv2/stitching/detail/util.hpp:107
	pub fn cv_detail_resultTl_const_vector_Point_R(corners: *const c_void, ocvrs_return: *mut Result<core::Point>);
	// selectRandomSubset(int, int, std::vector<int> &) /usr/include/opencv2/stitching/detail/util.hpp:110
	pub fn cv_detail_selectRandomSubset_int_int_vector_int_R(count: i32, size: i32, subset: *mut c_void, ocvrs_return: *mut Result_void);
	// stitchingLogLevel() /usr/include/opencv2/stitching/detail/util.hpp:112
	pub fn cv_detail_stitchingLogLevel(ocvrs_return: *mut Result<i32>);
	// waveCorrect(std::vector<Mat> &, cv::detail::WaveCorrectKind) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:349
	pub fn cv_detail_waveCorrect_vector_Mat_R_WaveCorrectKind(rmats: *mut c_void, kind: crate::stitching::Detail_WaveCorrectKind, ocvrs_return: *mut Result_void);
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:162
	pub fn cv_AffineWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// CompressedRectilinearPortraitWarper(float, float) /usr/include/opencv2/stitching/warpers.hpp:208
	pub fn cv_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float(a: f32, b: f32, ocvrs_return: *mut Result<*mut c_void>);
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:212
	pub fn cv_CompressedRectilinearPortraitWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// CompressedRectilinearWarper(float, float) /usr/include/opencv2/stitching/warpers.hpp:197
	pub fn cv_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float(a: f32, b: f32, ocvrs_return: *mut Result<*mut c_void>);
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:201
	pub fn cv_CompressedRectilinearWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:171
	pub fn cv_CylindricalWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:184
	pub fn cv_FisheyeWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:240
	pub fn cv_MercatorWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// PaniniPortraitWarper(float, float) /usr/include/opencv2/stitching/warpers.hpp:230
	pub fn cv_PaniniPortraitWarper_PaniniPortraitWarper_float_float(a: f32, b: f32, ocvrs_return: *mut Result<*mut c_void>);
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:234
	pub fn cv_PaniniPortraitWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// PaniniWarper(float, float) /usr/include/opencv2/stitching/warpers.hpp:219
	pub fn cv_PaniniWarper_PaniniWarper_float_float(a: f32, b: f32, ocvrs_return: *mut Result<*mut c_void>);
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:223
	pub fn cv_PaniniWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:153
	pub fn cv_PlaneWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// PyRotationWarper(cv::String, float) /usr/include/opencv2/stitching/warpers.hpp:55
	pub fn cv_PyRotationWarper_PyRotationWarper_String_float(typ: *mut c_char, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// PyRotationWarper() /usr/include/opencv2/stitching/warpers.hpp:56
	pub fn cv_PyRotationWarper_PyRotationWarper(ocvrs_return: *mut Result<*mut c_void>);
	// warpPoint(const cv::Point2f &, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/warpers.hpp:66
	pub fn cv_PyRotationWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
	// warpPointBackward(const cv::Point2f &, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/warpers.hpp:76
	pub fn cv_PyRotationWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/warpers.hpp:93
	pub fn cv_PyRotationWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/warpers.hpp:105
	pub fn cv_PyRotationWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
	// warpBackward(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::Size, cv::OutputArray) /usr/include/opencv2/stitching/warpers.hpp:118
	pub fn cv_PyRotationWarper_warpBackward_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_Size_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst_size: *const core::Size, dst: *const c_void, ocvrs_return: *mut Result_void);
	// warpRoi(cv::Size, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/warpers.hpp:127
	pub fn cv_PyRotationWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// getScale() /usr/include/opencv2/stitching/warpers.hpp:129
	pub fn cv_PyRotationWarper_getScale_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setScale(float) /usr/include/opencv2/stitching/warpers.hpp:130
	pub fn cv_PyRotationWarper_setScale_float(instance: *mut c_void, unnamed: f32, ocvrs_return: *mut Result_void);
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:178
	pub fn cv_SphericalWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:190
	pub fn cv_StereographicWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// create(cv::Stitcher::Mode) /usr/include/opencv2/stitching.hpp:184
	pub fn cv_Stitcher_create_Mode(mode: crate::stitching::Stitcher_Mode, ocvrs_return: *mut Result<*mut c_void>);
	// registrationResol() /usr/include/opencv2/stitching.hpp:186
	pub fn cv_Stitcher_registrationResol_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setRegistrationResol(double) /usr/include/opencv2/stitching.hpp:187
	pub fn cv_Stitcher_setRegistrationResol_double(instance: *mut c_void, resol_mpx: f64, ocvrs_return: *mut Result_void);
	// seamEstimationResol() /usr/include/opencv2/stitching.hpp:189
	pub fn cv_Stitcher_seamEstimationResol_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setSeamEstimationResol(double) /usr/include/opencv2/stitching.hpp:190
	pub fn cv_Stitcher_setSeamEstimationResol_double(instance: *mut c_void, resol_mpx: f64, ocvrs_return: *mut Result_void);
	// compositingResol() /usr/include/opencv2/stitching.hpp:192
	pub fn cv_Stitcher_compositingResol_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setCompositingResol(double) /usr/include/opencv2/stitching.hpp:193
	pub fn cv_Stitcher_setCompositingResol_double(instance: *mut c_void, resol_mpx: f64, ocvrs_return: *mut Result_void);
	// panoConfidenceThresh() /usr/include/opencv2/stitching.hpp:195
	pub fn cv_Stitcher_panoConfidenceThresh_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setPanoConfidenceThresh(double) /usr/include/opencv2/stitching.hpp:196
	pub fn cv_Stitcher_setPanoConfidenceThresh_double(instance: *mut c_void, conf_thresh: f64, ocvrs_return: *mut Result_void);
	// waveCorrection() /usr/include/opencv2/stitching.hpp:198
	pub fn cv_Stitcher_waveCorrection_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setWaveCorrection(bool) /usr/include/opencv2/stitching.hpp:199
	pub fn cv_Stitcher_setWaveCorrection_bool(instance: *mut c_void, flag: bool, ocvrs_return: *mut Result_void);
	// interpolationFlags() /usr/include/opencv2/stitching.hpp:201
	pub fn cv_Stitcher_interpolationFlags_const(instance: *const c_void, ocvrs_return: *mut Result<crate::imgproc::InterpolationFlags>);
	// setInterpolationFlags(cv::InterpolationFlags) /usr/include/opencv2/stitching.hpp:202
	pub fn cv_Stitcher_setInterpolationFlags_InterpolationFlags(instance: *mut c_void, interp_flags: crate::imgproc::InterpolationFlags, ocvrs_return: *mut Result_void);
	// waveCorrectKind() /usr/include/opencv2/stitching.hpp:204
	pub fn cv_Stitcher_waveCorrectKind_const(instance: *const c_void, ocvrs_return: *mut Result<crate::stitching::Detail_WaveCorrectKind>);
	// setWaveCorrectKind(detail::WaveCorrectKind) /usr/include/opencv2/stitching.hpp:205
	pub fn cv_Stitcher_setWaveCorrectKind_WaveCorrectKind(instance: *mut c_void, kind: crate::stitching::Detail_WaveCorrectKind, ocvrs_return: *mut Result_void);
	// featuresFinder() /usr/include/opencv2/stitching.hpp:207
	pub fn cv_Stitcher_featuresFinder(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// featuresFinder() /usr/include/opencv2/stitching.hpp:208
	pub fn cv_Stitcher_featuresFinder_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setFeaturesFinder(Ptr<cv::Feature2D>) /usr/include/opencv2/stitching.hpp:209
	pub fn cv_Stitcher_setFeaturesFinder_Ptr_Feature2D_(instance: *mut c_void, features_finder: *mut c_void, ocvrs_return: *mut Result_void);
	// featuresMatcher() /usr/include/opencv2/stitching.hpp:212
	pub fn cv_Stitcher_featuresMatcher(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// featuresMatcher() /usr/include/opencv2/stitching.hpp:213
	pub fn cv_Stitcher_featuresMatcher_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setFeaturesMatcher(Ptr<detail::FeaturesMatcher>) /usr/include/opencv2/stitching.hpp:214
	pub fn cv_Stitcher_setFeaturesMatcher_Ptr_FeaturesMatcher_(instance: *mut c_void, features_matcher: *mut c_void, ocvrs_return: *mut Result_void);
	// matchingMask() /usr/include/opencv2/stitching.hpp:217
	pub fn cv_Stitcher_matchingMask_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setMatchingMask(const cv::UMat &) /usr/include/opencv2/stitching.hpp:218
	pub fn cv_Stitcher_setMatchingMask_const_UMatR(instance: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// bundleAdjuster() /usr/include/opencv2/stitching.hpp:224
	pub fn cv_Stitcher_bundleAdjuster(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// bundleAdjuster() /usr/include/opencv2/stitching.hpp:225
	pub fn cv_Stitcher_bundleAdjuster_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setBundleAdjuster(Ptr<detail::BundleAdjusterBase>) /usr/include/opencv2/stitching.hpp:226
	pub fn cv_Stitcher_setBundleAdjuster_Ptr_BundleAdjusterBase_(instance: *mut c_void, bundle_adjuster: *mut c_void, ocvrs_return: *mut Result_void);
	// estimator() /usr/include/opencv2/stitching.hpp:229
	pub fn cv_Stitcher_estimator(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// estimator() /usr/include/opencv2/stitching.hpp:230
	pub fn cv_Stitcher_estimator_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setEstimator(Ptr<detail::Estimator>) /usr/include/opencv2/stitching.hpp:231
	pub fn cv_Stitcher_setEstimator_Ptr_Estimator_(instance: *mut c_void, estimator: *mut c_void, ocvrs_return: *mut Result_void);
	// warper() /usr/include/opencv2/stitching.hpp:234
	pub fn cv_Stitcher_warper(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// warper() /usr/include/opencv2/stitching.hpp:235
	pub fn cv_Stitcher_warper_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setWarper(Ptr<cv::WarperCreator>) /usr/include/opencv2/stitching.hpp:236
	pub fn cv_Stitcher_setWarper_Ptr_WarperCreator_(instance: *mut c_void, creator: *mut c_void, ocvrs_return: *mut Result_void);
	// exposureCompensator() /usr/include/opencv2/stitching.hpp:238
	pub fn cv_Stitcher_exposureCompensator(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// exposureCompensator() /usr/include/opencv2/stitching.hpp:239
	pub fn cv_Stitcher_exposureCompensator_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setExposureCompensator(Ptr<detail::ExposureCompensator>) /usr/include/opencv2/stitching.hpp:240
	pub fn cv_Stitcher_setExposureCompensator_Ptr_ExposureCompensator_(instance: *mut c_void, exposure_comp: *mut c_void, ocvrs_return: *mut Result_void);
	// seamFinder() /usr/include/opencv2/stitching.hpp:243
	pub fn cv_Stitcher_seamFinder(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// seamFinder() /usr/include/opencv2/stitching.hpp:244
	pub fn cv_Stitcher_seamFinder_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setSeamFinder(Ptr<detail::SeamFinder>) /usr/include/opencv2/stitching.hpp:245
	pub fn cv_Stitcher_setSeamFinder_Ptr_SeamFinder_(instance: *mut c_void, seam_finder: *mut c_void, ocvrs_return: *mut Result_void);
	// blender() /usr/include/opencv2/stitching.hpp:247
	pub fn cv_Stitcher_blender(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// blender() /usr/include/opencv2/stitching.hpp:248
	pub fn cv_Stitcher_blender_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setBlender(Ptr<detail::Blender>) /usr/include/opencv2/stitching.hpp:249
	pub fn cv_Stitcher_setBlender_Ptr_Blender_(instance: *mut c_void, b: *mut c_void, ocvrs_return: *mut Result_void);
	// estimateTransform(cv::InputArrayOfArrays, cv::InputArrayOfArrays) /usr/include/opencv2/stitching.hpp:260
	pub fn cv_Stitcher_estimateTransform_const__InputArrayR_const__InputArrayR(instance: *mut c_void, images: *const c_void, masks: *const c_void, ocvrs_return: *mut Result<crate::stitching::Stitcher_Status>);
	// setTransform(cv::InputArrayOfArrays, const std::vector<detail::CameraParams> &, const std::vector<int> &) /usr/include/opencv2/stitching.hpp:270
	pub fn cv_Stitcher_setTransform_const__InputArrayR_const_vector_CameraParams_R_const_vector_int_R(instance: *mut c_void, images: *const c_void, cameras: *const c_void, component: *const c_void, ocvrs_return: *mut Result<crate::stitching::Stitcher_Status>);
	// setTransform(cv::InputArrayOfArrays, const std::vector<detail::CameraParams> &) /usr/include/opencv2/stitching.hpp:274
	pub fn cv_Stitcher_setTransform_const__InputArrayR_const_vector_CameraParams_R(instance: *mut c_void, images: *const c_void, cameras: *const c_void, ocvrs_return: *mut Result<crate::stitching::Stitcher_Status>);
	// composePanorama(cv::OutputArray) /usr/include/opencv2/stitching.hpp:277
	pub fn cv_Stitcher_composePanorama_const__OutputArrayR(instance: *mut c_void, pano: *const c_void, ocvrs_return: *mut Result<crate::stitching::Stitcher_Status>);
	// composePanorama(cv::InputArrayOfArrays, cv::OutputArray) /usr/include/opencv2/stitching.hpp:289
	pub fn cv_Stitcher_composePanorama_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, images: *const c_void, pano: *const c_void, ocvrs_return: *mut Result<crate::stitching::Stitcher_Status>);
	// stitch(cv::InputArrayOfArrays, cv::OutputArray) /usr/include/opencv2/stitching.hpp:292
	pub fn cv_Stitcher_stitch_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, images: *const c_void, pano: *const c_void, ocvrs_return: *mut Result<crate::stitching::Stitcher_Status>);
	// stitch(cv::InputArrayOfArrays, cv::InputArrayOfArrays, cv::OutputArray) /usr/include/opencv2/stitching.hpp:300
	pub fn cv_Stitcher_stitch_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, images: *const c_void, masks: *const c_void, pano: *const c_void, ocvrs_return: *mut Result<crate::stitching::Stitcher_Status>);
	// component() /usr/include/opencv2/stitching.hpp:302
	pub fn cv_Stitcher_component_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// cameras() /usr/include/opencv2/stitching.hpp:303
	pub fn cv_Stitcher_cameras_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// workScale() /usr/include/opencv2/stitching.hpp:304
	pub fn cv_Stitcher_workScale_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// resultMask() /usr/include/opencv2/stitching.hpp:305
	pub fn cv_Stitcher_resultMask_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:246
	pub fn cv_TransverseMercatorWarper_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:143
	pub fn cv_WarperCreator_create_const_float(instance: *const c_void, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// AffineBasedEstimator() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:124
	pub fn cv_detail_AffineBasedEstimator_AffineBasedEstimator(ocvrs_return: *mut Result<*mut c_void>);
	// AffineBestOf2NearestMatcher(bool, bool, float, int) /usr/include/opencv2/stitching/detail/matchers.hpp:237
	pub fn cv_detail_AffineBestOf2NearestMatcher_AffineBestOf2NearestMatcher_bool_bool_float_int(full_affine: bool, try_use_gpu: bool, match_conf: f32, num_matches_thresh1: i32, ocvrs_return: *mut Result<*mut c_void>);
	// AffineWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:242
	pub fn cv_detail_AffineWarper_AffineWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// warpPoint(const cv::Point2f &, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:251
	pub fn cv_detail_AffineWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, h: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
	// warpPointBackward(const cv::Point2f &, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:260
	pub fn cv_detail_AffineWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, h: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:271
	pub fn cv_detail_AffineWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, h: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:283
	pub fn cv_detail_AffineWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, h: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
	// warpRoi(cv::Size, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:292
	pub fn cv_detail_AffineWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, h: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// BestOf2NearestMatcher(bool, float, int, int) /usr/include/opencv2/stitching/detail/matchers.hpp:184
	pub fn cv_detail_BestOf2NearestMatcher_BestOf2NearestMatcher_bool_float_int_int(try_use_gpu: bool, match_conf: f32, num_matches_thresh1: i32, num_matches_thresh2: i32, ocvrs_return: *mut Result<*mut c_void>);
	// collectGarbage() /usr/include/opencv2/stitching/detail/matchers.hpp:187
	pub fn cv_detail_BestOf2NearestMatcher_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// create(bool, float, int, int) /usr/include/opencv2/stitching/detail/matchers.hpp:188
	pub fn cv_detail_BestOf2NearestMatcher_create_bool_float_int_int(try_use_gpu: bool, match_conf: f32, num_matches_thresh1: i32, num_matches_thresh2: i32, ocvrs_return: *mut Result<*mut c_void>);
	// BestOf2NearestRangeMatcher(int, bool, float, int, int) /usr/include/opencv2/stitching/detail/matchers.hpp:202
	pub fn cv_detail_BestOf2NearestRangeMatcher_BestOf2NearestRangeMatcher_int_bool_float_int_int(range_width: i32, try_use_gpu: bool, match_conf: f32, num_matches_thresh1: i32, num_matches_thresh2: i32, ocvrs_return: *mut Result<*mut c_void>);
	// createDefault(int, bool) /usr/include/opencv2/stitching/detail/blenders.hpp:69
	pub fn cv_detail_Blender_createDefault_int_bool(typ: i32, try_gpu: bool, ocvrs_return: *mut Result<*mut c_void>);
	// prepare(const std::vector<Point> &, const std::vector<Size> &) /usr/include/opencv2/stitching/detail/blenders.hpp:76
	pub fn cv_detail_Blender_prepare_const_vector_Point_R_const_vector_Size_R(instance: *mut c_void, corners: *const c_void, sizes: *const c_void, ocvrs_return: *mut Result_void);
	// prepare(cv::Rect) /usr/include/opencv2/stitching/detail/blenders.hpp:78
	pub fn cv_detail_Blender_prepare_Rect(instance: *mut c_void, dst_roi: *const core::Rect, ocvrs_return: *mut Result_void);
	// feed(cv::InputArray, cv::InputArray, cv::Point) /usr/include/opencv2/stitching/detail/blenders.hpp:85
	pub fn cv_detail_Blender_feed_const__InputArrayR_const__InputArrayR_Point(instance: *mut c_void, img: *const c_void, mask: *const c_void, tl: *const core::Point, ocvrs_return: *mut Result_void);
	// blend(cv::InputOutputArray, cv::InputOutputArray) /usr/include/opencv2/stitching/detail/blenders.hpp:91
	pub fn cv_detail_Blender_blend_const__InputOutputArrayR_const__InputOutputArrayR(instance: *mut c_void, dst: *const c_void, dst_mask: *const c_void, ocvrs_return: *mut Result_void);
	// BlocksChannelsCompensator(int, int, int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:234
	pub fn cv_detail_BlocksChannelsCompensator_BlocksChannelsCompensator_int_int_int(bl_width: i32, bl_height: i32, nr_feeds: i32, ocvrs_return: *mut Result<*mut c_void>);
	// apply(int, cv::Point, cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:175
	pub fn cv_detail_BlocksCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(instance: *mut c_void, index: i32, corner: *const core::Point, image: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// getMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:176
	pub fn cv_detail_BlocksCompensator_getMatGains_vector_Mat_R(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result_void);
	// setMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:177
	pub fn cv_detail_BlocksCompensator_setMatGains_vector_Mat_R(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result_void);
	// setNrFeeds(int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:178
	pub fn cv_detail_BlocksCompensator_setNrFeeds_int(instance: *mut c_void, nr_feeds: i32, ocvrs_return: *mut Result_void);
	// getNrFeeds() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:179
	pub fn cv_detail_BlocksCompensator_getNrFeeds(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// setSimilarityThreshold(double) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:180
	pub fn cv_detail_BlocksCompensator_setSimilarityThreshold_double(instance: *mut c_void, similarity_threshold: f64, ocvrs_return: *mut Result_void);
	// getSimilarityThreshold() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:181
	pub fn cv_detail_BlocksCompensator_getSimilarityThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setBlockSize(int, int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:182
	pub fn cv_detail_BlocksCompensator_setBlockSize_int_int(instance: *mut c_void, width: i32, height: i32, ocvrs_return: *mut Result_void);
	// setBlockSize(cv::Size) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:183
	pub fn cv_detail_BlocksCompensator_setBlockSize_Size(instance: *mut c_void, size: *const core::Size, ocvrs_return: *mut Result_void);
	// getBlockSize() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:184
	pub fn cv_detail_BlocksCompensator_getBlockSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// setNrGainsFilteringIterations(int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:185
	pub fn cv_detail_BlocksCompensator_setNrGainsFilteringIterations_int(instance: *mut c_void, nr_iterations: i32, ocvrs_return: *mut Result_void);
	// getNrGainsFilteringIterations() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:186
	pub fn cv_detail_BlocksCompensator_getNrGainsFilteringIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// BlocksGainCompensator(int, int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:211
	pub fn cv_detail_BlocksGainCompensator_BlocksGainCompensator_int_int(bl_width: i32, bl_height: i32, ocvrs_return: *mut Result<*mut c_void>);
	// BlocksGainCompensator(int, int, int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:213
	pub fn cv_detail_BlocksGainCompensator_BlocksGainCompensator_int_int_int(bl_width: i32, bl_height: i32, nr_feeds: i32, ocvrs_return: *mut Result<*mut c_void>);
	// apply(int, cv::Point, cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:220
	pub fn cv_detail_BlocksGainCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(instance: *mut c_void, index: i32, corner: *const core::Point, image: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// getMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:223
	pub fn cv_detail_BlocksGainCompensator_getMatGains_vector_Mat_R(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result_void);
	// setMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:225
	pub fn cv_detail_BlocksGainCompensator_setMatGains_vector_Mat_R(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result_void);
	// BundleAdjusterAffine() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:292
	pub fn cv_detail_BundleAdjusterAffine_BundleAdjusterAffine(ocvrs_return: *mut Result<*mut c_void>);
	// BundleAdjusterAffinePartial() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:316
	pub fn cv_detail_BundleAdjusterAffinePartial_BundleAdjusterAffinePartial(ocvrs_return: *mut Result<*mut c_void>);
	// refinementMask() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:136
	pub fn cv_detail_BundleAdjusterBase_refinementMask_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setRefinementMask(const cv::Mat &) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:137
	pub fn cv_detail_BundleAdjusterBase_setRefinementMask_const_MatR(instance: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// confThresh() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:143
	pub fn cv_detail_BundleAdjusterBase_confThresh_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setConfThresh(double) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:144
	pub fn cv_detail_BundleAdjusterBase_setConfThresh_double(instance: *mut c_void, conf_thresh: f64, ocvrs_return: *mut Result_void);
	// termCriteria() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:146
	pub fn cv_detail_BundleAdjusterBase_termCriteria(instance: *mut c_void, ocvrs_return: *mut Result<core::TermCriteria>);
	// setTermCriteria(const cv::TermCriteria &) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:147
	pub fn cv_detail_BundleAdjusterBase_setTermCriteria_const_TermCriteriaR(instance: *mut c_void, term_criteria: *const core::TermCriteria, ocvrs_return: *mut Result_void);
	// BundleAdjusterRay() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:268
	pub fn cv_detail_BundleAdjusterRay_BundleAdjusterRay(ocvrs_return: *mut Result<*mut c_void>);
	// BundleAdjusterReproj() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:248
	pub fn cv_detail_BundleAdjusterReproj_BundleAdjusterReproj(ocvrs_return: *mut Result<*mut c_void>);
	// focal /usr/include/opencv2/stitching/detail/camera.hpp:65
	pub fn cv_detail_CameraParams_getPropFocal_const(instance: *const c_void) -> f64;
	// focal /usr/include/opencv2/stitching/detail/camera.hpp:65
	pub fn cv_detail_CameraParams_setPropFocal_double(instance: *mut c_void, val: f64);
	// aspect /usr/include/opencv2/stitching/detail/camera.hpp:66
	pub fn cv_detail_CameraParams_getPropAspect_const(instance: *const c_void) -> f64;
	// aspect /usr/include/opencv2/stitching/detail/camera.hpp:66
	pub fn cv_detail_CameraParams_setPropAspect_double(instance: *mut c_void, val: f64);
	// ppx /usr/include/opencv2/stitching/detail/camera.hpp:67
	pub fn cv_detail_CameraParams_getPropPpx_const(instance: *const c_void) -> f64;
	// ppx /usr/include/opencv2/stitching/detail/camera.hpp:67
	pub fn cv_detail_CameraParams_setPropPpx_double(instance: *mut c_void, val: f64);
	// ppy /usr/include/opencv2/stitching/detail/camera.hpp:68
	pub fn cv_detail_CameraParams_getPropPpy_const(instance: *const c_void) -> f64;
	// ppy /usr/include/opencv2/stitching/detail/camera.hpp:68
	pub fn cv_detail_CameraParams_setPropPpy_double(instance: *mut c_void, val: f64);
	// R /usr/include/opencv2/stitching/detail/camera.hpp:69
	pub fn cv_detail_CameraParams_getPropR_const(instance: *const c_void) -> *mut c_void;
	// R /usr/include/opencv2/stitching/detail/camera.hpp:69
	pub fn cv_detail_CameraParams_setPropR_Mat(instance: *mut c_void, val: *mut c_void);
	// t /usr/include/opencv2/stitching/detail/camera.hpp:70
	pub fn cv_detail_CameraParams_getPropT_const(instance: *const c_void) -> *mut c_void;
	// t /usr/include/opencv2/stitching/detail/camera.hpp:70
	pub fn cv_detail_CameraParams_setPropT_Mat(instance: *mut c_void, val: *mut c_void);
	// CameraParams() /usr/include/opencv2/stitching/detail/camera.hpp:60
	pub fn cv_detail_CameraParams_CameraParams(ocvrs_return: *mut Result<*mut c_void>);
	// CameraParams(const cv::detail::CameraParams &) /usr/include/opencv2/stitching/detail/camera.hpp:61
	pub fn cv_detail_CameraParams_CameraParams_const_CameraParamsR(other: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// K() /usr/include/opencv2/stitching/detail/camera.hpp:63
	pub fn cv_detail_CameraParams_K_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// ChannelsCompensator(int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:148
	pub fn cv_detail_ChannelsCompensator_ChannelsCompensator_int(nr_feeds: i32, ocvrs_return: *mut Result<*mut c_void>);
	// apply(int, cv::Point, cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:152
	pub fn cv_detail_ChannelsCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(instance: *mut c_void, index: i32, corner: *const core::Point, image: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// getMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:153
	pub fn cv_detail_ChannelsCompensator_getMatGains_vector_Mat_R(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result_void);
	// setMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:154
	pub fn cv_detail_ChannelsCompensator_setMatGains_vector_Mat_R(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result_void);
	// setNrFeeds(int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:155
	pub fn cv_detail_ChannelsCompensator_setNrFeeds_int(instance: *mut c_void, nr_feeds: i32, ocvrs_return: *mut Result_void);
	// getNrFeeds() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:156
	pub fn cv_detail_ChannelsCompensator_getNrFeeds(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// setSimilarityThreshold(double) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:157
	pub fn cv_detail_ChannelsCompensator_setSimilarityThreshold_double(instance: *mut c_void, similarity_threshold: f64, ocvrs_return: *mut Result_void);
	// getSimilarityThreshold() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:158
	pub fn cv_detail_ChannelsCompensator_getSimilarityThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// gains() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:159
	pub fn cv_detail_ChannelsCompensator_gains_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// a /usr/include/opencv2/stitching/detail/warpers.hpp:411
	pub fn cv_detail_CompressedRectilinearPortraitProjector_getPropA_const(instance: *const c_void) -> f32;
	// a /usr/include/opencv2/stitching/detail/warpers.hpp:411
	pub fn cv_detail_CompressedRectilinearPortraitProjector_setPropA_float(instance: *mut c_void, val: f32);
	// b /usr/include/opencv2/stitching/detail/warpers.hpp:411
	pub fn cv_detail_CompressedRectilinearPortraitProjector_getPropB_const(instance: *const c_void) -> f32;
	// b /usr/include/opencv2/stitching/detail/warpers.hpp:411
	pub fn cv_detail_CompressedRectilinearPortraitProjector_setPropB_float(instance: *mut c_void, val: f32);
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:413
	pub fn cv_detail_CompressedRectilinearPortraitProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result_void);
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:414
	pub fn cv_detail_CompressedRectilinearPortraitProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result_void);
	// CompressedRectilinearPortraitWarper(float, float, float) /usr/include/opencv2/stitching/detail/warpers.hpp:421
	pub fn cv_detail_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float_float(scale: f32, a: f32, b: f32, ocvrs_return: *mut Result<*mut c_void>);
	// a /usr/include/opencv2/stitching/detail/warpers.hpp:390
	pub fn cv_detail_CompressedRectilinearProjector_getPropA_const(instance: *const c_void) -> f32;
	// a /usr/include/opencv2/stitching/detail/warpers.hpp:390
	pub fn cv_detail_CompressedRectilinearProjector_setPropA_float(instance: *mut c_void, val: f32);
	// b /usr/include/opencv2/stitching/detail/warpers.hpp:390
	pub fn cv_detail_CompressedRectilinearProjector_getPropB_const(instance: *const c_void) -> f32;
	// b /usr/include/opencv2/stitching/detail/warpers.hpp:390
	pub fn cv_detail_CompressedRectilinearProjector_setPropB_float(instance: *mut c_void, val: f32);
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:392
	pub fn cv_detail_CompressedRectilinearProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result_void);
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:393
	pub fn cv_detail_CompressedRectilinearProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result_void);
	// CompressedRectilinearWarper(float, float, float) /usr/include/opencv2/stitching/detail/warpers.hpp:400
	pub fn cv_detail_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float_float(scale: f32, a: f32, b: f32, ocvrs_return: *mut Result<*mut c_void>);
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:639
	pub fn cv_detail_CylindricalPortraitProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result_void);
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:640
	pub fn cv_detail_CylindricalPortraitProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result_void);
	// CylindricalPortraitWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:647
	pub fn cv_detail_CylindricalPortraitWarper_CylindricalPortraitWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:334
	pub fn cv_detail_CylindricalProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result_void);
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:335
	pub fn cv_detail_CylindricalProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result_void);
	// CylindricalWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:348
	pub fn cv_detail_CylindricalWarper_CylindricalWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:350
	pub fn cv_detail_CylindricalWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:351
	pub fn cv_detail_CylindricalWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
	// CylindricalWarperGpu(float) /usr/include/opencv2/stitching/detail/warpers.hpp:590
	pub fn cv_detail_CylindricalWarperGpu_CylindricalWarperGpu_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:592
	pub fn cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:600
	pub fn cv_detail_CylindricalWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cuda::GpuMat &, cuda::GpuMat &) /usr/include/opencv2/stitching/detail/warpers.hpp:609
	pub fn cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *mut c_void, ymap: *mut c_void, ocvrs_return: *mut Result<core::Rect>);
	// warp(const cuda::GpuMat &, cv::InputArray, cv::InputArray, int, int, cuda::GpuMat &) /usr/include/opencv2/stitching/detail/warpers.hpp:611
	pub fn cv_detail_CylindricalWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *mut c_void, ocvrs_return: *mut Result<core::Point>);
	// parent /usr/include/opencv2/stitching/detail/util.hpp:64
	pub fn cv_detail_DisjointSets_getPropParent_const(instance: *const c_void) -> *mut c_void;
	// parent /usr/include/opencv2/stitching/detail/util.hpp:64
	pub fn cv_detail_DisjointSets_setPropParent_vector_int_(instance: *mut c_void, val: *mut c_void);
	// size /usr/include/opencv2/stitching/detail/util.hpp:65
	pub fn cv_detail_DisjointSets_getPropSize_const(instance: *const c_void) -> *mut c_void;
	// size /usr/include/opencv2/stitching/detail/util.hpp:65
	pub fn cv_detail_DisjointSets_setPropSize_vector_int_(instance: *mut c_void, val: *mut c_void);
	// DisjointSets(int) /usr/include/opencv2/stitching/detail/util.hpp:58
	pub fn cv_detail_DisjointSets_DisjointSets_int(elem_count: i32, ocvrs_return: *mut Result<*mut c_void>);
	// createOneElemSets(int) /usr/include/opencv2/stitching/detail/util.hpp:60
	pub fn cv_detail_DisjointSets_createOneElemSets_int(instance: *mut c_void, elem_count: i32, ocvrs_return: *mut Result_void);
	// findSetByElem(int) /usr/include/opencv2/stitching/detail/util.hpp:61
	pub fn cv_detail_DisjointSets_findSetByElem_int(instance: *mut c_void, elem: i32, ocvrs_return: *mut Result<i32>);
	// mergeSets(int, int) /usr/include/opencv2/stitching/detail/util.hpp:62
	pub fn cv_detail_DisjointSets_mergeSets_int_int(instance: *mut c_void, set1: i32, set2: i32, ocvrs_return: *mut Result<i32>);
	// DpSeamFinder(cv::detail::DpSeamFinder::CostFunction) /usr/include/opencv2/stitching/detail/seam_finders.hpp:125
	pub fn cv_detail_DpSeamFinder_DpSeamFinder_CostFunction(cost_func: crate::stitching::Detail_DpSeamFinder_CostFunction, ocvrs_return: *mut Result<*mut c_void>);
	// DpSeamFinder(cv::String) /usr/include/opencv2/stitching/detail/seam_finders.hpp:126
	pub fn cv_detail_DpSeamFinder_DpSeamFinder_String(cost_func: *mut c_char, ocvrs_return: *mut Result<*mut c_void>);
	// costFunction() /usr/include/opencv2/stitching/detail/seam_finders.hpp:128
	pub fn cv_detail_DpSeamFinder_costFunction_const(instance: *const c_void, ocvrs_return: *mut Result<crate::stitching::Detail_DpSeamFinder_CostFunction>);
	// setCostFunction(cv::detail::DpSeamFinder::CostFunction) /usr/include/opencv2/stitching/detail/seam_finders.hpp:129
	pub fn cv_detail_DpSeamFinder_setCostFunction_CostFunction(instance: *mut c_void, val: crate::stitching::Detail_DpSeamFinder_CostFunction, ocvrs_return: *mut Result_void);
	// setCostFunction(cv::String) /usr/include/opencv2/stitching/detail/seam_finders.hpp:130
	pub fn cv_detail_DpSeamFinder_setCostFunction_String(instance: *mut c_void, val: *mut c_char, ocvrs_return: *mut Result_void);
	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/seam_finders.hpp:132
	pub fn cv_detail_DpSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(instance: *mut c_void, src: *const c_void, corners: *const c_void, masks: *mut c_void, ocvrs_return: *mut Result_void);
	// createDefault(int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:67
	pub fn cv_detail_ExposureCompensator_createDefault_int(typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<UMat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:75
	pub fn cv_detail_ExposureCompensator_feed_const_vector_Point_R_const_vector_UMat_R_const_vector_UMat_R(instance: *mut c_void, corners: *const c_void, images: *const c_void, masks: *const c_void, ocvrs_return: *mut Result_void);
	// apply(int, cv::Point, cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:87
	pub fn cv_detail_ExposureCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(instance: *mut c_void, index: i32, corner: *const core::Point, image: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// getMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:88
	pub fn cv_detail_ExposureCompensator_getMatGains_vector_Mat_R(instance: *mut c_void, unnamed: *mut c_void, ocvrs_return: *mut Result_void);
	// setMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:89
	pub fn cv_detail_ExposureCompensator_setMatGains_vector_Mat_R(instance: *mut c_void, unnamed: *mut c_void, ocvrs_return: *mut Result_void);
	// setUpdateGain(bool) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:90
	pub fn cv_detail_ExposureCompensator_setUpdateGain_bool(instance: *mut c_void, b: bool, ocvrs_return: *mut Result_void);
	// getUpdateGain() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:91
	pub fn cv_detail_ExposureCompensator_getUpdateGain(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
	// FeatherBlender(float) /usr/include/opencv2/stitching/detail/blenders.hpp:103
	pub fn cv_detail_FeatherBlender_FeatherBlender_float(sharpness: f32, ocvrs_return: *mut Result<*mut c_void>);
	// sharpness() /usr/include/opencv2/stitching/detail/blenders.hpp:105
	pub fn cv_detail_FeatherBlender_sharpness_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setSharpness(float) /usr/include/opencv2/stitching/detail/blenders.hpp:106
	pub fn cv_detail_FeatherBlender_setSharpness_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// prepare(cv::Rect) /usr/include/opencv2/stitching/detail/blenders.hpp:108
	pub fn cv_detail_FeatherBlender_prepare_Rect(instance: *mut c_void, dst_roi: *const core::Rect, ocvrs_return: *mut Result_void);
	// feed(cv::InputArray, cv::InputArray, cv::Point) /usr/include/opencv2/stitching/detail/blenders.hpp:109
	pub fn cv_detail_FeatherBlender_feed_const__InputArrayR_const__InputArrayR_Point(instance: *mut c_void, img: *const c_void, mask: *const c_void, tl: *const core::Point, ocvrs_return: *mut Result_void);
	// blend(cv::InputOutputArray, cv::InputOutputArray) /usr/include/opencv2/stitching/detail/blenders.hpp:110
	pub fn cv_detail_FeatherBlender_blend_const__InputOutputArrayR_const__InputOutputArrayR(instance: *mut c_void, dst: *const c_void, dst_mask: *const c_void, ocvrs_return: *mut Result_void);
	// createWeightMaps(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/blenders.hpp:114
	pub fn cv_detail_FeatherBlender_createWeightMaps_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(instance: *mut c_void, masks: *const c_void, corners: *const c_void, weight_maps: *mut c_void, ocvrs_return: *mut Result<core::Rect>);
	// isThreadSafe() /usr/include/opencv2/stitching/detail/matchers.hpp:145
	pub fn cv_detail_FeaturesMatcher_isThreadSafe_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// collectGarbage() /usr/include/opencv2/stitching/detail/matchers.hpp:149
	pub fn cv_detail_FeaturesMatcher_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:362
	pub fn cv_detail_FisheyeProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result_void);
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:363
	pub fn cv_detail_FisheyeProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result_void);
	// FisheyeWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:370
	pub fn cv_detail_FisheyeWarper_FisheyeWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// GainCompensator() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:115
	pub fn cv_detail_GainCompensator_GainCompensator(ocvrs_return: *mut Result<*mut c_void>);
	// GainCompensator(int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:117
	pub fn cv_detail_GainCompensator_GainCompensator_int(nr_feeds: i32, ocvrs_return: *mut Result<*mut c_void>);
	// apply(int, cv::Point, cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:123
	pub fn cv_detail_GainCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(instance: *mut c_void, index: i32, corner: *const core::Point, image: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// getMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:124
	pub fn cv_detail_GainCompensator_getMatGains_vector_Mat_R(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result_void);
	// setMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:125
	pub fn cv_detail_GainCompensator_setMatGains_vector_Mat_R(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result_void);
	// setNrFeeds(int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:126
	pub fn cv_detail_GainCompensator_setNrFeeds_int(instance: *mut c_void, nr_feeds: i32, ocvrs_return: *mut Result_void);
	// getNrFeeds() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:127
	pub fn cv_detail_GainCompensator_getNrFeeds(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// setSimilarityThreshold(double) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:128
	pub fn cv_detail_GainCompensator_setSimilarityThreshold_double(instance: *mut c_void, similarity_threshold: f64, ocvrs_return: *mut Result_void);
	// getSimilarityThreshold() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:129
	pub fn cv_detail_GainCompensator_getSimilarityThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// prepareSimilarityMask(const std::vector<Point> &, const std::vector<UMat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:130
	pub fn cv_detail_GainCompensator_prepareSimilarityMask_const_vector_Point_R_const_vector_UMat_R(instance: *mut c_void, corners: *const c_void, images: *const c_void, ocvrs_return: *mut Result_void);
	// gains() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:131
	pub fn cv_detail_GainCompensator_gains_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Graph(int) /usr/include/opencv2/stitching/detail/util.hpp:88
	pub fn cv_detail_Graph_Graph_int(num_vertices: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(int) /usr/include/opencv2/stitching/detail/util.hpp:89
	pub fn cv_detail_Graph_create_int(instance: *mut c_void, num_vertices: i32, ocvrs_return: *mut Result_void);
	// numVertices() /usr/include/opencv2/stitching/detail/util.hpp:90
	pub fn cv_detail_Graph_numVertices_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// addEdge(int, int, float) /usr/include/opencv2/stitching/detail/util.hpp:91
	pub fn cv_detail_Graph_addEdge_int_int_float(instance: *mut c_void, from: i32, to: i32, weight: f32, ocvrs_return: *mut Result_void);
	// GraphCutSeamFinder(int, float, float) /usr/include/opencv2/stitching/detail/seam_finders.hpp:243
	pub fn cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_int_float_float(cost_type: i32, terminal_cost: f32, bad_region_penalty: f32, ocvrs_return: *mut Result<*mut c_void>);
	// GraphCutSeamFinder(cv::String, float, float) /usr/include/opencv2/stitching/detail/seam_finders.hpp:245
	pub fn cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_String_float_float(cost_type: *mut c_char, terminal_cost: f32, bad_region_penalty: f32, ocvrs_return: *mut Result<*mut c_void>);
	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/seam_finders.hpp:250
	pub fn cv_detail_GraphCutSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(instance: *mut c_void, src: *const c_void, corners: *const c_void, masks: *mut c_void, ocvrs_return: *mut Result_void);
	// from /usr/include/opencv2/stitching/detail/util.hpp:78
	pub fn cv_detail_GraphEdge_getPropFrom_const(instance: *const c_void) -> i32;
	// from /usr/include/opencv2/stitching/detail/util.hpp:78
	pub fn cv_detail_GraphEdge_setPropFrom_int(instance: *mut c_void, val: i32);
	// to /usr/include/opencv2/stitching/detail/util.hpp:78
	pub fn cv_detail_GraphEdge_getPropTo_const(instance: *const c_void) -> i32;
	// to /usr/include/opencv2/stitching/detail/util.hpp:78
	pub fn cv_detail_GraphEdge_setPropTo_int(instance: *mut c_void, val: i32);
	// weight /usr/include/opencv2/stitching/detail/util.hpp:79
	pub fn cv_detail_GraphEdge_getPropWeight_const(instance: *const c_void) -> f32;
	// weight /usr/include/opencv2/stitching/detail/util.hpp:79
	pub fn cv_detail_GraphEdge_setPropWeight_float(instance: *mut c_void, val: f32);
	// GraphEdge(int, int, float) /usr/include/opencv2/stitching/detail/util.hpp:74
	pub fn cv_detail_GraphEdge_GraphEdge_int_int_float(from: i32, to: i32, weight: f32, ocvrs_return: *mut Result<*mut c_void>);
	// HomographyBasedEstimator(bool) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:103
	pub fn cv_detail_HomographyBasedEstimator_HomographyBasedEstimator_bool(is_focals_estimated: bool, ocvrs_return: *mut Result<*mut c_void>);
	// img_idx /usr/include/opencv2/stitching/detail/matchers.hpp:60
	pub fn cv_detail_ImageFeatures_getPropImg_idx_const(instance: *const c_void) -> i32;
	// img_idx /usr/include/opencv2/stitching/detail/matchers.hpp:60
	pub fn cv_detail_ImageFeatures_setPropImg_idx_int(instance: *mut c_void, val: i32);
	// img_size /usr/include/opencv2/stitching/detail/matchers.hpp:61
	pub fn cv_detail_ImageFeatures_getPropImg_size_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// img_size /usr/include/opencv2/stitching/detail/matchers.hpp:61
	pub fn cv_detail_ImageFeatures_setPropImg_size_Size(instance: *mut c_void, val: *const core::Size);
	// keypoints /usr/include/opencv2/stitching/detail/matchers.hpp:62
	pub fn cv_detail_ImageFeatures_getPropKeypoints_const(instance: *const c_void) -> *mut c_void;
	// keypoints /usr/include/opencv2/stitching/detail/matchers.hpp:62
	pub fn cv_detail_ImageFeatures_setPropKeypoints_vector_KeyPoint_(instance: *mut c_void, val: *mut c_void);
	// descriptors /usr/include/opencv2/stitching/detail/matchers.hpp:63
	pub fn cv_detail_ImageFeatures_getPropDescriptors_const(instance: *const c_void) -> *mut c_void;
	// descriptors /usr/include/opencv2/stitching/detail/matchers.hpp:63
	pub fn cv_detail_ImageFeatures_setPropDescriptors_UMat(instance: *mut c_void, val: *mut c_void);
	// getKeypoints() /usr/include/opencv2/stitching/detail/matchers.hpp:64
	pub fn cv_detail_ImageFeatures_getKeypoints(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// src_img_idx /usr/include/opencv2/stitching/detail/matchers.hpp:105
	pub fn cv_detail_MatchesInfo_getPropSrc_img_idx_const(instance: *const c_void) -> i32;
	// src_img_idx /usr/include/opencv2/stitching/detail/matchers.hpp:105
	pub fn cv_detail_MatchesInfo_setPropSrc_img_idx_int(instance: *mut c_void, val: i32);
	// dst_img_idx /usr/include/opencv2/stitching/detail/matchers.hpp:106
	pub fn cv_detail_MatchesInfo_getPropDst_img_idx_const(instance: *const c_void) -> i32;
	// dst_img_idx /usr/include/opencv2/stitching/detail/matchers.hpp:106
	pub fn cv_detail_MatchesInfo_setPropDst_img_idx_int(instance: *mut c_void, val: i32);
	// matches /usr/include/opencv2/stitching/detail/matchers.hpp:107
	pub fn cv_detail_MatchesInfo_getPropMatches_const(instance: *const c_void) -> *mut c_void;
	// matches /usr/include/opencv2/stitching/detail/matchers.hpp:107
	pub fn cv_detail_MatchesInfo_setPropMatches_vector_DMatch_(instance: *mut c_void, val: *mut c_void);
	// inliers_mask /usr/include/opencv2/stitching/detail/matchers.hpp:108
	pub fn cv_detail_MatchesInfo_getPropInliers_mask_const(instance: *const c_void) -> *mut c_void;
	// inliers_mask /usr/include/opencv2/stitching/detail/matchers.hpp:108
	pub fn cv_detail_MatchesInfo_setPropInliers_mask_vector_unsigned_char_(instance: *mut c_void, val: *mut c_void);
	// num_inliers /usr/include/opencv2/stitching/detail/matchers.hpp:109
	pub fn cv_detail_MatchesInfo_getPropNum_inliers_const(instance: *const c_void) -> i32;
	// num_inliers /usr/include/opencv2/stitching/detail/matchers.hpp:109
	pub fn cv_detail_MatchesInfo_setPropNum_inliers_int(instance: *mut c_void, val: i32);
	// H /usr/include/opencv2/stitching/detail/matchers.hpp:110
	pub fn cv_detail_MatchesInfo_getPropH_const(instance: *const c_void) -> *mut c_void;
	// H /usr/include/opencv2/stitching/detail/matchers.hpp:110
	pub fn cv_detail_MatchesInfo_setPropH_Mat(instance: *mut c_void, val: *mut c_void);
	// confidence /usr/include/opencv2/stitching/detail/matchers.hpp:111
	pub fn cv_detail_MatchesInfo_getPropConfidence_const(instance: *const c_void) -> f64;
	// confidence /usr/include/opencv2/stitching/detail/matchers.hpp:111
	pub fn cv_detail_MatchesInfo_setPropConfidence_double(instance: *mut c_void, val: f64);
	// MatchesInfo() /usr/include/opencv2/stitching/detail/matchers.hpp:101
	pub fn cv_detail_MatchesInfo_MatchesInfo(ocvrs_return: *mut Result<*mut c_void>);
	// MatchesInfo(const cv::detail::MatchesInfo &) /usr/include/opencv2/stitching/detail/matchers.hpp:102
	pub fn cv_detail_MatchesInfo_MatchesInfo_const_MatchesInfoR(other: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getMatches() /usr/include/opencv2/stitching/detail/matchers.hpp:112
	pub fn cv_detail_MatchesInfo_getMatches(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getInliers() /usr/include/opencv2/stitching/detail/matchers.hpp:113
	pub fn cv_detail_MatchesInfo_getInliers(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:475
	pub fn cv_detail_MercatorProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result_void);
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:476
	pub fn cv_detail_MercatorProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result_void);
	// MercatorWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:483
	pub fn cv_detail_MercatorWarper_MercatorWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// MultiBandBlender(int, int, int) /usr/include/opencv2/stitching/detail/blenders.hpp:130
	pub fn cv_detail_MultiBandBlender_MultiBandBlender_int_int_int(try_gpu: i32, num_bands: i32, weight_type: i32, ocvrs_return: *mut Result<*mut c_void>);
	// numBands() /usr/include/opencv2/stitching/detail/blenders.hpp:132
	pub fn cv_detail_MultiBandBlender_numBands_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setNumBands(int) /usr/include/opencv2/stitching/detail/blenders.hpp:133
	pub fn cv_detail_MultiBandBlender_setNumBands_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// prepare(cv::Rect) /usr/include/opencv2/stitching/detail/blenders.hpp:135
	pub fn cv_detail_MultiBandBlender_prepare_Rect(instance: *mut c_void, dst_roi: *const core::Rect, ocvrs_return: *mut Result_void);
	// feed(cv::InputArray, cv::InputArray, cv::Point) /usr/include/opencv2/stitching/detail/blenders.hpp:136
	pub fn cv_detail_MultiBandBlender_feed_const__InputArrayR_const__InputArrayR_Point(instance: *mut c_void, img: *const c_void, mask: *const c_void, tl: *const core::Point, ocvrs_return: *mut Result_void);
	// blend(cv::InputOutputArray, cv::InputOutputArray) /usr/include/opencv2/stitching/detail/blenders.hpp:137
	pub fn cv_detail_MultiBandBlender_blend_const__InputOutputArrayR_const__InputOutputArrayR(instance: *mut c_void, dst: *const c_void, dst_mask: *const c_void, ocvrs_return: *mut Result_void);
	// NoBundleAdjuster() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:224
	pub fn cv_detail_NoBundleAdjuster_NoBundleAdjuster(ocvrs_return: *mut Result<*mut c_void>);
	// apply(int, cv::Point, cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:103
	pub fn cv_detail_NoExposureCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(instance: *mut c_void, unnamed: i32, unnamed_1: *const core::Point, unnamed_2: *const c_void, unnamed_3: *const c_void, ocvrs_return: *mut Result_void);
	// getMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:104
	pub fn cv_detail_NoExposureCompensator_getMatGains_vector_Mat_R(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result_void);
	// setMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:105
	pub fn cv_detail_NoExposureCompensator_setMatGains_vector_Mat_R(instance: *mut c_void, umv: *mut c_void, ocvrs_return: *mut Result_void);
	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/seam_finders.hpp:79
	pub fn cv_detail_NoSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(instance: *mut c_void, unnamed: *const c_void, unnamed_1: *const c_void, unnamed_2: *mut c_void, ocvrs_return: *mut Result_void);
	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/seam_finders.hpp:87
	pub fn cv_detail_PairwiseSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(instance: *mut c_void, src: *const c_void, corners: *const c_void, masks: *mut c_void, ocvrs_return: *mut Result_void);
	// a /usr/include/opencv2/stitching/detail/warpers.hpp:453
	pub fn cv_detail_PaniniPortraitProjector_getPropA_const(instance: *const c_void) -> f32;
	// a /usr/include/opencv2/stitching/detail/warpers.hpp:453
	pub fn cv_detail_PaniniPortraitProjector_setPropA_float(instance: *mut c_void, val: f32);
	// b /usr/include/opencv2/stitching/detail/warpers.hpp:453
	pub fn cv_detail_PaniniPortraitProjector_getPropB_const(instance: *const c_void) -> f32;
	// b /usr/include/opencv2/stitching/detail/warpers.hpp:453
	pub fn cv_detail_PaniniPortraitProjector_setPropB_float(instance: *mut c_void, val: f32);
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:455
	pub fn cv_detail_PaniniPortraitProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result_void);
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:456
	pub fn cv_detail_PaniniPortraitProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result_void);
	// PaniniPortraitWarper(float, float, float) /usr/include/opencv2/stitching/detail/warpers.hpp:463
	pub fn cv_detail_PaniniPortraitWarper_PaniniPortraitWarper_float_float_float(scale: f32, a: f32, b: f32, ocvrs_return: *mut Result<*mut c_void>);
	// a /usr/include/opencv2/stitching/detail/warpers.hpp:432
	pub fn cv_detail_PaniniProjector_getPropA_const(instance: *const c_void) -> f32;
	// a /usr/include/opencv2/stitching/detail/warpers.hpp:432
	pub fn cv_detail_PaniniProjector_setPropA_float(instance: *mut c_void, val: f32);
	// b /usr/include/opencv2/stitching/detail/warpers.hpp:432
	pub fn cv_detail_PaniniProjector_getPropB_const(instance: *const c_void) -> f32;
	// b /usr/include/opencv2/stitching/detail/warpers.hpp:432
	pub fn cv_detail_PaniniProjector_setPropB_float(instance: *mut c_void, val: f32);
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:434
	pub fn cv_detail_PaniniProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result_void);
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:435
	pub fn cv_detail_PaniniProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result_void);
	// PaniniWarper(float, float, float) /usr/include/opencv2/stitching/detail/warpers.hpp:442
	pub fn cv_detail_PaniniWarper_PaniniWarper_float_float_float(scale: f32, a: f32, b: f32, ocvrs_return: *mut Result<*mut c_void>);
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:658
	pub fn cv_detail_PlanePortraitProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result_void);
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:659
	pub fn cv_detail_PlanePortraitProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result_void);
	// PlanePortraitWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:666
	pub fn cv_detail_PlanePortraitWarper_PlanePortraitWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:193
	pub fn cv_detail_PlaneProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result_void);
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:194
	pub fn cv_detail_PlaneProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result_void);
	// PlaneWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:206
	pub fn cv_detail_PlaneWarper_PlaneWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// warpPoint(const cv::Point2f &, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:208
	pub fn cv_detail_PlaneWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
	// warpPoint(const cv::Point2f &, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:209
	pub fn cv_detail_PlaneWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, r: *const c_void, t: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
	// warpPointBackward(const cv::Point2f &, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:211
	pub fn cv_detail_PlaneWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
	// warpPointBackward(const cv::Point2f &, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:212
	pub fn cv_detail_PlaneWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, r: *const c_void, t: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:214
	pub fn cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, t: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:215
	pub fn cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:217
	pub fn cv_detail_PlaneWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:219
	pub fn cv_detail_PlaneWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, t: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
	// warpRoi(cv::Size, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:222
	pub fn cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// warpRoi(cv::Size, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:223
	pub fn cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, t: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// PlaneWarperGpu(float) /usr/include/opencv2/stitching/detail/warpers.hpp:504
	pub fn cv_detail_PlaneWarperGpu_PlaneWarperGpu_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:506
	pub fn cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:514
	pub fn cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, t: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:522
	pub fn cv_detail_PlaneWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:531
	pub fn cv_detail_PlaneWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, t: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cuda::GpuMat &, cuda::GpuMat &) /usr/include/opencv2/stitching/detail/warpers.hpp:540
	pub fn cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *mut c_void, ymap: *mut c_void, ocvrs_return: *mut Result<core::Rect>);
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::InputArray, cuda::GpuMat &, cuda::GpuMat &) /usr/include/opencv2/stitching/detail/warpers.hpp:542
	pub fn cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, t: *const c_void, xmap: *mut c_void, ymap: *mut c_void, ocvrs_return: *mut Result<core::Rect>);
	// warp(const cuda::GpuMat &, cv::InputArray, cv::InputArray, int, int, cuda::GpuMat &) /usr/include/opencv2/stitching/detail/warpers.hpp:544
	pub fn cv_detail_PlaneWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *mut c_void, ocvrs_return: *mut Result<core::Point>);
	// warp(const cuda::GpuMat &, cv::InputArray, cv::InputArray, cv::InputArray, int, int, cuda::GpuMat &) /usr/include/opencv2/stitching/detail/warpers.hpp:547
	pub fn cv_detail_PlaneWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, t: *const c_void, interp_mode: i32, border_mode: i32, dst: *mut c_void, ocvrs_return: *mut Result<core::Point>);
	// scale /usr/include/opencv2/stitching/detail/warpers.hpp:147
	pub fn cv_detail_ProjectorBase_getPropScale_const(instance: *const c_void) -> f32;
	// scale /usr/include/opencv2/stitching/detail/warpers.hpp:147
	pub fn cv_detail_ProjectorBase_setPropScale_float(instance: *mut c_void, val: f32);
	// k /usr/include/opencv2/stitching/detail/warpers.hpp:148
	pub fn cv_detail_ProjectorBase_getPropK(instance: *mut c_void) -> *mut [f32; 9];
	// rinv /usr/include/opencv2/stitching/detail/warpers.hpp:149
	pub fn cv_detail_ProjectorBase_getPropRinv(instance: *mut c_void) -> *mut [f32; 9];
	// r_kinv /usr/include/opencv2/stitching/detail/warpers.hpp:150
	pub fn cv_detail_ProjectorBase_getPropR_kinv(instance: *mut c_void) -> *mut [f32; 9];
	// k_rinv /usr/include/opencv2/stitching/detail/warpers.hpp:151
	pub fn cv_detail_ProjectorBase_getPropK_rinv(instance: *mut c_void) -> *mut [f32; 9];
	// t /usr/include/opencv2/stitching/detail/warpers.hpp:152
	pub fn cv_detail_ProjectorBase_getPropT(instance: *mut c_void) -> *mut [f32; 3];
	// setCameraParams(cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:143
	pub fn cv_detail_ProjectorBase_setCameraParams_const__InputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, k: *const c_void, r: *const c_void, t: *const c_void, ocvrs_return: *mut Result_void);
	// warpPoint(const cv::Point2f &, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:71
	pub fn cv_detail_RotationWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
	// warpPointBackward(const cv::Point2f &, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:81
	pub fn cv_detail_RotationWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, pt: *const core::Point2f, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:99
	pub fn cv_detail_RotationWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:111
	pub fn cv_detail_RotationWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
	// warpBackward(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::Size, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:124
	pub fn cv_detail_RotationWarper_warpBackward_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_Size_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst_size: *const core::Size, dst: *const c_void, ocvrs_return: *mut Result_void);
	// warpRoi(cv::Size, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:133
	pub fn cv_detail_RotationWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// getScale() /usr/include/opencv2/stitching/detail/warpers.hpp:135
	pub fn cv_detail_RotationWarper_getScale_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setScale(float) /usr/include/opencv2/stitching/detail/warpers.hpp:136
	pub fn cv_detail_RotationWarper_setScale_float(instance: *mut c_void, unnamed: f32, ocvrs_return: *mut Result_void);
	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/seam_finders.hpp:69
	pub fn cv_detail_SeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(instance: *mut c_void, src: *const c_void, corners: *const c_void, masks: *mut c_void, ocvrs_return: *mut Result_void);
	// createDefault(int) /usr/include/opencv2/stitching/detail/seam_finders.hpp:71
	pub fn cv_detail_SeamFinder_createDefault_int(typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:621
	pub fn cv_detail_SphericalPortraitProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result_void);
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:622
	pub fn cv_detail_SphericalPortraitProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result_void);
	// SphericalPortraitWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:631
	pub fn cv_detail_SphericalPortraitWarper_SphericalPortraitWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:304
	pub fn cv_detail_SphericalProjector_mapForward_float_float_floatR_floatR(instance: *const crate::stitching::Detail_SphericalProjector, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result_void);
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:305
	pub fn cv_detail_SphericalProjector_mapBackward_float_float_floatR_floatR(instance: *const crate::stitching::Detail_SphericalProjector, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result_void);
	// SphericalWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:323
	pub fn cv_detail_SphericalWarper_SphericalWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:325
	pub fn cv_detail_SphericalWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:326
	pub fn cv_detail_SphericalWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
	// SphericalWarperGpu(float) /usr/include/opencv2/stitching/detail/warpers.hpp:558
	pub fn cv_detail_SphericalWarperGpu_SphericalWarperGpu_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:560
	pub fn cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *const c_void, ymap: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:568
	pub fn cv_detail_SphericalWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *const c_void, ocvrs_return: *mut Result<core::Point>);
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cuda::GpuMat &, cuda::GpuMat &) /usr/include/opencv2/stitching/detail/warpers.hpp:577
	pub fn cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(instance: *mut c_void, src_size: *const core::Size, k: *const c_void, r: *const c_void, xmap: *mut c_void, ymap: *mut c_void, ocvrs_return: *mut Result<core::Rect>);
	// warp(const cuda::GpuMat &, cv::InputArray, cv::InputArray, int, int, cuda::GpuMat &) /usr/include/opencv2/stitching/detail/warpers.hpp:579
	pub fn cv_detail_SphericalWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(instance: *mut c_void, src: *const c_void, k: *const c_void, r: *const c_void, interp_mode: i32, border_mode: i32, dst: *mut c_void, ocvrs_return: *mut Result<core::Point>);
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:376
	pub fn cv_detail_StereographicProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result_void);
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:377
	pub fn cv_detail_StereographicProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result_void);
	// StereographicWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:384
	pub fn cv_detail_StereographicWarper_StereographicWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:489
	pub fn cv_detail_TransverseMercatorProjector_mapForward_float_float_floatR_floatR(instance: *mut c_void, x: f32, y: f32, u: *mut f32, v: *mut f32, ocvrs_return: *mut Result_void);
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:490
	pub fn cv_detail_TransverseMercatorProjector_mapBackward_float_float_floatR_floatR(instance: *mut c_void, u: f32, v: f32, x: *mut f32, y: *mut f32, ocvrs_return: *mut Result_void);
	// TransverseMercatorWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:497
	pub fn cv_detail_TransverseMercatorWarper_TransverseMercatorWarper_float(scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/seam_finders.hpp:111
	pub fn cv_detail_VoronoiSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(instance: *mut c_void, src: *const c_void, corners: *const c_void, masks: *mut c_void, ocvrs_return: *mut Result_void);
	// find(const std::vector<Size> &, const std::vector<Point> &, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/seam_finders.hpp:113
	pub fn cv_detail_VoronoiSeamFinder_find_const_vector_Size_R_const_vector_Point_R_vector_UMat_R(instance: *mut c_void, size: *const c_void, corners: *const c_void, masks: *mut c_void, ocvrs_return: *mut Result_void);
}
