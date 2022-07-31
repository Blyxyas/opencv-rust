extern "C" {
	// Canny(cv::InputArray, cv::InputArray, cv::OutputArray, double, double, bool) /usr/include/opencv2/imgproc.hpp:1854
	pub fn cv_Canny_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_bool(dx: *const c_void, dy: *const c_void, edges: *const c_void, threshold1: f64, threshold2: f64, l2gradient: bool, ocvrs_return: *mut Result_void);
	// Canny(cv::InputArray, cv::OutputArray, double, double, int, bool) /usr/include/opencv2/imgproc.hpp:1836
	pub fn cv_Canny_const__InputArrayR_const__OutputArrayR_double_double_int_bool(image: *const c_void, edges: *const c_void, threshold1: f64, threshold2: f64, aperture_size: i32, l2gradient: bool, ocvrs_return: *mut Result_void);
	// EMD(cv::InputArray, cv::InputArray, int, cv::InputArray, float *, cv::OutputArray) /usr/include/opencv2/imgproc.hpp:3320
	pub fn cv_EMD_const__InputArrayR_const__InputArrayR_int_const__InputArrayR_floatX_const__OutputArrayR(signature1: *const c_void, signature2: *const c_void, dist_type: i32, cost: *const c_void, lower_bound: *mut f32, flow: *const c_void, ocvrs_return: *mut Result<f32>);
	// GaussianBlur(cv::InputArray, cv::OutputArray, cv::Size, double, double, int) /usr/include/opencv2/imgproc.hpp:1509
	pub fn cv_GaussianBlur_const__InputArrayR_const__OutputArrayR_Size_double_double_int(src: *const c_void, dst: *const c_void, ksize: *const core::Size, sigma_x: f64, sigma_y: f64, border_type: i32, ocvrs_return: *mut Result_void);
	// HoughCircles(cv::InputArray, cv::OutputArray, int, double, double, double, double, int, int) /usr/include/opencv2/imgproc.hpp:2221
	pub fn cv_HoughCircles_const__InputArrayR_const__OutputArrayR_int_double_double_double_double_int_int(image: *const c_void, circles: *const c_void, method: i32, dp: f64, min_dist: f64, param1: f64, param2: f64, min_radius: i32, max_radius: i32, ocvrs_return: *mut Result_void);
	// HoughLinesP(cv::InputArray, cv::OutputArray, double, double, int, double, double) /usr/include/opencv2/imgproc.hpp:2149
	pub fn cv_HoughLinesP_const__InputArrayR_const__OutputArrayR_double_double_int_double_double(image: *const c_void, lines: *const c_void, rho: f64, theta: f64, threshold: i32, min_line_length: f64, max_line_gap: f64, ocvrs_return: *mut Result_void);
	// HoughLinesPointSet(cv::InputArray, cv::OutputArray, int, int, double, double, double, double, double, double) /usr/include/opencv2/imgproc.hpp:2170
	pub fn cv_HoughLinesPointSet_const__InputArrayR_const__OutputArrayR_int_int_double_double_double_double_double_double(point: *const c_void, lines: *const c_void, lines_max: i32, threshold: i32, min_rho: f64, max_rho: f64, rho_step: f64, min_theta: f64, max_theta: f64, theta_step: f64, ocvrs_return: *mut Result_void);
	// HoughLines(cv::InputArray, cv::OutputArray, double, double, int, double, double, double, double) /usr/include/opencv2/imgproc.hpp:2116
	pub fn cv_HoughLines_const__InputArrayR_const__OutputArrayR_double_double_int_double_double_double_double(image: *const c_void, lines: *const c_void, rho: f64, theta: f64, threshold: i32, srn: f64, stn: f64, min_theta: f64, max_theta: f64, ocvrs_return: *mut Result_void);
	// HuMoments(const cv::Moments &, cv::OutputArray) /usr/include/opencv2/imgproc.hpp:3764
	pub fn cv_HuMoments_const_MomentsR_const__OutputArrayR(m: *const core::Moments, hu: *const c_void, ocvrs_return: *mut Result_void);
	// HuMoments(const cv::Moments &, double *) /usr/include/opencv2/imgproc.hpp:3761
	pub fn cv_HuMoments_const_MomentsR_doubleXX(moments: *const core::Moments, hu: *mut [f64; 7], ocvrs_return: *mut Result_void);
	// Laplacian(cv::InputArray, cv::OutputArray, int, int, double, double, int) /usr/include/opencv2/imgproc.hpp:1804
	pub fn cv_Laplacian_const__InputArrayR_const__OutputArrayR_int_int_double_double_int(src: *const c_void, dst: *const c_void, ddepth: i32, ksize: i32, scale: f64, delta: f64, border_type: i32, ocvrs_return: *mut Result_void);
	// Scharr(cv::InputArray, cv::OutputArray, int, int, int, double, double, int) /usr/include/opencv2/imgproc.hpp:1773
	pub fn cv_Scharr_const__InputArrayR_const__OutputArrayR_int_int_int_double_double_int(src: *const c_void, dst: *const c_void, ddepth: i32, dx: i32, dy: i32, scale: f64, delta: f64, border_type: i32, ocvrs_return: *mut Result_void);
	// Sobel(cv::InputArray, cv::OutputArray, int, int, int, int, double, double, int) /usr/include/opencv2/imgproc.hpp:1723
	pub fn cv_Sobel_const__InputArrayR_const__OutputArrayR_int_int_int_int_double_double_int(src: *const c_void, dst: *const c_void, ddepth: i32, dx: i32, dy: i32, ksize: i32, scale: f64, delta: f64, border_type: i32, ocvrs_return: *mut Result_void);
	// accumulateProduct(cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/imgproc.hpp:2898
	pub fn cv_accumulateProduct_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// accumulateSquare(cv::InputArray, cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/imgproc.hpp:2879
	pub fn cv_accumulateSquare_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// accumulateWeighted(cv::InputArray, cv::InputOutputArray, double, cv::InputArray) /usr/include/opencv2/imgproc.hpp:2919
	pub fn cv_accumulateWeighted_const__InputArrayR_const__InputOutputArrayR_double_const__InputArrayR(src: *const c_void, dst: *const c_void, alpha: f64, mask: *const c_void, ocvrs_return: *mut Result_void);
	// accumulate(cv::InputArray, cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/imgproc.hpp:2860
	pub fn cv_accumulate_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// adaptiveThreshold(cv::InputArray, cv::OutputArray, double, int, int, int, double) /usr/include/opencv2/imgproc.hpp:3051
	pub fn cv_adaptiveThreshold_const__InputArrayR_const__OutputArrayR_double_int_int_int_double(src: *const c_void, dst: *const c_void, max_value: f64, adaptive_method: i32, threshold_type: i32, block_size: i32, c: f64, ocvrs_return: *mut Result_void);
	// applyColorMap(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/imgproc.hpp:4421
	pub fn cv_applyColorMap_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, user_color: *const c_void, ocvrs_return: *mut Result_void);
	// applyColorMap(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/imgproc.hpp:4413
	pub fn cv_applyColorMap_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, colormap: i32, ocvrs_return: *mut Result_void);
	// approxPolyDP(cv::InputArray, cv::OutputArray, double, bool) /usr/include/opencv2/imgproc.hpp:3989
	pub fn cv_approxPolyDP_const__InputArrayR_const__OutputArrayR_double_bool(curve: *const c_void, approx_curve: *const c_void, epsilon: f64, closed: bool, ocvrs_return: *mut Result_void);
	// arcLength(cv::InputArray, bool) /usr/include/opencv2/imgproc.hpp:4000
	pub fn cv_arcLength_const__InputArrayR_bool(curve: *const c_void, closed: bool, ocvrs_return: *mut Result<f64>);
	// arrowedLine(cv::InputOutputArray, cv::Point, cv::Point, const cv::Scalar &, int, int, int, double) /usr/include/opencv2/imgproc.hpp:4463
	pub fn cv_arrowedLine_const__InputOutputArrayR_Point_Point_const_ScalarR_int_int_int_double(img: *const c_void, pt1: *const core::Point, pt2: *const core::Point, color: *const core::Scalar, thickness: i32, line_type: i32, shift: i32, tip_length: f64, ocvrs_return: *mut Result_void);
	// bilateralFilter(cv::InputArray, cv::OutputArray, int, double, double, int) /usr/include/opencv2/imgproc.hpp:1541
	pub fn cv_bilateralFilter_const__InputArrayR_const__OutputArrayR_int_double_double_int(src: *const c_void, dst: *const c_void, d: i32, sigma_color: f64, sigma_space: f64, border_type: i32, ocvrs_return: *mut Result_void);
	// blendLinear(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/imgproc.hpp:3617
	pub fn cv_blendLinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, weights1: *const c_void, weights2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// blur(cv::InputArray, cv::OutputArray, cv::Size, cv::Point, int) /usr/include/opencv2/imgproc.hpp:1615
	pub fn cv_blur_const__InputArrayR_const__OutputArrayR_Size_Point_int(src: *const c_void, dst: *const c_void, ksize: *const core::Size, anchor: *const core::Point, border_type: i32, ocvrs_return: *mut Result_void);
	// boundingRect(cv::InputArray) /usr/include/opencv2/imgproc.hpp:4009
	pub fn cv_boundingRect_const__InputArrayR(array: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// boxFilter(cv::InputArray, cv::OutputArray, int, cv::Size, cv::Point, bool, int) /usr/include/opencv2/imgproc.hpp:1569
	pub fn cv_boxFilter_const__InputArrayR_const__OutputArrayR_int_Size_Point_bool_int(src: *const c_void, dst: *const c_void, ddepth: i32, ksize: *const core::Size, anchor: *const core::Point, normalize: bool, border_type: i32, ocvrs_return: *mut Result_void);
	// boxPoints(cv::RotatedRect, cv::OutputArray) /usr/include/opencv2/imgproc.hpp:4062
	pub fn cv_boxPoints_RotatedRect_const__OutputArrayR(box_: *mut c_void, points: *const c_void, ocvrs_return: *mut Result_void);
	// buildPyramid(cv::InputArray, cv::OutputArrayOfArrays, int, int) /usr/include/opencv2/imgproc.hpp:3117
	pub fn cv_buildPyramid_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, maxlevel: i32, border_type: i32, ocvrs_return: *mut Result_void);
	// calcBackProject(cv::InputArrayOfArrays, const std::vector<int> &, cv::InputArray, cv::OutputArray, const std::vector<float> &, double) /usr/include/opencv2/imgproc.hpp:3236
	pub fn cv_calcBackProject_const__InputArrayR_const_vector_int_R_const__InputArrayR_const__OutputArrayR_const_vector_float_R_double(images: *const c_void, channels: *const c_void, hist: *const c_void, dst: *const c_void, ranges: *const c_void, scale: f64, ocvrs_return: *mut Result_void);
	// calcHist(cv::InputArrayOfArrays, const std::vector<int> &, cv::InputArray, cv::OutputArray, const std::vector<int> &, const std::vector<float> &, bool) /usr/include/opencv2/imgproc.hpp:3178
	pub fn cv_calcHist_const__InputArrayR_const_vector_int_R_const__InputArrayR_const__OutputArrayR_const_vector_int_R_const_vector_float_R_bool(images: *const c_void, channels: *const c_void, mask: *const c_void, hist: *const c_void, hist_size: *const c_void, ranges: *const c_void, accumulate: bool, ocvrs_return: *mut Result_void);
	// circle(cv::InputOutputArray, cv::Point, int, const cv::Scalar &, int, int, int) /usr/include/opencv2/imgproc.hpp:4509
	pub fn cv_circle_const__InputOutputArrayR_Point_int_const_ScalarR_int_int_int(img: *const c_void, center: *const core::Point, radius: i32, color: *const core::Scalar, thickness: i32, line_type: i32, shift: i32, ocvrs_return: *mut Result_void);
	// clipLine(cv::Rect, cv::Point &, cv::Point &) /usr/include/opencv2/imgproc.hpp:4718
	pub fn cv_clipLine_Rect_PointR_PointR(img_rect: *const core::Rect, pt1: *mut core::Point, pt2: *mut core::Point, ocvrs_return: *mut Result<bool>);
	// clipLine(cv::Size2l, cv::Point2l &, cv::Point2l &) /usr/include/opencv2/imgproc.hpp:4711
	pub fn cv_clipLine_Size2l_Point2lR_Point2lR(img_size: *const core::Size2l, pt1: *mut core::Point2l, pt2: *mut core::Point2l, ocvrs_return: *mut Result<bool>);
	// clipLine(cv::Size, cv::Point &, cv::Point &) /usr/include/opencv2/imgproc.hpp:4704
	pub fn cv_clipLine_Size_PointR_PointR(img_size: *const core::Size, pt1: *mut core::Point, pt2: *mut core::Point, ocvrs_return: *mut Result<bool>);
	// compareHist(const cv::SparseMat &, const cv::SparseMat &, int) /usr/include/opencv2/imgproc.hpp:3259
	pub fn cv_compareHist_const_SparseMatR_const_SparseMatR_int(h1: *const c_void, h2: *const c_void, method: i32, ocvrs_return: *mut Result<f64>);
	// compareHist(cv::InputArray, cv::InputArray, int) /usr/include/opencv2/imgproc.hpp:3256
	pub fn cv_compareHist_const__InputArrayR_const__InputArrayR_int(h1: *const c_void, h2: *const c_void, method: i32, ocvrs_return: *mut Result<f64>);
	// connectedComponentsWithStats(cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray, int, int) /usr/include/opencv2/imgproc.hpp:3927
	pub fn cv_connectedComponentsWithStats_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(image: *const c_void, labels: *const c_void, stats: *const c_void, centroids: *const c_void, connectivity: i32, ltype: i32, ocvrs_return: *mut Result<i32>);
	// connectedComponentsWithStats(cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray, int, int, int) /usr/include/opencv2/imgproc.hpp:3912
	pub fn cv_connectedComponentsWithStats_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_int(image: *const c_void, labels: *const c_void, stats: *const c_void, centroids: *const c_void, connectivity: i32, ltype: i32, ccltype: i32, ocvrs_return: *mut Result<i32>);
	// connectedComponents(cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/imgproc.hpp:3885
	pub fn cv_connectedComponents_const__InputArrayR_const__OutputArrayR_int_int(image: *const c_void, labels: *const c_void, connectivity: i32, ltype: i32, ocvrs_return: *mut Result<i32>);
	// connectedComponents(cv::InputArray, cv::OutputArray, int, int, int) /usr/include/opencv2/imgproc.hpp:3874
	pub fn cv_connectedComponents_const__InputArrayR_const__OutputArrayR_int_int_int(image: *const c_void, labels: *const c_void, connectivity: i32, ltype: i32, ccltype: i32, ocvrs_return: *mut Result<i32>);
	// contourArea(cv::InputArray, bool) /usr/include/opencv2/imgproc.hpp:4041
	pub fn cv_contourArea_const__InputArrayR_bool(contour: *const c_void, oriented: bool, ocvrs_return: *mut Result<f64>);
	// convertMaps(cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray, int, bool) /usr/include/opencv2/imgproc.hpp:2498
	pub fn cv_convertMaps_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_bool(map1: *const c_void, map2: *const c_void, dstmap1: *const c_void, dstmap2: *const c_void, dstmap1type: i32, nninterpolation: bool, ocvrs_return: *mut Result_void);
	// convexHull(cv::InputArray, cv::OutputArray, bool, bool) /usr/include/opencv2/imgproc.hpp:4142
	pub fn cv_convexHull_const__InputArrayR_const__OutputArrayR_bool_bool(points: *const c_void, hull: *const c_void, clockwise: bool, return_points: bool, ocvrs_return: *mut Result_void);
	// convexityDefects(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/imgproc.hpp:4162
	pub fn cv_convexityDefects_const__InputArrayR_const__InputArrayR_const__OutputArrayR(contour: *const c_void, convexhull: *const c_void, convexity_defects: *const c_void, ocvrs_return: *mut Result_void);
	// cornerEigenValsAndVecs(cv::InputArray, cv::OutputArray, int, int, int) /usr/include/opencv2/imgproc.hpp:1925
	pub fn cv_cornerEigenValsAndVecs_const__InputArrayR_const__OutputArrayR_int_int_int(src: *const c_void, dst: *const c_void, block_size: i32, ksize: i32, border_type: i32, ocvrs_return: *mut Result_void);
	// cornerHarris(cv::InputArray, cv::OutputArray, int, int, double, int) /usr/include/opencv2/imgproc.hpp:1895
	pub fn cv_cornerHarris_const__InputArrayR_const__OutputArrayR_int_int_double_int(src: *const c_void, dst: *const c_void, block_size: i32, ksize: i32, k: f64, border_type: i32, ocvrs_return: *mut Result_void);
	// cornerMinEigenVal(cv::InputArray, cv::OutputArray, int, int, int) /usr/include/opencv2/imgproc.hpp:1872
	pub fn cv_cornerMinEigenVal_const__InputArrayR_const__OutputArrayR_int_int_int(src: *const c_void, dst: *const c_void, block_size: i32, ksize: i32, border_type: i32, ocvrs_return: *mut Result_void);
	// cornerSubPix(cv::InputArray, cv::InputOutputArray, cv::Size, cv::Size, cv::TermCriteria) /usr/include/opencv2/imgproc.hpp:1995
	pub fn cv_cornerSubPix_const__InputArrayR_const__InputOutputArrayR_Size_Size_TermCriteria(image: *const c_void, corners: *const c_void, win_size: *const core::Size, zero_zone: *const core::Size, criteria: *const core::TermCriteria, ocvrs_return: *mut Result_void);
	// createCLAHE(double, cv::Size) /usr/include/opencv2/imgproc.hpp:3284
	pub fn cv_createCLAHE_double_Size(clip_limit: f64, tile_grid_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
	// createGeneralizedHoughBallard() /usr/include/opencv2/imgproc.hpp:4365
	pub fn cv_createGeneralizedHoughBallard(ocvrs_return: *mut Result<*mut c_void>);
	// createGeneralizedHoughGuil() /usr/include/opencv2/imgproc.hpp:4369
	pub fn cv_createGeneralizedHoughGuil(ocvrs_return: *mut Result<*mut c_void>);
	// createHanningWindow(cv::OutputArray, cv::Size, int) /usr/include/opencv2/imgproc.hpp:2975
	pub fn cv_createHanningWindow_const__OutputArrayR_Size_int(dst: *const c_void, win_size: *const core::Size, typ: i32, ocvrs_return: *mut Result_void);
	// createLineSegmentDetector(int, double, double, double, double, double, double, int) /usr/include/opencv2/imgproc.hpp:1381
	pub fn cv_createLineSegmentDetector_int_double_double_double_double_double_double_int(refine: i32, scale: f64, sigma_scale: f64, quant: f64, ang_th: f64, log_eps: f64, density_th: f64, n_bins: i32, ocvrs_return: *mut Result<*mut c_void>);
	// cvtColorTwoPlane(cv::InputArray, cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/imgproc.hpp:3685
	pub fn cv_cvtColorTwoPlane_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(src1: *const c_void, src2: *const c_void, dst: *const c_void, code: i32, ocvrs_return: *mut Result_void);
	// cvtColor(cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/imgproc.hpp:3665
	pub fn cv_cvtColor_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, code: i32, dst_cn: i32, ocvrs_return: *mut Result_void);
	// demosaicing(cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/imgproc.hpp:3717
	pub fn cv_demosaicing_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, code: i32, dst_cn: i32, ocvrs_return: *mut Result_void);
	// dilate(cv::InputArray, cv::OutputArray, cv::InputArray, cv::Point, int, int, const cv::Scalar &) /usr/include/opencv2/imgproc.hpp:2291
	pub fn cv_dilate_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Point_int_int_const_ScalarR(src: *const c_void, dst: *const c_void, kernel: *const c_void, anchor: *const core::Point, iterations: i32, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result_void);
	// distanceTransform(cv::InputArray, cv::OutputArray, cv::OutputArray, int, int, int) /usr/include/opencv2/imgproc.hpp:3502
	pub fn cv_distanceTransform_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_int(src: *const c_void, dst: *const c_void, labels: *const c_void, distance_type: i32, mask_size: i32, label_type: i32, ocvrs_return: *mut Result_void);
	// distanceTransform(cv::InputArray, cv::OutputArray, int, int, int) /usr/include/opencv2/imgproc.hpp:3517
	pub fn cv_distanceTransform_const__InputArrayR_const__OutputArrayR_int_int_int(src: *const c_void, dst: *const c_void, distance_type: i32, mask_size: i32, dst_type: i32, ocvrs_return: *mut Result_void);
	// divSpectrums(cv::InputArray, cv::InputArray, cv::OutputArray, int, bool) /usr/include/opencv2/imgproc.hpp:2990
	pub fn cv_divSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_bool(a: *const c_void, b: *const c_void, c: *const c_void, flags: i32, conj_b: bool, ocvrs_return: *mut Result_void);
	// drawContours(cv::InputOutputArray, cv::InputArrayOfArrays, int, const cv::Scalar &, int, int, cv::InputArray, int, cv::Point) /usr/include/opencv2/imgproc.hpp:4689
	pub fn cv_drawContours_const__InputOutputArrayR_const__InputArrayR_int_const_ScalarR_int_int_const__InputArrayR_int_Point(image: *const c_void, contours: *const c_void, contour_idx: i32, color: *const core::Scalar, thickness: i32, line_type: i32, hierarchy: *const c_void, max_level: i32, offset: *const core::Point, ocvrs_return: *mut Result_void);
	// drawMarker(cv::InputOutputArray, cv::Point, const cv::Scalar &, int, int, int, int) /usr/include/opencv2/imgproc.hpp:4572
	pub fn cv_drawMarker_const__InputOutputArrayR_Point_const_ScalarR_int_int_int_int(img: *const c_void, position: *const core::Point, color: *const core::Scalar, marker_type: i32, marker_size: i32, thickness: i32, line_type: i32, ocvrs_return: *mut Result_void);
	// ellipse2Poly(cv::Point2d, cv::Size2d, int, int, int, int, std::vector<Point2d> &) /usr/include/opencv2/imgproc.hpp:4747
	pub fn cv_ellipse2Poly_Point2d_Size2d_int_int_int_int_vector_Point2d_R(center: *const core::Point2d, axes: *const core::Size2d, angle: i32, arc_start: i32, arc_end: i32, delta: i32, pts: *mut c_void, ocvrs_return: *mut Result_void);
	// ellipse2Poly(cv::Point, cv::Size, int, int, int, int, std::vector<Point> &) /usr/include/opencv2/imgproc.hpp:4734
	pub fn cv_ellipse2Poly_Point_Size_int_int_int_int_vector_Point_R(center: *const core::Point, axes: *const core::Size, angle: i32, arc_start: i32, arc_end: i32, delta: i32, pts: *mut c_void, ocvrs_return: *mut Result_void);
	// ellipse(cv::InputOutputArray, cv::Point, cv::Size, double, double, double, const cv::Scalar &, int, int, int) /usr/include/opencv2/imgproc.hpp:4538
	pub fn cv_ellipse_const__InputOutputArrayR_Point_Size_double_double_double_const_ScalarR_int_int_int(img: *const c_void, center: *const core::Point, axes: *const core::Size, angle: f64, start_angle: f64, end_angle: f64, color: *const core::Scalar, thickness: i32, line_type: i32, shift: i32, ocvrs_return: *mut Result_void);
	// ellipse(cv::InputOutputArray, const cv::RotatedRect &, const cv::Scalar &, int, int) /usr/include/opencv2/imgproc.hpp:4552
	pub fn cv_ellipse_const__InputOutputArrayR_const_RotatedRectR_const_ScalarR_int_int(img: *const c_void, box_: *const c_void, color: *const core::Scalar, thickness: i32, line_type: i32, ocvrs_return: *mut Result_void);
	// equalizeHist(cv::InputArray, cv::OutputArray) /usr/include/opencv2/imgproc.hpp:3276
	pub fn cv_equalizeHist_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// erode(cv::InputArray, cv::OutputArray, cv::InputArray, cv::Point, int, int, const cv::Scalar &) /usr/include/opencv2/imgproc.hpp:2259
	pub fn cv_erode_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Point_int_int_const_ScalarR(src: *const c_void, dst: *const c_void, kernel: *const c_void, anchor: *const core::Point, iterations: i32, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result_void);
	// fillConvexPoly(cv::InputOutputArray, cv::InputArray, const cv::Scalar &, int, int) /usr/include/opencv2/imgproc.hpp:4593
	pub fn cv_fillConvexPoly_const__InputOutputArrayR_const__InputArrayR_const_ScalarR_int_int(img: *const c_void, points: *const c_void, color: *const core::Scalar, line_type: i32, shift: i32, ocvrs_return: *mut Result_void);
	// fillPoly(cv::InputOutputArray, cv::InputArrayOfArrays, const cv::Scalar &, int, int, cv::Point) /usr/include/opencv2/imgproc.hpp:4620
	pub fn cv_fillPoly_const__InputOutputArrayR_const__InputArrayR_const_ScalarR_int_int_Point(img: *const c_void, pts: *const c_void, color: *const core::Scalar, line_type: i32, shift: i32, offset: *const core::Point, ocvrs_return: *mut Result_void);
	// filter2D(cv::InputArray, cv::OutputArray, int, cv::InputArray, cv::Point, double, int) /usr/include/opencv2/imgproc.hpp:1649
	pub fn cv_filter2D_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_Point_double_int(src: *const c_void, dst: *const c_void, ddepth: i32, kernel: *const c_void, anchor: *const core::Point, delta: f64, border_type: i32, ocvrs_return: *mut Result_void);
	// findContours(cv::InputArray, cv::OutputArrayOfArrays, cv::OutputArray, int, int, cv::Point) /usr/include/opencv2/imgproc.hpp:3958
	pub fn cv_findContours_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_Point(image: *const c_void, contours: *const c_void, hierarchy: *const c_void, mode: i32, method: i32, offset: *const core::Point, ocvrs_return: *mut Result_void);
	// findContours(cv::InputArray, cv::OutputArrayOfArrays, int, int, cv::Point) /usr/include/opencv2/imgproc.hpp:3963
	pub fn cv_findContours_const__InputArrayR_const__OutputArrayR_int_int_Point(image: *const c_void, contours: *const c_void, mode: i32, method: i32, offset: *const core::Point, ocvrs_return: *mut Result_void);
	// fitEllipseAMS(cv::InputArray) /usr/include/opencv2/imgproc.hpp:4244
	pub fn cv_fitEllipseAMS_const__InputArrayR(points: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// fitEllipseDirect(cv::InputArray) /usr/include/opencv2/imgproc.hpp:4289
	pub fn cv_fitEllipseDirect_const__InputArrayR(points: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// fitEllipse(cv::InputArray) /usr/include/opencv2/imgproc.hpp:4207
	pub fn cv_fitEllipse_const__InputArrayR(points: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// fitLine(cv::InputArray, cv::OutputArray, int, double, double, double) /usr/include/opencv2/imgproc.hpp:4325
	pub fn cv_fitLine_const__InputArrayR_const__OutputArrayR_int_double_double_double(points: *const c_void, line: *const c_void, dist_type: i32, param: f64, reps: f64, aeps: f64, ocvrs_return: *mut Result_void);
	// floodFill(cv::InputOutputArray, cv::Point, cv::Scalar, cv::Rect *, cv::Scalar, cv::Scalar, int) /usr/include/opencv2/imgproc.hpp:3605
	pub fn cv_floodFill_const__InputOutputArrayR_Point_Scalar_RectX_Scalar_Scalar_int(image: *const c_void, seed_point: *const core::Point, new_val: *const core::Scalar, rect: *mut core::Rect, lo_diff: *const core::Scalar, up_diff: *const core::Scalar, flags: i32, ocvrs_return: *mut Result<i32>);
	// floodFill(cv::InputOutputArray, cv::InputOutputArray, cv::Point, cv::Scalar, cv::Rect *, cv::Scalar, cv::Scalar, int) /usr/include/opencv2/imgproc.hpp:3592
	pub fn cv_floodFill_const__InputOutputArrayR_const__InputOutputArrayR_Point_Scalar_RectX_Scalar_Scalar_int(image: *const c_void, mask: *const c_void, seed_point: *const core::Point, new_val: *const core::Scalar, rect: *mut core::Rect, lo_diff: *const core::Scalar, up_diff: *const core::Scalar, flags: i32, ocvrs_return: *mut Result<i32>);
	// getAffineTransform(const cv::Point2f *, const cv::Point2f *) /usr/include/opencv2/imgproc.hpp:2547
	pub fn cv_getAffineTransform_const_Point2fX_const_Point2fX(src: *const core::Point2f, dst: *const core::Point2f, ocvrs_return: *mut Result<*mut c_void>);
	// getAffineTransform(cv::InputArray, cv::InputArray) /usr/include/opencv2/imgproc.hpp:2584
	pub fn cv_getAffineTransform_const__InputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getDerivKernels(cv::OutputArray, cv::OutputArray, int, int, int, bool, int) /usr/include/opencv2/imgproc.hpp:1429
	pub fn cv_getDerivKernels_const__OutputArrayR_const__OutputArrayR_int_int_int_bool_int(kx: *const c_void, ky: *const c_void, dx: i32, dy: i32, ksize: i32, normalize: bool, ktype: i32, ocvrs_return: *mut Result_void);
	// getFontScaleFromHeight(const int, const int, const int) /usr/include/opencv2/imgproc.hpp:4832
	pub fn cv_getFontScaleFromHeight_const_int_const_int_const_int(font_face: i32, pixel_height: i32, thickness: i32, ocvrs_return: *mut Result<f64>);
	// getGaborKernel(cv::Size, double, double, double, double, double, int) /usr/include/opencv2/imgproc.hpp:1446
	pub fn cv_getGaborKernel_Size_double_double_double_double_double_int(ksize: *const core::Size, sigma: f64, theta: f64, lambd: f64, gamma: f64, psi: f64, ktype: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getGaussianKernel(int, double, int) /usr/include/opencv2/imgproc.hpp:1409
	pub fn cv_getGaussianKernel_int_double_int(ksize: i32, sigma: f64, ktype: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getPerspectiveTransform(const cv::Point2f *, const cv::Point2f *, int) /usr/include/opencv2/imgproc.hpp:2581
	pub fn cv_getPerspectiveTransform_const_Point2fX_const_Point2fX_int(src: *const core::Point2f, dst: *const core::Point2f, solve_method: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getPerspectiveTransform(cv::InputArray, cv::InputArray, int) /usr/include/opencv2/imgproc.hpp:2578
	pub fn cv_getPerspectiveTransform_const__InputArrayR_const__InputArrayR_int(src: *const c_void, dst: *const c_void, solve_method: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getRectSubPix(cv::InputArray, cv::Size, cv::Point2f, cv::OutputArray, int) /usr/include/opencv2/imgproc.hpp:2606
	pub fn cv_getRectSubPix_const__InputArrayR_Size_Point2f_const__OutputArrayR_int(image: *const c_void, patch_size: *const core::Size, center: *const core::Point2f, patch: *const c_void, patch_type: i32, ocvrs_return: *mut Result_void);
	// getRotationMatrix2D(cv::Point2f, double, double) /usr/include/opencv2/imgproc.hpp:2521
	pub fn cv_getRotationMatrix2D_Point2f_double_double(center: *const core::Point2f, angle: f64, scale: f64, ocvrs_return: *mut Result<*mut c_void>);
	#[cfg(not(target_os = "windows"))]
	// getRotationMatrix2D_(cv::Point2f, double, double) /usr/include/opencv2/imgproc.hpp:2524
	pub fn cv_getRotationMatrix2D__Point2f_double_double(center: *const core::Point2f, angle: f64, scale: f64, ocvrs_return: *mut Result<core::Matx23d>);
	// getStructuringElement(int, cv::Size, cv::Point) /usr/include/opencv2/imgproc.hpp:1465
	pub fn cv_getStructuringElement_int_Size_Point(shape: i32, ksize: *const core::Size, anchor: *const core::Point, ocvrs_return: *mut Result<*mut c_void>);
	// getTextSize(const cv::String &, int, double, int, int *) /usr/include/opencv2/imgproc.hpp:4818
	pub fn cv_getTextSize_const_StringR_int_double_int_intX(text: *const c_char, font_face: i32, font_scale: f64, thickness: i32, base_line: *mut i32, ocvrs_return: *mut Result<core::Size>);
	// goodFeaturesToTrack(cv::InputArray, cv::OutputArray, int, double, double, cv::InputArray, cv::OutputArray, int, int, bool, double) /usr/include/opencv2/imgproc.hpp:2079
	pub fn cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_const__OutputArrayR_int_int_bool_double(image: *const c_void, corners: *const c_void, max_corners: i32, quality_level: f64, min_distance: f64, mask: *const c_void, corners_quality: *const c_void, block_size: i32, gradient_size: i32, use_harris_detector: bool, k: f64, ocvrs_return: *mut Result_void);
	// goodFeaturesToTrack(cv::InputArray, cv::OutputArray, int, double, double, cv::InputArray, int, bool, double) /usr/include/opencv2/imgproc.hpp:2043
	pub fn cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_int_bool_double(image: *const c_void, corners: *const c_void, max_corners: i32, quality_level: f64, min_distance: f64, mask: *const c_void, block_size: i32, use_harris_detector: bool, k: f64, ocvrs_return: *mut Result_void);
	// goodFeaturesToTrack(cv::InputArray, cv::OutputArray, int, double, double, cv::InputArray, int, int, bool, double) /usr/include/opencv2/imgproc.hpp:2048
	pub fn cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_int_int_bool_double(image: *const c_void, corners: *const c_void, max_corners: i32, quality_level: f64, min_distance: f64, mask: *const c_void, block_size: i32, gradient_size: i32, use_harris_detector: bool, k: f64, ocvrs_return: *mut Result_void);
	// grabCut(cv::InputArray, cv::InputOutputArray, cv::Rect, cv::InputOutputArray, cv::InputOutputArray, int, int) /usr/include/opencv2/imgproc.hpp:3436
	pub fn cv_grabCut_const__InputArrayR_const__InputOutputArrayR_Rect_const__InputOutputArrayR_const__InputOutputArrayR_int_int(img: *const c_void, mask: *const c_void, rect: *const core::Rect, bgd_model: *const c_void, fgd_model: *const c_void, iter_count: i32, mode: i32, ocvrs_return: *mut Result_void);
	// integral(cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray, int, int) /usr/include/opencv2/imgproc.hpp:2827
	pub fn cv_integral_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(src: *const c_void, sum: *const c_void, sqsum: *const c_void, tilted: *const c_void, sdepth: i32, sqdepth: i32, ocvrs_return: *mut Result_void);
	// integral(cv::InputArray, cv::OutputArray, cv::OutputArray, int, int) /usr/include/opencv2/imgproc.hpp:2835
	pub fn cv_integral_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(src: *const c_void, sum: *const c_void, sqsum: *const c_void, sdepth: i32, sqdepth: i32, ocvrs_return: *mut Result_void);
	// integral(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/imgproc.hpp:2832
	pub fn cv_integral_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, sum: *const c_void, sdepth: i32, ocvrs_return: *mut Result_void);
	// intersectConvexConvex(cv::InputArray, cv::InputArray, cv::OutputArray, bool) /usr/include/opencv2/imgproc.hpp:4190
	pub fn cv_intersectConvexConvex_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(p1: *const c_void, p2: *const c_void, p12: *const c_void, handle_nested: bool, ocvrs_return: *mut Result<f32>);
	// invertAffineTransform(cv::InputArray, cv::OutputArray) /usr/include/opencv2/imgproc.hpp:2560
	pub fn cv_invertAffineTransform_const__InputArrayR_const__OutputArrayR(m: *const c_void, i_m: *const c_void, ocvrs_return: *mut Result_void);
	// isContourConvex(cv::InputArray) /usr/include/opencv2/imgproc.hpp:4171
	pub fn cv_isContourConvex_const__InputArrayR(contour: *const c_void, ocvrs_return: *mut Result<bool>);
	// line(cv::InputOutputArray, cv::Point, cv::Point, const cv::Scalar &, int, int, int) /usr/include/opencv2/imgproc.hpp:4447
	pub fn cv_line_const__InputOutputArrayR_Point_Point_const_ScalarR_int_int_int(img: *const c_void, pt1: *const core::Point, pt2: *const core::Point, color: *const core::Scalar, thickness: i32, line_type: i32, shift: i32, ocvrs_return: *mut Result_void);
	// linearPolar(cv::InputArray, cv::OutputArray, cv::Point2f, double, int) /usr/include/opencv2/imgproc.hpp:2693
	pub fn cv_linearPolar_const__InputArrayR_const__OutputArrayR_Point2f_double_int(src: *const c_void, dst: *const c_void, center: *const core::Point2f, max_radius: f64, flags: i32, ocvrs_return: *mut Result_void);
	// logPolar(cv::InputArray, cv::OutputArray, cv::Point2f, double, int) /usr/include/opencv2/imgproc.hpp:2652
	pub fn cv_logPolar_const__InputArrayR_const__OutputArrayR_Point2f_double_int(src: *const c_void, dst: *const c_void, center: *const core::Point2f, m: f64, flags: i32, ocvrs_return: *mut Result_void);
	// matchShapes(cv::InputArray, cv::InputArray, int, double) /usr/include/opencv2/imgproc.hpp:4108
	pub fn cv_matchShapes_const__InputArrayR_const__InputArrayR_int_double(contour1: *const c_void, contour2: *const c_void, method: i32, parameter: f64, ocvrs_return: *mut Result<f64>);
	// matchTemplate(cv::InputArray, cv::InputArray, cv::OutputArray, int, cv::InputArray) /usr/include/opencv2/imgproc.hpp:3844
	pub fn cv_matchTemplate_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR(image: *const c_void, templ: *const c_void, result: *const c_void, method: i32, mask: *const c_void, ocvrs_return: *mut Result_void);
	// medianBlur(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/imgproc.hpp:1487
	pub fn cv_medianBlur_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, ksize: i32, ocvrs_return: *mut Result_void);
	// minAreaRect(cv::InputArray) /usr/include/opencv2/imgproc.hpp:4051
	pub fn cv_minAreaRect_const__InputArrayR(points: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// minEnclosingCircle(cv::InputArray, cv::Point2f &, float &) /usr/include/opencv2/imgproc.hpp:4072
	pub fn cv_minEnclosingCircle_const__InputArrayR_Point2fR_floatR(points: *const c_void, center: *mut core::Point2f, radius: *mut f32, ocvrs_return: *mut Result_void);
	// minEnclosingTriangle(cv::InputArray, cv::OutputArray) /usr/include/opencv2/imgproc.hpp:4097
	pub fn cv_minEnclosingTriangle_const__InputArrayR_const__OutputArrayR(points: *const c_void, triangle: *const c_void, ocvrs_return: *mut Result<f64>);
	// moments(cv::InputArray, bool) /usr/include/opencv2/imgproc.hpp:3740
	pub fn cv_moments_const__InputArrayR_bool(array: *const c_void, binary_image: bool, ocvrs_return: *mut Result<core::Moments>);
	// morphologyDefaultBorderValue() /usr/include/opencv2/imgproc.hpp:1450
	pub fn cv_morphologyDefaultBorderValue(ocvrs_return: *mut Result<core::Scalar>);
	// morphologyEx(cv::InputArray, cv::OutputArray, int, cv::InputArray, cv::Point, int, int, const cv::Scalar &) /usr/include/opencv2/imgproc.hpp:2320
	pub fn cv_morphologyEx_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_Point_int_int_const_ScalarR(src: *const c_void, dst: *const c_void, op: i32, kernel: *const c_void, anchor: *const core::Point, iterations: i32, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result_void);
	// phaseCorrelate(cv::InputArray, cv::InputArray, cv::InputArray, double *) /usr/include/opencv2/imgproc.hpp:2957
	pub fn cv_phaseCorrelate_const__InputArrayR_const__InputArrayR_const__InputArrayR_doubleX(src1: *const c_void, src2: *const c_void, window: *const c_void, response: *mut f64, ocvrs_return: *mut Result<core::Point2d>);
	// pointPolygonTest(cv::InputArray, cv::Point2f, bool) /usr/include/opencv2/imgproc.hpp:4344
	pub fn cv_pointPolygonTest_const__InputArrayR_Point2f_bool(contour: *const c_void, pt: *const core::Point2f, measure_dist: bool, ocvrs_return: *mut Result<f64>);
	// polylines(cv::InputOutputArray, cv::InputArrayOfArrays, bool, const cv::Scalar &, int, int, int) /usr/include/opencv2/imgproc.hpp:4643
	pub fn cv_polylines_const__InputOutputArrayR_const__InputArrayR_bool_const_ScalarR_int_int_int(img: *const c_void, pts: *const c_void, is_closed: bool, color: *const core::Scalar, thickness: i32, line_type: i32, shift: i32, ocvrs_return: *mut Result_void);
	// preCornerDetect(cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/imgproc.hpp:1952
	pub fn cv_preCornerDetect_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, ksize: i32, border_type: i32, ocvrs_return: *mut Result_void);
	// putText(cv::InputOutputArray, const cv::String &, cv::Point, int, double, cv::Scalar, int, int, bool) /usr/include/opencv2/imgproc.hpp:4768
	pub fn cv_putText_const__InputOutputArrayR_const_StringR_Point_int_double_Scalar_int_int_bool(img: *const c_void, text: *const c_char, org: *const core::Point, font_face: i32, font_scale: f64, color: *const core::Scalar, thickness: i32, line_type: i32, bottom_left_origin: bool, ocvrs_return: *mut Result_void);
	// pyrDown(cv::InputArray, cv::OutputArray, const cv::Size &, int) /usr/include/opencv2/imgproc.hpp:3083
	pub fn cv_pyrDown_const__InputArrayR_const__OutputArrayR_const_SizeR_int(src: *const c_void, dst: *const c_void, dstsize: *const core::Size, border_type: i32, ocvrs_return: *mut Result_void);
	// pyrMeanShiftFiltering(cv::InputArray, cv::OutputArray, double, double, int, cv::TermCriteria) /usr/include/opencv2/imgproc.hpp:3404
	pub fn cv_pyrMeanShiftFiltering_const__InputArrayR_const__OutputArrayR_double_double_int_TermCriteria(src: *const c_void, dst: *const c_void, sp: f64, sr: f64, max_level: i32, termcrit: *const core::TermCriteria, ocvrs_return: *mut Result_void);
	// pyrUp(cv::InputArray, cv::OutputArray, const cv::Size &, int) /usr/include/opencv2/imgproc.hpp:3103
	pub fn cv_pyrUp_const__InputArrayR_const__OutputArrayR_const_SizeR_int(src: *const c_void, dst: *const c_void, dstsize: *const core::Size, border_type: i32, ocvrs_return: *mut Result_void);
	// rectangle(cv::InputOutputArray, cv::Point, cv::Point, const cv::Scalar &, int, int, int) /usr/include/opencv2/imgproc.hpp:4480
	pub fn cv_rectangle_const__InputOutputArrayR_Point_Point_const_ScalarR_int_int_int(img: *const c_void, pt1: *const core::Point, pt2: *const core::Point, color: *const core::Scalar, thickness: i32, line_type: i32, shift: i32, ocvrs_return: *mut Result_void);
	// rectangle(cv::InputOutputArray, cv::Rect, const cv::Scalar &, int, int, int) /usr/include/opencv2/imgproc.hpp:4489
	pub fn cv_rectangle_const__InputOutputArrayR_Rect_const_ScalarR_int_int_int(img: *const c_void, rec: *const core::Rect, color: *const core::Scalar, thickness: i32, line_type: i32, shift: i32, ocvrs_return: *mut Result_void);
	// remap(cv::InputArray, cv::OutputArray, cv::InputArray, cv::InputArray, int, int, const cv::Scalar &) /usr/include/opencv2/imgproc.hpp:2463
	pub fn cv_remap_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int_int_const_ScalarR(src: *const c_void, dst: *const c_void, map1: *const c_void, map2: *const c_void, interpolation: i32, border_mode: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result_void);
	// resize(cv::InputArray, cv::OutputArray, cv::Size, double, double, int) /usr/include/opencv2/imgproc.hpp:2365
	pub fn cv_resize_const__InputArrayR_const__OutputArrayR_Size_double_double_int(src: *const c_void, dst: *const c_void, dsize: *const core::Size, fx: f64, fy: f64, interpolation: i32, ocvrs_return: *mut Result_void);
	// rotatedRectangleIntersection(const cv::RotatedRect &, const cv::RotatedRect &, cv::OutputArray) /usr/include/opencv2/imgproc.hpp:4361
	pub fn cv_rotatedRectangleIntersection_const_RotatedRectR_const_RotatedRectR_const__OutputArrayR(rect1: *const c_void, rect2: *const c_void, intersecting_region: *const c_void, ocvrs_return: *mut Result<i32>);
	// sepFilter2D(cv::InputArray, cv::OutputArray, int, cv::InputArray, cv::InputArray, cv::Point, double, int) /usr/include/opencv2/imgproc.hpp:1670
	pub fn cv_sepFilter2D_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_const__InputArrayR_Point_double_int(src: *const c_void, dst: *const c_void, ddepth: i32, kernel_x: *const c_void, kernel_y: *const c_void, anchor: *const core::Point, delta: f64, border_type: i32, ocvrs_return: *mut Result_void);
	// spatialGradient(cv::InputArray, cv::OutputArray, cv::OutputArray, int, int) /usr/include/opencv2/imgproc.hpp:1747
	pub fn cv_spatialGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(src: *const c_void, dx: *const c_void, dy: *const c_void, ksize: i32, border_type: i32, ocvrs_return: *mut Result_void);
	// sqrBoxFilter(cv::InputArray, cv::OutputArray, int, cv::Size, cv::Point, bool, int) /usr/include/opencv2/imgproc.hpp:1592
	pub fn cv_sqrBoxFilter_const__InputArrayR_const__OutputArrayR_int_Size_Point_bool_int(src: *const c_void, dst: *const c_void, ddepth: i32, ksize: *const core::Size, anchor: *const core::Point, normalize: bool, border_type: i32, ocvrs_return: *mut Result_void);
	// threshold(cv::InputArray, cv::OutputArray, double, double, int) /usr/include/opencv2/imgproc.hpp:3022
	pub fn cv_threshold_const__InputArrayR_const__OutputArrayR_double_double_int(src: *const c_void, dst: *const c_void, thresh: f64, maxval: f64, typ: i32, ocvrs_return: *mut Result<f64>);
	// warpAffine(cv::InputArray, cv::OutputArray, cv::InputArray, cv::Size, int, int, const cv::Scalar &) /usr/include/opencv2/imgproc.hpp:2393
	pub fn cv_warpAffine_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_const_ScalarR(src: *const c_void, dst: *const c_void, m: *const c_void, dsize: *const core::Size, flags: i32, border_mode: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result_void);
	// warpPerspective(cv::InputArray, cv::OutputArray, cv::InputArray, cv::Size, int, int, const cv::Scalar &) /usr/include/opencv2/imgproc.hpp:2425
	pub fn cv_warpPerspective_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_const_ScalarR(src: *const c_void, dst: *const c_void, m: *const c_void, dsize: *const core::Size, flags: i32, border_mode: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result_void);
	// warpPolar(cv::InputArray, cv::OutputArray, cv::Size, cv::Point2f, double, int) /usr/include/opencv2/imgproc.hpp:2784
	pub fn cv_warpPolar_const__InputArrayR_const__OutputArrayR_Size_Point2f_double_int(src: *const c_void, dst: *const c_void, dsize: *const core::Size, center: *const core::Point2f, max_radius: f64, flags: i32, ocvrs_return: *mut Result_void);
	// watershed(cv::InputArray, cv::InputOutputArray) /usr/include/opencv2/imgproc.hpp:3361
	pub fn cv_watershed_const__InputArrayR_const__InputOutputArrayR(image: *const c_void, markers: *const c_void, ocvrs_return: *mut Result_void);
	// wrapperEMD(cv::InputArray, cv::InputArray, int, cv::InputArray, Ptr<float>, cv::OutputArray) /usr/include/opencv2/imgproc.hpp:3324
	pub fn cv_wrapperEMD_const__InputArrayR_const__InputArrayR_int_const__InputArrayR_Ptr_float__const__OutputArrayR(signature1: *const c_void, signature2: *const c_void, dist_type: i32, cost: *const c_void, lower_bound: *mut c_void, flow: *const c_void, ocvrs_return: *mut Result<f32>);
	// apply(cv::InputArray, cv::OutputArray) /usr/include/opencv2/imgproc.hpp:1014
	pub fn cv_CLAHE_apply_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// setClipLimit(double) /usr/include/opencv2/imgproc.hpp:1020
	pub fn cv_CLAHE_setClipLimit_double(instance: *mut c_void, clip_limit: f64, ocvrs_return: *mut Result_void);
	// getClipLimit() /usr/include/opencv2/imgproc.hpp:1023
	pub fn cv_CLAHE_getClipLimit_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setTilesGridSize(cv::Size) /usr/include/opencv2/imgproc.hpp:1030
	pub fn cv_CLAHE_setTilesGridSize_Size(instance: *mut c_void, tile_grid_size: *const core::Size, ocvrs_return: *mut Result_void);
	// getTilesGridSize() /usr/include/opencv2/imgproc.hpp:1033
	pub fn cv_CLAHE_getTilesGridSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// collectGarbage() /usr/include/opencv2/imgproc.hpp:1035
	pub fn cv_CLAHE_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// setTemplate(cv::InputArray, cv::Point) /usr/include/opencv2/imgproc.hpp:899
	pub fn cv_GeneralizedHough_setTemplate_const__InputArrayR_Point(instance: *mut c_void, templ: *const c_void, templ_center: *const core::Point, ocvrs_return: *mut Result_void);
	// setTemplate(cv::InputArray, cv::InputArray, cv::InputArray, cv::Point) /usr/include/opencv2/imgproc.hpp:900
	pub fn cv_GeneralizedHough_setTemplate_const__InputArrayR_const__InputArrayR_const__InputArrayR_Point(instance: *mut c_void, edges: *const c_void, dx: *const c_void, dy: *const c_void, templ_center: *const core::Point, ocvrs_return: *mut Result_void);
	// detect(cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/imgproc.hpp:903
	pub fn cv_GeneralizedHough_detect_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, positions: *const c_void, votes: *const c_void, ocvrs_return: *mut Result_void);
	// detect(cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/imgproc.hpp:904
	pub fn cv_GeneralizedHough_detect_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, edges: *const c_void, dx: *const c_void, dy: *const c_void, positions: *const c_void, votes: *const c_void, ocvrs_return: *mut Result_void);
	// setCannyLowThresh(int) /usr/include/opencv2/imgproc.hpp:907
	pub fn cv_GeneralizedHough_setCannyLowThresh_int(instance: *mut c_void, canny_low_thresh: i32, ocvrs_return: *mut Result_void);
	// getCannyLowThresh() /usr/include/opencv2/imgproc.hpp:908
	pub fn cv_GeneralizedHough_getCannyLowThresh_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setCannyHighThresh(int) /usr/include/opencv2/imgproc.hpp:911
	pub fn cv_GeneralizedHough_setCannyHighThresh_int(instance: *mut c_void, canny_high_thresh: i32, ocvrs_return: *mut Result_void);
	// getCannyHighThresh() /usr/include/opencv2/imgproc.hpp:912
	pub fn cv_GeneralizedHough_getCannyHighThresh_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMinDist(double) /usr/include/opencv2/imgproc.hpp:915
	pub fn cv_GeneralizedHough_setMinDist_double(instance: *mut c_void, min_dist: f64, ocvrs_return: *mut Result_void);
	// getMinDist() /usr/include/opencv2/imgproc.hpp:916
	pub fn cv_GeneralizedHough_getMinDist_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setDp(double) /usr/include/opencv2/imgproc.hpp:919
	pub fn cv_GeneralizedHough_setDp_double(instance: *mut c_void, dp: f64, ocvrs_return: *mut Result_void);
	// getDp() /usr/include/opencv2/imgproc.hpp:920
	pub fn cv_GeneralizedHough_getDp_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxBufferSize(int) /usr/include/opencv2/imgproc.hpp:923
	pub fn cv_GeneralizedHough_setMaxBufferSize_int(instance: *mut c_void, max_buffer_size: i32, ocvrs_return: *mut Result_void);
	// getMaxBufferSize() /usr/include/opencv2/imgproc.hpp:924
	pub fn cv_GeneralizedHough_getMaxBufferSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setLevels(int) /usr/include/opencv2/imgproc.hpp:935
	pub fn cv_GeneralizedHoughBallard_setLevels_int(instance: *mut c_void, levels: i32, ocvrs_return: *mut Result_void);
	// getLevels() /usr/include/opencv2/imgproc.hpp:936
	pub fn cv_GeneralizedHoughBallard_getLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setVotesThreshold(int) /usr/include/opencv2/imgproc.hpp:939
	pub fn cv_GeneralizedHoughBallard_setVotesThreshold_int(instance: *mut c_void, votes_threshold: i32, ocvrs_return: *mut Result_void);
	// getVotesThreshold() /usr/include/opencv2/imgproc.hpp:940
	pub fn cv_GeneralizedHoughBallard_getVotesThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setXi(double) /usr/include/opencv2/imgproc.hpp:951
	pub fn cv_GeneralizedHoughGuil_setXi_double(instance: *mut c_void, xi: f64, ocvrs_return: *mut Result_void);
	// getXi() /usr/include/opencv2/imgproc.hpp:952
	pub fn cv_GeneralizedHoughGuil_getXi_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setLevels(int) /usr/include/opencv2/imgproc.hpp:955
	pub fn cv_GeneralizedHoughGuil_setLevels_int(instance: *mut c_void, levels: i32, ocvrs_return: *mut Result_void);
	// getLevels() /usr/include/opencv2/imgproc.hpp:956
	pub fn cv_GeneralizedHoughGuil_getLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setAngleEpsilon(double) /usr/include/opencv2/imgproc.hpp:959
	pub fn cv_GeneralizedHoughGuil_setAngleEpsilon_double(instance: *mut c_void, angle_epsilon: f64, ocvrs_return: *mut Result_void);
	// getAngleEpsilon() /usr/include/opencv2/imgproc.hpp:960
	pub fn cv_GeneralizedHoughGuil_getAngleEpsilon_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMinAngle(double) /usr/include/opencv2/imgproc.hpp:963
	pub fn cv_GeneralizedHoughGuil_setMinAngle_double(instance: *mut c_void, min_angle: f64, ocvrs_return: *mut Result_void);
	// getMinAngle() /usr/include/opencv2/imgproc.hpp:964
	pub fn cv_GeneralizedHoughGuil_getMinAngle_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxAngle(double) /usr/include/opencv2/imgproc.hpp:967
	pub fn cv_GeneralizedHoughGuil_setMaxAngle_double(instance: *mut c_void, max_angle: f64, ocvrs_return: *mut Result_void);
	// getMaxAngle() /usr/include/opencv2/imgproc.hpp:968
	pub fn cv_GeneralizedHoughGuil_getMaxAngle_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setAngleStep(double) /usr/include/opencv2/imgproc.hpp:971
	pub fn cv_GeneralizedHoughGuil_setAngleStep_double(instance: *mut c_void, angle_step: f64, ocvrs_return: *mut Result_void);
	// getAngleStep() /usr/include/opencv2/imgproc.hpp:972
	pub fn cv_GeneralizedHoughGuil_getAngleStep_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setAngleThresh(int) /usr/include/opencv2/imgproc.hpp:975
	pub fn cv_GeneralizedHoughGuil_setAngleThresh_int(instance: *mut c_void, angle_thresh: i32, ocvrs_return: *mut Result_void);
	// getAngleThresh() /usr/include/opencv2/imgproc.hpp:976
	pub fn cv_GeneralizedHoughGuil_getAngleThresh_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMinScale(double) /usr/include/opencv2/imgproc.hpp:979
	pub fn cv_GeneralizedHoughGuil_setMinScale_double(instance: *mut c_void, min_scale: f64, ocvrs_return: *mut Result_void);
	// getMinScale() /usr/include/opencv2/imgproc.hpp:980
	pub fn cv_GeneralizedHoughGuil_getMinScale_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxScale(double) /usr/include/opencv2/imgproc.hpp:983
	pub fn cv_GeneralizedHoughGuil_setMaxScale_double(instance: *mut c_void, max_scale: f64, ocvrs_return: *mut Result_void);
	// getMaxScale() /usr/include/opencv2/imgproc.hpp:984
	pub fn cv_GeneralizedHoughGuil_getMaxScale_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setScaleStep(double) /usr/include/opencv2/imgproc.hpp:987
	pub fn cv_GeneralizedHoughGuil_setScaleStep_double(instance: *mut c_void, scale_step: f64, ocvrs_return: *mut Result_void);
	// getScaleStep() /usr/include/opencv2/imgproc.hpp:988
	pub fn cv_GeneralizedHoughGuil_getScaleStep_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setScaleThresh(int) /usr/include/opencv2/imgproc.hpp:991
	pub fn cv_GeneralizedHoughGuil_setScaleThresh_int(instance: *mut c_void, scale_thresh: i32, ocvrs_return: *mut Result_void);
	// getScaleThresh() /usr/include/opencv2/imgproc.hpp:992
	pub fn cv_GeneralizedHoughGuil_getScaleThresh_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setPosThresh(int) /usr/include/opencv2/imgproc.hpp:995
	pub fn cv_GeneralizedHoughGuil_setPosThresh_int(instance: *mut c_void, pos_thresh: i32, ocvrs_return: *mut Result_void);
	// getPosThresh() /usr/include/opencv2/imgproc.hpp:996
	pub fn cv_GeneralizedHoughGuil_getPosThresh_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// ptr /usr/include/opencv2/imgproc.hpp:4934
	pub fn cv_LineIterator_getPropPtr(instance: *mut c_void) -> *mut u8;
	// ptr /usr/include/opencv2/imgproc.hpp:4934
	pub fn cv_LineIterator_setPropPtr_unsigned_charX(instance: *mut c_void, val: *mut u8);
	// ptr0 /usr/include/opencv2/imgproc.hpp:4935
	pub fn cv_LineIterator_getPropPtr0_const(instance: *const c_void) -> *const u8;
	// step /usr/include/opencv2/imgproc.hpp:4936
	pub fn cv_LineIterator_getPropStep_const(instance: *const c_void) -> i32;
	// step /usr/include/opencv2/imgproc.hpp:4936
	pub fn cv_LineIterator_setPropStep_int(instance: *mut c_void, val: i32);
	// elemSize /usr/include/opencv2/imgproc.hpp:4936
	pub fn cv_LineIterator_getPropElemSize_const(instance: *const c_void) -> i32;
	// elemSize /usr/include/opencv2/imgproc.hpp:4936
	pub fn cv_LineIterator_setPropElemSize_int(instance: *mut c_void, val: i32);
	// err /usr/include/opencv2/imgproc.hpp:4937
	pub fn cv_LineIterator_getPropErr_const(instance: *const c_void) -> i32;
	// err /usr/include/opencv2/imgproc.hpp:4937
	pub fn cv_LineIterator_setPropErr_int(instance: *mut c_void, val: i32);
	// count /usr/include/opencv2/imgproc.hpp:4937
	pub fn cv_LineIterator_getPropCount_const(instance: *const c_void) -> i32;
	// count /usr/include/opencv2/imgproc.hpp:4937
	pub fn cv_LineIterator_setPropCount_int(instance: *mut c_void, val: i32);
	// minusDelta /usr/include/opencv2/imgproc.hpp:4938
	pub fn cv_LineIterator_getPropMinusDelta_const(instance: *const c_void) -> i32;
	// minusDelta /usr/include/opencv2/imgproc.hpp:4938
	pub fn cv_LineIterator_setPropMinusDelta_int(instance: *mut c_void, val: i32);
	// plusDelta /usr/include/opencv2/imgproc.hpp:4938
	pub fn cv_LineIterator_getPropPlusDelta_const(instance: *const c_void) -> i32;
	// plusDelta /usr/include/opencv2/imgproc.hpp:4938
	pub fn cv_LineIterator_setPropPlusDelta_int(instance: *mut c_void, val: i32);
	// minusStep /usr/include/opencv2/imgproc.hpp:4939
	pub fn cv_LineIterator_getPropMinusStep_const(instance: *const c_void) -> i32;
	// minusStep /usr/include/opencv2/imgproc.hpp:4939
	pub fn cv_LineIterator_setPropMinusStep_int(instance: *mut c_void, val: i32);
	// plusStep /usr/include/opencv2/imgproc.hpp:4939
	pub fn cv_LineIterator_getPropPlusStep_const(instance: *const c_void) -> i32;
	// plusStep /usr/include/opencv2/imgproc.hpp:4939
	pub fn cv_LineIterator_setPropPlusStep_int(instance: *mut c_void, val: i32);
	// minusShift /usr/include/opencv2/imgproc.hpp:4940
	pub fn cv_LineIterator_getPropMinusShift_const(instance: *const c_void) -> i32;
	// minusShift /usr/include/opencv2/imgproc.hpp:4940
	pub fn cv_LineIterator_setPropMinusShift_int(instance: *mut c_void, val: i32);
	// plusShift /usr/include/opencv2/imgproc.hpp:4940
	pub fn cv_LineIterator_getPropPlusShift_const(instance: *const c_void) -> i32;
	// plusShift /usr/include/opencv2/imgproc.hpp:4940
	pub fn cv_LineIterator_setPropPlusShift_int(instance: *mut c_void, val: i32);
	// p /usr/include/opencv2/imgproc.hpp:4941
	pub fn cv_LineIterator_getPropP_const(instance: *const c_void, ocvrs_return: *mut core::Point);
	// p /usr/include/opencv2/imgproc.hpp:4941
	pub fn cv_LineIterator_setPropP_Point(instance: *mut c_void, val: *const core::Point);
	// ptmode /usr/include/opencv2/imgproc.hpp:4942
	pub fn cv_LineIterator_getPropPtmode_const(instance: *const c_void) -> bool;
	// ptmode /usr/include/opencv2/imgproc.hpp:4942
	pub fn cv_LineIterator_setPropPtmode_bool(instance: *mut c_void, val: bool);
	// LineIterator(const cv::Mat &, cv::Point, cv::Point, int, bool) /usr/include/opencv2/imgproc.hpp:4883
	pub fn cv_LineIterator_LineIterator_const_MatR_Point_Point_int_bool(img: *const c_void, pt1: *const core::Point, pt2: *const core::Point, connectivity: i32, left_to_right: bool, ocvrs_return: *mut Result<*mut c_void>);
	// LineIterator(cv::Point, cv::Point, int, bool) /usr/include/opencv2/imgproc.hpp:4889
	pub fn cv_LineIterator_LineIterator_Point_Point_int_bool(pt1: *const core::Point, pt2: *const core::Point, connectivity: i32, left_to_right: bool, ocvrs_return: *mut Result<*mut c_void>);
	// LineIterator(cv::Size, cv::Point, cv::Point, int, bool) /usr/include/opencv2/imgproc.hpp:4899
	pub fn cv_LineIterator_LineIterator_Size_Point_Point_int_bool(bounding_area_size: *const core::Size, pt1: *const core::Point, pt2: *const core::Point, connectivity: i32, left_to_right: bool, ocvrs_return: *mut Result<*mut c_void>);
	// LineIterator(cv::Rect, cv::Point, cv::Point, int, bool) /usr/include/opencv2/imgproc.hpp:4906
	pub fn cv_LineIterator_LineIterator_Rect_Point_Point_int_bool(bounding_area_rect: *const core::Rect, pt1: *const core::Point, pt2: *const core::Point, connectivity: i32, left_to_right: bool, ocvrs_return: *mut Result<*mut c_void>);
	// init(const cv::Mat *, cv::Rect, cv::Point, cv::Point, int, bool) /usr/include/opencv2/imgproc.hpp:4912
	pub fn cv_LineIterator_init_const_MatX_Rect_Point_Point_int_bool(instance: *mut c_void, img: *const c_void, bounding_area_rect: *const core::Rect, pt1: *const core::Point, pt2: *const core::Point, connectivity: i32, left_to_right: bool, ocvrs_return: *mut Result_void);
	// operator*() /usr/include/opencv2/imgproc.hpp:4916
	pub fn cv_LineIterator_operatorX(instance: *mut c_void, ocvrs_return: *mut Result<*mut u8>);
	// operator++() /usr/include/opencv2/imgproc.hpp:4922
	pub fn cv_LineIterator_operatorAA(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// pos() /usr/include/opencv2/imgproc.hpp:4932
	pub fn cv_LineIterator_pos_const(instance: *const c_void, ocvrs_return: *mut Result<core::Point>);
	// detect(cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/imgproc.hpp:1343
	pub fn cv_LineSegmentDetector_detect_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, lines: *const c_void, width: *const c_void, prec: *const c_void, nfa: *const c_void, ocvrs_return: *mut Result_void);
	// drawSegments(cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/imgproc.hpp:1352
	pub fn cv_LineSegmentDetector_drawSegments_const__InputOutputArrayR_const__InputArrayR(instance: *mut c_void, image: *const c_void, lines: *const c_void, ocvrs_return: *mut Result_void);
	// compareSegments(const cv::Size &, cv::InputArray, cv::InputArray, cv::InputOutputArray) /usr/include/opencv2/imgproc.hpp:1362
	pub fn cv_LineSegmentDetector_compareSegments_const_SizeR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(instance: *mut c_void, size: *const core::Size, lines1: *const c_void, lines2: *const c_void, image: *const c_void, ocvrs_return: *mut Result<i32>);
	// Subdiv2D() /usr/include/opencv2/imgproc.hpp:1068
	pub fn cv_Subdiv2D_Subdiv2D(ocvrs_return: *mut Result<*mut c_void>);
	// Subdiv2D(cv::Rect) /usr/include/opencv2/imgproc.hpp:1078
	pub fn cv_Subdiv2D_Subdiv2D_Rect(rect: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
	// initDelaunay(cv::Rect) /usr/include/opencv2/imgproc.hpp:1085
	pub fn cv_Subdiv2D_initDelaunay_Rect(instance: *mut c_void, rect: *const core::Rect, ocvrs_return: *mut Result_void);
	// insert(cv::Point2f) /usr/include/opencv2/imgproc.hpp:1097
	pub fn cv_Subdiv2D_insert_Point2f(instance: *mut c_void, pt: *const core::Point2f, ocvrs_return: *mut Result<i32>);
	// insert(const std::vector<Point2f> &) /usr/include/opencv2/imgproc.hpp:1106
	pub fn cv_Subdiv2D_insert_const_vector_Point2f_R(instance: *mut c_void, ptvec: *const c_void, ocvrs_return: *mut Result_void);
	// locate(cv::Point2f, int &, int &) /usr/include/opencv2/imgproc.hpp:1128
	pub fn cv_Subdiv2D_locate_Point2f_intR_intR(instance: *mut c_void, pt: *const core::Point2f, edge: *mut i32, vertex: *mut i32, ocvrs_return: *mut Result<i32>);
	// findNearest(cv::Point2f, cv::Point2f *) /usr/include/opencv2/imgproc.hpp:1142
	pub fn cv_Subdiv2D_findNearest_Point2f_Point2fX(instance: *mut c_void, pt: *const core::Point2f, nearest_pt: *mut core::Point2f, ocvrs_return: *mut Result<i32>);
	// getEdgeList(std::vector<Vec4f> &) /usr/include/opencv2/imgproc.hpp:1151
	pub fn cv_Subdiv2D_getEdgeList_const_vector_Vec4f_R(instance: *const c_void, edge_list: *mut c_void, ocvrs_return: *mut Result_void);
	// getLeadingEdgeList(std::vector<int> &) /usr/include/opencv2/imgproc.hpp:1159
	pub fn cv_Subdiv2D_getLeadingEdgeList_const_vector_int_R(instance: *const c_void, leading_edge_list: *mut c_void, ocvrs_return: *mut Result_void);
	// getTriangleList(std::vector<Vec6f> &) /usr/include/opencv2/imgproc.hpp:1168
	pub fn cv_Subdiv2D_getTriangleList_const_vector_Vec6f_R(instance: *const c_void, triangle_list: *mut c_void, ocvrs_return: *mut Result_void);
	// getVoronoiFacetList(const std::vector<int> &, std::vector<std::vector<Point2f>> &, std::vector<Point2f> &) /usr/include/opencv2/imgproc.hpp:1177
	pub fn cv_Subdiv2D_getVoronoiFacetList_const_vector_int_R_vector_vector_Point2f__R_vector_Point2f_R(instance: *mut c_void, idx: *const c_void, facet_list: *mut c_void, facet_centers: *mut c_void, ocvrs_return: *mut Result_void);
	// getVertex(int, int *) /usr/include/opencv2/imgproc.hpp:1187
	pub fn cv_Subdiv2D_getVertex_const_int_intX(instance: *const c_void, vertex: i32, first_edge: *mut i32, ocvrs_return: *mut Result<core::Point2f>);
	// getEdge(int, int) /usr/include/opencv2/imgproc.hpp:1207
	pub fn cv_Subdiv2D_getEdge_const_int_int(instance: *const c_void, edge: i32, next_edge_type: i32, ocvrs_return: *mut Result<i32>);
	// nextEdge(int) /usr/include/opencv2/imgproc.hpp:1216
	pub fn cv_Subdiv2D_nextEdge_const_int(instance: *const c_void, edge: i32, ocvrs_return: *mut Result<i32>);
	// rotateEdge(int, int) /usr/include/opencv2/imgproc.hpp:1230
	pub fn cv_Subdiv2D_rotateEdge_const_int_int(instance: *const c_void, edge: i32, rotate: i32, ocvrs_return: *mut Result<i32>);
	// symEdge(int) /usr/include/opencv2/imgproc.hpp:1231
	pub fn cv_Subdiv2D_symEdge_const_int(instance: *const c_void, edge: i32, ocvrs_return: *mut Result<i32>);
	// edgeOrg(int, cv::Point2f *) /usr/include/opencv2/imgproc.hpp:1240
	pub fn cv_Subdiv2D_edgeOrg_const_int_Point2fX(instance: *const c_void, edge: i32, orgpt: *mut core::Point2f, ocvrs_return: *mut Result<i32>);
	// edgeDst(int, cv::Point2f *) /usr/include/opencv2/imgproc.hpp:1249
	pub fn cv_Subdiv2D_edgeDst_const_int_Point2fX(instance: *const c_void, edge: i32, dstpt: *mut core::Point2f, ocvrs_return: *mut Result<i32>);
	// IntelligentScissorsMB() /usr/include/opencv2/imgproc/segmentation.hpp:34
	pub fn cv_segmentation_IntelligentScissorsMB_IntelligentScissorsMB(ocvrs_return: *mut Result<*mut c_void>);
	// setWeights(float, float, float) /usr/include/opencv2/imgproc/segmentation.hpp:46
	pub fn cv_segmentation_IntelligentScissorsMB_setWeights_float_float_float(instance: *mut c_void, weight_non_edge: f32, weight_gradient_direction: f32, weight_gradient_magnitude: f32, ocvrs_return: *mut Result<*mut c_void>);
	// setGradientMagnitudeMaxLimit(float) /usr/include/opencv2/imgproc/segmentation.hpp:58
	pub fn cv_segmentation_IntelligentScissorsMB_setGradientMagnitudeMaxLimit_float(instance: *mut c_void, gradient_magnitude_threshold_max: f32, ocvrs_return: *mut Result<*mut c_void>);
	// setEdgeFeatureZeroCrossingParameters(float) /usr/include/opencv2/imgproc/segmentation.hpp:74
	pub fn cv_segmentation_IntelligentScissorsMB_setEdgeFeatureZeroCrossingParameters_float(instance: *mut c_void, gradient_magnitude_min_value: f32, ocvrs_return: *mut Result<*mut c_void>);
	// setEdgeFeatureCannyParameters(double, double, int, bool) /usr/include/opencv2/imgproc/segmentation.hpp:83
	pub fn cv_segmentation_IntelligentScissorsMB_setEdgeFeatureCannyParameters_double_double_int_bool(instance: *mut c_void, threshold1: f64, threshold2: f64, aperture_size: i32, l2gradient: bool, ocvrs_return: *mut Result<*mut c_void>);
	// applyImage(cv::InputArray) /usr/include/opencv2/imgproc/segmentation.hpp:93
	pub fn cv_segmentation_IntelligentScissorsMB_applyImage_const__InputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// applyImageFeatures(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/imgproc/segmentation.hpp:105
	pub fn cv_segmentation_IntelligentScissorsMB_applyImageFeatures_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(instance: *mut c_void, non_edge: *const c_void, gradient_direction: *const c_void, gradient_magnitude: *const c_void, image: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// buildMap(const cv::Point &) /usr/include/opencv2/imgproc/segmentation.hpp:116
	pub fn cv_segmentation_IntelligentScissorsMB_buildMap_const_PointR(instance: *mut c_void, source_pt: *const core::Point, ocvrs_return: *mut Result_void);
	// getContour(const cv::Point &, cv::OutputArray, bool) /usr/include/opencv2/imgproc/segmentation.hpp:126
	pub fn cv_segmentation_IntelligentScissorsMB_getContour_const_const_PointR_const__OutputArrayR_bool(instance: *const c_void, target_pt: *const core::Point, contour: *const c_void, backward: bool, ocvrs_return: *mut Result_void);
}
