extern "C" {
	// BrightEdges(cv::Mat &, cv::Mat &, int, int, int) /usr/include/opencv2/ximgproc/brightedges.hpp:48
	pub fn cv_ximgproc_BrightEdges_MatR_MatR_int_int_int(_original: *mut c_void, _edgeview: *mut c_void, contrast: i32, shortrange: i32, longrange: i32, ocvrs_return: *mut Result_void);
	// FastHoughTransform(cv::InputArray, cv::OutputArray, int, int, int, int) /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:133
	pub fn cv_ximgproc_FastHoughTransform_const__InputArrayR_const__OutputArrayR_int_int_int_int(src: *const c_void, dst: *const c_void, dst_mat_depth: i32, angle_range: i32, op: i32, make_skew: i32, ocvrs_return: *mut Result_void);
	// GradientDericheX(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/ximgproc/deriche_filter.hpp:72
	pub fn cv_ximgproc_GradientDericheX_const__InputArrayR_const__OutputArrayR_double_double(op: *const c_void, dst: *const c_void, alpha: f64, omega: f64, ocvrs_return: *mut Result_void);
	// GradientDericheY(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/ximgproc/deriche_filter.hpp:60
	pub fn cv_ximgproc_GradientDericheY_const__InputArrayR_const__OutputArrayR_double_double(op: *const c_void, dst: *const c_void, alpha: f64, omega: f64, ocvrs_return: *mut Result_void);
	// GradientPaillouX(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/ximgproc/paillou_filter.hpp:62
	pub fn cv_ximgproc_GradientPaillouX_const__InputArrayR_const__OutputArrayR_double_double(op: *const c_void, _dst: *const c_void, alpha: f64, omega: f64, ocvrs_return: *mut Result_void);
	// GradientPaillouY(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/ximgproc/paillou_filter.hpp:61
	pub fn cv_ximgproc_GradientPaillouY_const__InputArrayR_const__OutputArrayR_double_double(op: *const c_void, _dst: *const c_void, alpha: f64, omega: f64, ocvrs_return: *mut Result_void);
	// HoughPoint2Line(const cv::Point &, cv::InputArray, int, int, int) /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:155
	pub fn cv_ximgproc_HoughPoint2Line_const_PointR_const__InputArrayR_int_int_int(hough_point: *const core::Point, src_img_info: *const c_void, angle_range: i32, make_skew: i32, rules: i32, ocvrs_return: *mut Result<core::Vec4i>);
	// PeiLinNormalization(cv::InputArray) /usr/include/opencv2/ximgproc/peilin.hpp:26
	pub fn cv_ximgproc_PeiLinNormalization_const__InputArrayR(i: *const c_void, ocvrs_return: *mut Result<core::Matx23d>);
	// PeiLinNormalization(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/peilin.hpp:28
	pub fn cv_ximgproc_PeiLinNormalization_const__InputArrayR_const__OutputArrayR(i: *const c_void, t: *const c_void, ocvrs_return: *mut Result_void);
	// RadonTransform(cv::InputArray, cv::OutputArray, double, double, double, bool, bool) /usr/include/opencv2/ximgproc/radon_transform.hpp:31
	pub fn cv_ximgproc_RadonTransform_const__InputArrayR_const__OutputArrayR_double_double_double_bool_bool(src: *const c_void, dst: *const c_void, theta: f64, start_angle: f64, end_angle: f64, crop: bool, norm: bool, ocvrs_return: *mut Result_void);
	// amFilter(cv::InputArray, cv::InputArray, cv::OutputArray, double, double, bool) /usr/include/opencv2/ximgproc/edge_filter.hpp:284
	pub fn cv_ximgproc_amFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_bool(joint: *const c_void, src: *const c_void, dst: *const c_void, sigma_s: f64, sigma_r: f64, adjust_outliers: bool, ocvrs_return: *mut Result_void);
	// anisotropicDiffusion(cv::InputArray, cv::OutputArray, float, float, int) /usr/include/opencv2/ximgproc.hpp:211
	pub fn cv_ximgproc_anisotropicDiffusion_const__InputArrayR_const__OutputArrayR_float_float_int(src: *const c_void, dst: *const c_void, alpha: f32, k: f32, niters: i32, ocvrs_return: *mut Result_void);
	// bilateralTextureFilter(cv::InputArray, cv::OutputArray, int, int, double, double) /usr/include/opencv2/ximgproc/edge_filter.hpp:339
	pub fn cv_ximgproc_bilateralTextureFilter_const__InputArrayR_const__OutputArrayR_int_int_double_double(src: *const c_void, dst: *const c_void, fr: i32, num_iter: i32, sigma_alpha: f64, sigma_avg: f64, ocvrs_return: *mut Result_void);
	// colorMatchTemplate(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/color_match.hpp:62
	pub fn cv_ximgproc_colorMatchTemplate_const__InputArrayR_const__InputArrayR_const__OutputArrayR(img: *const c_void, templ: *const c_void, result: *const c_void, ocvrs_return: *mut Result_void);
	// computeBadPixelPercent(cv::InputArray, cv::InputArray, cv::Rect, int) /usr/include/opencv2/ximgproc/disparity_filter.hpp:193
	pub fn cv_ximgproc_computeBadPixelPercent_const__InputArrayR_const__InputArrayR_Rect_int(gt: *const c_void, src: *const c_void, roi: *const core::Rect, thresh: i32, ocvrs_return: *mut Result<f64>);
	// computeMSE(cv::InputArray, cv::InputArray, cv::Rect) /usr/include/opencv2/ximgproc/disparity_filter.hpp:177
	pub fn cv_ximgproc_computeMSE_const__InputArrayR_const__InputArrayR_Rect(gt: *const c_void, src: *const c_void, roi: *const core::Rect, ocvrs_return: *mut Result<f64>);
	// contourSampling(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:106
	pub fn cv_ximgproc_contourSampling_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, out: *const c_void, nb_elt: i32, ocvrs_return: *mut Result_void);
	// covarianceEstimation(cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/ximgproc/estimated_covariance.hpp:77
	pub fn cv_ximgproc_covarianceEstimation_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, window_rows: i32, window_cols: i32, ocvrs_return: *mut Result_void);
	// createAMFilter(double, double, bool) /usr/include/opencv2/ximgproc/edge_filter.hpp:262
	pub fn cv_ximgproc_createAMFilter_double_double_bool(sigma_s: f64, sigma_r: f64, adjust_outliers: bool, ocvrs_return: *mut Result<*mut c_void>);
	// createContourFitting(int, int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:114
	pub fn cv_ximgproc_createContourFitting_int_int(ctr: i32, fd: i32, ocvrs_return: *mut Result<*mut c_void>);
	// createDTFilter(cv::InputArray, double, double, int, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:102
	pub fn cv_ximgproc_createDTFilter_const__InputArrayR_double_double_int_int(guide: *const c_void, sigma_spatial: f64, sigma_color: f64, mode: i32, num_iters: i32, ocvrs_return: *mut Result<*mut c_void>);
	// createDisparityWLSFilterGeneric(bool) /usr/include/opencv2/ximgproc/disparity_filter.hpp:149
	pub fn cv_ximgproc_createDisparityWLSFilterGeneric_bool(use_confidence: bool, ocvrs_return: *mut Result<*mut c_void>);
	// createDisparityWLSFilter(Ptr<cv::StereoMatcher>) /usr/include/opencv2/ximgproc/disparity_filter.hpp:131
	pub fn cv_ximgproc_createDisparityWLSFilter_Ptr_StereoMatcher_(matcher_left: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// createEdgeAwareInterpolator() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:138
	pub fn cv_ximgproc_createEdgeAwareInterpolator(ocvrs_return: *mut Result<*mut c_void>);
	// createEdgeBoxes(float, float, float, float, int, float, float, float, float, float, float, float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:183
	pub fn cv_ximgproc_createEdgeBoxes_float_float_float_float_int_float_float_float_float_float_float_float(alpha: f32, beta: f32, eta: f32, min_score: f32, max_boxes: i32, edge_min_mag: f32, edge_merge_thr: f32, cluster_min_mag: f32, max_aspect_ratio: f32, min_box_area: f32, gamma: f32, kappa: f32, ocvrs_return: *mut Result<*mut c_void>);
	// createEdgeDrawing() /usr/include/opencv2/ximgproc/edge_drawing.hpp:126
	pub fn cv_ximgproc_createEdgeDrawing(ocvrs_return: *mut Result<*mut c_void>);
	// createFastBilateralSolverFilter(cv::InputArray, double, double, double, double, int, double) /usr/include/opencv2/ximgproc/edge_filter.hpp:417
	pub fn cv_ximgproc_createFastBilateralSolverFilter_const__InputArrayR_double_double_double_double_int_double(guide: *const c_void, sigma_spatial: f64, sigma_luma: f64, sigma_chroma: f64, lambda: f64, num_iter: i32, max_tol: f64, ocvrs_return: *mut Result<*mut c_void>);
	// createFastGlobalSmootherFilter(cv::InputArray, double, double, double, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:489
	pub fn cv_ximgproc_createFastGlobalSmootherFilter_const__InputArrayR_double_double_double_int(guide: *const c_void, lambda: f64, sigma_color: f64, lambda_attenuation: f64, num_iter: i32, ocvrs_return: *mut Result<*mut c_void>);
	// createFastLineDetector(int, float, double, double, int, bool) /usr/include/opencv2/ximgproc/fast_line_detector.hpp:71
	pub fn cv_ximgproc_createFastLineDetector_int_float_double_double_int_bool(length_threshold: i32, distance_threshold: f32, canny_th1: f64, canny_th2: f64, canny_aperture_size: i32, do_merge: bool, ocvrs_return: *mut Result<*mut c_void>);
	// createGuidedFilter(cv::InputArray, int, double) /usr/include/opencv2/ximgproc/edge_filter.hpp:158
	pub fn cv_ximgproc_createGuidedFilter_const__InputArrayR_int_double(guide: *const c_void, radius: i32, eps: f64, ocvrs_return: *mut Result<*mut c_void>);
	// createQuaternionImage(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/color_match.hpp:22
	pub fn cv_ximgproc_createQuaternionImage_const__InputArrayR_const__OutputArrayR(img: *const c_void, qimg: *const c_void, ocvrs_return: *mut Result_void);
	// createRFFeatureGetter() /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:91
	pub fn cv_ximgproc_createRFFeatureGetter(ocvrs_return: *mut Result<*mut c_void>);
	// createRICInterpolator() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:265
	pub fn cv_ximgproc_createRICInterpolator(ocvrs_return: *mut Result<*mut c_void>);
	// createRightMatcher(Ptr<cv::StereoMatcher>) /usr/include/opencv2/ximgproc/disparity_filter.hpp:139
	pub fn cv_ximgproc_createRightMatcher_Ptr_StereoMatcher_(matcher_left: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// createScanSegment(int, int, int, int, bool) /usr/include/opencv2/ximgproc/scansegment.hpp:80
	pub fn cv_ximgproc_createScanSegment_int_int_int_int_bool(image_width: i32, image_height: i32, num_superpixels: i32, slices: i32, merge_small: bool, ocvrs_return: *mut Result<*mut c_void>);
	// createStructuredEdgeDetection(const cv::String &, Ptr<const cv::ximgproc::RFFeatureGetter>) /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:140
	pub fn cv_ximgproc_createStructuredEdgeDetection_const_StringR_Ptr_const_RFFeatureGetter_(model: *const c_char, how_to_get_features: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// createSuperpixelLSC(cv::InputArray, int, float) /usr/include/opencv2/ximgproc/lsc.hpp:150
	pub fn cv_ximgproc_createSuperpixelLSC_const__InputArrayR_int_float(image: *const c_void, region_size: i32, ratio: f32, ocvrs_return: *mut Result<*mut c_void>);
	// createSuperpixelSEEDS(int, int, int, int, int, int, int, bool) /usr/include/opencv2/ximgproc/seeds.hpp:173
	pub fn cv_ximgproc_createSuperpixelSEEDS_int_int_int_int_int_int_int_bool(image_width: i32, image_height: i32, image_channels: i32, num_superpixels: i32, num_levels: i32, prior: i32, histogram_bins: i32, double_step: bool, ocvrs_return: *mut Result<*mut c_void>);
	// createSuperpixelSLIC(cv::InputArray, int, int, float) /usr/include/opencv2/ximgproc/slic.hpp:160
	pub fn cv_ximgproc_createSuperpixelSLIC_const__InputArrayR_int_int_float(image: *const c_void, algorithm: i32, region_size: i32, ruler: f32, ocvrs_return: *mut Result<*mut c_void>);
	// dtFilter(cv::InputArray, cv::InputArray, cv::OutputArray, double, double, int, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:121
	pub fn cv_ximgproc_dtFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_int_int(guide: *const c_void, src: *const c_void, dst: *const c_void, sigma_spatial: f64, sigma_color: f64, mode: i32, num_iters: i32, ocvrs_return: *mut Result_void);
	// edgePreservingFilter(cv::InputArray, cv::OutputArray, int, double) /usr/include/opencv2/ximgproc/edgepreserving_filter.hpp:27
	pub fn cv_ximgproc_edgePreservingFilter_const__InputArrayR_const__OutputArrayR_int_double(src: *const c_void, dst: *const c_void, d: i32, threshold: f64, ocvrs_return: *mut Result_void);
	// fastBilateralSolverFilter(cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray, double, double, double, double, int, double) /usr/include/opencv2/ximgproc/edge_filter.hpp:448
	pub fn cv_ximgproc_fastBilateralSolverFilter_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_double_double_int_double(guide: *const c_void, src: *const c_void, confidence: *const c_void, dst: *const c_void, sigma_spatial: f64, sigma_luma: f64, sigma_chroma: f64, lambda: f64, num_iter: i32, max_tol: f64, ocvrs_return: *mut Result_void);
	// fastGlobalSmootherFilter(cv::InputArray, cv::InputArray, cv::OutputArray, double, double, double, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:509
	pub fn cv_ximgproc_fastGlobalSmootherFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_double_int(guide: *const c_void, src: *const c_void, dst: *const c_void, lambda: f64, sigma_color: f64, lambda_attenuation: f64, num_iter: i32, ocvrs_return: *mut Result_void);
	// fourierDescriptor(cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:87
	pub fn cv_ximgproc_fourierDescriptor_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, nb_elt: i32, nb_fd: i32, ocvrs_return: *mut Result_void);
	// getDisparityVis(cv::InputArray, cv::OutputArray, double) /usr/include/opencv2/ximgproc/disparity_filter.hpp:204
	pub fn cv_ximgproc_getDisparityVis_const__InputArrayR_const__OutputArrayR_double(src: *const c_void, dst: *const c_void, scale: f64, ocvrs_return: *mut Result_void);
	// guidedFilter(cv::InputArray, cv::InputArray, cv::OutputArray, int, double, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:180
	pub fn cv_ximgproc_guidedFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_int(guide: *const c_void, src: *const c_void, dst: *const c_void, radius: i32, eps: f64, d_depth: i32, ocvrs_return: *mut Result_void);
	// jointBilateralFilter(cv::InputArray, cv::InputArray, cv::OutputArray, int, double, double, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:317
	pub fn cv_ximgproc_jointBilateralFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_double_int(joint: *const c_void, src: *const c_void, dst: *const c_void, d: i32, sigma_color: f64, sigma_space: f64, border_type: i32, ocvrs_return: *mut Result_void);
	// l0Smooth(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/ximgproc/edge_filter.hpp:523
	pub fn cv_ximgproc_l0Smooth_const__InputArrayR_const__OutputArrayR_double_double(src: *const c_void, dst: *const c_void, lambda: f64, kappa: f64, ocvrs_return: *mut Result_void);
	// niBlackThreshold(cv::InputArray, cv::OutputArray, double, int, int, double, int, double) /usr/include/opencv2/ximgproc.hpp:176
	pub fn cv_ximgproc_niBlackThreshold_const__InputArrayR_const__OutputArrayR_double_int_int_double_int_double(_src: *const c_void, _dst: *const c_void, max_value: f64, typ: i32, block_size: i32, k: f64, binarization_method: i32, r: f64, ocvrs_return: *mut Result_void);
	// qconj(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/color_match.hpp:30
	pub fn cv_ximgproc_qconj_const__InputArrayR_const__OutputArrayR(qimg: *const c_void, qcimg: *const c_void, ocvrs_return: *mut Result_void);
	// qdft(cv::InputArray, cv::OutputArray, int, bool) /usr/include/opencv2/ximgproc/color_match.hpp:54
	pub fn cv_ximgproc_qdft_const__InputArrayR_const__OutputArrayR_int_bool(img: *const c_void, qimg: *const c_void, flags: i32, side_left: bool, ocvrs_return: *mut Result_void);
	// qmultiply(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/color_match.hpp:45
	pub fn cv_ximgproc_qmultiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// qunitary(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/color_match.hpp:37
	pub fn cv_ximgproc_qunitary_const__InputArrayR_const__OutputArrayR(qimg: *const c_void, qnimg: *const c_void, ocvrs_return: *mut Result_void);
	// readGT(cv::String, cv::OutputArray) /usr/include/opencv2/ximgproc/disparity_filter.hpp:164
	pub fn cv_ximgproc_readGT_String_const__OutputArrayR(src_path: *mut c_char, dst: *const c_void, ocvrs_return: *mut Result<i32>);
	// createRLEImage(const std::vector<cv::Point3i> &, cv::OutputArray, cv::Size) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:97
	pub fn cv_ximgproc_rl_createRLEImage_const_vector_Point3i_R_const__OutputArrayR_Size(runs: *const c_void, res: *const c_void, size: *const core::Size, ocvrs_return: *mut Result_void);
	// dilate(cv::InputArray, cv::OutputArray, cv::InputArray, cv::Point) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:42
	pub fn cv_ximgproc_rl_dilate_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Point(rl_src: *const c_void, rl_dest: *const c_void, rl_kernel: *const c_void, anchor: *const core::Point, ocvrs_return: *mut Result_void);
	// erode(cv::InputArray, cv::OutputArray, cv::InputArray, bool, cv::Point) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:57
	pub fn cv_ximgproc_rl_erode_const__InputArrayR_const__OutputArrayR_const__InputArrayR_bool_Point(rl_src: *const c_void, rl_dest: *const c_void, rl_kernel: *const c_void, b_boundary_on: bool, anchor: *const core::Point, ocvrs_return: *mut Result_void);
	// getStructuringElement(int, cv::Size) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:68
	pub fn cv_ximgproc_rl_getStructuringElement_int_Size(shape: i32, ksize: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
	// isRLMorphologyPossible(cv::InputArray) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:87
	pub fn cv_ximgproc_rl_isRLMorphologyPossible_const__InputArrayR(rl_structuring_element: *const c_void, ocvrs_return: *mut Result<bool>);
	// morphologyEx(cv::InputArray, cv::OutputArray, int, cv::InputArray, bool, cv::Point) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:113
	pub fn cv_ximgproc_rl_morphologyEx_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_bool_Point(rl_src: *const c_void, rl_dest: *const c_void, op: i32, rl_kernel: *const c_void, b_boundary_on_for_erosion: bool, anchor: *const core::Point, ocvrs_return: *mut Result_void);
	// paint(cv::InputOutputArray, cv::InputArray, const cv::Scalar &) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:79
	pub fn cv_ximgproc_rl_paint_const__InputOutputArrayR_const__InputArrayR_const_ScalarR(image: *const c_void, rl_src: *const c_void, value: *const core::Scalar, ocvrs_return: *mut Result_void);
	// threshold(cv::InputArray, cv::OutputArray, double, int) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:28
	pub fn cv_ximgproc_rl_threshold_const__InputArrayR_const__OutputArrayR_double_int(src: *const c_void, rl_dest: *const c_void, thresh: f64, typ: i32, ocvrs_return: *mut Result_void);
	// rollingGuidanceFilter(cv::InputArray, cv::OutputArray, int, double, double, int, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:373
	pub fn cv_ximgproc_rollingGuidanceFilter_const__InputArrayR_const__OutputArrayR_int_double_double_int_int(src: *const c_void, dst: *const c_void, d: i32, sigma_color: f64, sigma_space: f64, num_of_iter: i32, border_type: i32, ocvrs_return: *mut Result_void);
	// createGraphSegmentation(double, float, int) /usr/include/opencv2/ximgproc/segmentation.hpp:69
	pub fn cv_ximgproc_segmentation_createGraphSegmentation_double_float_int(sigma: f64, k: f32, min_size: i32, ocvrs_return: *mut Result<*mut c_void>);
	// createSelectiveSearchSegmentation() /usr/include/opencv2/ximgproc/segmentation.hpp:244
	pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentation(ocvrs_return: *mut Result<*mut c_void>);
	// createSelectiveSearchSegmentationStrategyColor() /usr/include/opencv2/ximgproc/segmentation.hpp:104
	pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyColor(ocvrs_return: *mut Result<*mut c_void>);
	// createSelectiveSearchSegmentationStrategyFill() /usr/include/opencv2/ximgproc/segmentation.hpp:131
	pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyFill(ocvrs_return: *mut Result<*mut c_void>);
	// createSelectiveSearchSegmentationStrategyMultiple() /usr/include/opencv2/ximgproc/segmentation.hpp:149
	pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple(ocvrs_return: *mut Result<*mut c_void>);
	// createSelectiveSearchSegmentationStrategyMultiple(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>) /usr/include/opencv2/ximgproc/segmentation.hpp:154
	pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_Ptr_SelectiveSearchSegmentationStrategy_(s1: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// createSelectiveSearchSegmentationStrategyMultiple(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>) /usr/include/opencv2/ximgproc/segmentation.hpp:160
	pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy_(s1: *mut c_void, s2: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// createSelectiveSearchSegmentationStrategyMultiple(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>) /usr/include/opencv2/ximgproc/segmentation.hpp:168
	pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy_(s1: *mut c_void, s2: *mut c_void, s3: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// createSelectiveSearchSegmentationStrategyMultiple(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>) /usr/include/opencv2/ximgproc/segmentation.hpp:176
	pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy_(s1: *mut c_void, s2: *mut c_void, s3: *mut c_void, s4: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// createSelectiveSearchSegmentationStrategySize() /usr/include/opencv2/ximgproc/segmentation.hpp:113
	pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategySize(ocvrs_return: *mut Result<*mut c_void>);
	// createSelectiveSearchSegmentationStrategyTexture() /usr/include/opencv2/ximgproc/segmentation.hpp:122
	pub fn cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyTexture(ocvrs_return: *mut Result<*mut c_void>);
	// thinning(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ximgproc.hpp:189
	pub fn cv_ximgproc_thinning_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, thinning_type: i32, ocvrs_return: *mut Result_void);
	// transformFD(cv::InputArray, cv::InputArray, cv::OutputArray, bool) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:97
	pub fn cv_ximgproc_transformFD_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(src: *const c_void, t: *const c_void, dst: *const c_void, fd_contour: bool, ocvrs_return: *mut Result_void);
	// weightedMedianFilter(cv::InputArray, cv::InputArray, cv::OutputArray, int, double, int, cv::InputArray) /usr/include/opencv2/ximgproc/weighted_median_filter.hpp:90
	pub fn cv_ximgproc_weightedMedianFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_int_const__InputArrayR(joint: *const c_void, src: *const c_void, dst: *const c_void, r: i32, sigma: f64, weight_type: i32, mask: *const c_void, ocvrs_return: *mut Result_void);
	// filter(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/ximgproc/edge_filter.hpp:214
	pub fn cv_ximgproc_AdaptiveManifoldFilter_filter_const__InputArrayR_const__OutputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, joint: *const c_void, ocvrs_return: *mut Result_void);
	// collectGarbage() /usr/include/opencv2/ximgproc/edge_filter.hpp:216
	pub fn cv_ximgproc_AdaptiveManifoldFilter_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// create() /usr/include/opencv2/ximgproc/edge_filter.hpp:218
	pub fn cv_ximgproc_AdaptiveManifoldFilter_create(ocvrs_return: *mut Result<*mut c_void>);
	// getSigmaS() /usr/include/opencv2/ximgproc/edge_filter.hpp:221
	pub fn cv_ximgproc_AdaptiveManifoldFilter_getSigmaS_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setSigmaS(double) /usr/include/opencv2/ximgproc/edge_filter.hpp:223
	pub fn cv_ximgproc_AdaptiveManifoldFilter_setSigmaS_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getSigmaR() /usr/include/opencv2/ximgproc/edge_filter.hpp:225
	pub fn cv_ximgproc_AdaptiveManifoldFilter_getSigmaR_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setSigmaR(double) /usr/include/opencv2/ximgproc/edge_filter.hpp:227
	pub fn cv_ximgproc_AdaptiveManifoldFilter_setSigmaR_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getTreeHeight() /usr/include/opencv2/ximgproc/edge_filter.hpp:229
	pub fn cv_ximgproc_AdaptiveManifoldFilter_getTreeHeight_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setTreeHeight(int) /usr/include/opencv2/ximgproc/edge_filter.hpp:231
	pub fn cv_ximgproc_AdaptiveManifoldFilter_setTreeHeight_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getPCAIterations() /usr/include/opencv2/ximgproc/edge_filter.hpp:233
	pub fn cv_ximgproc_AdaptiveManifoldFilter_getPCAIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setPCAIterations(int) /usr/include/opencv2/ximgproc/edge_filter.hpp:235
	pub fn cv_ximgproc_AdaptiveManifoldFilter_setPCAIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getAdjustOutliers() /usr/include/opencv2/ximgproc/edge_filter.hpp:237
	pub fn cv_ximgproc_AdaptiveManifoldFilter_getAdjustOutliers_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setAdjustOutliers(bool) /usr/include/opencv2/ximgproc/edge_filter.hpp:239
	pub fn cv_ximgproc_AdaptiveManifoldFilter_setAdjustOutliers_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// getUseRNG() /usr/include/opencv2/ximgproc/edge_filter.hpp:241
	pub fn cv_ximgproc_AdaptiveManifoldFilter_getUseRNG_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUseRNG(bool) /usr/include/opencv2/ximgproc/edge_filter.hpp:243
	pub fn cv_ximgproc_AdaptiveManifoldFilter_setUseRNG_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// ContourFitting(int, int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:38
	pub fn cv_ximgproc_ContourFitting_ContourFitting_int_int(ctr: i32, fd: i32, ocvrs_return: *mut Result<*mut c_void>);
	// estimateTransformation(cv::InputArray, cv::InputArray, cv::OutputArray, double *, bool) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:47
	pub fn cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_doubleX_bool(instance: *mut c_void, src: *const c_void, dst: *const c_void, alpha_phi_st: *const c_void, dist: *mut f64, fd_contour: bool, ocvrs_return: *mut Result_void);
	// estimateTransformation(cv::InputArray, cv::InputArray, cv::OutputArray, double &, bool) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:56
	pub fn cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_doubleR_bool(instance: *mut c_void, src: *const c_void, dst: *const c_void, alpha_phi_st: *const c_void, dist: *mut f64, fd_contour: bool, ocvrs_return: *mut Result_void);
	// setCtrSize(int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:61
	pub fn cv_ximgproc_ContourFitting_setCtrSize_int(instance: *mut c_void, n: i32, ocvrs_return: *mut Result_void);
	// setFDSize(int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:66
	pub fn cv_ximgproc_ContourFitting_setFDSize_int(instance: *mut c_void, n: i32, ocvrs_return: *mut Result_void);
	// getCtrSize() /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:70
	pub fn cv_ximgproc_ContourFitting_getCtrSize(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// getFDSize() /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:74
	pub fn cv_ximgproc_ContourFitting_getFDSize(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// filter(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:79
	pub fn cv_ximgproc_DTFilter_filter_const__InputArrayR_const__OutputArrayR_int(instance: *mut c_void, src: *const c_void, dst: *const c_void, d_depth: i32, ocvrs_return: *mut Result_void);
	// filter(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray, cv::Rect, cv::InputArray) /usr/include/opencv2/ximgproc/disparity_filter.hpp:75
	pub fn cv_ximgproc_DisparityFilter_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Rect_const__InputArrayR(instance: *mut c_void, disparity_map_left: *const c_void, left_view: *const c_void, filtered_disparity_map: *const c_void, disparity_map_right: *const c_void, roi: *const core::Rect, right_view: *const c_void, ocvrs_return: *mut Result_void);
	// getLambda() /usr/include/opencv2/ximgproc/disparity_filter.hpp:90
	pub fn cv_ximgproc_DisparityWLSFilter_getLambda(instance: *mut c_void, ocvrs_return: *mut Result<f64>);
	// setLambda(double) /usr/include/opencv2/ximgproc/disparity_filter.hpp:92
	pub fn cv_ximgproc_DisparityWLSFilter_setLambda_double(instance: *mut c_void, _lambda: f64, ocvrs_return: *mut Result_void);
	// getSigmaColor() /usr/include/opencv2/ximgproc/disparity_filter.hpp:97
	pub fn cv_ximgproc_DisparityWLSFilter_getSigmaColor(instance: *mut c_void, ocvrs_return: *mut Result<f64>);
	// setSigmaColor(double) /usr/include/opencv2/ximgproc/disparity_filter.hpp:99
	pub fn cv_ximgproc_DisparityWLSFilter_setSigmaColor_double(instance: *mut c_void, _sigma_color: f64, ocvrs_return: *mut Result_void);
	// getLRCthresh() /usr/include/opencv2/ximgproc/disparity_filter.hpp:106
	pub fn cv_ximgproc_DisparityWLSFilter_getLRCthresh(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// setLRCthresh(int) /usr/include/opencv2/ximgproc/disparity_filter.hpp:108
	pub fn cv_ximgproc_DisparityWLSFilter_setLRCthresh_int(instance: *mut c_void, _lrc_thresh: i32, ocvrs_return: *mut Result_void);
	// getDepthDiscontinuityRadius() /usr/include/opencv2/ximgproc/disparity_filter.hpp:112
	pub fn cv_ximgproc_DisparityWLSFilter_getDepthDiscontinuityRadius(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// setDepthDiscontinuityRadius(int) /usr/include/opencv2/ximgproc/disparity_filter.hpp:114
	pub fn cv_ximgproc_DisparityWLSFilter_setDepthDiscontinuityRadius_int(instance: *mut c_void, _disc_radius: i32, ocvrs_return: *mut Result_void);
	// getConfidenceMap() /usr/include/opencv2/ximgproc/disparity_filter.hpp:119
	pub fn cv_ximgproc_DisparityWLSFilter_getConfidenceMap(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getROI() /usr/include/opencv2/ximgproc/disparity_filter.hpp:122
	pub fn cv_ximgproc_DisparityWLSFilter_getROI(instance: *mut c_void, ocvrs_return: *mut Result<core::Rect>);
	// setCostMap(const cv::Mat &) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:88
	pub fn cv_ximgproc_EdgeAwareInterpolator_setCostMap_const_MatR(instance: *mut c_void, _cost_map: *const c_void, ocvrs_return: *mut Result_void);
	// setK(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:96
	pub fn cv_ximgproc_EdgeAwareInterpolator_setK_int(instance: *mut c_void, _k: i32, ocvrs_return: *mut Result_void);
	// getK() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:98
	pub fn cv_ximgproc_EdgeAwareInterpolator_getK(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// setSigma(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:104
	pub fn cv_ximgproc_EdgeAwareInterpolator_setSigma_float(instance: *mut c_void, _sigma: f32, ocvrs_return: *mut Result_void);
	// getSigma() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:106
	pub fn cv_ximgproc_EdgeAwareInterpolator_getSigma(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
	// setLambda(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:111
	pub fn cv_ximgproc_EdgeAwareInterpolator_setLambda_float(instance: *mut c_void, _lambda: f32, ocvrs_return: *mut Result_void);
	// getLambda() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:113
	pub fn cv_ximgproc_EdgeAwareInterpolator_getLambda(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
	// setUsePostProcessing(bool) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:118
	pub fn cv_ximgproc_EdgeAwareInterpolator_setUsePostProcessing_bool(instance: *mut c_void, _use_post_proc: bool, ocvrs_return: *mut Result_void);
	// getUsePostProcessing() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:120
	pub fn cv_ximgproc_EdgeAwareInterpolator_getUsePostProcessing(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
	// setFGSLambda(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:124
	pub fn cv_ximgproc_EdgeAwareInterpolator_setFGSLambda_float(instance: *mut c_void, _lambda: f32, ocvrs_return: *mut Result_void);
	// getFGSLambda() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:126
	pub fn cv_ximgproc_EdgeAwareInterpolator_getFGSLambda(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
	// setFGSSigma(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:129
	pub fn cv_ximgproc_EdgeAwareInterpolator_setFGSSigma_float(instance: *mut c_void, _sigma: f32, ocvrs_return: *mut Result_void);
	// getFGSSigma() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:131
	pub fn cv_ximgproc_EdgeAwareInterpolator_getFGSSigma(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
	// getBoundingBoxes(cv::InputArray, cv::InputArray, std::vector<Rect> &, cv::OutputArray) /usr/include/opencv2/ximgproc/edgeboxes.hpp:79
	pub fn cv_ximgproc_EdgeBoxes_getBoundingBoxes_const__InputArrayR_const__InputArrayR_vector_Rect_R_const__OutputArrayR(instance: *mut c_void, edge_map: *const c_void, orientation_map: *const c_void, boxes: *mut c_void, scores: *const c_void, ocvrs_return: *mut Result_void);
	// getAlpha() /usr/include/opencv2/ximgproc/edgeboxes.hpp:83
	pub fn cv_ximgproc_EdgeBoxes_getAlpha_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setAlpha(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:86
	pub fn cv_ximgproc_EdgeBoxes_setAlpha_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result_void);
	// getBeta() /usr/include/opencv2/ximgproc/edgeboxes.hpp:90
	pub fn cv_ximgproc_EdgeBoxes_getBeta_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setBeta(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:93
	pub fn cv_ximgproc_EdgeBoxes_setBeta_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result_void);
	// getEta() /usr/include/opencv2/ximgproc/edgeboxes.hpp:97
	pub fn cv_ximgproc_EdgeBoxes_getEta_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setEta(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:100
	pub fn cv_ximgproc_EdgeBoxes_setEta_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result_void);
	// getMinScore() /usr/include/opencv2/ximgproc/edgeboxes.hpp:104
	pub fn cv_ximgproc_EdgeBoxes_getMinScore_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setMinScore(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:107
	pub fn cv_ximgproc_EdgeBoxes_setMinScore_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result_void);
	// getMaxBoxes() /usr/include/opencv2/ximgproc/edgeboxes.hpp:111
	pub fn cv_ximgproc_EdgeBoxes_getMaxBoxes_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMaxBoxes(int) /usr/include/opencv2/ximgproc/edgeboxes.hpp:114
	pub fn cv_ximgproc_EdgeBoxes_setMaxBoxes_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result_void);
	// getEdgeMinMag() /usr/include/opencv2/ximgproc/edgeboxes.hpp:118
	pub fn cv_ximgproc_EdgeBoxes_getEdgeMinMag_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setEdgeMinMag(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:121
	pub fn cv_ximgproc_EdgeBoxes_setEdgeMinMag_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result_void);
	// getEdgeMergeThr() /usr/include/opencv2/ximgproc/edgeboxes.hpp:125
	pub fn cv_ximgproc_EdgeBoxes_getEdgeMergeThr_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setEdgeMergeThr(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:128
	pub fn cv_ximgproc_EdgeBoxes_setEdgeMergeThr_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result_void);
	// getClusterMinMag() /usr/include/opencv2/ximgproc/edgeboxes.hpp:132
	pub fn cv_ximgproc_EdgeBoxes_getClusterMinMag_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setClusterMinMag(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:135
	pub fn cv_ximgproc_EdgeBoxes_setClusterMinMag_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result_void);
	// getMaxAspectRatio() /usr/include/opencv2/ximgproc/edgeboxes.hpp:139
	pub fn cv_ximgproc_EdgeBoxes_getMaxAspectRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setMaxAspectRatio(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:142
	pub fn cv_ximgproc_EdgeBoxes_setMaxAspectRatio_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result_void);
	// getMinBoxArea() /usr/include/opencv2/ximgproc/edgeboxes.hpp:146
	pub fn cv_ximgproc_EdgeBoxes_getMinBoxArea_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setMinBoxArea(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:149
	pub fn cv_ximgproc_EdgeBoxes_setMinBoxArea_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result_void);
	// getGamma() /usr/include/opencv2/ximgproc/edgeboxes.hpp:153
	pub fn cv_ximgproc_EdgeBoxes_getGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setGamma(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:156
	pub fn cv_ximgproc_EdgeBoxes_setGamma_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result_void);
	// getKappa() /usr/include/opencv2/ximgproc/edgeboxes.hpp:160
	pub fn cv_ximgproc_EdgeBoxes_getKappa_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setKappa(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:163
	pub fn cv_ximgproc_EdgeBoxes_setKappa_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result_void);
	// params /usr/include/opencv2/ximgproc/edge_drawing.hpp:113
	pub fn cv_ximgproc_EdgeDrawing_getPropParams_const(instance: *const c_void, ocvrs_return: *mut crate::ximgproc::EdgeDrawing_Params);
	// params /usr/include/opencv2/ximgproc/edge_drawing.hpp:113
	pub fn cv_ximgproc_EdgeDrawing_setPropParams_Params(instance: *mut c_void, val: *const crate::ximgproc::EdgeDrawing_Params);
	// detectEdges(cv::InputArray) /usr/include/opencv2/ximgproc/edge_drawing.hpp:77
	pub fn cv_ximgproc_EdgeDrawing_detectEdges_const__InputArrayR(instance: *mut c_void, src: *const c_void, ocvrs_return: *mut Result_void);
	// getEdgeImage(cv::OutputArray) /usr/include/opencv2/ximgproc/edge_drawing.hpp:83
	pub fn cv_ximgproc_EdgeDrawing_getEdgeImage_const__OutputArrayR(instance: *mut c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// getGradientImage(cv::OutputArray) /usr/include/opencv2/ximgproc/edge_drawing.hpp:89
	pub fn cv_ximgproc_EdgeDrawing_getGradientImage_const__OutputArrayR(instance: *mut c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// getSegments() /usr/include/opencv2/ximgproc/edge_drawing.hpp:93
	pub fn cv_ximgproc_EdgeDrawing_getSegments(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getSegmentIndicesOfLines() /usr/include/opencv2/ximgproc/edge_drawing.hpp:97
	pub fn cv_ximgproc_EdgeDrawing_getSegmentIndicesOfLines_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// detectLines(cv::OutputArray) /usr/include/opencv2/ximgproc/edge_drawing.hpp:104
	pub fn cv_ximgproc_EdgeDrawing_detectLines_const__OutputArrayR(instance: *mut c_void, lines: *const c_void, ocvrs_return: *mut Result_void);
	// detectEllipses(cv::OutputArray) /usr/include/opencv2/ximgproc/edge_drawing.hpp:111
	pub fn cv_ximgproc_EdgeDrawing_detectEllipses_const__OutputArrayR(instance: *mut c_void, ellipses: *const c_void, ocvrs_return: *mut Result_void);
	// setParams(const EdgeDrawing::Params &) /usr/include/opencv2/ximgproc/edge_drawing.hpp:120
	pub fn cv_ximgproc_EdgeDrawing_setParams_const_ParamsR(instance: *mut c_void, parameters: *const crate::ximgproc::EdgeDrawing_Params, ocvrs_return: *mut Result_void);
	// Params() /usr/include/opencv2/ximgproc/edge_drawing.hpp:35
	pub fn cv_ximgproc_EdgeDrawing_Params_Params(ocvrs_return: *mut Result<crate::ximgproc::EdgeDrawing_Params>);
	// read(const cv::FileNode &) /usr/include/opencv2/ximgproc/edge_drawing.hpp:69
	pub fn cv_ximgproc_EdgeDrawing_Params_read_const_FileNodeR(instance: *const crate::ximgproc::EdgeDrawing_Params, fn_: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/ximgproc/edge_drawing.hpp:70
	pub fn cv_ximgproc_EdgeDrawing_Params_write_const_FileStorageR(instance: *const crate::ximgproc::EdgeDrawing_Params, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// filter(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/edge_filter.hpp:395
	pub fn cv_ximgproc_FastBilateralSolverFilter_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, confidence: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// filter(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/edge_filter.hpp:466
	pub fn cv_ximgproc_FastGlobalSmootherFilter_filter_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// detect(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/fast_line_detector.hpp:44
	pub fn cv_ximgproc_FastLineDetector_detect_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, lines: *const c_void, ocvrs_return: *mut Result_void);
	// drawSegments(cv::InputOutputArray, cv::InputArray, bool, cv::Scalar, int) /usr/include/opencv2/ximgproc/fast_line_detector.hpp:54
	pub fn cv_ximgproc_FastLineDetector_drawSegments_const__InputOutputArrayR_const__InputArrayR_bool_Scalar_int(instance: *mut c_void, image: *const c_void, lines: *const c_void, draw_arrow: bool, linecolor: *const core::Scalar, linethickness: i32, ocvrs_return: *mut Result_void);
	// filter(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:143
	pub fn cv_ximgproc_GuidedFilter_filter_const__InputArrayR_const__OutputArrayR_int(instance: *mut c_void, src: *const c_void, dst: *const c_void, d_depth: i32, ocvrs_return: *mut Result_void);
	// getFeatures(const cv::Mat &, cv::Mat &, const int, const int, const int, const int, const int) /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:83
	pub fn cv_ximgproc_RFFeatureGetter_getFeatures_const_const_MatR_MatR_const_int_const_int_const_int_const_int_const_int(instance: *const c_void, src: *const c_void, features: *mut c_void, gnrm_rad: i32, gsmth_rad: i32, shrink: i32, out_num: i32, grad_num: i32, ocvrs_return: *mut Result_void);
	// setK(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:153
	pub fn cv_ximgproc_RICInterpolator_setK_int(instance: *mut c_void, k: i32, ocvrs_return: *mut Result_void);
	// getK() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:157
	pub fn cv_ximgproc_RICInterpolator_getK_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setCostMap(const cv::Mat &) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:166
	pub fn cv_ximgproc_RICInterpolator_setCostMap_const_MatR(instance: *mut c_void, cost_map: *const c_void, ocvrs_return: *mut Result_void);
	// setSuperpixelSize(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:170
	pub fn cv_ximgproc_RICInterpolator_setSuperpixelSize_int(instance: *mut c_void, sp_size: i32, ocvrs_return: *mut Result_void);
	// getSuperpixelSize() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:174
	pub fn cv_ximgproc_RICInterpolator_getSuperpixelSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setSuperpixelNNCnt(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:178
	pub fn cv_ximgproc_RICInterpolator_setSuperpixelNNCnt_int(instance: *mut c_void, sp_nn: i32, ocvrs_return: *mut Result_void);
	// getSuperpixelNNCnt() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:182
	pub fn cv_ximgproc_RICInterpolator_getSuperpixelNNCnt_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setSuperpixelRuler(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:186
	pub fn cv_ximgproc_RICInterpolator_setSuperpixelRuler_float(instance: *mut c_void, ruler: f32, ocvrs_return: *mut Result_void);
	// getSuperpixelRuler() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:190
	pub fn cv_ximgproc_RICInterpolator_getSuperpixelRuler_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setSuperpixelMode(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:197
	pub fn cv_ximgproc_RICInterpolator_setSuperpixelMode_int(instance: *mut c_void, mode: i32, ocvrs_return: *mut Result_void);
	// getSuperpixelMode() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:201
	pub fn cv_ximgproc_RICInterpolator_getSuperpixelMode_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setAlpha(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:204
	pub fn cv_ximgproc_RICInterpolator_setAlpha_float(instance: *mut c_void, alpha: f32, ocvrs_return: *mut Result_void);
	// getAlpha() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:208
	pub fn cv_ximgproc_RICInterpolator_getAlpha_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setModelIter(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:211
	pub fn cv_ximgproc_RICInterpolator_setModelIter_int(instance: *mut c_void, model_iter: i32, ocvrs_return: *mut Result_void);
	// getModelIter() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:215
	pub fn cv_ximgproc_RICInterpolator_getModelIter_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setRefineModels(bool) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:218
	pub fn cv_ximgproc_RICInterpolator_setRefineModels_bool(instance: *mut c_void, refine_modles: bool, ocvrs_return: *mut Result_void);
	// getRefineModels() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:222
	pub fn cv_ximgproc_RICInterpolator_getRefineModels_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setMaxFlow(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:226
	pub fn cv_ximgproc_RICInterpolator_setMaxFlow_float(instance: *mut c_void, max_flow: f32, ocvrs_return: *mut Result_void);
	// getMaxFlow() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:230
	pub fn cv_ximgproc_RICInterpolator_getMaxFlow_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setUseVariationalRefinement(bool) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:233
	pub fn cv_ximgproc_RICInterpolator_setUseVariationalRefinement_bool(instance: *mut c_void, use_variational_refinement: bool, ocvrs_return: *mut Result_void);
	// getUseVariationalRefinement() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:237
	pub fn cv_ximgproc_RICInterpolator_getUseVariationalRefinement_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUseGlobalSmootherFilter(bool) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:240
	pub fn cv_ximgproc_RICInterpolator_setUseGlobalSmootherFilter_bool(instance: *mut c_void, use_fgs: bool, ocvrs_return: *mut Result_void);
	// getUseGlobalSmootherFilter() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:244
	pub fn cv_ximgproc_RICInterpolator_getUseGlobalSmootherFilter_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setFGSLambda(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:247
	pub fn cv_ximgproc_RICInterpolator_setFGSLambda_float(instance: *mut c_void, lambda: f32, ocvrs_return: *mut Result_void);
	// getFGSLambda() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:251
	pub fn cv_ximgproc_RICInterpolator_getFGSLambda_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setFGSSigma(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:254
	pub fn cv_ximgproc_RICInterpolator_setFGSSigma_float(instance: *mut c_void, sigma: f32, ocvrs_return: *mut Result_void);
	// getFGSSigma() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:258
	pub fn cv_ximgproc_RICInterpolator_getFGSSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// create(int, int, int, int, int, double, double, int) /usr/include/opencv2/ximgproc/ridgefilter.hpp:42
	pub fn cv_ximgproc_RidgeDetectionFilter_create_int_int_int_int_int_double_double_int(ddepth: i32, dx: i32, dy: i32, ksize: i32, out_dtype: i32, scale: f64, delta: f64, border_type: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getRidgeFilteredImage(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/ridgefilter.hpp:48
	pub fn cv_ximgproc_RidgeDetectionFilter_getRidgeFilteredImage_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, _img: *const c_void, out: *const c_void, ocvrs_return: *mut Result_void);
	// getNumberOfSuperpixels() /usr/include/opencv2/ximgproc/scansegment.hpp:32
	pub fn cv_ximgproc_ScanSegment_getNumberOfSuperpixels(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// iterate(cv::InputArray) /usr/include/opencv2/ximgproc/scansegment.hpp:43
	pub fn cv_ximgproc_ScanSegment_iterate_const__InputArrayR(instance: *mut c_void, img: *const c_void, ocvrs_return: *mut Result_void);
	// getLabels(cv::OutputArray) /usr/include/opencv2/ximgproc/scansegment.hpp:52
	pub fn cv_ximgproc_ScanSegment_getLabels_const__OutputArrayR(instance: *mut c_void, labels_out: *const c_void, ocvrs_return: *mut Result_void);
	// getLabelContourMask(cv::OutputArray, bool) /usr/include/opencv2/ximgproc/scansegment.hpp:61
	pub fn cv_ximgproc_ScanSegment_getLabelContourMask_const__OutputArrayR_bool(instance: *mut c_void, image: *const c_void, thick_line: bool, ocvrs_return: *mut Result_void);
	// interpolate(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:69
	pub fn cv_ximgproc_SparseMatchInterpolator_interpolate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, from_image: *const c_void, from_points: *const c_void, to_image: *const c_void, to_points: *const c_void, dense_flow: *const c_void, ocvrs_return: *mut Result_void);
	// detectEdges(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:109
	pub fn cv_ximgproc_StructuredEdgeDetection_detectEdges_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// computeOrientation(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:116
	pub fn cv_ximgproc_StructuredEdgeDetection_computeOrientation_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// edgesNms(cv::InputArray, cv::InputArray, cv::OutputArray, int, int, float, bool) /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:129
	pub fn cv_ximgproc_StructuredEdgeDetection_edgesNms_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_float_bool(instance: *const c_void, edge_image: *const c_void, orientation_image: *const c_void, dst: *const c_void, r: i32, s: i32, m: f32, is_parallel: bool, ocvrs_return: *mut Result_void);
	// getNumberOfSuperpixels() /usr/include/opencv2/ximgproc/lsc.hpp:78
	pub fn cv_ximgproc_SuperpixelLSC_getNumberOfSuperpixels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// iterate(int) /usr/include/opencv2/ximgproc/lsc.hpp:94
	pub fn cv_ximgproc_SuperpixelLSC_iterate_int(instance: *mut c_void, num_iterations: i32, ocvrs_return: *mut Result_void);
	// getLabels(cv::OutputArray) /usr/include/opencv2/ximgproc/lsc.hpp:106
	pub fn cv_ximgproc_SuperpixelLSC_getLabels_const_const__OutputArrayR(instance: *const c_void, labels_out: *const c_void, ocvrs_return: *mut Result_void);
	// getLabelContourMask(cv::OutputArray, bool) /usr/include/opencv2/ximgproc/lsc.hpp:118
	pub fn cv_ximgproc_SuperpixelLSC_getLabelContourMask_const_const__OutputArrayR_bool(instance: *const c_void, image: *const c_void, thick_line: bool, ocvrs_return: *mut Result_void);
	// enforceLabelConnectivity(int) /usr/include/opencv2/ximgproc/lsc.hpp:129
	pub fn cv_ximgproc_SuperpixelLSC_enforceLabelConnectivity_int(instance: *mut c_void, min_element_size: i32, ocvrs_return: *mut Result_void);
	// getNumberOfSuperpixels() /usr/include/opencv2/ximgproc/seeds.hpp:75
	pub fn cv_ximgproc_SuperpixelSEEDS_getNumberOfSuperpixels(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// iterate(cv::InputArray, int) /usr/include/opencv2/ximgproc/seeds.hpp:99
	pub fn cv_ximgproc_SuperpixelSEEDS_iterate_const__InputArrayR_int(instance: *mut c_void, img: *const c_void, num_iterations: i32, ocvrs_return: *mut Result_void);
	// getLabels(cv::OutputArray) /usr/include/opencv2/ximgproc/seeds.hpp:111
	pub fn cv_ximgproc_SuperpixelSEEDS_getLabels_const__OutputArrayR(instance: *mut c_void, labels_out: *const c_void, ocvrs_return: *mut Result_void);
	// getLabelContourMask(cv::OutputArray, bool) /usr/include/opencv2/ximgproc/seeds.hpp:139
	pub fn cv_ximgproc_SuperpixelSEEDS_getLabelContourMask_const__OutputArrayR_bool(instance: *mut c_void, image: *const c_void, thick_line: bool, ocvrs_return: *mut Result_void);
	// getNumberOfSuperpixels() /usr/include/opencv2/ximgproc/slic.hpp:85
	pub fn cv_ximgproc_SuperpixelSLIC_getNumberOfSuperpixels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// iterate(int) /usr/include/opencv2/ximgproc/slic.hpp:101
	pub fn cv_ximgproc_SuperpixelSLIC_iterate_int(instance: *mut c_void, num_iterations: i32, ocvrs_return: *mut Result_void);
	// getLabels(cv::OutputArray) /usr/include/opencv2/ximgproc/slic.hpp:113
	pub fn cv_ximgproc_SuperpixelSLIC_getLabels_const_const__OutputArrayR(instance: *const c_void, labels_out: *const c_void, ocvrs_return: *mut Result_void);
	// getLabelContourMask(cv::OutputArray, bool) /usr/include/opencv2/ximgproc/slic.hpp:125
	pub fn cv_ximgproc_SuperpixelSLIC_getLabelContourMask_const_const__OutputArrayR_bool(instance: *const c_void, image: *const c_void, thick_line: bool, ocvrs_return: *mut Result_void);
	// enforceLabelConnectivity(int) /usr/include/opencv2/ximgproc/slic.hpp:136
	pub fn cv_ximgproc_SuperpixelSLIC_enforceLabelConnectivity_int(instance: *mut c_void, min_element_size: i32, ocvrs_return: *mut Result_void);
	// processImage(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/segmentation.hpp:52
	pub fn cv_ximgproc_segmentation_GraphSegmentation_processImage_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// setSigma(double) /usr/include/opencv2/ximgproc/segmentation.hpp:54
	pub fn cv_ximgproc_segmentation_GraphSegmentation_setSigma_double(instance: *mut c_void, sigma: f64, ocvrs_return: *mut Result_void);
	// getSigma() /usr/include/opencv2/ximgproc/segmentation.hpp:55
	pub fn cv_ximgproc_segmentation_GraphSegmentation_getSigma(instance: *mut c_void, ocvrs_return: *mut Result<f64>);
	// setK(float) /usr/include/opencv2/ximgproc/segmentation.hpp:57
	pub fn cv_ximgproc_segmentation_GraphSegmentation_setK_float(instance: *mut c_void, k: f32, ocvrs_return: *mut Result_void);
	// getK() /usr/include/opencv2/ximgproc/segmentation.hpp:58
	pub fn cv_ximgproc_segmentation_GraphSegmentation_getK(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
	// setMinSize(int) /usr/include/opencv2/ximgproc/segmentation.hpp:60
	pub fn cv_ximgproc_segmentation_GraphSegmentation_setMinSize_int(instance: *mut c_void, min_size: i32, ocvrs_return: *mut Result_void);
	// getMinSize() /usr/include/opencv2/ximgproc/segmentation.hpp:61
	pub fn cv_ximgproc_segmentation_GraphSegmentation_getMinSize(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// setBaseImage(cv::InputArray) /usr/include/opencv2/ximgproc/segmentation.hpp:187
	pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_setBaseImage_const__InputArrayR(instance: *mut c_void, img: *const c_void, ocvrs_return: *mut Result_void);
	// switchToSingleStrategy(int, float) /usr/include/opencv2/ximgproc/segmentation.hpp:193
	pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSingleStrategy_int_float(instance: *mut c_void, k: i32, sigma: f32, ocvrs_return: *mut Result_void);
	// switchToSelectiveSearchFast(int, int, float) /usr/include/opencv2/ximgproc/segmentation.hpp:200
	pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchFast_int_int_float(instance: *mut c_void, base_k: i32, inc_k: i32, sigma: f32, ocvrs_return: *mut Result_void);
	// switchToSelectiveSearchQuality(int, int, float) /usr/include/opencv2/ximgproc/segmentation.hpp:207
	pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchQuality_int_int_float(instance: *mut c_void, base_k: i32, inc_k: i32, sigma: f32, ocvrs_return: *mut Result_void);
	// addImage(cv::InputArray) /usr/include/opencv2/ximgproc/segmentation.hpp:212
	pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_addImage_const__InputArrayR(instance: *mut c_void, img: *const c_void, ocvrs_return: *mut Result_void);
	// clearImages() /usr/include/opencv2/ximgproc/segmentation.hpp:216
	pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearImages(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// addGraphSegmentation(Ptr<cv::ximgproc::segmentation::GraphSegmentation>) /usr/include/opencv2/ximgproc/segmentation.hpp:221
	pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_addGraphSegmentation_Ptr_GraphSegmentation_(instance: *mut c_void, g: *mut c_void, ocvrs_return: *mut Result_void);
	// clearGraphSegmentations() /usr/include/opencv2/ximgproc/segmentation.hpp:225
	pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearGraphSegmentations(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// addStrategy(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>) /usr/include/opencv2/ximgproc/segmentation.hpp:230
	pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_addStrategy_Ptr_SelectiveSearchSegmentationStrategy_(instance: *mut c_void, s: *mut c_void, ocvrs_return: *mut Result_void);
	// clearStrategies() /usr/include/opencv2/ximgproc/segmentation.hpp:234
	pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearStrategies(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// process(std::vector<Rect> &) /usr/include/opencv2/ximgproc/segmentation.hpp:239
	pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentation_process_vector_Rect_R(instance: *mut c_void, rects: *mut c_void, ocvrs_return: *mut Result_void);
	// setImage(cv::InputArray, cv::InputArray, cv::InputArray, int) /usr/include/opencv2/ximgproc/segmentation.hpp:82
	pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_setImage_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(instance: *mut c_void, img: *const c_void, regions: *const c_void, sizes: *const c_void, image_id: i32, ocvrs_return: *mut Result_void);
	// get(int, int) /usr/include/opencv2/ximgproc/segmentation.hpp:88
	pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_get_int_int(instance: *mut c_void, r1: i32, r2: i32, ocvrs_return: *mut Result<f32>);
	// merge(int, int) /usr/include/opencv2/ximgproc/segmentation.hpp:94
	pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_merge_int_int(instance: *mut c_void, r1: i32, r2: i32, ocvrs_return: *mut Result_void);
	// addStrategy(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, float) /usr/include/opencv2/ximgproc/segmentation.hpp:142
	pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_addStrategy_Ptr_SelectiveSearchSegmentationStrategy__float(instance: *mut c_void, g: *mut c_void, weight: f32, ocvrs_return: *mut Result_void);
	// clearStrategies() /usr/include/opencv2/ximgproc/segmentation.hpp:145
	pub fn cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_clearStrategies(instance: *mut c_void, ocvrs_return: *mut Result_void);
}
