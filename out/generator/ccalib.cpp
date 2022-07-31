#include "ccalib.hpp"
#include "ccalib_types.hpp"

extern "C" {
	// calibrate(cv::InputArrayOfArrays, cv::InputArrayOfArrays, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, int, cv::TermCriteria, cv::OutputArray) /usr/include/opencv2/ccalib/omnidir.hpp:176
	void cv_omnidir_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* size, const cv::_InputOutputArray* K, const cv::_InputOutputArray* xi, const cv::_InputOutputArray* D, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, cv::TermCriteria* criteria, const cv::_OutputArray* idx, Result<double>* ocvrs_return) {
		try {
			double ret = cv::omnidir::calibrate(*objectPoints, *imagePoints, *size, *K, *xi, *D, *rvecs, *tvecs, flags, *criteria, *idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// initUndistortRectifyMap(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, const cv::Size &, int, cv::OutputArray, cv::OutputArray, int) /usr/include/opencv2/ccalib/omnidir.hpp:141
	void cv_omnidir_initUndistortRectifyMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_int_const__OutputArrayR_const__OutputArrayR_int(const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* xi, const cv::_InputArray* R, const cv::_InputArray* P, const cv::Size* size, int m1type, const cv::_OutputArray* map1, const cv::_OutputArray* map2, int flags, Result_void* ocvrs_return) {
		try {
			cv::omnidir::initUndistortRectifyMap(*K, *D, *xi, *R, *P, *size, m1type, *map1, *map2, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// projectPoints(cv::InputArray, cv::OutputArray, const cv::Affine3d &, cv::InputArray, double, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ccalib/omnidir.hpp:107
	void cv_omnidir_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_OutputArray* imagePoints, const cv::Affine3d* affine, const cv::_InputArray* K, double xi, const cv::_InputArray* D, const cv::_OutputArray* jacobian, Result_void* ocvrs_return) {
		try {
			cv::omnidir::projectPoints(*objectPoints, *imagePoints, *affine, *K, xi, *D, *jacobian);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// projectPoints(cv::InputArray, cv::OutputArray, cv::InputArray, cv::InputArray, cv::InputArray, double, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ccalib/omnidir.hpp:103
	void cv_omnidir_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* objectPoints, const cv::_OutputArray* imagePoints, const cv::_InputArray* rvec, const cv::_InputArray* tvec, const cv::_InputArray* K, double xi, const cv::_InputArray* D, const cv::_OutputArray* jacobian, Result_void* ocvrs_return) {
		try {
			cv::omnidir::projectPoints(*objectPoints, *imagePoints, *rvec, *tvec, *K, xi, *D, *jacobian);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// stereoCalibrate(cv::InputOutputArrayOfArrays, cv::InputOutputArrayOfArrays, cv::InputOutputArrayOfArrays, const cv::Size &, const cv::Size &, cv::InputOutputArray, cv::InputOutputArray, cv::InputOutputArray, cv::InputOutputArray, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, int, cv::TermCriteria, cv::OutputArray) /usr/include/opencv2/ccalib/omnidir.hpp:207
	void cv_omnidir_stereoCalibrate_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_SizeR_const_SizeR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria_const__OutputArrayR(const cv::_InputOutputArray* objectPoints, const cv::_InputOutputArray* imagePoints1, const cv::_InputOutputArray* imagePoints2, const cv::Size* imageSize1, const cv::Size* imageSize2, const cv::_InputOutputArray* K1, const cv::_InputOutputArray* xi1, const cv::_InputOutputArray* D1, const cv::_InputOutputArray* K2, const cv::_InputOutputArray* xi2, const cv::_InputOutputArray* D2, const cv::_OutputArray* rvec, const cv::_OutputArray* tvec, const cv::_OutputArray* rvecsL, const cv::_OutputArray* tvecsL, int flags, cv::TermCriteria* criteria, const cv::_OutputArray* idx, Result<double>* ocvrs_return) {
		try {
			double ret = cv::omnidir::stereoCalibrate(*objectPoints, *imagePoints1, *imagePoints2, *imageSize1, *imageSize2, *K1, *xi1, *D1, *K2, *xi2, *D2, *rvec, *tvec, *rvecsL, *tvecsL, flags, *criteria, *idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// stereoReconstruct(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, int, int, int, cv::OutputArray, cv::OutputArray, cv::OutputArray, const cv::Size &, cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ccalib/omnidir.hpp:243
	void cv_omnidir_stereoReconstruct_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_int_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* image1, const cv::_InputArray* image2, const cv::_InputArray* K1, const cv::_InputArray* D1, const cv::_InputArray* xi1, const cv::_InputArray* K2, const cv::_InputArray* D2, const cv::_InputArray* xi2, const cv::_InputArray* R, const cv::_InputArray* T, int flag, int numDisparities, int SADWindowSize, const cv::_OutputArray* disparity, const cv::_OutputArray* image1Rec, const cv::_OutputArray* image2Rec, const cv::Size* newSize, const cv::_InputArray* Knew, const cv::_OutputArray* pointCloud, int pointType, Result_void* ocvrs_return) {
		try {
			cv::omnidir::stereoReconstruct(*image1, *image2, *K1, *D1, *xi1, *K2, *D2, *xi2, *R, *T, flag, numDisparities, SADWindowSize, *disparity, *image1Rec, *image2Rec, *newSize, *Knew, *pointCloud, pointType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// stereoRectify(cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ccalib/omnidir.hpp:218
	void cv_omnidir_stereoRectify_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* R, const cv::_InputArray* T, const cv::_OutputArray* R1, const cv::_OutputArray* R2, Result_void* ocvrs_return) {
		try {
			cv::omnidir::stereoRectify(*R, *T, *R1, *R2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// undistortImage(cv::InputArray, cv::OutputArray, cv::InputArray, cv::InputArray, cv::InputArray, int, cv::InputArray, const cv::Size &, cv::InputArray) /usr/include/opencv2/ccalib/omnidir.hpp:156
	void cv_omnidir_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_const__InputArrayR_const_SizeR_const__InputArrayR(const cv::_InputArray* distorted, const cv::_OutputArray* undistorted, const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* xi, int flags, const cv::_InputArray* Knew, const cv::Size* new_size, const cv::_InputArray* R, Result_void* ocvrs_return) {
		try {
			cv::omnidir::undistortImage(*distorted, *undistorted, *K, *D, *xi, flags, *Knew, *new_size, *R);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// undistortPoints(cv::InputArray, cv::OutputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/ccalib/omnidir.hpp:122
	void cv_omnidir_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* distorted, const cv::_OutputArray* undistorted, const cv::_InputArray* K, const cv::_InputArray* D, const cv::_InputArray* xi, const cv::_InputArray* R, Result_void* ocvrs_return) {
		try {
			cv::omnidir::undistortPoints(*distorted, *undistorted, *K, *D, *xi, *R);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::Algorithm* cv_CustomPattern_to_Algorithm(cv::ccalib::CustomPattern* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_CustomPattern_delete(cv::ccalib::CustomPattern* instance) {
		delete instance;
	}
	// CustomPattern() /usr/include/opencv2/ccalib.hpp:63
	void cv_ccalib_CustomPattern_CustomPattern(Result<cv::ccalib::CustomPattern*>* ocvrs_return) {
		try {
			cv::ccalib::CustomPattern* ret = new cv::ccalib::CustomPattern();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ccalib::CustomPattern*>))
	}
	
	// create(cv::InputArray, const cv::Size2f, cv::OutputArray) /usr/include/opencv2/ccalib.hpp:66
	void cv_ccalib_CustomPattern_create_const__InputArrayR_const_Size2f_const__OutputArrayR(cv::ccalib::CustomPattern* instance, const cv::_InputArray* pattern, const cv::Size2f* boardSize, const cv::_OutputArray* output, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->create(*pattern, *boardSize, *output);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// findPattern(cv::InputArray, cv::OutputArray, cv::OutputArray, const double, const double, const bool, cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/ccalib.hpp:68
	void cv_ccalib_CustomPattern_findPattern_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const_double_const_double_const_bool_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::ccalib::CustomPattern* instance, const cv::_InputArray* image, const cv::_OutputArray* matched_features, const cv::_OutputArray* pattern_points, const double ratio, const double proj_error, const bool refine_position, const cv::_OutputArray* out, const cv::_OutputArray* H, const cv::_OutputArray* pattern_corners, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->findPattern(*image, *matched_features, *pattern_points, ratio, proj_error, refine_position, *out, *H, *pattern_corners);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isInitialized() /usr/include/opencv2/ccalib.hpp:72
	void cv_ccalib_CustomPattern_isInitialized(cv::ccalib::CustomPattern* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isInitialized();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// getPatternPoints(std::vector<KeyPoint> &) /usr/include/opencv2/ccalib.hpp:74
	void cv_ccalib_CustomPattern_getPatternPoints_vector_KeyPoint_R(cv::ccalib::CustomPattern* instance, std::vector<cv::KeyPoint>* original_points, Result_void* ocvrs_return) {
		try {
			instance->getPatternPoints(*original_points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPixelSize() /usr/include/opencv2/ccalib.hpp:78
	void cv_ccalib_CustomPattern_getPixelSize(cv::ccalib::CustomPattern* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getPixelSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setFeatureDetector(Ptr<cv::FeatureDetector>) /usr/include/opencv2/ccalib.hpp:83
	void cv_ccalib_CustomPattern_setFeatureDetector_Ptr_Feature2D_(cv::ccalib::CustomPattern* instance, cv::Ptr<cv::Feature2D>* featureDetector, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setFeatureDetector(*featureDetector);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setDescriptorExtractor(Ptr<cv::DescriptorExtractor>) /usr/include/opencv2/ccalib.hpp:84
	void cv_ccalib_CustomPattern_setDescriptorExtractor_Ptr_Feature2D_(cv::ccalib::CustomPattern* instance, cv::Ptr<cv::Feature2D>* extractor, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setDescriptorExtractor(*extractor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setDescriptorMatcher(Ptr<cv::DescriptorMatcher>) /usr/include/opencv2/ccalib.hpp:85
	void cv_ccalib_CustomPattern_setDescriptorMatcher_Ptr_DescriptorMatcher_(cv::ccalib::CustomPattern* instance, cv::Ptr<cv::DescriptorMatcher>* matcher, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setDescriptorMatcher(*matcher);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// getFeatureDetector() /usr/include/opencv2/ccalib.hpp:87
	void cv_ccalib_CustomPattern_getFeatureDetector(cv::ccalib::CustomPattern* instance, Result<cv::Ptr<cv::Feature2D>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Feature2D> ret = instance->getFeatureDetector();
			Ok(new cv::Ptr<cv::Feature2D>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::Feature2D>*>))
	}
	
	// getDescriptorExtractor() /usr/include/opencv2/ccalib.hpp:88
	void cv_ccalib_CustomPattern_getDescriptorExtractor(cv::ccalib::CustomPattern* instance, Result<cv::Ptr<cv::Feature2D>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Feature2D> ret = instance->getDescriptorExtractor();
			Ok(new cv::Ptr<cv::Feature2D>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::Feature2D>*>))
	}
	
	// getDescriptorMatcher() /usr/include/opencv2/ccalib.hpp:89
	void cv_ccalib_CustomPattern_getDescriptorMatcher(cv::ccalib::CustomPattern* instance, Result<cv::Ptr<cv::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DescriptorMatcher> ret = instance->getDescriptorMatcher();
			Ok(new cv::Ptr<cv::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DescriptorMatcher>*>))
	}
	
	// calibrate(cv::InputArrayOfArrays, cv::InputArrayOfArrays, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, int, cv::TermCriteria) /usr/include/opencv2/ccalib.hpp:91
	void cv_ccalib_CustomPattern_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(cv::ccalib::CustomPattern* instance, const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = instance->calibrate(*objectPoints, *imagePoints, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// findRt(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool, int) /usr/include/opencv2/ccalib.hpp:99
	void cv_ccalib_CustomPattern_findRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool_int(cv::ccalib::CustomPattern* instance, const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, bool useExtrinsicGuess, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->findRt(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// findRt(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool, int) /usr/include/opencv2/ccalib.hpp:101
	void cv_ccalib_CustomPattern_findRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool_int(cv::ccalib::CustomPattern* instance, const cv::_InputArray* image, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, bool useExtrinsicGuess, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->findRt(*image, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// findRtRANSAC(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool, int, float, int, cv::OutputArray, int) /usr/include/opencv2/ccalib.hpp:108
	void cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool_int_float_int_const__OutputArrayR_int(cv::ccalib::CustomPattern* instance, const cv::_InputArray* objectPoints, const cv::_InputArray* imagePoints, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, bool useExtrinsicGuess, int iterationsCount, float reprojectionError, int minInliersCount, const cv::_OutputArray* inliers, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->findRtRANSAC(*objectPoints, *imagePoints, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, iterationsCount, reprojectionError, minInliersCount, *inliers, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// findRtRANSAC(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool, int, float, int, cv::OutputArray, int) /usr/include/opencv2/ccalib.hpp:111
	void cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool_int_float_int_const__OutputArrayR_int(cv::ccalib::CustomPattern* instance, const cv::_InputArray* image, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, bool useExtrinsicGuess, int iterationsCount, float reprojectionError, int minInliersCount, const cv::_OutputArray* inliers, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->findRtRANSAC(*image, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess, iterationsCount, reprojectionError, minInliersCount, *inliers, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// drawOrientation(cv::InputOutputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, double, int) /usr/include/opencv2/ccalib.hpp:118
	void cv_ccalib_CustomPattern_drawOrientation_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_int(cv::ccalib::CustomPattern* instance, const cv::_InputOutputArray* image, const cv::_InputArray* tvec, const cv::_InputArray* rvec, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, double axis_length, int axis_width, Result_void* ocvrs_return) {
		try {
			instance->drawOrientation(*image, *tvec, *rvec, *cameraMatrix, *distCoeffs, axis_length, axis_width);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_MultiCameraCalibration_delete(cv::multicalib::MultiCameraCalibration* instance) {
		delete instance;
	}
	// MultiCameraCalibration(int, int, const std::string &, float, float, int, int, int, int, cv::TermCriteria, Ptr<cv::FeatureDetector>, Ptr<cv::DescriptorExtractor>, Ptr<cv::DescriptorMatcher>) /usr/include/opencv2/ccalib/multicalib.hpp:132
	void cv_multicalib_MultiCameraCalibration_MultiCameraCalibration_int_int_const_stringR_float_float_int_int_int_int_TermCriteria_Ptr_Feature2D__Ptr_Feature2D__Ptr_DescriptorMatcher_(int cameraType, int nCameras, const char* fileName, float patternWidth, float patternHeight, int verbose, int showExtration, int nMiniMatches, int flags, cv::TermCriteria* criteria, cv::Ptr<cv::Feature2D>* detector, cv::Ptr<cv::Feature2D>* descriptor, cv::Ptr<cv::DescriptorMatcher>* matcher, Result<cv::multicalib::MultiCameraCalibration*>* ocvrs_return) {
		try {
			cv::multicalib::MultiCameraCalibration* ret = new cv::multicalib::MultiCameraCalibration(cameraType, nCameras, std::string(fileName), patternWidth, patternHeight, verbose, showExtration, nMiniMatches, flags, *criteria, *detector, *descriptor, *matcher);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::multicalib::MultiCameraCalibration*>))
	}
	
	// loadImages() /usr/include/opencv2/ccalib/multicalib.hpp:141
	void cv_multicalib_MultiCameraCalibration_loadImages(cv::multicalib::MultiCameraCalibration* instance, Result_void* ocvrs_return) {
		try {
			instance->loadImages();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// initialize() /usr/include/opencv2/ccalib/multicalib.hpp:145
	void cv_multicalib_MultiCameraCalibration_initialize(cv::multicalib::MultiCameraCalibration* instance, Result_void* ocvrs_return) {
		try {
			instance->initialize();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// optimizeExtrinsics() /usr/include/opencv2/ccalib/multicalib.hpp:149
	void cv_multicalib_MultiCameraCalibration_optimizeExtrinsics(cv::multicalib::MultiCameraCalibration* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->optimizeExtrinsics();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// run() /usr/include/opencv2/ccalib/multicalib.hpp:153
	void cv_multicalib_MultiCameraCalibration_run(cv::multicalib::MultiCameraCalibration* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->run();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// writeParameters(const std::string &) /usr/include/opencv2/ccalib/multicalib.hpp:157
	void cv_multicalib_MultiCameraCalibration_writeParameters_const_stringR(cv::multicalib::MultiCameraCalibration* instance, const char* filename, Result_void* ocvrs_return) {
		try {
			instance->writeParameters(std::string(filename));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// cameraVertex /usr/include/opencv2/ccalib/multicalib.hpp:84
	int cv_multicalib_MultiCameraCalibration_edge_getPropCameraVertex_const(const cv::multicalib::MultiCameraCalibration::edge* instance) {
			int ret = instance->cameraVertex;
			return ret;
	}
	
	// cameraVertex /usr/include/opencv2/ccalib/multicalib.hpp:84
	void cv_multicalib_MultiCameraCalibration_edge_setPropCameraVertex_int(cv::multicalib::MultiCameraCalibration::edge* instance, int val) {
			instance->cameraVertex = val;
	}
	
	// photoVertex /usr/include/opencv2/ccalib/multicalib.hpp:85
	int cv_multicalib_MultiCameraCalibration_edge_getPropPhotoVertex_const(const cv::multicalib::MultiCameraCalibration::edge* instance) {
			int ret = instance->photoVertex;
			return ret;
	}
	
	// photoVertex /usr/include/opencv2/ccalib/multicalib.hpp:85
	void cv_multicalib_MultiCameraCalibration_edge_setPropPhotoVertex_int(cv::multicalib::MultiCameraCalibration::edge* instance, int val) {
			instance->photoVertex = val;
	}
	
	// photoIndex /usr/include/opencv2/ccalib/multicalib.hpp:86
	int cv_multicalib_MultiCameraCalibration_edge_getPropPhotoIndex_const(const cv::multicalib::MultiCameraCalibration::edge* instance) {
			int ret = instance->photoIndex;
			return ret;
	}
	
	// photoIndex /usr/include/opencv2/ccalib/multicalib.hpp:86
	void cv_multicalib_MultiCameraCalibration_edge_setPropPhotoIndex_int(cv::multicalib::MultiCameraCalibration::edge* instance, int val) {
			instance->photoIndex = val;
	}
	
	// transform /usr/include/opencv2/ccalib/multicalib.hpp:87
	cv::Mat* cv_multicalib_MultiCameraCalibration_edge_getPropTransform_const(const cv::multicalib::MultiCameraCalibration::edge* instance) {
			cv::Mat ret = instance->transform;
			return new cv::Mat(ret);
	}
	
	// transform /usr/include/opencv2/ccalib/multicalib.hpp:87
	void cv_multicalib_MultiCameraCalibration_edge_setPropTransform_Mat(cv::multicalib::MultiCameraCalibration::edge* instance, cv::Mat* val) {
			instance->transform = *val;
	}
	
	void cv_MultiCameraCalibration_edge_delete(cv::multicalib::MultiCameraCalibration::edge* instance) {
		delete instance;
	}
	// edge(int, int, int, cv::Mat) /usr/include/opencv2/ccalib/multicalib.hpp:89
	void cv_multicalib_MultiCameraCalibration_edge_edge_int_int_int_Mat(int cv, int pv, int pi, cv::Mat* trans, Result<cv::multicalib::MultiCameraCalibration::edge*>* ocvrs_return) {
		try {
			cv::multicalib::MultiCameraCalibration::edge* ret = new cv::multicalib::MultiCameraCalibration::edge(cv, pv, pi, *trans);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::multicalib::MultiCameraCalibration::edge*>))
	}
	
	// pose /usr/include/opencv2/ccalib/multicalib.hpp:100
	cv::Mat* cv_multicalib_MultiCameraCalibration_vertex_getPropPose_const(const cv::multicalib::MultiCameraCalibration::vertex* instance) {
			cv::Mat ret = instance->pose;
			return new cv::Mat(ret);
	}
	
	// pose /usr/include/opencv2/ccalib/multicalib.hpp:100
	void cv_multicalib_MultiCameraCalibration_vertex_setPropPose_Mat(cv::multicalib::MultiCameraCalibration::vertex* instance, cv::Mat* val) {
			instance->pose = *val;
	}
	
	// timestamp /usr/include/opencv2/ccalib/multicalib.hpp:103
	int cv_multicalib_MultiCameraCalibration_vertex_getPropTimestamp_const(const cv::multicalib::MultiCameraCalibration::vertex* instance) {
			int ret = instance->timestamp;
			return ret;
	}
	
	// timestamp /usr/include/opencv2/ccalib/multicalib.hpp:103
	void cv_multicalib_MultiCameraCalibration_vertex_setPropTimestamp_int(cv::multicalib::MultiCameraCalibration::vertex* instance, int val) {
			instance->timestamp = val;
	}
	
	void cv_MultiCameraCalibration_vertex_delete(cv::multicalib::MultiCameraCalibration::vertex* instance) {
		delete instance;
	}
	// vertex(cv::Mat, int) /usr/include/opencv2/ccalib/multicalib.hpp:105
	void cv_multicalib_MultiCameraCalibration_vertex_vertex_Mat_int(cv::Mat* po, int ts, Result<cv::multicalib::MultiCameraCalibration::vertex*>* ocvrs_return) {
		try {
			cv::multicalib::MultiCameraCalibration::vertex* ret = new cv::multicalib::MultiCameraCalibration::vertex(*po, ts);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::multicalib::MultiCameraCalibration::vertex*>))
	}
	
	// vertex() /usr/include/opencv2/ccalib/multicalib.hpp:111
	void cv_multicalib_MultiCameraCalibration_vertex_vertex(Result<cv::multicalib::MultiCameraCalibration::vertex*>* ocvrs_return) {
		try {
			cv::multicalib::MultiCameraCalibration::vertex* ret = new cv::multicalib::MultiCameraCalibration::vertex();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::multicalib::MultiCameraCalibration::vertex*>))
	}
	
	void cv_RandomPatternCornerFinder_delete(cv::randpattern::RandomPatternCornerFinder* instance) {
		delete instance;
	}
	// RandomPatternCornerFinder(float, float, int, int, int, int, Ptr<cv::FeatureDetector>, Ptr<cv::DescriptorExtractor>, Ptr<cv::DescriptorMatcher>) /usr/include/opencv2/ccalib/randpattern.hpp:80
	void cv_randpattern_RandomPatternCornerFinder_RandomPatternCornerFinder_float_float_int_int_int_int_Ptr_Feature2D__Ptr_Feature2D__Ptr_DescriptorMatcher_(float patternWidth, float patternHeight, int nminiMatch, int depth, int verbose, int showExtraction, cv::Ptr<cv::Feature2D>* detector, cv::Ptr<cv::Feature2D>* descriptor, cv::Ptr<cv::DescriptorMatcher>* matcher, Result<cv::randpattern::RandomPatternCornerFinder*>* ocvrs_return) {
		try {
			cv::randpattern::RandomPatternCornerFinder* ret = new cv::randpattern::RandomPatternCornerFinder(patternWidth, patternHeight, nminiMatch, depth, verbose, showExtraction, *detector, *descriptor, *matcher);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::randpattern::RandomPatternCornerFinder*>))
	}
	
	// loadPattern(const cv::Mat &) /usr/include/opencv2/ccalib/randpattern.hpp:89
	void cv_randpattern_RandomPatternCornerFinder_loadPattern_const_MatR(cv::randpattern::RandomPatternCornerFinder* instance, const cv::Mat* patternImage, Result_void* ocvrs_return) {
		try {
			instance->loadPattern(*patternImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// loadPattern(const cv::Mat &, const std::vector<cv::KeyPoint> &, const cv::Mat &) /usr/include/opencv2/ccalib/randpattern.hpp:96
	void cv_randpattern_RandomPatternCornerFinder_loadPattern_const_MatR_const_vector_KeyPoint_R_const_MatR(cv::randpattern::RandomPatternCornerFinder* instance, const cv::Mat* patternImage, const std::vector<cv::KeyPoint>* patternKeyPoints, const cv::Mat* patternDescriptors, Result_void* ocvrs_return) {
		try {
			instance->loadPattern(*patternImage, *patternKeyPoints, *patternDescriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// computeObjectImagePoints(std::vector<cv::Mat>) /usr/include/opencv2/ccalib/randpattern.hpp:105
	void cv_randpattern_RandomPatternCornerFinder_computeObjectImagePoints_vector_Mat_(cv::randpattern::RandomPatternCornerFinder* instance, std::vector<cv::Mat>* inputImages, Result_void* ocvrs_return) {
		try {
			instance->computeObjectImagePoints(*inputImages);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// computeObjectImagePointsForSingle(cv::Mat) /usr/include/opencv2/ccalib/randpattern.hpp:114
	void cv_randpattern_RandomPatternCornerFinder_computeObjectImagePointsForSingle_Mat(cv::randpattern::RandomPatternCornerFinder* instance, cv::Mat* inputImage, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			std::vector<cv::Mat> ret = instance->computeObjectImagePointsForSingle(*inputImage);
			Ok(new std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	// getObjectPoints() /usr/include/opencv2/ccalib/randpattern.hpp:118
	void cv_randpattern_RandomPatternCornerFinder_getObjectPoints(cv::randpattern::RandomPatternCornerFinder* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->getObjectPoints();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	// getImagePoints() /usr/include/opencv2/ccalib/randpattern.hpp:122
	void cv_randpattern_RandomPatternCornerFinder_getImagePoints(cv::randpattern::RandomPatternCornerFinder* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->getImagePoints();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	void cv_RandomPatternGenerator_delete(cv::randpattern::RandomPatternGenerator* instance) {
		delete instance;
	}
	// RandomPatternGenerator(int, int) /usr/include/opencv2/ccalib/randpattern.hpp:168
	void cv_randpattern_RandomPatternGenerator_RandomPatternGenerator_int_int(int imageWidth, int imageHeight, Result<cv::randpattern::RandomPatternGenerator*>* ocvrs_return) {
		try {
			cv::randpattern::RandomPatternGenerator* ret = new cv::randpattern::RandomPatternGenerator(imageWidth, imageHeight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::randpattern::RandomPatternGenerator*>))
	}
	
	// generatePattern() /usr/include/opencv2/ccalib/randpattern.hpp:172
	void cv_randpattern_RandomPatternGenerator_generatePattern(cv::randpattern::RandomPatternGenerator* instance, Result_void* ocvrs_return) {
		try {
			instance->generatePattern();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPattern() /usr/include/opencv2/ccalib/randpattern.hpp:175
	void cv_randpattern_RandomPatternGenerator_getPattern(cv::randpattern::RandomPatternGenerator* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getPattern();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
}
