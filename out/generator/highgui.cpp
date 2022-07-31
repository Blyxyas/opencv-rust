#include "ocvrs_common.hpp"
#include <opencv2/highgui.hpp>
#include "highgui_types.hpp"

extern "C" {
	// addText(const cv::Mat &, const cv::String &, cv::Point, const cv::QtFont &) /usr/include/opencv2/highgui.hpp:763
	void cv_addText_const_MatR_const_StringR_Point_const_QtFontR(const cv::Mat* img, const char* text, cv::Point* org, const cv::QtFont* font, Result_void* ocvrs_return) {
		try {
			cv::addText(*img, std::string(text), *org, *font);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// addText(const cv::Mat &, const cv::String &, cv::Point, const cv::String &, int, cv::Scalar, int, int, int) /usr/include/opencv2/highgui.hpp:779
	void cv_addText_const_MatR_const_StringR_Point_const_StringR_int_Scalar_int_int_int(const cv::Mat* img, const char* text, cv::Point* org, const char* nameFont, int pointSize, cv::Scalar* color, int weight, int style, int spacing, Result_void* ocvrs_return) {
		try {
			cv::addText(*img, std::string(text), *org, std::string(nameFont), pointSize, *color, weight, style, spacing);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// createButton(const cv::String &, cv::ButtonCallback, void *, int, bool) /usr/include/opencv2/highgui.hpp:858
	void cv_createButton_const_StringR_ButtonCallback_voidX_int_bool(const char* bar_name, cv::ButtonCallback on_change, void* userdata, int type, bool initial_button_state, Result<int>* ocvrs_return) {
		try {
			int ret = cv::createButton(std::string(bar_name), on_change, userdata, type, initial_button_state);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// createTrackbar(const cv::String &, const cv::String &, int *, int, cv::TrackbarCallback, void *) /usr/include/opencv2/highgui.hpp:579
	void cv_createTrackbar_const_StringR_const_StringR_intX_int_TrackbarCallback_voidX(const char* trackbarname, const char* winname, int* value, int count, cv::TrackbarCallback onChange, void* userdata, Result<int>* ocvrs_return) {
		try {
			int ret = cv::createTrackbar(std::string(trackbarname), std::string(winname), value, count, onChange, userdata);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// destroyAllWindows() /usr/include/opencv2/highgui.hpp:332
	void cv_destroyAllWindows(Result_void* ocvrs_return) {
		try {
			cv::destroyAllWindows();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// destroyWindow(const cv::String &) /usr/include/opencv2/highgui.hpp:326
	void cv_destroyWindow_const_StringR(const char* winname, Result_void* ocvrs_return) {
		try {
			cv::destroyWindow(std::string(winname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// displayOverlay(const cv::String &, const cv::String &, int) /usr/include/opencv2/highgui.hpp:794
	void cv_displayOverlay_const_StringR_const_StringR_int(const char* winname, const char* text, int delayms, Result_void* ocvrs_return) {
		try {
			cv::displayOverlay(std::string(winname), std::string(text), delayms);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// displayStatusBar(const cv::String &, const cv::String &, int) /usr/include/opencv2/highgui.hpp:808
	void cv_displayStatusBar_const_StringR_const_StringR_int(const char* winname, const char* text, int delayms, Result_void* ocvrs_return) {
		try {
			cv::displayStatusBar(std::string(winname), std::string(text), delayms);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// fontQt(const cv::String &, int, cv::Scalar, int, int, int) /usr/include/opencv2/highgui.hpp:749
	void cv_fontQt_const_StringR_int_Scalar_int_int_int(const char* nameFont, int pointSize, cv::Scalar* color, int weight, int style, int spacing, Result<cv::QtFont*>* ocvrs_return) {
		try {
			cv::QtFont ret = cv::fontQt(std::string(nameFont), pointSize, *color, weight, style, spacing);
			Ok(new cv::QtFont(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::QtFont*>))
	}
	
	// getMouseWheelDelta(int) /usr/include/opencv2/highgui.hpp:511
	void cv_getMouseWheelDelta_int(int flags, Result<int>* ocvrs_return) {
		try {
			int ret = cv::getMouseWheelDelta(flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getTrackbarPos(const cv::String &, const cv::String &) /usr/include/opencv2/highgui.hpp:596
	void cv_getTrackbarPos_const_StringR_const_StringR(const char* trackbarname, const char* winname, Result<int>* ocvrs_return) {
		try {
			int ret = cv::getTrackbarPos(std::string(trackbarname), std::string(winname));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getWindowImageRect(const cv::String &) /usr/include/opencv2/highgui.hpp:480
	void cv_getWindowImageRect_const_StringR(const char* winname, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::getWindowImageRect(std::string(winname));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// getWindowProperty(const cv::String &, int) /usr/include/opencv2/highgui.hpp:470
	void cv_getWindowProperty_const_StringR_int(const char* winname, int prop_id, Result<double>* ocvrs_return) {
		try {
			double ret = cv::getWindowProperty(std::string(winname), prop_id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// imshow(const cv::String &, cv::InputArray) /usr/include/opencv2/highgui.hpp:416
	void cv_imshow_const_StringR_const__InputArrayR(const char* winname, const cv::_InputArray* mat, Result_void* ocvrs_return) {
		try {
			cv::imshow(std::string(winname), *mat);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// loadWindowParameters(const cv::String &) /usr/include/opencv2/highgui.hpp:826
	void cv_loadWindowParameters_const_StringR(const char* windowName, Result_void* ocvrs_return) {
		try {
			cv::loadWindowParameters(std::string(windowName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// moveWindow(const cv::String &, int, int) /usr/include/opencv2/highgui.hpp:443
	void cv_moveWindow_const_StringR_int_int(const char* winname, int x, int y, Result_void* ocvrs_return) {
		try {
			cv::moveWindow(std::string(winname), x, y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// namedWindow(const cv::String &, int) /usr/include/opencv2/highgui.hpp:318
	void cv_namedWindow_const_StringR_int(const char* winname, int flags, Result_void* ocvrs_return) {
		try {
			cv::namedWindow(std::string(winname), flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// pollKey() /usr/include/opencv2/highgui.hpp:377
	void cv_pollKey(Result<int>* ocvrs_return) {
		try {
			int ret = cv::pollKey();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// resizeWindow(const cv::String &, const cv::Size &) /usr/include/opencv2/highgui.hpp:435
	void cv_resizeWindow_const_StringR_const_SizeR(const char* winname, const cv::Size* size, Result_void* ocvrs_return) {
		try {
			cv::resizeWindow(std::string(winname), *size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// resizeWindow(const cv::String &, int, int) /usr/include/opencv2/highgui.hpp:429
	void cv_resizeWindow_const_StringR_int_int(const char* winname, int width, int height, Result_void* ocvrs_return) {
		try {
			cv::resizeWindow(std::string(winname), width, height);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// saveWindowParameters(const cv::String &) /usr/include/opencv2/highgui.hpp:817
	void cv_saveWindowParameters_const_StringR(const char* windowName, Result_void* ocvrs_return) {
		try {
			cv::saveWindowParameters(std::string(windowName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// selectROI(const cv::String &, cv::InputArray, bool, bool) /usr/include/opencv2/highgui.hpp:528
	void cv_selectROI_const_StringR_const__InputArrayR_bool_bool(const char* windowName, const cv::_InputArray* img, bool showCrosshair, bool fromCenter, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::selectROI(std::string(windowName), *img, showCrosshair, fromCenter);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// selectROI(cv::InputArray, bool, bool) /usr/include/opencv2/highgui.hpp:532
	void cv_selectROI_const__InputArrayR_bool_bool(const cv::_InputArray* img, bool showCrosshair, bool fromCenter, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::selectROI(*img, showCrosshair, fromCenter);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// selectROIs(const cv::String &, cv::InputArray, std::vector<Rect> &, bool, bool) /usr/include/opencv2/highgui.hpp:550
	void cv_selectROIs_const_StringR_const__InputArrayR_vector_Rect_R_bool_bool(const char* windowName, const cv::_InputArray* img, std::vector<cv::Rect>* boundingBoxes, bool showCrosshair, bool fromCenter, Result_void* ocvrs_return) {
		try {
			cv::selectROIs(std::string(windowName), *img, *boundingBoxes, showCrosshair, fromCenter);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setMouseCallback(const cv::String &, cv::MouseCallback, void *) /usr/include/opencv2/highgui.hpp:491
	void cv_setMouseCallback_const_StringR_MouseCallback_voidX(const char* winname, cv::MouseCallback onMouse, void* userdata, Result_void* ocvrs_return) {
		try {
			cv::setMouseCallback(std::string(winname), onMouse, userdata);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setOpenGlContext(const cv::String &) /usr/include/opencv2/highgui.hpp:699
	void cv_setOpenGlContext_const_StringR(const char* winname, Result_void* ocvrs_return) {
		try {
			cv::setOpenGlContext(std::string(winname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setOpenGlDrawCallback(const cv::String &, cv::OpenGlDrawCallback, void *) /usr/include/opencv2/highgui.hpp:693
	void cv_setOpenGlDrawCallback_const_StringR_OpenGlDrawCallback_voidX(const char* winname, cv::OpenGlDrawCallback onOpenGlDraw, void* userdata, Result_void* ocvrs_return) {
		try {
			cv::setOpenGlDrawCallback(std::string(winname), onOpenGlDraw, userdata);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setTrackbarMax(const cv::String &, const cv::String &, int) /usr/include/opencv2/highgui.hpp:626
	void cv_setTrackbarMax_const_StringR_const_StringR_int(const char* trackbarname, const char* winname, int maxval, Result_void* ocvrs_return) {
		try {
			cv::setTrackbarMax(std::string(trackbarname), std::string(winname), maxval);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setTrackbarMin(const cv::String &, const cv::String &, int) /usr/include/opencv2/highgui.hpp:641
	void cv_setTrackbarMin_const_StringR_const_StringR_int(const char* trackbarname, const char* winname, int minval, Result_void* ocvrs_return) {
		try {
			cv::setTrackbarMin(std::string(trackbarname), std::string(winname), minval);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setTrackbarPos(const cv::String &, const cv::String &, int) /usr/include/opencv2/highgui.hpp:611
	void cv_setTrackbarPos_const_StringR_const_StringR_int(const char* trackbarname, const char* winname, int pos, Result_void* ocvrs_return) {
		try {
			cv::setTrackbarPos(std::string(trackbarname), std::string(winname), pos);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setWindowProperty(const cv::String &, int, double) /usr/include/opencv2/highgui.hpp:453
	void cv_setWindowProperty_const_StringR_int_double(const char* winname, int prop_id, double prop_value, Result_void* ocvrs_return) {
		try {
			cv::setWindowProperty(std::string(winname), prop_id, prop_value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setWindowTitle(const cv::String &, const cv::String &) /usr/include/opencv2/highgui.hpp:459
	void cv_setWindowTitle_const_StringR_const_StringR(const char* winname, const char* title, Result_void* ocvrs_return) {
		try {
			cv::setWindowTitle(std::string(winname), std::string(title));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// startLoop(int (*)(int, char **), int, char **) /usr/include/opencv2/highgui.hpp:828
	void cv_startLoop_int__X__int__charXX__int_charXX(int (*pt2Func)(int, char**), int argc, char** argv, Result<int>* ocvrs_return) {
		try {
			int ret = cv::startLoop(pt2Func, argc, argv);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// startWindowThread() /usr/include/opencv2/highgui.hpp:334
	void cv_startWindowThread(Result<int>* ocvrs_return) {
		try {
			int ret = cv::startWindowThread();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// stopLoop() /usr/include/opencv2/highgui.hpp:830
	void cv_stopLoop(Result_void* ocvrs_return) {
		try {
			cv::stopLoop();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// updateWindow(const cv::String &) /usr/include/opencv2/highgui.hpp:705
	void cv_updateWindow_const_StringR(const char* winname, Result_void* ocvrs_return) {
		try {
			cv::updateWindow(std::string(winname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// waitKeyEx(int) /usr/include/opencv2/highgui.hpp:343
	void cv_waitKeyEx_int(int delay, Result<int>* ocvrs_return) {
		try {
			int ret = cv::waitKeyEx(delay);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// waitKey(int) /usr/include/opencv2/highgui.hpp:363
	void cv_waitKey_int(int delay, Result<int>* ocvrs_return) {
		try {
			int ret = cv::waitKey(delay);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// nameFont /usr/include/opencv2/highgui.hpp:716
	void* cv_QtFont_getPropNameFont_const(const cv::QtFont* instance) {
			const char* ret = instance->nameFont;
			return ocvrs_create_string(ret);
	}
	
	// color /usr/include/opencv2/highgui.hpp:717
	void cv_QtFont_getPropColor_const(const cv::QtFont* instance, cv::Scalar* ocvrs_return) {
			cv::Scalar ret = instance->color;
			*ocvrs_return = ret;
	}
	
	// color /usr/include/opencv2/highgui.hpp:717
	void cv_QtFont_setPropColor_Scalar(cv::QtFont* instance, cv::Scalar* val) {
			instance->color = *val;
	}
	
	// font_face /usr/include/opencv2/highgui.hpp:718
	int cv_QtFont_getPropFont_face_const(const cv::QtFont* instance) {
			int ret = instance->font_face;
			return ret;
	}
	
	// font_face /usr/include/opencv2/highgui.hpp:718
	void cv_QtFont_setPropFont_face_int(cv::QtFont* instance, int val) {
			instance->font_face = val;
	}
	
	// ascii /usr/include/opencv2/highgui.hpp:719
	const int* cv_QtFont_getPropAscii_const(const cv::QtFont* instance) {
			const int* ret = instance->ascii;
			return ret;
	}
	
	// greek /usr/include/opencv2/highgui.hpp:720
	const int* cv_QtFont_getPropGreek_const(const cv::QtFont* instance) {
			const int* ret = instance->greek;
			return ret;
	}
	
	// cyrillic /usr/include/opencv2/highgui.hpp:721
	const int* cv_QtFont_getPropCyrillic_const(const cv::QtFont* instance) {
			const int* ret = instance->cyrillic;
			return ret;
	}
	
	// hscale /usr/include/opencv2/highgui.hpp:722
	float cv_QtFont_getPropHscale_const(const cv::QtFont* instance) {
			float ret = instance->hscale;
			return ret;
	}
	
	// hscale /usr/include/opencv2/highgui.hpp:722
	void cv_QtFont_setPropHscale_float(cv::QtFont* instance, float val) {
			instance->hscale = val;
	}
	
	// vscale /usr/include/opencv2/highgui.hpp:722
	float cv_QtFont_getPropVscale_const(const cv::QtFont* instance) {
			float ret = instance->vscale;
			return ret;
	}
	
	// vscale /usr/include/opencv2/highgui.hpp:722
	void cv_QtFont_setPropVscale_float(cv::QtFont* instance, float val) {
			instance->vscale = val;
	}
	
	// shear /usr/include/opencv2/highgui.hpp:723
	float cv_QtFont_getPropShear_const(const cv::QtFont* instance) {
			float ret = instance->shear;
			return ret;
	}
	
	// shear /usr/include/opencv2/highgui.hpp:723
	void cv_QtFont_setPropShear_float(cv::QtFont* instance, float val) {
			instance->shear = val;
	}
	
	// thickness /usr/include/opencv2/highgui.hpp:724
	int cv_QtFont_getPropThickness_const(const cv::QtFont* instance) {
			int ret = instance->thickness;
			return ret;
	}
	
	// thickness /usr/include/opencv2/highgui.hpp:724
	void cv_QtFont_setPropThickness_int(cv::QtFont* instance, int val) {
			instance->thickness = val;
	}
	
	// dx /usr/include/opencv2/highgui.hpp:725
	float cv_QtFont_getPropDx_const(const cv::QtFont* instance) {
			float ret = instance->dx;
			return ret;
	}
	
	// dx /usr/include/opencv2/highgui.hpp:725
	void cv_QtFont_setPropDx_float(cv::QtFont* instance, float val) {
			instance->dx = val;
	}
	
	// line_type /usr/include/opencv2/highgui.hpp:726
	int cv_QtFont_getPropLine_type_const(const cv::QtFont* instance) {
			int ret = instance->line_type;
			return ret;
	}
	
	// line_type /usr/include/opencv2/highgui.hpp:726
	void cv_QtFont_setPropLine_type_int(cv::QtFont* instance, int val) {
			instance->line_type = val;
	}
	
	void cv_QtFont_delete(cv::QtFont* instance) {
		delete instance;
	}
}
