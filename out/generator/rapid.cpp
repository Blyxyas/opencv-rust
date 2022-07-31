#include "ocvrs_common.hpp"
#include <opencv2/rapid.hpp>
#include "rapid_types.hpp"

extern "C" {
	// convertCorrespondencies(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/rapid.hpp:100
	void cv_rapid_convertCorrespondencies_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputArrayR(const cv::_InputArray* cols, const cv::_InputArray* srcLocations, const cv::_OutputArray* pts2d, const cv::_InputOutputArray* pts3d, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::rapid::convertCorrespondencies(*cols, *srcLocations, *pts2d, *pts3d, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// drawCorrespondencies(cv::InputOutputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/rapid.hpp:30
	void cv_rapid_drawCorrespondencies_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputOutputArray* bundle, const cv::_InputArray* cols, const cv::_InputArray* colors, Result_void* ocvrs_return) {
		try {
			cv::rapid::drawCorrespondencies(*bundle, *cols, *colors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// drawSearchLines(cv::InputOutputArray, cv::InputArray, const cv::Scalar &) /usr/include/opencv2/rapid.hpp:39
	void cv_rapid_drawSearchLines_const__InputOutputArrayR_const__InputArrayR_const_ScalarR(const cv::_InputOutputArray* img, const cv::_InputArray* locations, const cv::Scalar* color, Result_void* ocvrs_return) {
		try {
			cv::rapid::drawSearchLines(*img, *locations, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// drawWireframe(cv::InputOutputArray, cv::InputArray, cv::InputArray, const cv::Scalar &, int, bool) /usr/include/opencv2/rapid.hpp:50
	void cv_rapid_drawWireframe_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const_ScalarR_int_bool(const cv::_InputOutputArray* img, const cv::_InputArray* pts2d, const cv::_InputArray* tris, const cv::Scalar* color, int type, bool cullBackface, Result_void* ocvrs_return) {
		try {
			cv::rapid::drawWireframe(*img, *pts2d, *tris, *color, type, cullBackface);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// extractControlPoints(int, int, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, const cv::Size &, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rapid.hpp:67
	void cv_rapid_extractControlPoints_int_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(int num, int len, const cv::_InputArray* pts3d, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* K, const cv::Size* imsize, const cv::_InputArray* tris, const cv::_OutputArray* ctl2d, const cv::_OutputArray* ctl3d, Result_void* ocvrs_return) {
		try {
			cv::rapid::extractControlPoints(num, len, *pts3d, *rvec, *tvec, *K, *imsize, *tris, *ctl2d, *ctl3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// extractLineBundle(int, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rapid.hpp:79
	void cv_rapid_extractLineBundle_int_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(int len, const cv::_InputArray* ctl2d, const cv::_InputArray* img, const cv::_OutputArray* bundle, const cv::_OutputArray* srcLocations, Result_void* ocvrs_return) {
		try {
			cv::rapid::extractLineBundle(len, *ctl2d, *img, *bundle, *srcLocations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// findCorrespondencies(cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rapid.hpp:89
	void cv_rapid_findCorrespondencies_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* bundle, const cv::_OutputArray* cols, const cv::_OutputArray* response, Result_void* ocvrs_return) {
		try {
			cv::rapid::findCorrespondencies(*bundle, *cols, *response);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// rapid(cv::InputArray, int, int, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, double *) /usr/include/opencv2/rapid.hpp:123
	void cv_rapid_rapid_const__InputArrayR_int_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_doubleX(const cv::_InputArray* img, int num, int len, const cv::_InputArray* pts3d, const cv::_InputArray* tris, const cv::_InputArray* K, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, double* rmsd, Result<float>* ocvrs_return) {
		try {
			float ret = cv::rapid::rapid(*img, num, len, *pts3d, *tris, *K, *rvec, *tvec, rmsd);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// create(cv::InputArray, cv::InputArray, int, uchar) /usr/include/opencv2/rapid.hpp:158
	void cv_rapid_GOSTracker_create_const__InputArrayR_const__InputArrayR_int_unsigned_char(const cv::_InputArray* pts3d, const cv::_InputArray* tris, int histBins, unsigned char sobelThesh, Result<cv::Ptr<cv::rapid::OLSTracker>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rapid::OLSTracker> ret = cv::rapid::GOSTracker::create(*pts3d, *tris, histBins, sobelThesh);
			Ok(new cv::Ptr<cv::rapid::OLSTracker>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rapid::OLSTracker>*>))
	}
	
	// create(cv::InputArray, cv::InputArray, int, uchar) /usr/include/opencv2/rapid.hpp:150
	void cv_rapid_OLSTracker_create_const__InputArrayR_const__InputArrayR_int_unsigned_char(const cv::_InputArray* pts3d, const cv::_InputArray* tris, int histBins, unsigned char sobelThesh, Result<cv::Ptr<cv::rapid::OLSTracker>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rapid::OLSTracker> ret = cv::rapid::OLSTracker::create(*pts3d, *tris, histBins, sobelThesh);
			Ok(new cv::Ptr<cv::rapid::OLSTracker>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rapid::OLSTracker>*>))
	}
	
	// create(cv::InputArray, cv::InputArray) /usr/include/opencv2/rapid.hpp:141
	void cv_rapid_Rapid_create_const__InputArrayR_const__InputArrayR(const cv::_InputArray* pts3d, const cv::_InputArray* tris, Result<cv::Ptr<cv::rapid::Rapid>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rapid::Rapid> ret = cv::rapid::Rapid::create(*pts3d, *tris);
			Ok(new cv::Ptr<cv::rapid::Rapid>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rapid::Rapid>*>))
	}
	
	// compute(cv::InputArray, int, int, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, const cv::TermCriteria &) /usr/include/opencv2/rapid.hpp:132
	void cv_rapid_Tracker_compute_const__InputArrayR_int_int_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_TermCriteriaR(cv::rapid::Tracker* instance, const cv::_InputArray* img, int num, int len, const cv::_InputArray* K, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, const cv::TermCriteria* termcrit, Result<float>* ocvrs_return) {
		try {
			float ret = instance->compute(*img, num, len, *K, *rvec, *tvec, *termcrit);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// clearState() /usr/include/opencv2/rapid.hpp:134
	void cv_rapid_Tracker_clearState(cv::rapid::Tracker* instance, Result_void* ocvrs_return) {
		try {
			instance->clearState();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
