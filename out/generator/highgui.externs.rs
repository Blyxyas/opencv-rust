extern "C" {
	// addText(const cv::Mat &, const cv::String &, cv::Point, const cv::QtFont &) /usr/include/opencv2/highgui.hpp:763
	pub fn cv_addText_const_MatR_const_StringR_Point_const_QtFontR(img: *const c_void, text: *const c_char, org: *const core::Point, font: *const c_void, ocvrs_return: *mut Result_void);
	// addText(const cv::Mat &, const cv::String &, cv::Point, const cv::String &, int, cv::Scalar, int, int, int) /usr/include/opencv2/highgui.hpp:779
	pub fn cv_addText_const_MatR_const_StringR_Point_const_StringR_int_Scalar_int_int_int(img: *const c_void, text: *const c_char, org: *const core::Point, name_font: *const c_char, point_size: i32, color: *const core::Scalar, weight: i32, style: i32, spacing: i32, ocvrs_return: *mut Result_void);
	// createButton(const cv::String &, cv::ButtonCallback, void *, int, bool) /usr/include/opencv2/highgui.hpp:858
	pub fn cv_createButton_const_StringR_ButtonCallback_voidX_int_bool(bar_name: *const c_char, on_change: Option<unsafe extern "C" fn(i32, *mut c_void) -> ()>, userdata: *mut c_void, typ: i32, initial_button_state: bool, ocvrs_return: *mut Result<i32>);
	// createTrackbar(const cv::String &, const cv::String &, int *, int, cv::TrackbarCallback, void *) /usr/include/opencv2/highgui.hpp:579
	pub fn cv_createTrackbar_const_StringR_const_StringR_intX_int_TrackbarCallback_voidX(trackbarname: *const c_char, winname: *const c_char, value: *mut i32, count: i32, on_change: Option<unsafe extern "C" fn(i32, *mut c_void) -> ()>, userdata: *mut c_void, ocvrs_return: *mut Result<i32>);
	// destroyAllWindows() /usr/include/opencv2/highgui.hpp:332
	pub fn cv_destroyAllWindows(ocvrs_return: *mut Result_void);
	// destroyWindow(const cv::String &) /usr/include/opencv2/highgui.hpp:326
	pub fn cv_destroyWindow_const_StringR(winname: *const c_char, ocvrs_return: *mut Result_void);
	// displayOverlay(const cv::String &, const cv::String &, int) /usr/include/opencv2/highgui.hpp:794
	pub fn cv_displayOverlay_const_StringR_const_StringR_int(winname: *const c_char, text: *const c_char, delayms: i32, ocvrs_return: *mut Result_void);
	// displayStatusBar(const cv::String &, const cv::String &, int) /usr/include/opencv2/highgui.hpp:808
	pub fn cv_displayStatusBar_const_StringR_const_StringR_int(winname: *const c_char, text: *const c_char, delayms: i32, ocvrs_return: *mut Result_void);
	// fontQt(const cv::String &, int, cv::Scalar, int, int, int) /usr/include/opencv2/highgui.hpp:749
	pub fn cv_fontQt_const_StringR_int_Scalar_int_int_int(name_font: *const c_char, point_size: i32, color: *const core::Scalar, weight: i32, style: i32, spacing: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getMouseWheelDelta(int) /usr/include/opencv2/highgui.hpp:511
	pub fn cv_getMouseWheelDelta_int(flags: i32, ocvrs_return: *mut Result<i32>);
	// getTrackbarPos(const cv::String &, const cv::String &) /usr/include/opencv2/highgui.hpp:596
	pub fn cv_getTrackbarPos_const_StringR_const_StringR(trackbarname: *const c_char, winname: *const c_char, ocvrs_return: *mut Result<i32>);
	// getWindowImageRect(const cv::String &) /usr/include/opencv2/highgui.hpp:480
	pub fn cv_getWindowImageRect_const_StringR(winname: *const c_char, ocvrs_return: *mut Result<core::Rect>);
	// getWindowProperty(const cv::String &, int) /usr/include/opencv2/highgui.hpp:470
	pub fn cv_getWindowProperty_const_StringR_int(winname: *const c_char, prop_id: i32, ocvrs_return: *mut Result<f64>);
	// imshow(const cv::String &, cv::InputArray) /usr/include/opencv2/highgui.hpp:416
	pub fn cv_imshow_const_StringR_const__InputArrayR(winname: *const c_char, mat: *const c_void, ocvrs_return: *mut Result_void);
	// loadWindowParameters(const cv::String &) /usr/include/opencv2/highgui.hpp:826
	pub fn cv_loadWindowParameters_const_StringR(window_name: *const c_char, ocvrs_return: *mut Result_void);
	// moveWindow(const cv::String &, int, int) /usr/include/opencv2/highgui.hpp:443
	pub fn cv_moveWindow_const_StringR_int_int(winname: *const c_char, x: i32, y: i32, ocvrs_return: *mut Result_void);
	// namedWindow(const cv::String &, int) /usr/include/opencv2/highgui.hpp:318
	pub fn cv_namedWindow_const_StringR_int(winname: *const c_char, flags: i32, ocvrs_return: *mut Result_void);
	// pollKey() /usr/include/opencv2/highgui.hpp:377
	pub fn cv_pollKey(ocvrs_return: *mut Result<i32>);
	// resizeWindow(const cv::String &, const cv::Size &) /usr/include/opencv2/highgui.hpp:435
	pub fn cv_resizeWindow_const_StringR_const_SizeR(winname: *const c_char, size: *const core::Size, ocvrs_return: *mut Result_void);
	// resizeWindow(const cv::String &, int, int) /usr/include/opencv2/highgui.hpp:429
	pub fn cv_resizeWindow_const_StringR_int_int(winname: *const c_char, width: i32, height: i32, ocvrs_return: *mut Result_void);
	// saveWindowParameters(const cv::String &) /usr/include/opencv2/highgui.hpp:817
	pub fn cv_saveWindowParameters_const_StringR(window_name: *const c_char, ocvrs_return: *mut Result_void);
	// selectROI(const cv::String &, cv::InputArray, bool, bool) /usr/include/opencv2/highgui.hpp:528
	pub fn cv_selectROI_const_StringR_const__InputArrayR_bool_bool(window_name: *const c_char, img: *const c_void, show_crosshair: bool, from_center: bool, ocvrs_return: *mut Result<core::Rect>);
	// selectROI(cv::InputArray, bool, bool) /usr/include/opencv2/highgui.hpp:532
	pub fn cv_selectROI_const__InputArrayR_bool_bool(img: *const c_void, show_crosshair: bool, from_center: bool, ocvrs_return: *mut Result<core::Rect>);
	// selectROIs(const cv::String &, cv::InputArray, std::vector<Rect> &, bool, bool) /usr/include/opencv2/highgui.hpp:550
	pub fn cv_selectROIs_const_StringR_const__InputArrayR_vector_Rect_R_bool_bool(window_name: *const c_char, img: *const c_void, bounding_boxes: *mut c_void, show_crosshair: bool, from_center: bool, ocvrs_return: *mut Result_void);
	// setMouseCallback(const cv::String &, cv::MouseCallback, void *) /usr/include/opencv2/highgui.hpp:491
	pub fn cv_setMouseCallback_const_StringR_MouseCallback_voidX(winname: *const c_char, on_mouse: Option<unsafe extern "C" fn(i32, i32, i32, i32, *mut c_void) -> ()>, userdata: *mut c_void, ocvrs_return: *mut Result_void);
	// setOpenGlContext(const cv::String &) /usr/include/opencv2/highgui.hpp:699
	pub fn cv_setOpenGlContext_const_StringR(winname: *const c_char, ocvrs_return: *mut Result_void);
	// setOpenGlDrawCallback(const cv::String &, cv::OpenGlDrawCallback, void *) /usr/include/opencv2/highgui.hpp:693
	pub fn cv_setOpenGlDrawCallback_const_StringR_OpenGlDrawCallback_voidX(winname: *const c_char, on_opengl_draw: Option<unsafe extern "C" fn(*mut c_void) -> ()>, userdata: *mut c_void, ocvrs_return: *mut Result_void);
	// setTrackbarMax(const cv::String &, const cv::String &, int) /usr/include/opencv2/highgui.hpp:626
	pub fn cv_setTrackbarMax_const_StringR_const_StringR_int(trackbarname: *const c_char, winname: *const c_char, maxval: i32, ocvrs_return: *mut Result_void);
	// setTrackbarMin(const cv::String &, const cv::String &, int) /usr/include/opencv2/highgui.hpp:641
	pub fn cv_setTrackbarMin_const_StringR_const_StringR_int(trackbarname: *const c_char, winname: *const c_char, minval: i32, ocvrs_return: *mut Result_void);
	// setTrackbarPos(const cv::String &, const cv::String &, int) /usr/include/opencv2/highgui.hpp:611
	pub fn cv_setTrackbarPos_const_StringR_const_StringR_int(trackbarname: *const c_char, winname: *const c_char, pos: i32, ocvrs_return: *mut Result_void);
	// setWindowProperty(const cv::String &, int, double) /usr/include/opencv2/highgui.hpp:453
	pub fn cv_setWindowProperty_const_StringR_int_double(winname: *const c_char, prop_id: i32, prop_value: f64, ocvrs_return: *mut Result_void);
	// setWindowTitle(const cv::String &, const cv::String &) /usr/include/opencv2/highgui.hpp:459
	pub fn cv_setWindowTitle_const_StringR_const_StringR(winname: *const c_char, title: *const c_char, ocvrs_return: *mut Result_void);
	// startLoop(int (*)(int, char **), int, char **) /usr/include/opencv2/highgui.hpp:828
	pub fn cv_startLoop_int__X__int__charXX__int_charXX(pt2_func: Option<unsafe extern "C" fn(i32, *mut *mut c_char) -> i32>, argc: i32, argv: *mut *mut c_char, ocvrs_return: *mut Result<i32>);
	// startWindowThread() /usr/include/opencv2/highgui.hpp:334
	pub fn cv_startWindowThread(ocvrs_return: *mut Result<i32>);
	// stopLoop() /usr/include/opencv2/highgui.hpp:830
	pub fn cv_stopLoop(ocvrs_return: *mut Result_void);
	// updateWindow(const cv::String &) /usr/include/opencv2/highgui.hpp:705
	pub fn cv_updateWindow_const_StringR(winname: *const c_char, ocvrs_return: *mut Result_void);
	// waitKeyEx(int) /usr/include/opencv2/highgui.hpp:343
	pub fn cv_waitKeyEx_int(delay: i32, ocvrs_return: *mut Result<i32>);
	// waitKey(int) /usr/include/opencv2/highgui.hpp:363
	pub fn cv_waitKey_int(delay: i32, ocvrs_return: *mut Result<i32>);
	// nameFont /usr/include/opencv2/highgui.hpp:716
	pub fn cv_QtFont_getPropNameFont_const(instance: *const c_void) -> *mut c_void;
	// color /usr/include/opencv2/highgui.hpp:717
	pub fn cv_QtFont_getPropColor_const(instance: *const c_void, ocvrs_return: *mut core::Scalar);
	// color /usr/include/opencv2/highgui.hpp:717
	pub fn cv_QtFont_setPropColor_Scalar(instance: *mut c_void, val: *const core::Scalar);
	// font_face /usr/include/opencv2/highgui.hpp:718
	pub fn cv_QtFont_getPropFont_face_const(instance: *const c_void) -> i32;
	// font_face /usr/include/opencv2/highgui.hpp:718
	pub fn cv_QtFont_setPropFont_face_int(instance: *mut c_void, val: i32);
	// ascii /usr/include/opencv2/highgui.hpp:719
	pub fn cv_QtFont_getPropAscii_const(instance: *const c_void) -> *const i32;
	// greek /usr/include/opencv2/highgui.hpp:720
	pub fn cv_QtFont_getPropGreek_const(instance: *const c_void) -> *const i32;
	// cyrillic /usr/include/opencv2/highgui.hpp:721
	pub fn cv_QtFont_getPropCyrillic_const(instance: *const c_void) -> *const i32;
	// hscale /usr/include/opencv2/highgui.hpp:722
	pub fn cv_QtFont_getPropHscale_const(instance: *const c_void) -> f32;
	// hscale /usr/include/opencv2/highgui.hpp:722
	pub fn cv_QtFont_setPropHscale_float(instance: *mut c_void, val: f32);
	// vscale /usr/include/opencv2/highgui.hpp:722
	pub fn cv_QtFont_getPropVscale_const(instance: *const c_void) -> f32;
	// vscale /usr/include/opencv2/highgui.hpp:722
	pub fn cv_QtFont_setPropVscale_float(instance: *mut c_void, val: f32);
	// shear /usr/include/opencv2/highgui.hpp:723
	pub fn cv_QtFont_getPropShear_const(instance: *const c_void) -> f32;
	// shear /usr/include/opencv2/highgui.hpp:723
	pub fn cv_QtFont_setPropShear_float(instance: *mut c_void, val: f32);
	// thickness /usr/include/opencv2/highgui.hpp:724
	pub fn cv_QtFont_getPropThickness_const(instance: *const c_void) -> i32;
	// thickness /usr/include/opencv2/highgui.hpp:724
	pub fn cv_QtFont_setPropThickness_int(instance: *mut c_void, val: i32);
	// dx /usr/include/opencv2/highgui.hpp:725
	pub fn cv_QtFont_getPropDx_const(instance: *const c_void) -> f32;
	// dx /usr/include/opencv2/highgui.hpp:725
	pub fn cv_QtFont_setPropDx_float(instance: *mut c_void, val: f32);
	// line_type /usr/include/opencv2/highgui.hpp:726
	pub fn cv_QtFont_getPropLine_type_const(instance: *const c_void) -> i32;
	// line_type /usr/include/opencv2/highgui.hpp:726
	pub fn cv_QtFont_setPropLine_type_int(instance: *mut c_void, val: i32);
}
