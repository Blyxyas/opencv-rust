extern "C" {
	// calibrateCameraAruco(cv::InputArrayOfArrays, cv::InputArray, cv::InputArray, const Ptr<cv::aruco::Board> &, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, cv::OutputArray, cv::OutputArray, cv::OutputArray, int, cv::TermCriteria) /usr/include/opencv2/aruco.hpp:648
	pub fn cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_Ptr_Board_R_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(corners: *const c_void, ids: *const c_void, counter: *const c_void, board: *const c_void, image_size: *const core::Size, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvecs: *const c_void, tvecs: *const c_void, std_deviations_intrinsics: *const c_void, std_deviations_extrinsics: *const c_void, per_view_errors: *const c_void, flags: i32, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<f64>);
	// calibrateCameraAruco(cv::InputArrayOfArrays, cv::InputArray, cv::InputArray, const Ptr<cv::aruco::Board> &, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, int, cv::TermCriteria) /usr/include/opencv2/aruco.hpp:659
	pub fn cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_Ptr_Board_R_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(corners: *const c_void, ids: *const c_void, counter: *const c_void, board: *const c_void, image_size: *const core::Size, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvecs: *const c_void, tvecs: *const c_void, flags: i32, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<f64>);
	// calibrateCameraCharuco(cv::InputArrayOfArrays, cv::InputArrayOfArrays, const Ptr<cv::aruco::CharucoBoard> &, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, cv::OutputArray, cv::OutputArray, cv::OutputArray, int, cv::TermCriteria) /usr/include/opencv2/aruco/charuco.hpp:245
	pub fn cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_Ptr_CharucoBoard_R_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(charuco_corners: *const c_void, charuco_ids: *const c_void, board: *const c_void, image_size: *const core::Size, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvecs: *const c_void, tvecs: *const c_void, std_deviations_intrinsics: *const c_void, std_deviations_extrinsics: *const c_void, per_view_errors: *const c_void, flags: i32, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<f64>);
	// calibrateCameraCharuco(cv::InputArrayOfArrays, cv::InputArrayOfArrays, const Ptr<cv::aruco::CharucoBoard> &, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, int, cv::TermCriteria) /usr/include/opencv2/aruco/charuco.hpp:255
	pub fn cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_Ptr_CharucoBoard_R_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(charuco_corners: *const c_void, charuco_ids: *const c_void, board: *const c_void, image_size: *const core::Size, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvecs: *const c_void, tvecs: *const c_void, flags: i32, criteria: *const core::TermCriteria, ocvrs_return: *mut Result<f64>);
	// detectCharucoDiamond(cv::InputArray, cv::InputArrayOfArrays, cv::InputArray, float, cv::OutputArrayOfArrays, cv::OutputArray, cv::InputArray, cv::InputArray, Ptr<cv::aruco::Dictionary>) /usr/include/opencv2/aruco/charuco.hpp:286
	pub fn cv_aruco_detectCharucoDiamond_const__InputArrayR_const__InputArrayR_const__InputArrayR_float_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_Ptr_Dictionary_(image: *const c_void, marker_corners: *const c_void, marker_ids: *const c_void, square_marker_length_rate: f32, diamond_corners: *const c_void, diamond_ids: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, dictionary: *mut c_void, ocvrs_return: *mut Result_void);
	// detectMarkers(cv::InputArray, const Ptr<cv::aruco::Dictionary> &, cv::OutputArrayOfArrays, cv::OutputArray, const Ptr<cv::aruco::DetectorParameters> &, cv::OutputArrayOfArrays) /usr/include/opencv2/aruco.hpp:244
	pub fn cv_aruco_detectMarkers_const__InputArrayR_const_Ptr_Dictionary_R_const__OutputArrayR_const__OutputArrayR_const_Ptr_DetectorParameters_R_const__OutputArrayR(image: *const c_void, dictionary: *const c_void, corners: *const c_void, ids: *const c_void, parameters: *const c_void, rejected_img_points: *const c_void, ocvrs_return: *mut Result_void);
	// drawCharucoDiamond(const Ptr<cv::aruco::Dictionary> &, cv::Vec4i, int, int, cv::OutputArray, int, int) /usr/include/opencv2/aruco/charuco.hpp:334
	pub fn cv_aruco_drawCharucoDiamond_const_Ptr_Dictionary_R_Vec4i_int_int_const__OutputArrayR_int_int(dictionary: *const c_void, ids: *const core::Vec4i, square_length: i32, marker_length: i32, img: *const c_void, margin_size: i32, border_bits: i32, ocvrs_return: *mut Result_void);
	// drawDetectedCornersCharuco(cv::InputOutputArray, cv::InputArray, cv::InputArray, cv::Scalar) /usr/include/opencv2/aruco/charuco.hpp:205
	pub fn cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image: *const c_void, charuco_corners: *const c_void, charuco_ids: *const c_void, corner_color: *const core::Scalar, ocvrs_return: *mut Result_void);
	// drawDetectedDiamonds(cv::InputOutputArray, cv::InputArrayOfArrays, cv::InputArray, cv::Scalar) /usr/include/opencv2/aruco/charuco.hpp:313
	pub fn cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image: *const c_void, diamond_corners: *const c_void, diamond_ids: *const c_void, border_color: *const core::Scalar, ocvrs_return: *mut Result_void);
	// drawDetectedMarkers(cv::InputOutputArray, cv::InputArrayOfArrays, cv::InputArray, cv::Scalar) /usr/include/opencv2/aruco.hpp:561
	pub fn cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image: *const c_void, corners: *const c_void, ids: *const c_void, border_color: *const core::Scalar, ocvrs_return: *mut Result_void);
	// drawMarker(const Ptr<cv::aruco::Dictionary> &, int, int, cv::OutputArray, int) /usr/include/opencv2/aruco.hpp:579
	pub fn cv_aruco_drawMarker_const_Ptr_Dictionary_R_int_int_const__OutputArrayR_int(dictionary: *const c_void, id: i32, side_pixels: i32, img: *const c_void, border_bits: i32, ocvrs_return: *mut Result_void);
	// drawPlanarBoard(const Ptr<cv::aruco::Board> &, cv::Size, cv::OutputArray, int, int) /usr/include/opencv2/aruco.hpp:599
	pub fn cv_aruco_drawPlanarBoard_const_Ptr_Board_R_Size_const__OutputArrayR_int_int(board: *const c_void, out_size: *const core::Size, img: *const c_void, margin_size: i32, border_bits: i32, ocvrs_return: *mut Result_void);
	// estimatePoseBoard(cv::InputArrayOfArrays, cv::InputArray, const Ptr<cv::aruco::Board> &, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool) /usr/include/opencv2/aruco.hpp:496
	pub fn cv_aruco_estimatePoseBoard_const__InputArrayR_const__InputArrayR_const_Ptr_Board_R_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool(corners: *const c_void, ids: *const c_void, board: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, use_extrinsic_guess: bool, ocvrs_return: *mut Result<i32>);
	// estimatePoseCharucoBoard(cv::InputArray, cv::InputArray, const Ptr<cv::aruco::CharucoBoard> &, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool) /usr/include/opencv2/aruco/charuco.hpp:186
	pub fn cv_aruco_estimatePoseCharucoBoard_const__InputArrayR_const__InputArrayR_const_Ptr_CharucoBoard_R_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool(charuco_corners: *const c_void, charuco_ids: *const c_void, board: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvec: *const c_void, tvec: *const c_void, use_extrinsic_guess: bool, ocvrs_return: *mut Result<bool>);
	// estimatePoseSingleMarkers(cv::InputArrayOfArrays, float, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray, Ptr<cv::aruco::EstimateParameters>) /usr/include/opencv2/aruco.hpp:334
	pub fn cv_aruco_estimatePoseSingleMarkers_const__InputArrayR_float_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_Ptr_EstimateParameters_(corners: *const c_void, marker_length: f32, camera_matrix: *const c_void, dist_coeffs: *const c_void, rvecs: *const c_void, tvecs: *const c_void, _obj_points: *const c_void, estimate_parameters: *mut c_void, ocvrs_return: *mut Result_void);
	// generateCustomDictionary(int, int, const Ptr<cv::aruco::Dictionary> &, int) /usr/include/opencv2/aruco/dictionary.hpp:215
	pub fn cv_aruco_generateCustomDictionary_int_int_const_Ptr_Dictionary_R_int(n_markers: i32, marker_size: i32, base_dictionary: *const c_void, random_seed: i32, ocvrs_return: *mut Result<*mut c_void>);
	// generateCustomDictionary(int, int, int) /usr/include/opencv2/aruco/dictionary.hpp:196
	pub fn cv_aruco_generateCustomDictionary_int_int_int(n_markers: i32, marker_size: i32, random_seed: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getBoardObjectAndImagePoints(const Ptr<cv::aruco::Board> &, cv::InputArrayOfArrays, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/aruco.hpp:676
	pub fn cv_aruco_getBoardObjectAndImagePoints_const_Ptr_Board_R_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(board: *const c_void, detected_corners: *const c_void, detected_ids: *const c_void, obj_points: *const c_void, img_points: *const c_void, ocvrs_return: *mut Result_void);
	// getPredefinedDictionary(cv::aruco::PREDEFINED_DICTIONARY_NAME) /usr/include/opencv2/aruco/dictionary.hpp:184
	pub fn cv_aruco_getPredefinedDictionary_PREDEFINED_DICTIONARY_NAME(name: crate::aruco::PREDEFINED_DICTIONARY_NAME, ocvrs_return: *mut Result<*mut c_void>);
	// getPredefinedDictionary(int) /usr/include/opencv2/aruco/dictionary.hpp:190
	pub fn cv_aruco_getPredefinedDictionary_int(dict: i32, ocvrs_return: *mut Result<*mut c_void>);
	// interpolateCornersCharuco(cv::InputArrayOfArrays, cv::InputArray, cv::InputArray, const Ptr<cv::aruco::CharucoBoard> &, cv::OutputArray, cv::OutputArray, cv::InputArray, cv::InputArray, int) /usr/include/opencv2/aruco/charuco.hpp:158
	pub fn cv_aruco_interpolateCornersCharuco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_Ptr_CharucoBoard_R_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(marker_corners: *const c_void, marker_ids: *const c_void, image: *const c_void, board: *const c_void, charuco_corners: *const c_void, charuco_ids: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, min_markers: i32, ocvrs_return: *mut Result<i32>);
	// refineDetectedMarkers(cv::InputArray, const Ptr<cv::aruco::Board> &, cv::InputOutputArrayOfArrays, cv::InputOutputArray, cv::InputOutputArrayOfArrays, cv::InputArray, cv::InputArray, float, float, bool, cv::OutputArray, const Ptr<cv::aruco::DetectorParameters> &) /usr/include/opencv2/aruco.hpp:534
	pub fn cv_aruco_refineDetectedMarkers_const__InputArrayR_const_Ptr_Board_R_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_float_float_bool_const__OutputArrayR_const_Ptr_DetectorParameters_R(image: *const c_void, board: *const c_void, detected_corners: *const c_void, detected_ids: *const c_void, rejected_corners: *const c_void, camera_matrix: *const c_void, dist_coeffs: *const c_void, min_rep_distance: f32, error_correction_rate: f32, check_all_orders: bool, recovered_idxs: *const c_void, parameters: *const c_void, ocvrs_return: *mut Result_void);
	// testCharucoCornersCollinear(const Ptr<cv::aruco::CharucoBoard> &, cv::InputArray) /usr/include/opencv2/aruco/charuco.hpp:349
	pub fn cv_aruco_testCharucoCornersCollinear_const_Ptr_CharucoBoard_R_const__InputArrayR(_board: *const c_void, _charuco_ids: *const c_void, ocvrs_return: *mut Result<bool>);
	// objPoints /usr/include/opencv2/aruco.hpp:384
	pub fn cv_aruco_Board_getPropObjPoints_const(instance: *const c_void) -> *mut c_void;
	// objPoints /usr/include/opencv2/aruco.hpp:384
	pub fn cv_aruco_Board_setPropObjPoints_vector_vector_Point3f__(instance: *mut c_void, val: *mut c_void);
	// dictionary /usr/include/opencv2/aruco.hpp:387
	pub fn cv_aruco_Board_getPropDictionary(instance: *mut c_void) -> *mut c_void;
	// dictionary /usr/include/opencv2/aruco.hpp:387
	pub fn cv_aruco_Board_setPropDictionary_Ptr_Dictionary_(instance: *mut c_void, val: *mut c_void);
	// ids /usr/include/opencv2/aruco.hpp:391
	pub fn cv_aruco_Board_getPropIds_const(instance: *const c_void) -> *mut c_void;
	// ids /usr/include/opencv2/aruco.hpp:391
	pub fn cv_aruco_Board_setPropIds_vector_int_(instance: *mut c_void, val: *mut c_void);
	// rightBottomBorder /usr/include/opencv2/aruco.hpp:394
	pub fn cv_aruco_Board_getPropRightBottomBorder_const(instance: *const c_void, ocvrs_return: *mut core::Point3f);
	// rightBottomBorder /usr/include/opencv2/aruco.hpp:394
	pub fn cv_aruco_Board_setPropRightBottomBorder_Point3f(instance: *mut c_void, val: *const core::Point3f);
	// create(cv::InputArrayOfArrays, const Ptr<cv::aruco::Dictionary> &, cv::InputArray) /usr/include/opencv2/aruco.hpp:362
	pub fn cv_aruco_Board_create_const__InputArrayR_const_Ptr_Dictionary_R_const__InputArrayR(obj_points: *const c_void, dictionary: *const c_void, ids: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setIds(cv::InputArray) /usr/include/opencv2/aruco.hpp:373
	pub fn cv_aruco_Board_setIds_const__InputArrayR(instance: *mut c_void, ids: *const c_void, ocvrs_return: *mut Result_void);
	// chessboardCorners /usr/include/opencv2/aruco/charuco.hpp:66
	pub fn cv_aruco_CharucoBoard_getPropChessboardCorners_const(instance: *const c_void) -> *mut c_void;
	// chessboardCorners /usr/include/opencv2/aruco/charuco.hpp:66
	pub fn cv_aruco_CharucoBoard_setPropChessboardCorners_vector_Point3f_(instance: *mut c_void, val: *mut c_void);
	// nearestMarkerIdx /usr/include/opencv2/aruco/charuco.hpp:69
	pub fn cv_aruco_CharucoBoard_getPropNearestMarkerIdx_const(instance: *const c_void) -> *mut c_void;
	// nearestMarkerIdx /usr/include/opencv2/aruco/charuco.hpp:69
	pub fn cv_aruco_CharucoBoard_setPropNearestMarkerIdx_vector_vector_int__(instance: *mut c_void, val: *mut c_void);
	// nearestMarkerCorners /usr/include/opencv2/aruco/charuco.hpp:70
	pub fn cv_aruco_CharucoBoard_getPropNearestMarkerCorners_const(instance: *const c_void) -> *mut c_void;
	// nearestMarkerCorners /usr/include/opencv2/aruco/charuco.hpp:70
	pub fn cv_aruco_CharucoBoard_setPropNearestMarkerCorners_vector_vector_int__(instance: *mut c_void, val: *mut c_void);
	// draw(cv::Size, cv::OutputArray, int, int) /usr/include/opencv2/aruco/charuco.hpp:83
	pub fn cv_aruco_CharucoBoard_draw_Size_const__OutputArrayR_int_int(instance: *mut c_void, out_size: *const core::Size, img: *const c_void, margin_size: i32, border_bits: i32, ocvrs_return: *mut Result_void);
	// create(int, int, float, float, const Ptr<cv::aruco::Dictionary> &) /usr/include/opencv2/aruco/charuco.hpp:100
	pub fn cv_aruco_CharucoBoard_create_int_int_float_float_const_Ptr_Dictionary_R(squares_x: i32, squares_y: i32, square_length: f32, marker_length: f32, dictionary: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getChessboardSize() /usr/include/opencv2/aruco/charuco.hpp:106
	pub fn cv_aruco_CharucoBoard_getChessboardSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// getSquareLength() /usr/include/opencv2/aruco/charuco.hpp:111
	pub fn cv_aruco_CharucoBoard_getSquareLength_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// getMarkerLength() /usr/include/opencv2/aruco/charuco.hpp:116
	pub fn cv_aruco_CharucoBoard_getMarkerLength_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// adaptiveThreshWinSizeMin /usr/include/opencv2/aruco.hpp:175
	pub fn cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeMin_const(instance: *const c_void) -> i32;
	// adaptiveThreshWinSizeMin /usr/include/opencv2/aruco.hpp:175
	pub fn cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeMin_int(instance: *mut c_void, val: i32);
	// adaptiveThreshWinSizeMax /usr/include/opencv2/aruco.hpp:176
	pub fn cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeMax_const(instance: *const c_void) -> i32;
	// adaptiveThreshWinSizeMax /usr/include/opencv2/aruco.hpp:176
	pub fn cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeMax_int(instance: *mut c_void, val: i32);
	// adaptiveThreshWinSizeStep /usr/include/opencv2/aruco.hpp:177
	pub fn cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeStep_const(instance: *const c_void) -> i32;
	// adaptiveThreshWinSizeStep /usr/include/opencv2/aruco.hpp:177
	pub fn cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeStep_int(instance: *mut c_void, val: i32);
	// adaptiveThreshConstant /usr/include/opencv2/aruco.hpp:178
	pub fn cv_aruco_DetectorParameters_getPropAdaptiveThreshConstant_const(instance: *const c_void) -> f64;
	// adaptiveThreshConstant /usr/include/opencv2/aruco.hpp:178
	pub fn cv_aruco_DetectorParameters_setPropAdaptiveThreshConstant_double(instance: *mut c_void, val: f64);
	// minMarkerPerimeterRate /usr/include/opencv2/aruco.hpp:179
	pub fn cv_aruco_DetectorParameters_getPropMinMarkerPerimeterRate_const(instance: *const c_void) -> f64;
	// minMarkerPerimeterRate /usr/include/opencv2/aruco.hpp:179
	pub fn cv_aruco_DetectorParameters_setPropMinMarkerPerimeterRate_double(instance: *mut c_void, val: f64);
	// maxMarkerPerimeterRate /usr/include/opencv2/aruco.hpp:180
	pub fn cv_aruco_DetectorParameters_getPropMaxMarkerPerimeterRate_const(instance: *const c_void) -> f64;
	// maxMarkerPerimeterRate /usr/include/opencv2/aruco.hpp:180
	pub fn cv_aruco_DetectorParameters_setPropMaxMarkerPerimeterRate_double(instance: *mut c_void, val: f64);
	// polygonalApproxAccuracyRate /usr/include/opencv2/aruco.hpp:181
	pub fn cv_aruco_DetectorParameters_getPropPolygonalApproxAccuracyRate_const(instance: *const c_void) -> f64;
	// polygonalApproxAccuracyRate /usr/include/opencv2/aruco.hpp:181
	pub fn cv_aruco_DetectorParameters_setPropPolygonalApproxAccuracyRate_double(instance: *mut c_void, val: f64);
	// minCornerDistanceRate /usr/include/opencv2/aruco.hpp:182
	pub fn cv_aruco_DetectorParameters_getPropMinCornerDistanceRate_const(instance: *const c_void) -> f64;
	// minCornerDistanceRate /usr/include/opencv2/aruco.hpp:182
	pub fn cv_aruco_DetectorParameters_setPropMinCornerDistanceRate_double(instance: *mut c_void, val: f64);
	// minDistanceToBorder /usr/include/opencv2/aruco.hpp:183
	pub fn cv_aruco_DetectorParameters_getPropMinDistanceToBorder_const(instance: *const c_void) -> i32;
	// minDistanceToBorder /usr/include/opencv2/aruco.hpp:183
	pub fn cv_aruco_DetectorParameters_setPropMinDistanceToBorder_int(instance: *mut c_void, val: i32);
	// minMarkerDistanceRate /usr/include/opencv2/aruco.hpp:184
	pub fn cv_aruco_DetectorParameters_getPropMinMarkerDistanceRate_const(instance: *const c_void) -> f64;
	// minMarkerDistanceRate /usr/include/opencv2/aruco.hpp:184
	pub fn cv_aruco_DetectorParameters_setPropMinMarkerDistanceRate_double(instance: *mut c_void, val: f64);
	// cornerRefinementMethod /usr/include/opencv2/aruco.hpp:185
	pub fn cv_aruco_DetectorParameters_getPropCornerRefinementMethod_const(instance: *const c_void) -> i32;
	// cornerRefinementMethod /usr/include/opencv2/aruco.hpp:185
	pub fn cv_aruco_DetectorParameters_setPropCornerRefinementMethod_int(instance: *mut c_void, val: i32);
	// cornerRefinementWinSize /usr/include/opencv2/aruco.hpp:186
	pub fn cv_aruco_DetectorParameters_getPropCornerRefinementWinSize_const(instance: *const c_void) -> i32;
	// cornerRefinementWinSize /usr/include/opencv2/aruco.hpp:186
	pub fn cv_aruco_DetectorParameters_setPropCornerRefinementWinSize_int(instance: *mut c_void, val: i32);
	// cornerRefinementMaxIterations /usr/include/opencv2/aruco.hpp:187
	pub fn cv_aruco_DetectorParameters_getPropCornerRefinementMaxIterations_const(instance: *const c_void) -> i32;
	// cornerRefinementMaxIterations /usr/include/opencv2/aruco.hpp:187
	pub fn cv_aruco_DetectorParameters_setPropCornerRefinementMaxIterations_int(instance: *mut c_void, val: i32);
	// cornerRefinementMinAccuracy /usr/include/opencv2/aruco.hpp:188
	pub fn cv_aruco_DetectorParameters_getPropCornerRefinementMinAccuracy_const(instance: *const c_void) -> f64;
	// cornerRefinementMinAccuracy /usr/include/opencv2/aruco.hpp:188
	pub fn cv_aruco_DetectorParameters_setPropCornerRefinementMinAccuracy_double(instance: *mut c_void, val: f64);
	// markerBorderBits /usr/include/opencv2/aruco.hpp:189
	pub fn cv_aruco_DetectorParameters_getPropMarkerBorderBits_const(instance: *const c_void) -> i32;
	// markerBorderBits /usr/include/opencv2/aruco.hpp:189
	pub fn cv_aruco_DetectorParameters_setPropMarkerBorderBits_int(instance: *mut c_void, val: i32);
	// perspectiveRemovePixelPerCell /usr/include/opencv2/aruco.hpp:190
	pub fn cv_aruco_DetectorParameters_getPropPerspectiveRemovePixelPerCell_const(instance: *const c_void) -> i32;
	// perspectiveRemovePixelPerCell /usr/include/opencv2/aruco.hpp:190
	pub fn cv_aruco_DetectorParameters_setPropPerspectiveRemovePixelPerCell_int(instance: *mut c_void, val: i32);
	// perspectiveRemoveIgnoredMarginPerCell /usr/include/opencv2/aruco.hpp:191
	pub fn cv_aruco_DetectorParameters_getPropPerspectiveRemoveIgnoredMarginPerCell_const(instance: *const c_void) -> f64;
	// perspectiveRemoveIgnoredMarginPerCell /usr/include/opencv2/aruco.hpp:191
	pub fn cv_aruco_DetectorParameters_setPropPerspectiveRemoveIgnoredMarginPerCell_double(instance: *mut c_void, val: f64);
	// maxErroneousBitsInBorderRate /usr/include/opencv2/aruco.hpp:192
	pub fn cv_aruco_DetectorParameters_getPropMaxErroneousBitsInBorderRate_const(instance: *const c_void) -> f64;
	// maxErroneousBitsInBorderRate /usr/include/opencv2/aruco.hpp:192
	pub fn cv_aruco_DetectorParameters_setPropMaxErroneousBitsInBorderRate_double(instance: *mut c_void, val: f64);
	// minOtsuStdDev /usr/include/opencv2/aruco.hpp:193
	pub fn cv_aruco_DetectorParameters_getPropMinOtsuStdDev_const(instance: *const c_void) -> f64;
	// minOtsuStdDev /usr/include/opencv2/aruco.hpp:193
	pub fn cv_aruco_DetectorParameters_setPropMinOtsuStdDev_double(instance: *mut c_void, val: f64);
	// errorCorrectionRate /usr/include/opencv2/aruco.hpp:194
	pub fn cv_aruco_DetectorParameters_getPropErrorCorrectionRate_const(instance: *const c_void) -> f64;
	// errorCorrectionRate /usr/include/opencv2/aruco.hpp:194
	pub fn cv_aruco_DetectorParameters_setPropErrorCorrectionRate_double(instance: *mut c_void, val: f64);
	// aprilTagQuadDecimate /usr/include/opencv2/aruco.hpp:197
	pub fn cv_aruco_DetectorParameters_getPropAprilTagQuadDecimate_const(instance: *const c_void) -> f32;
	// aprilTagQuadDecimate /usr/include/opencv2/aruco.hpp:197
	pub fn cv_aruco_DetectorParameters_setPropAprilTagQuadDecimate_float(instance: *mut c_void, val: f32);
	// aprilTagQuadSigma /usr/include/opencv2/aruco.hpp:198
	pub fn cv_aruco_DetectorParameters_getPropAprilTagQuadSigma_const(instance: *const c_void) -> f32;
	// aprilTagQuadSigma /usr/include/opencv2/aruco.hpp:198
	pub fn cv_aruco_DetectorParameters_setPropAprilTagQuadSigma_float(instance: *mut c_void, val: f32);
	// aprilTagMinClusterPixels /usr/include/opencv2/aruco.hpp:201
	pub fn cv_aruco_DetectorParameters_getPropAprilTagMinClusterPixels_const(instance: *const c_void) -> i32;
	// aprilTagMinClusterPixels /usr/include/opencv2/aruco.hpp:201
	pub fn cv_aruco_DetectorParameters_setPropAprilTagMinClusterPixels_int(instance: *mut c_void, val: i32);
	// aprilTagMaxNmaxima /usr/include/opencv2/aruco.hpp:202
	pub fn cv_aruco_DetectorParameters_getPropAprilTagMaxNmaxima_const(instance: *const c_void) -> i32;
	// aprilTagMaxNmaxima /usr/include/opencv2/aruco.hpp:202
	pub fn cv_aruco_DetectorParameters_setPropAprilTagMaxNmaxima_int(instance: *mut c_void, val: i32);
	// aprilTagCriticalRad /usr/include/opencv2/aruco.hpp:203
	pub fn cv_aruco_DetectorParameters_getPropAprilTagCriticalRad_const(instance: *const c_void) -> f32;
	// aprilTagCriticalRad /usr/include/opencv2/aruco.hpp:203
	pub fn cv_aruco_DetectorParameters_setPropAprilTagCriticalRad_float(instance: *mut c_void, val: f32);
	// aprilTagMaxLineFitMse /usr/include/opencv2/aruco.hpp:204
	pub fn cv_aruco_DetectorParameters_getPropAprilTagMaxLineFitMse_const(instance: *const c_void) -> f32;
	// aprilTagMaxLineFitMse /usr/include/opencv2/aruco.hpp:204
	pub fn cv_aruco_DetectorParameters_setPropAprilTagMaxLineFitMse_float(instance: *mut c_void, val: f32);
	// aprilTagMinWhiteBlackDiff /usr/include/opencv2/aruco.hpp:205
	pub fn cv_aruco_DetectorParameters_getPropAprilTagMinWhiteBlackDiff_const(instance: *const c_void) -> i32;
	// aprilTagMinWhiteBlackDiff /usr/include/opencv2/aruco.hpp:205
	pub fn cv_aruco_DetectorParameters_setPropAprilTagMinWhiteBlackDiff_int(instance: *mut c_void, val: i32);
	// aprilTagDeglitch /usr/include/opencv2/aruco.hpp:206
	pub fn cv_aruco_DetectorParameters_getPropAprilTagDeglitch_const(instance: *const c_void) -> i32;
	// aprilTagDeglitch /usr/include/opencv2/aruco.hpp:206
	pub fn cv_aruco_DetectorParameters_setPropAprilTagDeglitch_int(instance: *mut c_void, val: i32);
	// detectInvertedMarker /usr/include/opencv2/aruco.hpp:209
	pub fn cv_aruco_DetectorParameters_getPropDetectInvertedMarker_const(instance: *const c_void) -> bool;
	// detectInvertedMarker /usr/include/opencv2/aruco.hpp:209
	pub fn cv_aruco_DetectorParameters_setPropDetectInvertedMarker_bool(instance: *mut c_void, val: bool);
	// useAruco3Detection /usr/include/opencv2/aruco.hpp:213
	pub fn cv_aruco_DetectorParameters_getPropUseAruco3Detection_const(instance: *const c_void) -> bool;
	// useAruco3Detection /usr/include/opencv2/aruco.hpp:213
	pub fn cv_aruco_DetectorParameters_setPropUseAruco3Detection_bool(instance: *mut c_void, val: bool);
	// minSideLengthCanonicalImg /usr/include/opencv2/aruco.hpp:214
	pub fn cv_aruco_DetectorParameters_getPropMinSideLengthCanonicalImg_const(instance: *const c_void) -> i32;
	// minSideLengthCanonicalImg /usr/include/opencv2/aruco.hpp:214
	pub fn cv_aruco_DetectorParameters_setPropMinSideLengthCanonicalImg_int(instance: *mut c_void, val: i32);
	// minMarkerLengthRatioOriginalImg /usr/include/opencv2/aruco.hpp:215
	pub fn cv_aruco_DetectorParameters_getPropMinMarkerLengthRatioOriginalImg_const(instance: *const c_void) -> f32;
	// minMarkerLengthRatioOriginalImg /usr/include/opencv2/aruco.hpp:215
	pub fn cv_aruco_DetectorParameters_setPropMinMarkerLengthRatioOriginalImg_float(instance: *mut c_void, val: f32);
	// DetectorParameters() /usr/include/opencv2/aruco.hpp:171
	pub fn cv_aruco_DetectorParameters_DetectorParameters(ocvrs_return: *mut Result<*mut c_void>);
	// create() /usr/include/opencv2/aruco.hpp:172
	pub fn cv_aruco_DetectorParameters_create(ocvrs_return: *mut Result<*mut c_void>);
	// readDetectorParameters(const cv::FileNode &) /usr/include/opencv2/aruco.hpp:173
	pub fn cv_aruco_DetectorParameters_readDetectorParameters_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<bool>);
	// bytesList /usr/include/opencv2/aruco/dictionary.hpp:64
	pub fn cv_aruco_Dictionary_getPropBytesList_const(instance: *const c_void) -> *mut c_void;
	// bytesList /usr/include/opencv2/aruco/dictionary.hpp:64
	pub fn cv_aruco_Dictionary_setPropBytesList_Mat(instance: *mut c_void, val: *mut c_void);
	// markerSize /usr/include/opencv2/aruco/dictionary.hpp:65
	pub fn cv_aruco_Dictionary_getPropMarkerSize_const(instance: *const c_void) -> i32;
	// markerSize /usr/include/opencv2/aruco/dictionary.hpp:65
	pub fn cv_aruco_Dictionary_setPropMarkerSize_int(instance: *mut c_void, val: i32);
	// maxCorrectionBits /usr/include/opencv2/aruco/dictionary.hpp:66
	pub fn cv_aruco_Dictionary_getPropMaxCorrectionBits_const(instance: *const c_void) -> i32;
	// maxCorrectionBits /usr/include/opencv2/aruco/dictionary.hpp:66
	pub fn cv_aruco_Dictionary_setPropMaxCorrectionBits_int(instance: *mut c_void, val: i32);
	// Dictionary(const cv::Mat &, int, int) /usr/include/opencv2/aruco/dictionary.hpp:71
	pub fn cv_aruco_Dictionary_Dictionary_const_MatR_int_int(_bytes_list: *const c_void, _marker_size: i32, _maxcorr: i32, ocvrs_return: *mut Result<*mut c_void>);
	// Dictionary(const Ptr<cv::aruco::Dictionary> &) /usr/include/opencv2/aruco/dictionary.hpp:81
	pub fn cv_aruco_Dictionary_Dictionary_const_Ptr_Dictionary_R(_dictionary: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, int) /usr/include/opencv2/aruco/dictionary.hpp:87
	pub fn cv_aruco_Dictionary_create_int_int_int(n_markers: i32, marker_size: i32, random_seed: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, const Ptr<cv::aruco::Dictionary> &, int) /usr/include/opencv2/aruco/dictionary.hpp:93
	pub fn cv_aruco_Dictionary_create_int_int_const_Ptr_Dictionary_R_int(n_markers: i32, marker_size: i32, base_dictionary: *const c_void, random_seed: i32, ocvrs_return: *mut Result<*mut c_void>);
	// readDictionary(const cv::FileNode &) /usr/include/opencv2/aruco/dictionary.hpp:105
	pub fn cv_aruco_Dictionary_readDictionary_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<bool>);
	// writeDictionary(Ptr<cv::FileStorage> &) /usr/include/opencv2/aruco/dictionary.hpp:110
	pub fn cv_aruco_Dictionary_writeDictionary_Ptr_FileStorage_R(instance: *mut c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// get(int) /usr/include/opencv2/aruco/dictionary.hpp:114
	pub fn cv_aruco_Dictionary_get_int(dict: i32, ocvrs_return: *mut Result<*mut c_void>);
	// identify(const cv::Mat &, int &, int &, double) /usr/include/opencv2/aruco/dictionary.hpp:120
	pub fn cv_aruco_Dictionary_identify_const_const_MatR_intR_intR_double(instance: *const c_void, only_bits: *const c_void, idx: *mut i32, rotation: *mut i32, max_correction_rate: f64, ocvrs_return: *mut Result<bool>);
	// getDistanceToId(cv::InputArray, int, bool) /usr/include/opencv2/aruco/dictionary.hpp:126
	pub fn cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int_bool(instance: *const c_void, bits: *const c_void, id: i32, all_rotations: bool, ocvrs_return: *mut Result<i32>);
	// drawMarker(int, int, cv::OutputArray, int) /usr/include/opencv2/aruco/dictionary.hpp:132
	pub fn cv_aruco_Dictionary_drawMarker_const_int_int_const__OutputArrayR_int(instance: *const c_void, id: i32, side_pixels: i32, _img: *const c_void, border_bits: i32, ocvrs_return: *mut Result_void);
	// getByteListFromBits(const cv::Mat &) /usr/include/opencv2/aruco/dictionary.hpp:138
	pub fn cv_aruco_Dictionary_getByteListFromBits_const_MatR(bits: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getBitsFromByteList(const cv::Mat &, int) /usr/include/opencv2/aruco/dictionary.hpp:144
	pub fn cv_aruco_Dictionary_getBitsFromByteList_const_MatR_int(byte_list: *const c_void, marker_size: i32, ocvrs_return: *mut Result<*mut c_void>);
	// pattern /usr/include/opencv2/aruco.hpp:285
	pub fn cv_aruco_EstimateParameters_getPropPattern_const(instance: *const c_void, ocvrs_return: *mut crate::aruco::PatternPos);
	// pattern /usr/include/opencv2/aruco.hpp:285
	pub fn cv_aruco_EstimateParameters_setPropPattern_PatternPos(instance: *mut c_void, val: crate::aruco::PatternPos);
	// useExtrinsicGuess /usr/include/opencv2/aruco.hpp:286
	pub fn cv_aruco_EstimateParameters_getPropUseExtrinsicGuess_const(instance: *const c_void) -> bool;
	// useExtrinsicGuess /usr/include/opencv2/aruco.hpp:286
	pub fn cv_aruco_EstimateParameters_setPropUseExtrinsicGuess_bool(instance: *mut c_void, val: bool);
	// solvePnPMethod /usr/include/opencv2/aruco.hpp:287
	pub fn cv_aruco_EstimateParameters_getPropSolvePnPMethod_const(instance: *const c_void, ocvrs_return: *mut crate::calib3d::SolvePnPMethod);
	// solvePnPMethod /usr/include/opencv2/aruco.hpp:287
	pub fn cv_aruco_EstimateParameters_setPropSolvePnPMethod_SolvePnPMethod(instance: *mut c_void, val: crate::calib3d::SolvePnPMethod);
	// EstimateParameters() /usr/include/opencv2/aruco.hpp:289
	pub fn cv_aruco_EstimateParameters_EstimateParameters(ocvrs_return: *mut Result<*mut c_void>);
	// create() /usr/include/opencv2/aruco.hpp:292
	pub fn cv_aruco_EstimateParameters_create(ocvrs_return: *mut Result<*mut c_void>);
	// draw(cv::Size, cv::OutputArray, int, int) /usr/include/opencv2/aruco.hpp:418
	pub fn cv_aruco_GridBoard_draw_Size_const__OutputArrayR_int_int(instance: *mut c_void, out_size: *const core::Size, img: *const c_void, margin_size: i32, border_bits: i32, ocvrs_return: *mut Result_void);
	// create(int, int, float, float, const Ptr<cv::aruco::Dictionary> &, int) /usr/include/opencv2/aruco.hpp:435
	pub fn cv_aruco_GridBoard_create_int_int_float_float_const_Ptr_Dictionary_R_int(markers_x: i32, markers_y: i32, marker_length: f32, marker_separation: f32, dictionary: *const c_void, first_marker: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getGridSize() /usr/include/opencv2/aruco.hpp:441
	pub fn cv_aruco_GridBoard_getGridSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// getMarkerLength() /usr/include/opencv2/aruco.hpp:446
	pub fn cv_aruco_GridBoard_getMarkerLength_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// getMarkerSeparation() /usr/include/opencv2/aruco.hpp:451
	pub fn cv_aruco_GridBoard_getMarkerSeparation_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
}
