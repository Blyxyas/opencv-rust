extern "C" {
	// calibrate(cv::InputArrayOfArrays, cv::InputArrayOfArrays, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, int, cv::TermCriteria, cv::OutputArray) /usr/include/opencv2/ccalib/omnidir.hpp:176
	pub fn cv_omnidir_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria_const__OutputArrayR(object_points: *const c_void, image_points: *const c_void, size: *const core::Size, k: *const c_void, xi: *const c_void, d: *const c_void, rvecs: *const c_void, tvecs: *const c_void, flags: i32, criteria: *const core::TermCriteria, idx: *const c_void, ocvrs_return: *mut Result<f64>);
	// initUndistortRectifyMap(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, const cv::Size &, int, cv::OutputArray, cv::OutputArray, int) /usr/include/opencv2/ccalib/omnidir.hpp:141
	pub fn cv_omnidir_initUndistortRectifyMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_int_const__OutputArrayR_const__OutputArrayR_int(k: *const c_void, d: *const c_void, xi: *const c_void, r: *const c_void, p: *const c_void, size: *const core::Size, m1type: i32, map1: *const c_void, map2: *const c_void, flags: i32, ocvrs_return: *mut Result_void);
	// projectPoints(cv::InputArray, cv::OutputArray, const cv::Affine3d &, cv::InputArray, double, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ccalib/omnidir.hpp:107
	pub fn cv_omnidir_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(object_points: *const c_void, image_points: *const c_void, affine: *const core::Affine3d, k: *const c_void, xi: f64, d: *const c_void, jacobian: *const c_void, ocvrs_return: *mut Result_void);
	// projectPoints(cv::InputArray, cv::OutputArray, cv::InputArray, cv::InputArray, cv::InputArray, double, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ccalib/omnidir.hpp:103
	pub fn cv_omnidir_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(object_points: *const c_void, image_points: *const c_void, rvec: *const c_void, tvec: *const c_void, k: *const c_void, xi: f64, d: *const c_void, jacobian: *const c_void, ocvrs_return: *mut Result_void);
	// stereoCalibrate(cv::InputOutputArrayOfArrays, cv::InputOutputArrayOfArrays, cv::InputOutputArrayOfArrays, const cv::Size &, const cv::Size &, cv::InputOutputArray, cv::InputOutputArray, cv::InputOutputArray, cv::InputOutputArray, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, int, cv::TermCriteria, cv::OutputArray) /usr/include/opencv2/ccalib/omnidir.hpp:207
	pub fn cv_omnidir_stereoCalibrate_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_SizeR_const_SizeR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria_const__OutputArrayR(object_points: *const c_void, image_points1: *const c_void, image_points2: *const c_void, image_size1: *const core::Size, image_size2: *const core::Size, k1: *const c_void, xi1: *const c_void, d1: *const c_void, k2: *const c_void, xi2: *const c_void, d2: *const c_void, rvec: *const c_void, tvec: *const c_void, rvecs_l: *const c_void, tvecs_l: *const c_void, flags: i32, criteria: *const core::TermCriteria, idx: *const c_void, ocvrs_return: *mut Result<f64>);
	// stereoReconstruct(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, int, int, int, cv::OutputArray, cv::OutputArray, cv::OutputArray, const cv::Size &, cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ccalib/omnidir.hpp:243
	pub fn cv_omnidir_stereoReconstruct_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_int_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_int(image1: *const c_void, image2: *const c_void, k1: *const c_void, d1: *const c_void, xi1: *const c_void, k2: *const c_void, d2: *const c_void, xi2: *const c_void, r: *const c_void, t: *const c_void, flag: i32, num_disparities: i32, sad_window_size: i32, disparity: *const c_void, image1_rec: *const c_void, image2_rec: *const c_void, new_size: *const core::Size, knew: *const c_void, point_cloud: *const c_void, point_type: i32, ocvrs_return: *mut Result_void);
	// stereoRectify(cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ccalib/omnidir.hpp:218
	pub fn cv_omnidir_stereoRectify_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(r: *const c_void, t: *const c_void, r1: *const c_void, r2: *const c_void, ocvrs_return: *mut Result_void);
	// undistortImage(cv::InputArray, cv::OutputArray, cv::InputArray, cv::InputArray, cv::InputArray, int, cv::InputArray, const cv::Size &, cv::InputArray) /usr/include/opencv2/ccalib/omnidir.hpp:156
	pub fn cv_omnidir_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_const__InputArrayR_const_SizeR_const__InputArrayR(distorted: *const c_void, undistorted: *const c_void, k: *const c_void, d: *const c_void, xi: *const c_void, flags: i32, knew: *const c_void, new_size: *const core::Size, r: *const c_void, ocvrs_return: *mut Result_void);
	// undistortPoints(cv::InputArray, cv::OutputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/ccalib/omnidir.hpp:122
	pub fn cv_omnidir_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(distorted: *const c_void, undistorted: *const c_void, k: *const c_void, d: *const c_void, xi: *const c_void, r: *const c_void, ocvrs_return: *mut Result_void);
	// CustomPattern() /usr/include/opencv2/ccalib.hpp:63
	pub fn cv_ccalib_CustomPattern_CustomPattern(ocvrs_return: *mut Result<*mut c_void>);
	// create(cv::InputArray, const cv::Size2f, cv::OutputArray) /usr/include/opencv2/ccalib.hpp:66
	pub fn cv_ccalib_CustomPattern_create_const__InputArrayR_const_Size2f_const__OutputArrayR(instance: *mut c_void, pattern: *const c_void, board_size: *const core::Size2f, output: *const c_void, ocvrs_return: *mut Result<bool>);
	// findPattern(cv::InputArray, cv::OutputArray, cv::OutputArray, const double, const double, const bool, cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ccalib.hpp:68
	pub fn cv_ccalib_CustomPattern_findPattern_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const_double_const_double_const_bool_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, matched_features: *const c_void, pattern_points: *const c_void, ratio: f64, proj_error: f64, refine_position: bool, out: *const c_void, h: *const c_void, pattern_corners: *const c_void, ocvrs_return: *mut Result<bool>);
	// isInitialized() /usr/include/opencv2/ccalib.hpp:72
	pub fn cv_ccalib_CustomPattern_isInitialized(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
	// getPatternPoints(std::vector<KeyPoint> &) /usr/include/opencv2/ccalib.hpp:74
	pub fn cv_ccalib_CustomPattern_getPatternPoints_vector_KeyPoint_R(instance: *mut c_void, original_points: *mut c_void, ocvrs_return: *mut Result_void);
	// getPixelSize() /usr/include/opencv2/ccalib.hpp:78
	pub fn cv_ccalib_CustomPattern_getPixelSize(instance: *mut c_void, ocvrs_return: *mut Result<f64>);
	// setFeatureDetector(Ptr<cv::FeatureDetector>) /usr/include/opencv2/ccalib.hpp:83
	pub fn cv_ccalib_CustomPattern_setFeatureDetector_Ptr_Feature2D_(instance: *mut c_void, feature_detector: *mut c_void, ocvrs_return: *mut Result<bool>);
	// setDescriptorExtractor(Ptr<cv::DescriptorExtractor>) /usr/include/opencv2/ccalib.hpp:84
	pub fn cv_ccalib_CustomPattern_setDescriptorExtractor_Ptr_Feature2D_(instance: *mut c_void, extractor: *mut c_void, ocvrs_return: *mut Result<bool>);
	// setDescriptorMatcher(Ptr<cv::DescriptorMatcher>) /usr/include/opencv2/ccalib.hpp:85
	pub fn cv_ccalib_CustomPattern_setDescriptorMatcher_Ptr_DescriptorMatcher_(instance: *mut c_void, matcher: *mut c_void, ocvrs_return: *mut Result<bool>);
	// getFeatureDetector() /usr/include/opencv2/ccalib.hpp:87
	pub fn cv_ccalib_CustomPattern_getFeatureDetector(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getDescriptorExtractor() /usr/include/opencv2/ccalib.hpp:88
	pub fn cv_ccalib_CustomPattern_getDescriptorExtractor(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getDescriptorMatcher() /usr/include/opencv2/ccalib.hpp:89
	pub fn cv_ccalib_CustomPattern_getDescriptorMatcher(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// calibrate(cv::InputArrayOfArrays, cv::InputArrayOfArrays, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, int, cv::TermCriteria) /usr/include/opencv2/ccalib.hpp:91
	pub fn cv_ccalib_CustomPattern_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(instance: *mut c_void, object_points: *const c_void, image_points: *const c_void, image_size: *const core::Size, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvecs: *const c_void, tvecs: *const c_void, flags: i32, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<f64>);
	// findRt(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool, int) /usr/include/opencv2/ccalib.hpp:99
	pub fn cv_ccalib_CustomPattern_findRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool_int(instance: *mut c_void, object_points: *const c_void, image_points: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, use_extrinsic_guess: bool, flags: i32, ocvrs_return: *mut Result<bool>);
	// findRt(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool, int) /usr/include/opencv2/ccalib.hpp:101
	pub fn cv_ccalib_CustomPattern_findRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool_int(instance: *mut c_void, image: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, use_extrinsic_guess: bool, flags: i32, ocvrs_return: *mut Result<bool>);
	// findRtRANSAC(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool, int, float, int, cv::OutputArray, int) /usr/include/opencv2/ccalib.hpp:108
	pub fn cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool_int_float_int_const__OutputArrayR_int(instance: *mut c_void, object_points: *const c_void, image_points: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, min_inliers_count: i32, inliers: *const c_void, flags: i32, ocvrs_return: *mut Result<bool>);
	// findRtRANSAC(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool, int, float, int, cv::OutputArray, int) /usr/include/opencv2/ccalib.hpp:111
	pub fn cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool_int_float_int_const__OutputArrayR_int(instance: *mut c_void, image: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, min_inliers_count: i32, inliers: *const c_void, flags: i32, ocvrs_return: *mut Result<bool>);
	// drawOrientation(cv::InputOutputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, double, int) /usr/include/opencv2/ccalib.hpp:118
	pub fn cv_ccalib_CustomPattern_drawOrientation_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_int(instance: *mut c_void, image: *const c_void, tvec: *const c_void, rvec: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, axis_length: f64, axis_width: i32, ocvrs_return: *mut Result_void);
	// MultiCameraCalibration(int, int, const std::string &, float, float, int, int, int, int, cv::TermCriteria, Ptr<cv::FeatureDetector>, Ptr<cv::DescriptorExtractor>, Ptr<cv::DescriptorMatcher>) /usr/include/opencv2/ccalib/multicalib.hpp:132
	pub fn cv_multicalib_MultiCameraCalibration_MultiCameraCalibration_int_int_const_stringR_float_float_int_int_int_int_TermCriteria_Ptr_Feature2D__Ptr_Feature2D__Ptr_DescriptorMatcher_(camera_type: i32, n_cameras: i32, file_name: *const c_char, pattern_width: f32, pattern_height: f32, verbose: i32, show_extration: i32, n_mini_matches: i32, flags: i32, criteria: *const core::TermCriteria, detector: *mut c_void, descriptor: *mut c_void, matcher: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// loadImages() /usr/include/opencv2/ccalib/multicalib.hpp:141
	pub fn cv_multicalib_MultiCameraCalibration_loadImages(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// initialize() /usr/include/opencv2/ccalib/multicalib.hpp:145
	pub fn cv_multicalib_MultiCameraCalibration_initialize(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// optimizeExtrinsics() /usr/include/opencv2/ccalib/multicalib.hpp:149
	pub fn cv_multicalib_MultiCameraCalibration_optimizeExtrinsics(instance: *mut c_void, ocvrs_return: *mut Result<f64>);
	// run() /usr/include/opencv2/ccalib/multicalib.hpp:153
	pub fn cv_multicalib_MultiCameraCalibration_run(instance: *mut c_void, ocvrs_return: *mut Result<f64>);
	// writeParameters(const std::string &) /usr/include/opencv2/ccalib/multicalib.hpp:157
	pub fn cv_multicalib_MultiCameraCalibration_writeParameters_const_stringR(instance: *mut c_void, filename: *const c_char, ocvrs_return: *mut Result_void);
	// cameraVertex /usr/include/opencv2/ccalib/multicalib.hpp:84
	pub fn cv_multicalib_MultiCameraCalibration_edge_getPropCameraVertex_const(instance: *const c_void) -> i32;
	// cameraVertex /usr/include/opencv2/ccalib/multicalib.hpp:84
	pub fn cv_multicalib_MultiCameraCalibration_edge_setPropCameraVertex_int(instance: *mut c_void, val: i32);
	// photoVertex /usr/include/opencv2/ccalib/multicalib.hpp:85
	pub fn cv_multicalib_MultiCameraCalibration_edge_getPropPhotoVertex_const(instance: *const c_void) -> i32;
	// photoVertex /usr/include/opencv2/ccalib/multicalib.hpp:85
	pub fn cv_multicalib_MultiCameraCalibration_edge_setPropPhotoVertex_int(instance: *mut c_void, val: i32);
	// photoIndex /usr/include/opencv2/ccalib/multicalib.hpp:86
	pub fn cv_multicalib_MultiCameraCalibration_edge_getPropPhotoIndex_const(instance: *const c_void) -> i32;
	// photoIndex /usr/include/opencv2/ccalib/multicalib.hpp:86
	pub fn cv_multicalib_MultiCameraCalibration_edge_setPropPhotoIndex_int(instance: *mut c_void, val: i32);
	// transform /usr/include/opencv2/ccalib/multicalib.hpp:87
	pub fn cv_multicalib_MultiCameraCalibration_edge_getPropTransform_const(instance: *const c_void) -> *mut c_void;
	// transform /usr/include/opencv2/ccalib/multicalib.hpp:87
	pub fn cv_multicalib_MultiCameraCalibration_edge_setPropTransform_Mat(instance: *mut c_void, val: *mut c_void);
	// edge(int, int, int, cv::Mat) /usr/include/opencv2/ccalib/multicalib.hpp:89
	pub fn cv_multicalib_MultiCameraCalibration_edge_edge_int_int_int_Mat(cv: i32, pv: i32, pi: i32, trans: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// pose /usr/include/opencv2/ccalib/multicalib.hpp:100
	pub fn cv_multicalib_MultiCameraCalibration_vertex_getPropPose_const(instance: *const c_void) -> *mut c_void;
	// pose /usr/include/opencv2/ccalib/multicalib.hpp:100
	pub fn cv_multicalib_MultiCameraCalibration_vertex_setPropPose_Mat(instance: *mut c_void, val: *mut c_void);
	// timestamp /usr/include/opencv2/ccalib/multicalib.hpp:103
	pub fn cv_multicalib_MultiCameraCalibration_vertex_getPropTimestamp_const(instance: *const c_void) -> i32;
	// timestamp /usr/include/opencv2/ccalib/multicalib.hpp:103
	pub fn cv_multicalib_MultiCameraCalibration_vertex_setPropTimestamp_int(instance: *mut c_void, val: i32);
	// vertex(cv::Mat, int) /usr/include/opencv2/ccalib/multicalib.hpp:105
	pub fn cv_multicalib_MultiCameraCalibration_vertex_vertex_Mat_int(po: *mut c_void, ts: i32, ocvrs_return: *mut Result<*mut c_void>);
	// vertex() /usr/include/opencv2/ccalib/multicalib.hpp:111
	pub fn cv_multicalib_MultiCameraCalibration_vertex_vertex(ocvrs_return: *mut Result<*mut c_void>);
	// RandomPatternCornerFinder(float, float, int, int, int, int, Ptr<cv::FeatureDetector>, Ptr<cv::DescriptorExtractor>, Ptr<cv::DescriptorMatcher>) /usr/include/opencv2/ccalib/randpattern.hpp:80
	pub fn cv_randpattern_RandomPatternCornerFinder_RandomPatternCornerFinder_float_float_int_int_int_int_Ptr_Feature2D__Ptr_Feature2D__Ptr_DescriptorMatcher_(pattern_width: f32, pattern_height: f32, nmini_match: i32, depth: i32, verbose: i32, show_extraction: i32, detector: *mut c_void, descriptor: *mut c_void, matcher: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// loadPattern(const cv::Mat &) /usr/include/opencv2/ccalib/randpattern.hpp:89
	pub fn cv_randpattern_RandomPatternCornerFinder_loadPattern_const_MatR(instance: *mut c_void, pattern_image: *const c_void, ocvrs_return: *mut Result_void);
	// loadPattern(const cv::Mat &, const std::vector<cv::KeyPoint> &, const cv::Mat &) /usr/include/opencv2/ccalib/randpattern.hpp:96
	pub fn cv_randpattern_RandomPatternCornerFinder_loadPattern_const_MatR_const_vector_KeyPoint_R_const_MatR(instance: *mut c_void, pattern_image: *const c_void, pattern_key_points: *const c_void, pattern_descriptors: *const c_void, ocvrs_return: *mut Result_void);
	// computeObjectImagePoints(std::vector<cv::Mat>) /usr/include/opencv2/ccalib/randpattern.hpp:105
	pub fn cv_randpattern_RandomPatternCornerFinder_computeObjectImagePoints_vector_Mat_(instance: *mut c_void, input_images: *mut c_void, ocvrs_return: *mut Result_void);
	// computeObjectImagePointsForSingle(cv::Mat) /usr/include/opencv2/ccalib/randpattern.hpp:114
	pub fn cv_randpattern_RandomPatternCornerFinder_computeObjectImagePointsForSingle_Mat(instance: *mut c_void, input_image: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getObjectPoints() /usr/include/opencv2/ccalib/randpattern.hpp:118
	pub fn cv_randpattern_RandomPatternCornerFinder_getObjectPoints(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getImagePoints() /usr/include/opencv2/ccalib/randpattern.hpp:122
	pub fn cv_randpattern_RandomPatternCornerFinder_getImagePoints(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// RandomPatternGenerator(int, int) /usr/include/opencv2/ccalib/randpattern.hpp:168
	pub fn cv_randpattern_RandomPatternGenerator_RandomPatternGenerator_int_int(image_width: i32, image_height: i32, ocvrs_return: *mut Result<*mut c_void>);
	// generatePattern() /usr/include/opencv2/ccalib/randpattern.hpp:172
	pub fn cv_randpattern_RandomPatternGenerator_generatePattern(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// getPattern() /usr/include/opencv2/ccalib/randpattern.hpp:175
	pub fn cv_randpattern_RandomPatternGenerator_getPattern(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
}
