#include "aruco.hpp"
#include "aruco_types.hpp"

extern "C" {
	// calibrateCameraAruco(cv::InputArrayOfArrays, cv::InputArray, cv::InputArray, const Ptr<cv::aruco::Board> &, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, cv::OutputArray, cv::OutputArray, cv::OutputArray, int, cv::TermCriteria) /usr/include/opencv2/aruco.hpp:648
	void cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_Ptr_Board_R_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::_InputArray* counter, const cv::Ptr<cv::aruco::Board>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* perViewErrors, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraAruco(*corners, *ids, *counter, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *perViewErrors, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// calibrateCameraAruco(cv::InputArrayOfArrays, cv::InputArray, cv::InputArray, const Ptr<cv::aruco::Board> &, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, int, cv::TermCriteria) /usr/include/opencv2/aruco.hpp:659
	void cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_Ptr_Board_R_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::_InputArray* counter, const cv::Ptr<cv::aruco::Board>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraAruco(*corners, *ids, *counter, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// calibrateCameraCharuco(cv::InputArrayOfArrays, cv::InputArrayOfArrays, const Ptr<cv::aruco::CharucoBoard> &, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, cv::OutputArray, cv::OutputArray, cv::OutputArray, int, cv::TermCriteria) /usr/include/opencv2/aruco/charuco.hpp:245
	void cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_Ptr_CharucoBoard_R_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* stdDeviationsIntrinsics, const cv::_OutputArray* stdDeviationsExtrinsics, const cv::_OutputArray* perViewErrors, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraCharuco(*charucoCorners, *charucoIds, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *stdDeviationsIntrinsics, *stdDeviationsExtrinsics, *perViewErrors, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// calibrateCameraCharuco(cv::InputArrayOfArrays, cv::InputArrayOfArrays, const Ptr<cv::aruco::CharucoBoard> &, cv::Size, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays, int, cv::TermCriteria) /usr/include/opencv2/aruco/charuco.hpp:255
	void cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_Ptr_CharucoBoard_R_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, cv::Size* imageSize, const cv::_InputOutputArray* cameraMatrix, const cv::_InputOutputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, int flags, cv::TermCriteria* criteria, Result<double>* ocvrs_return) {
		try {
			double ret = cv::aruco::calibrateCameraCharuco(*charucoCorners, *charucoIds, *board, *imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, flags, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// detectCharucoDiamond(cv::InputArray, cv::InputArrayOfArrays, cv::InputArray, float, cv::OutputArrayOfArrays, cv::OutputArray, cv::InputArray, cv::InputArray, Ptr<cv::aruco::Dictionary>) /usr/include/opencv2/aruco/charuco.hpp:286
	void cv_aruco_detectCharucoDiamond_const__InputArrayR_const__InputArrayR_const__InputArrayR_float_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_Ptr_Dictionary_(const cv::_InputArray* image, const cv::_InputArray* markerCorners, const cv::_InputArray* markerIds, float squareMarkerLengthRate, const cv::_OutputArray* diamondCorners, const cv::_OutputArray* diamondIds, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, cv::Ptr<cv::aruco::Dictionary>* dictionary, Result_void* ocvrs_return) {
		try {
			cv::aruco::detectCharucoDiamond(*image, *markerCorners, *markerIds, squareMarkerLengthRate, *diamondCorners, *diamondIds, *cameraMatrix, *distCoeffs, *dictionary);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detectMarkers(cv::InputArray, const Ptr<cv::aruco::Dictionary> &, cv::OutputArrayOfArrays, cv::OutputArray, const Ptr<cv::aruco::DetectorParameters> &, cv::OutputArrayOfArrays) /usr/include/opencv2/aruco.hpp:244
	void cv_aruco_detectMarkers_const__InputArrayR_const_Ptr_Dictionary_R_const__OutputArrayR_const__OutputArrayR_const_Ptr_DetectorParameters_R_const__OutputArrayR(const cv::_InputArray* image, const cv::Ptr<cv::aruco::Dictionary>* dictionary, const cv::_OutputArray* corners, const cv::_OutputArray* ids, const cv::Ptr<cv::aruco::DetectorParameters>* parameters, const cv::_OutputArray* rejectedImgPoints, Result_void* ocvrs_return) {
		try {
			cv::aruco::detectMarkers(*image, *dictionary, *corners, *ids, *parameters, *rejectedImgPoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// drawCharucoDiamond(const Ptr<cv::aruco::Dictionary> &, cv::Vec4i, int, int, cv::OutputArray, int, int) /usr/include/opencv2/aruco/charuco.hpp:334
	void cv_aruco_drawCharucoDiamond_const_Ptr_Dictionary_R_Vec4i_int_int_const__OutputArrayR_int_int(const cv::Ptr<cv::aruco::Dictionary>* dictionary, cv::Vec4i* ids, int squareLength, int markerLength, const cv::_OutputArray* img, int marginSize, int borderBits, Result_void* ocvrs_return) {
		try {
			cv::aruco::drawCharucoDiamond(*dictionary, *ids, squareLength, markerLength, *img, marginSize, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// drawDetectedCornersCharuco(cv::InputOutputArray, cv::InputArray, cv::InputArray, cv::Scalar) /usr/include/opencv2/aruco/charuco.hpp:205
	void cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, cv::Scalar* cornerColor, Result_void* ocvrs_return) {
		try {
			cv::aruco::drawDetectedCornersCharuco(*image, *charucoCorners, *charucoIds, *cornerColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// drawDetectedDiamonds(cv::InputOutputArray, cv::InputArrayOfArrays, cv::InputArray, cv::Scalar) /usr/include/opencv2/aruco/charuco.hpp:313
	void cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* diamondCorners, const cv::_InputArray* diamondIds, cv::Scalar* borderColor, Result_void* ocvrs_return) {
		try {
			cv::aruco::drawDetectedDiamonds(*image, *diamondCorners, *diamondIds, *borderColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// drawDetectedMarkers(cv::InputOutputArray, cv::InputArrayOfArrays, cv::InputArray, cv::Scalar) /usr/include/opencv2/aruco.hpp:561
	void cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* corners, const cv::_InputArray* ids, cv::Scalar* borderColor, Result_void* ocvrs_return) {
		try {
			cv::aruco::drawDetectedMarkers(*image, *corners, *ids, *borderColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// drawMarker(const Ptr<cv::aruco::Dictionary> &, int, int, cv::OutputArray, int) /usr/include/opencv2/aruco.hpp:579
	void cv_aruco_drawMarker_const_Ptr_Dictionary_R_int_int_const__OutputArrayR_int(const cv::Ptr<cv::aruco::Dictionary>* dictionary, int id, int sidePixels, const cv::_OutputArray* img, int borderBits, Result_void* ocvrs_return) {
		try {
			cv::aruco::drawMarker(*dictionary, id, sidePixels, *img, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// drawPlanarBoard(const Ptr<cv::aruco::Board> &, cv::Size, cv::OutputArray, int, int) /usr/include/opencv2/aruco.hpp:599
	void cv_aruco_drawPlanarBoard_const_Ptr_Board_R_Size_const__OutputArrayR_int_int(const cv::Ptr<cv::aruco::Board>* board, cv::Size* outSize, const cv::_OutputArray* img, int marginSize, int borderBits, Result_void* ocvrs_return) {
		try {
			cv::aruco::drawPlanarBoard(*board, *outSize, *img, marginSize, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// estimatePoseBoard(cv::InputArrayOfArrays, cv::InputArray, const Ptr<cv::aruco::Board> &, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool) /usr/include/opencv2/aruco.hpp:496
	void cv_aruco_estimatePoseBoard_const__InputArrayR_const__InputArrayR_const_Ptr_Board_R_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool(const cv::_InputArray* corners, const cv::_InputArray* ids, const cv::Ptr<cv::aruco::Board>* board, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, bool useExtrinsicGuess, Result<int>* ocvrs_return) {
		try {
			int ret = cv::aruco::estimatePoseBoard(*corners, *ids, *board, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// estimatePoseCharucoBoard(cv::InputArray, cv::InputArray, const Ptr<cv::aruco::CharucoBoard> &, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, bool) /usr/include/opencv2/aruco/charuco.hpp:186
	void cv_aruco_estimatePoseCharucoBoard_const__InputArrayR_const__InputArrayR_const_Ptr_CharucoBoard_R_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool(const cv::_InputArray* charucoCorners, const cv::_InputArray* charucoIds, const cv::Ptr<cv::aruco::CharucoBoard>* board, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_InputOutputArray* rvec, const cv::_InputOutputArray* tvec, bool useExtrinsicGuess, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::aruco::estimatePoseCharucoBoard(*charucoCorners, *charucoIds, *board, *cameraMatrix, *distCoeffs, *rvec, *tvec, useExtrinsicGuess);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// estimatePoseSingleMarkers(cv::InputArrayOfArrays, float, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray, Ptr<cv::aruco::EstimateParameters>) /usr/include/opencv2/aruco.hpp:334
	void cv_aruco_estimatePoseSingleMarkers_const__InputArrayR_float_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_Ptr_EstimateParameters_(const cv::_InputArray* corners, float markerLength, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, const cv::_OutputArray* rvecs, const cv::_OutputArray* tvecs, const cv::_OutputArray* _objPoints, cv::Ptr<cv::aruco::EstimateParameters>* estimateParameters, Result_void* ocvrs_return) {
		try {
			cv::aruco::estimatePoseSingleMarkers(*corners, markerLength, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, *_objPoints, *estimateParameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// generateCustomDictionary(int, int, const Ptr<cv::aruco::Dictionary> &, int) /usr/include/opencv2/aruco/dictionary.hpp:215
	void cv_aruco_generateCustomDictionary_int_int_const_Ptr_Dictionary_R_int(int nMarkers, int markerSize, const cv::Ptr<cv::aruco::Dictionary>* baseDictionary, int randomSeed, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::generateCustomDictionary(nMarkers, markerSize, *baseDictionary, randomSeed);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::Dictionary>*>))
	}
	
	// generateCustomDictionary(int, int, int) /usr/include/opencv2/aruco/dictionary.hpp:196
	void cv_aruco_generateCustomDictionary_int_int_int(int nMarkers, int markerSize, int randomSeed, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::generateCustomDictionary(nMarkers, markerSize, randomSeed);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::Dictionary>*>))
	}
	
	// getBoardObjectAndImagePoints(const Ptr<cv::aruco::Board> &, cv::InputArrayOfArrays, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/aruco.hpp:676
	void cv_aruco_getBoardObjectAndImagePoints_const_Ptr_Board_R_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::Ptr<cv::aruco::Board>* board, const cv::_InputArray* detectedCorners, const cv::_InputArray* detectedIds, const cv::_OutputArray* objPoints, const cv::_OutputArray* imgPoints, Result_void* ocvrs_return) {
		try {
			cv::aruco::getBoardObjectAndImagePoints(*board, *detectedCorners, *detectedIds, *objPoints, *imgPoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPredefinedDictionary(cv::aruco::PREDEFINED_DICTIONARY_NAME) /usr/include/opencv2/aruco/dictionary.hpp:184
	void cv_aruco_getPredefinedDictionary_PREDEFINED_DICTIONARY_NAME(cv::aruco::PREDEFINED_DICTIONARY_NAME name, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::getPredefinedDictionary(name);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::Dictionary>*>))
	}
	
	// getPredefinedDictionary(int) /usr/include/opencv2/aruco/dictionary.hpp:190
	void cv_aruco_getPredefinedDictionary_int(int dict, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::getPredefinedDictionary(dict);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::Dictionary>*>))
	}
	
	// interpolateCornersCharuco(cv::InputArrayOfArrays, cv::InputArray, cv::InputArray, const Ptr<cv::aruco::CharucoBoard> &, cv::OutputArray, cv::OutputArray, cv::InputArray, cv::InputArray, int) /usr/include/opencv2/aruco/charuco.hpp:158
	void cv_aruco_interpolateCornersCharuco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_Ptr_CharucoBoard_R_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* markerCorners, const cv::_InputArray* markerIds, const cv::_InputArray* image, const cv::Ptr<cv::aruco::CharucoBoard>* board, const cv::_OutputArray* charucoCorners, const cv::_OutputArray* charucoIds, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, int minMarkers, Result<int>* ocvrs_return) {
		try {
			int ret = cv::aruco::interpolateCornersCharuco(*markerCorners, *markerIds, *image, *board, *charucoCorners, *charucoIds, *cameraMatrix, *distCoeffs, minMarkers);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// refineDetectedMarkers(cv::InputArray, const Ptr<cv::aruco::Board> &, cv::InputOutputArrayOfArrays, cv::InputOutputArray, cv::InputOutputArrayOfArrays, cv::InputArray, cv::InputArray, float, float, bool, cv::OutputArray, const Ptr<cv::aruco::DetectorParameters> &) /usr/include/opencv2/aruco.hpp:534
	void cv_aruco_refineDetectedMarkers_const__InputArrayR_const_Ptr_Board_R_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_float_float_bool_const__OutputArrayR_const_Ptr_DetectorParameters_R(const cv::_InputArray* image, const cv::Ptr<cv::aruco::Board>* board, const cv::_InputOutputArray* detectedCorners, const cv::_InputOutputArray* detectedIds, const cv::_InputOutputArray* rejectedCorners, const cv::_InputArray* cameraMatrix, const cv::_InputArray* distCoeffs, float minRepDistance, float errorCorrectionRate, bool checkAllOrders, const cv::_OutputArray* recoveredIdxs, const cv::Ptr<cv::aruco::DetectorParameters>* parameters, Result_void* ocvrs_return) {
		try {
			cv::aruco::refineDetectedMarkers(*image, *board, *detectedCorners, *detectedIds, *rejectedCorners, *cameraMatrix, *distCoeffs, minRepDistance, errorCorrectionRate, checkAllOrders, *recoveredIdxs, *parameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// testCharucoCornersCollinear(const Ptr<cv::aruco::CharucoBoard> &, cv::InputArray) /usr/include/opencv2/aruco/charuco.hpp:349
	void cv_aruco_testCharucoCornersCollinear_const_Ptr_CharucoBoard_R_const__InputArrayR(const cv::Ptr<cv::aruco::CharucoBoard>* _board, const cv::_InputArray* _charucoIds, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::aruco::testCharucoCornersCollinear(*_board, *_charucoIds);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// objPoints /usr/include/opencv2/aruco.hpp:384
	std::vector<std::vector<cv::Point3f>>* cv_aruco_Board_getPropObjPoints_const(const cv::aruco::Board* instance) {
			std::vector<std::vector<cv::Point3f>> ret = instance->objPoints;
			return new std::vector<std::vector<cv::Point3f>>(ret);
	}
	
	// objPoints /usr/include/opencv2/aruco.hpp:384
	void cv_aruco_Board_setPropObjPoints_vector_vector_Point3f__(cv::aruco::Board* instance, std::vector<std::vector<cv::Point3f>>* val) {
			instance->objPoints = *val;
	}
	
	// dictionary /usr/include/opencv2/aruco.hpp:387
	cv::Ptr<cv::aruco::Dictionary>* cv_aruco_Board_getPropDictionary(cv::aruco::Board* instance) {
			cv::Ptr<cv::aruco::Dictionary> ret = instance->dictionary;
			return new cv::Ptr<cv::aruco::Dictionary>(ret);
	}
	
	// dictionary /usr/include/opencv2/aruco.hpp:387
	void cv_aruco_Board_setPropDictionary_Ptr_Dictionary_(cv::aruco::Board* instance, cv::Ptr<cv::aruco::Dictionary>* val) {
			instance->dictionary = *val;
	}
	
	// ids /usr/include/opencv2/aruco.hpp:391
	std::vector<int>* cv_aruco_Board_getPropIds_const(const cv::aruco::Board* instance) {
			std::vector<int> ret = instance->ids;
			return new std::vector<int>(ret);
	}
	
	// ids /usr/include/opencv2/aruco.hpp:391
	void cv_aruco_Board_setPropIds_vector_int_(cv::aruco::Board* instance, std::vector<int>* val) {
			instance->ids = *val;
	}
	
	// rightBottomBorder /usr/include/opencv2/aruco.hpp:394
	void cv_aruco_Board_getPropRightBottomBorder_const(const cv::aruco::Board* instance, cv::Point3f* ocvrs_return) {
			cv::Point3f ret = instance->rightBottomBorder;
			*ocvrs_return = ret;
	}
	
	// rightBottomBorder /usr/include/opencv2/aruco.hpp:394
	void cv_aruco_Board_setPropRightBottomBorder_Point3f(cv::aruco::Board* instance, cv::Point3f* val) {
			instance->rightBottomBorder = *val;
	}
	
	void cv_Board_delete(cv::aruco::Board* instance) {
		delete instance;
	}
	// create(cv::InputArrayOfArrays, const Ptr<cv::aruco::Dictionary> &, cv::InputArray) /usr/include/opencv2/aruco.hpp:362
	void cv_aruco_Board_create_const__InputArrayR_const_Ptr_Dictionary_R_const__InputArrayR(const cv::_InputArray* objPoints, const cv::Ptr<cv::aruco::Dictionary>* dictionary, const cv::_InputArray* ids, Result<cv::Ptr<cv::aruco::Board>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Board> ret = cv::aruco::Board::create(*objPoints, *dictionary, *ids);
			Ok(new cv::Ptr<cv::aruco::Board>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::Board>*>))
	}
	
	// setIds(cv::InputArray) /usr/include/opencv2/aruco.hpp:373
	void cv_aruco_Board_setIds_const__InputArrayR(cv::aruco::Board* instance, const cv::_InputArray* ids, Result_void* ocvrs_return) {
		try {
			instance->setIds(*ids);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// chessboardCorners /usr/include/opencv2/aruco/charuco.hpp:66
	std::vector<cv::Point3f>* cv_aruco_CharucoBoard_getPropChessboardCorners_const(const cv::aruco::CharucoBoard* instance) {
			std::vector<cv::Point3f> ret = instance->chessboardCorners;
			return new std::vector<cv::Point3f>(ret);
	}
	
	// chessboardCorners /usr/include/opencv2/aruco/charuco.hpp:66
	void cv_aruco_CharucoBoard_setPropChessboardCorners_vector_Point3f_(cv::aruco::CharucoBoard* instance, std::vector<cv::Point3f>* val) {
			instance->chessboardCorners = *val;
	}
	
	// nearestMarkerIdx /usr/include/opencv2/aruco/charuco.hpp:69
	std::vector<std::vector<int>>* cv_aruco_CharucoBoard_getPropNearestMarkerIdx_const(const cv::aruco::CharucoBoard* instance) {
			std::vector<std::vector<int>> ret = instance->nearestMarkerIdx;
			return new std::vector<std::vector<int>>(ret);
	}
	
	// nearestMarkerIdx /usr/include/opencv2/aruco/charuco.hpp:69
	void cv_aruco_CharucoBoard_setPropNearestMarkerIdx_vector_vector_int__(cv::aruco::CharucoBoard* instance, std::vector<std::vector<int>>* val) {
			instance->nearestMarkerIdx = *val;
	}
	
	// nearestMarkerCorners /usr/include/opencv2/aruco/charuco.hpp:70
	std::vector<std::vector<int>>* cv_aruco_CharucoBoard_getPropNearestMarkerCorners_const(const cv::aruco::CharucoBoard* instance) {
			std::vector<std::vector<int>> ret = instance->nearestMarkerCorners;
			return new std::vector<std::vector<int>>(ret);
	}
	
	// nearestMarkerCorners /usr/include/opencv2/aruco/charuco.hpp:70
	void cv_aruco_CharucoBoard_setPropNearestMarkerCorners_vector_vector_int__(cv::aruco::CharucoBoard* instance, std::vector<std::vector<int>>* val) {
			instance->nearestMarkerCorners = *val;
	}
	
	cv::aruco::Board* cv_CharucoBoard_to_Board(cv::aruco::CharucoBoard* instance) {
		return dynamic_cast<cv::aruco::Board*>(instance);
	}
	
	void cv_CharucoBoard_delete(cv::aruco::CharucoBoard* instance) {
		delete instance;
	}
	// draw(cv::Size, cv::OutputArray, int, int) /usr/include/opencv2/aruco/charuco.hpp:83
	void cv_aruco_CharucoBoard_draw_Size_const__OutputArrayR_int_int(cv::aruco::CharucoBoard* instance, cv::Size* outSize, const cv::_OutputArray* img, int marginSize, int borderBits, Result_void* ocvrs_return) {
		try {
			instance->draw(*outSize, *img, marginSize, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(int, int, float, float, const Ptr<cv::aruco::Dictionary> &) /usr/include/opencv2/aruco/charuco.hpp:100
	void cv_aruco_CharucoBoard_create_int_int_float_float_const_Ptr_Dictionary_R(int squaresX, int squaresY, float squareLength, float markerLength, const cv::Ptr<cv::aruco::Dictionary>* dictionary, Result<cv::Ptr<cv::aruco::CharucoBoard>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::CharucoBoard> ret = cv::aruco::CharucoBoard::create(squaresX, squaresY, squareLength, markerLength, *dictionary);
			Ok(new cv::Ptr<cv::aruco::CharucoBoard>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::CharucoBoard>*>))
	}
	
	// getChessboardSize() /usr/include/opencv2/aruco/charuco.hpp:106
	void cv_aruco_CharucoBoard_getChessboardSize_const(const cv::aruco::CharucoBoard* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getChessboardSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// getSquareLength() /usr/include/opencv2/aruco/charuco.hpp:111
	void cv_aruco_CharucoBoard_getSquareLength_const(const cv::aruco::CharucoBoard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSquareLength();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// getMarkerLength() /usr/include/opencv2/aruco/charuco.hpp:116
	void cv_aruco_CharucoBoard_getMarkerLength_const(const cv::aruco::CharucoBoard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMarkerLength();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// adaptiveThreshWinSizeMin /usr/include/opencv2/aruco.hpp:175
	int cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeMin_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeMin;
			return ret;
	}
	
	// adaptiveThreshWinSizeMin /usr/include/opencv2/aruco.hpp:175
	void cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeMin_int(cv::aruco::DetectorParameters* instance, int val) {
			instance->adaptiveThreshWinSizeMin = val;
	}
	
	// adaptiveThreshWinSizeMax /usr/include/opencv2/aruco.hpp:176
	int cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeMax_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeMax;
			return ret;
	}
	
	// adaptiveThreshWinSizeMax /usr/include/opencv2/aruco.hpp:176
	void cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeMax_int(cv::aruco::DetectorParameters* instance, int val) {
			instance->adaptiveThreshWinSizeMax = val;
	}
	
	// adaptiveThreshWinSizeStep /usr/include/opencv2/aruco.hpp:177
	int cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeStep_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeStep;
			return ret;
	}
	
	// adaptiveThreshWinSizeStep /usr/include/opencv2/aruco.hpp:177
	void cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeStep_int(cv::aruco::DetectorParameters* instance, int val) {
			instance->adaptiveThreshWinSizeStep = val;
	}
	
	// adaptiveThreshConstant /usr/include/opencv2/aruco.hpp:178
	double cv_aruco_DetectorParameters_getPropAdaptiveThreshConstant_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->adaptiveThreshConstant;
			return ret;
	}
	
	// adaptiveThreshConstant /usr/include/opencv2/aruco.hpp:178
	void cv_aruco_DetectorParameters_setPropAdaptiveThreshConstant_double(cv::aruco::DetectorParameters* instance, double val) {
			instance->adaptiveThreshConstant = val;
	}
	
	// minMarkerPerimeterRate /usr/include/opencv2/aruco.hpp:179
	double cv_aruco_DetectorParameters_getPropMinMarkerPerimeterRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->minMarkerPerimeterRate;
			return ret;
	}
	
	// minMarkerPerimeterRate /usr/include/opencv2/aruco.hpp:179
	void cv_aruco_DetectorParameters_setPropMinMarkerPerimeterRate_double(cv::aruco::DetectorParameters* instance, double val) {
			instance->minMarkerPerimeterRate = val;
	}
	
	// maxMarkerPerimeterRate /usr/include/opencv2/aruco.hpp:180
	double cv_aruco_DetectorParameters_getPropMaxMarkerPerimeterRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->maxMarkerPerimeterRate;
			return ret;
	}
	
	// maxMarkerPerimeterRate /usr/include/opencv2/aruco.hpp:180
	void cv_aruco_DetectorParameters_setPropMaxMarkerPerimeterRate_double(cv::aruco::DetectorParameters* instance, double val) {
			instance->maxMarkerPerimeterRate = val;
	}
	
	// polygonalApproxAccuracyRate /usr/include/opencv2/aruco.hpp:181
	double cv_aruco_DetectorParameters_getPropPolygonalApproxAccuracyRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->polygonalApproxAccuracyRate;
			return ret;
	}
	
	// polygonalApproxAccuracyRate /usr/include/opencv2/aruco.hpp:181
	void cv_aruco_DetectorParameters_setPropPolygonalApproxAccuracyRate_double(cv::aruco::DetectorParameters* instance, double val) {
			instance->polygonalApproxAccuracyRate = val;
	}
	
	// minCornerDistanceRate /usr/include/opencv2/aruco.hpp:182
	double cv_aruco_DetectorParameters_getPropMinCornerDistanceRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->minCornerDistanceRate;
			return ret;
	}
	
	// minCornerDistanceRate /usr/include/opencv2/aruco.hpp:182
	void cv_aruco_DetectorParameters_setPropMinCornerDistanceRate_double(cv::aruco::DetectorParameters* instance, double val) {
			instance->minCornerDistanceRate = val;
	}
	
	// minDistanceToBorder /usr/include/opencv2/aruco.hpp:183
	int cv_aruco_DetectorParameters_getPropMinDistanceToBorder_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->minDistanceToBorder;
			return ret;
	}
	
	// minDistanceToBorder /usr/include/opencv2/aruco.hpp:183
	void cv_aruco_DetectorParameters_setPropMinDistanceToBorder_int(cv::aruco::DetectorParameters* instance, int val) {
			instance->minDistanceToBorder = val;
	}
	
	// minMarkerDistanceRate /usr/include/opencv2/aruco.hpp:184
	double cv_aruco_DetectorParameters_getPropMinMarkerDistanceRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->minMarkerDistanceRate;
			return ret;
	}
	
	// minMarkerDistanceRate /usr/include/opencv2/aruco.hpp:184
	void cv_aruco_DetectorParameters_setPropMinMarkerDistanceRate_double(cv::aruco::DetectorParameters* instance, double val) {
			instance->minMarkerDistanceRate = val;
	}
	
	// cornerRefinementMethod /usr/include/opencv2/aruco.hpp:185
	int cv_aruco_DetectorParameters_getPropCornerRefinementMethod_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->cornerRefinementMethod;
			return ret;
	}
	
	// cornerRefinementMethod /usr/include/opencv2/aruco.hpp:185
	void cv_aruco_DetectorParameters_setPropCornerRefinementMethod_int(cv::aruco::DetectorParameters* instance, int val) {
			instance->cornerRefinementMethod = val;
	}
	
	// cornerRefinementWinSize /usr/include/opencv2/aruco.hpp:186
	int cv_aruco_DetectorParameters_getPropCornerRefinementWinSize_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->cornerRefinementWinSize;
			return ret;
	}
	
	// cornerRefinementWinSize /usr/include/opencv2/aruco.hpp:186
	void cv_aruco_DetectorParameters_setPropCornerRefinementWinSize_int(cv::aruco::DetectorParameters* instance, int val) {
			instance->cornerRefinementWinSize = val;
	}
	
	// cornerRefinementMaxIterations /usr/include/opencv2/aruco.hpp:187
	int cv_aruco_DetectorParameters_getPropCornerRefinementMaxIterations_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->cornerRefinementMaxIterations;
			return ret;
	}
	
	// cornerRefinementMaxIterations /usr/include/opencv2/aruco.hpp:187
	void cv_aruco_DetectorParameters_setPropCornerRefinementMaxIterations_int(cv::aruco::DetectorParameters* instance, int val) {
			instance->cornerRefinementMaxIterations = val;
	}
	
	// cornerRefinementMinAccuracy /usr/include/opencv2/aruco.hpp:188
	double cv_aruco_DetectorParameters_getPropCornerRefinementMinAccuracy_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->cornerRefinementMinAccuracy;
			return ret;
	}
	
	// cornerRefinementMinAccuracy /usr/include/opencv2/aruco.hpp:188
	void cv_aruco_DetectorParameters_setPropCornerRefinementMinAccuracy_double(cv::aruco::DetectorParameters* instance, double val) {
			instance->cornerRefinementMinAccuracy = val;
	}
	
	// markerBorderBits /usr/include/opencv2/aruco.hpp:189
	int cv_aruco_DetectorParameters_getPropMarkerBorderBits_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->markerBorderBits;
			return ret;
	}
	
	// markerBorderBits /usr/include/opencv2/aruco.hpp:189
	void cv_aruco_DetectorParameters_setPropMarkerBorderBits_int(cv::aruco::DetectorParameters* instance, int val) {
			instance->markerBorderBits = val;
	}
	
	// perspectiveRemovePixelPerCell /usr/include/opencv2/aruco.hpp:190
	int cv_aruco_DetectorParameters_getPropPerspectiveRemovePixelPerCell_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->perspectiveRemovePixelPerCell;
			return ret;
	}
	
	// perspectiveRemovePixelPerCell /usr/include/opencv2/aruco.hpp:190
	void cv_aruco_DetectorParameters_setPropPerspectiveRemovePixelPerCell_int(cv::aruco::DetectorParameters* instance, int val) {
			instance->perspectiveRemovePixelPerCell = val;
	}
	
	// perspectiveRemoveIgnoredMarginPerCell /usr/include/opencv2/aruco.hpp:191
	double cv_aruco_DetectorParameters_getPropPerspectiveRemoveIgnoredMarginPerCell_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->perspectiveRemoveIgnoredMarginPerCell;
			return ret;
	}
	
	// perspectiveRemoveIgnoredMarginPerCell /usr/include/opencv2/aruco.hpp:191
	void cv_aruco_DetectorParameters_setPropPerspectiveRemoveIgnoredMarginPerCell_double(cv::aruco::DetectorParameters* instance, double val) {
			instance->perspectiveRemoveIgnoredMarginPerCell = val;
	}
	
	// maxErroneousBitsInBorderRate /usr/include/opencv2/aruco.hpp:192
	double cv_aruco_DetectorParameters_getPropMaxErroneousBitsInBorderRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->maxErroneousBitsInBorderRate;
			return ret;
	}
	
	// maxErroneousBitsInBorderRate /usr/include/opencv2/aruco.hpp:192
	void cv_aruco_DetectorParameters_setPropMaxErroneousBitsInBorderRate_double(cv::aruco::DetectorParameters* instance, double val) {
			instance->maxErroneousBitsInBorderRate = val;
	}
	
	// minOtsuStdDev /usr/include/opencv2/aruco.hpp:193
	double cv_aruco_DetectorParameters_getPropMinOtsuStdDev_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->minOtsuStdDev;
			return ret;
	}
	
	// minOtsuStdDev /usr/include/opencv2/aruco.hpp:193
	void cv_aruco_DetectorParameters_setPropMinOtsuStdDev_double(cv::aruco::DetectorParameters* instance, double val) {
			instance->minOtsuStdDev = val;
	}
	
	// errorCorrectionRate /usr/include/opencv2/aruco.hpp:194
	double cv_aruco_DetectorParameters_getPropErrorCorrectionRate_const(const cv::aruco::DetectorParameters* instance) {
			double ret = instance->errorCorrectionRate;
			return ret;
	}
	
	// errorCorrectionRate /usr/include/opencv2/aruco.hpp:194
	void cv_aruco_DetectorParameters_setPropErrorCorrectionRate_double(cv::aruco::DetectorParameters* instance, double val) {
			instance->errorCorrectionRate = val;
	}
	
	// aprilTagQuadDecimate /usr/include/opencv2/aruco.hpp:197
	float cv_aruco_DetectorParameters_getPropAprilTagQuadDecimate_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->aprilTagQuadDecimate;
			return ret;
	}
	
	// aprilTagQuadDecimate /usr/include/opencv2/aruco.hpp:197
	void cv_aruco_DetectorParameters_setPropAprilTagQuadDecimate_float(cv::aruco::DetectorParameters* instance, float val) {
			instance->aprilTagQuadDecimate = val;
	}
	
	// aprilTagQuadSigma /usr/include/opencv2/aruco.hpp:198
	float cv_aruco_DetectorParameters_getPropAprilTagQuadSigma_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->aprilTagQuadSigma;
			return ret;
	}
	
	// aprilTagQuadSigma /usr/include/opencv2/aruco.hpp:198
	void cv_aruco_DetectorParameters_setPropAprilTagQuadSigma_float(cv::aruco::DetectorParameters* instance, float val) {
			instance->aprilTagQuadSigma = val;
	}
	
	// aprilTagMinClusterPixels /usr/include/opencv2/aruco.hpp:201
	int cv_aruco_DetectorParameters_getPropAprilTagMinClusterPixels_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->aprilTagMinClusterPixels;
			return ret;
	}
	
	// aprilTagMinClusterPixels /usr/include/opencv2/aruco.hpp:201
	void cv_aruco_DetectorParameters_setPropAprilTagMinClusterPixels_int(cv::aruco::DetectorParameters* instance, int val) {
			instance->aprilTagMinClusterPixels = val;
	}
	
	// aprilTagMaxNmaxima /usr/include/opencv2/aruco.hpp:202
	int cv_aruco_DetectorParameters_getPropAprilTagMaxNmaxima_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->aprilTagMaxNmaxima;
			return ret;
	}
	
	// aprilTagMaxNmaxima /usr/include/opencv2/aruco.hpp:202
	void cv_aruco_DetectorParameters_setPropAprilTagMaxNmaxima_int(cv::aruco::DetectorParameters* instance, int val) {
			instance->aprilTagMaxNmaxima = val;
	}
	
	// aprilTagCriticalRad /usr/include/opencv2/aruco.hpp:203
	float cv_aruco_DetectorParameters_getPropAprilTagCriticalRad_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->aprilTagCriticalRad;
			return ret;
	}
	
	// aprilTagCriticalRad /usr/include/opencv2/aruco.hpp:203
	void cv_aruco_DetectorParameters_setPropAprilTagCriticalRad_float(cv::aruco::DetectorParameters* instance, float val) {
			instance->aprilTagCriticalRad = val;
	}
	
	// aprilTagMaxLineFitMse /usr/include/opencv2/aruco.hpp:204
	float cv_aruco_DetectorParameters_getPropAprilTagMaxLineFitMse_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->aprilTagMaxLineFitMse;
			return ret;
	}
	
	// aprilTagMaxLineFitMse /usr/include/opencv2/aruco.hpp:204
	void cv_aruco_DetectorParameters_setPropAprilTagMaxLineFitMse_float(cv::aruco::DetectorParameters* instance, float val) {
			instance->aprilTagMaxLineFitMse = val;
	}
	
	// aprilTagMinWhiteBlackDiff /usr/include/opencv2/aruco.hpp:205
	int cv_aruco_DetectorParameters_getPropAprilTagMinWhiteBlackDiff_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->aprilTagMinWhiteBlackDiff;
			return ret;
	}
	
	// aprilTagMinWhiteBlackDiff /usr/include/opencv2/aruco.hpp:205
	void cv_aruco_DetectorParameters_setPropAprilTagMinWhiteBlackDiff_int(cv::aruco::DetectorParameters* instance, int val) {
			instance->aprilTagMinWhiteBlackDiff = val;
	}
	
	// aprilTagDeglitch /usr/include/opencv2/aruco.hpp:206
	int cv_aruco_DetectorParameters_getPropAprilTagDeglitch_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->aprilTagDeglitch;
			return ret;
	}
	
	// aprilTagDeglitch /usr/include/opencv2/aruco.hpp:206
	void cv_aruco_DetectorParameters_setPropAprilTagDeglitch_int(cv::aruco::DetectorParameters* instance, int val) {
			instance->aprilTagDeglitch = val;
	}
	
	// detectInvertedMarker /usr/include/opencv2/aruco.hpp:209
	bool cv_aruco_DetectorParameters_getPropDetectInvertedMarker_const(const cv::aruco::DetectorParameters* instance) {
			bool ret = instance->detectInvertedMarker;
			return ret;
	}
	
	// detectInvertedMarker /usr/include/opencv2/aruco.hpp:209
	void cv_aruco_DetectorParameters_setPropDetectInvertedMarker_bool(cv::aruco::DetectorParameters* instance, bool val) {
			instance->detectInvertedMarker = val;
	}
	
	// useAruco3Detection /usr/include/opencv2/aruco.hpp:213
	bool cv_aruco_DetectorParameters_getPropUseAruco3Detection_const(const cv::aruco::DetectorParameters* instance) {
			bool ret = instance->useAruco3Detection;
			return ret;
	}
	
	// useAruco3Detection /usr/include/opencv2/aruco.hpp:213
	void cv_aruco_DetectorParameters_setPropUseAruco3Detection_bool(cv::aruco::DetectorParameters* instance, bool val) {
			instance->useAruco3Detection = val;
	}
	
	// minSideLengthCanonicalImg /usr/include/opencv2/aruco.hpp:214
	int cv_aruco_DetectorParameters_getPropMinSideLengthCanonicalImg_const(const cv::aruco::DetectorParameters* instance) {
			int ret = instance->minSideLengthCanonicalImg;
			return ret;
	}
	
	// minSideLengthCanonicalImg /usr/include/opencv2/aruco.hpp:214
	void cv_aruco_DetectorParameters_setPropMinSideLengthCanonicalImg_int(cv::aruco::DetectorParameters* instance, int val) {
			instance->minSideLengthCanonicalImg = val;
	}
	
	// minMarkerLengthRatioOriginalImg /usr/include/opencv2/aruco.hpp:215
	float cv_aruco_DetectorParameters_getPropMinMarkerLengthRatioOriginalImg_const(const cv::aruco::DetectorParameters* instance) {
			float ret = instance->minMarkerLengthRatioOriginalImg;
			return ret;
	}
	
	// minMarkerLengthRatioOriginalImg /usr/include/opencv2/aruco.hpp:215
	void cv_aruco_DetectorParameters_setPropMinMarkerLengthRatioOriginalImg_float(cv::aruco::DetectorParameters* instance, float val) {
			instance->minMarkerLengthRatioOriginalImg = val;
	}
	
	void cv_DetectorParameters_delete(cv::aruco::DetectorParameters* instance) {
		delete instance;
	}
	// DetectorParameters() /usr/include/opencv2/aruco.hpp:171
	void cv_aruco_DetectorParameters_DetectorParameters(Result<cv::aruco::DetectorParameters*>* ocvrs_return) {
		try {
			cv::aruco::DetectorParameters* ret = new cv::aruco::DetectorParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::aruco::DetectorParameters*>))
	}
	
	// create() /usr/include/opencv2/aruco.hpp:172
	void cv_aruco_DetectorParameters_create(Result<cv::Ptr<cv::aruco::DetectorParameters>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::DetectorParameters> ret = cv::aruco::DetectorParameters::create();
			Ok(new cv::Ptr<cv::aruco::DetectorParameters>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::DetectorParameters>*>))
	}
	
	// readDetectorParameters(const cv::FileNode &) /usr/include/opencv2/aruco.hpp:173
	void cv_aruco_DetectorParameters_readDetectorParameters_const_FileNodeR(cv::aruco::DetectorParameters* instance, const cv::FileNode* fn, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->readDetectorParameters(*fn);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// bytesList /usr/include/opencv2/aruco/dictionary.hpp:64
	cv::Mat* cv_aruco_Dictionary_getPropBytesList_const(const cv::aruco::Dictionary* instance) {
			cv::Mat ret = instance->bytesList;
			return new cv::Mat(ret);
	}
	
	// bytesList /usr/include/opencv2/aruco/dictionary.hpp:64
	void cv_aruco_Dictionary_setPropBytesList_Mat(cv::aruco::Dictionary* instance, cv::Mat* val) {
			instance->bytesList = *val;
	}
	
	// markerSize /usr/include/opencv2/aruco/dictionary.hpp:65
	int cv_aruco_Dictionary_getPropMarkerSize_const(const cv::aruco::Dictionary* instance) {
			int ret = instance->markerSize;
			return ret;
	}
	
	// markerSize /usr/include/opencv2/aruco/dictionary.hpp:65
	void cv_aruco_Dictionary_setPropMarkerSize_int(cv::aruco::Dictionary* instance, int val) {
			instance->markerSize = val;
	}
	
	// maxCorrectionBits /usr/include/opencv2/aruco/dictionary.hpp:66
	int cv_aruco_Dictionary_getPropMaxCorrectionBits_const(const cv::aruco::Dictionary* instance) {
			int ret = instance->maxCorrectionBits;
			return ret;
	}
	
	// maxCorrectionBits /usr/include/opencv2/aruco/dictionary.hpp:66
	void cv_aruco_Dictionary_setPropMaxCorrectionBits_int(cv::aruco::Dictionary* instance, int val) {
			instance->maxCorrectionBits = val;
	}
	
	void cv_Dictionary_delete(cv::aruco::Dictionary* instance) {
		delete instance;
	}
	// Dictionary(const cv::Mat &, int, int) /usr/include/opencv2/aruco/dictionary.hpp:71
	void cv_aruco_Dictionary_Dictionary_const_MatR_int_int(const cv::Mat* _bytesList, int _markerSize, int _maxcorr, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary* ret = new cv::aruco::Dictionary(*_bytesList, _markerSize, _maxcorr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::aruco::Dictionary*>))
	}
	
	// Dictionary(const Ptr<cv::aruco::Dictionary> &) /usr/include/opencv2/aruco/dictionary.hpp:81
	void cv_aruco_Dictionary_Dictionary_const_Ptr_Dictionary_R(const cv::Ptr<cv::aruco::Dictionary>* _dictionary, Result<cv::aruco::Dictionary*>* ocvrs_return) {
		try {
			cv::aruco::Dictionary* ret = new cv::aruco::Dictionary(*_dictionary);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::aruco::Dictionary*>))
	}
	
	// create(int, int, int) /usr/include/opencv2/aruco/dictionary.hpp:87
	void cv_aruco_Dictionary_create_int_int_int(int nMarkers, int markerSize, int randomSeed, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::Dictionary::create(nMarkers, markerSize, randomSeed);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::Dictionary>*>))
	}
	
	// create(int, int, const Ptr<cv::aruco::Dictionary> &, int) /usr/include/opencv2/aruco/dictionary.hpp:93
	void cv_aruco_Dictionary_create_int_int_const_Ptr_Dictionary_R_int(int nMarkers, int markerSize, const cv::Ptr<cv::aruco::Dictionary>* baseDictionary, int randomSeed, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::Dictionary::create(nMarkers, markerSize, *baseDictionary, randomSeed);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::Dictionary>*>))
	}
	
	// readDictionary(const cv::FileNode &) /usr/include/opencv2/aruco/dictionary.hpp:105
	void cv_aruco_Dictionary_readDictionary_const_FileNodeR(cv::aruco::Dictionary* instance, const cv::FileNode* fn, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->readDictionary(*fn);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// writeDictionary(Ptr<cv::FileStorage> &) /usr/include/opencv2/aruco/dictionary.hpp:110
	void cv_aruco_Dictionary_writeDictionary_Ptr_FileStorage_R(cv::aruco::Dictionary* instance, cv::Ptr<cv::FileStorage>* fs, Result_void* ocvrs_return) {
		try {
			instance->writeDictionary(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// get(int) /usr/include/opencv2/aruco/dictionary.hpp:114
	void cv_aruco_Dictionary_get_int(int dict, Result<cv::Ptr<cv::aruco::Dictionary>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::Dictionary> ret = cv::aruco::Dictionary::get(dict);
			Ok(new cv::Ptr<cv::aruco::Dictionary>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::Dictionary>*>))
	}
	
	// identify(const cv::Mat &, int &, int &, double) /usr/include/opencv2/aruco/dictionary.hpp:120
	void cv_aruco_Dictionary_identify_const_const_MatR_intR_intR_double(const cv::aruco::Dictionary* instance, const cv::Mat* onlyBits, int* idx, int* rotation, double maxCorrectionRate, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->identify(*onlyBits, *idx, *rotation, maxCorrectionRate);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// getDistanceToId(cv::InputArray, int, bool) /usr/include/opencv2/aruco/dictionary.hpp:126
	void cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int_bool(const cv::aruco::Dictionary* instance, const cv::_InputArray* bits, int id, bool allRotations, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDistanceToId(*bits, id, allRotations);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// drawMarker(int, int, cv::OutputArray, int) /usr/include/opencv2/aruco/dictionary.hpp:132
	void cv_aruco_Dictionary_drawMarker_const_int_int_const__OutputArrayR_int(const cv::aruco::Dictionary* instance, int id, int sidePixels, const cv::_OutputArray* _img, int borderBits, Result_void* ocvrs_return) {
		try {
			instance->drawMarker(id, sidePixels, *_img, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getByteListFromBits(const cv::Mat &) /usr/include/opencv2/aruco/dictionary.hpp:138
	void cv_aruco_Dictionary_getByteListFromBits_const_MatR(const cv::Mat* bits, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::aruco::Dictionary::getByteListFromBits(*bits);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getBitsFromByteList(const cv::Mat &, int) /usr/include/opencv2/aruco/dictionary.hpp:144
	void cv_aruco_Dictionary_getBitsFromByteList_const_MatR_int(const cv::Mat* byteList, int markerSize, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::aruco::Dictionary::getBitsFromByteList(*byteList, markerSize);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// pattern /usr/include/opencv2/aruco.hpp:285
	void cv_aruco_EstimateParameters_getPropPattern_const(const cv::aruco::EstimateParameters* instance, cv::aruco::PatternPos* ocvrs_return) {
			cv::aruco::PatternPos ret = instance->pattern;
			*ocvrs_return = ret;
	}
	
	// pattern /usr/include/opencv2/aruco.hpp:285
	void cv_aruco_EstimateParameters_setPropPattern_PatternPos(cv::aruco::EstimateParameters* instance, cv::aruco::PatternPos val) {
			instance->pattern = val;
	}
	
	// useExtrinsicGuess /usr/include/opencv2/aruco.hpp:286
	bool cv_aruco_EstimateParameters_getPropUseExtrinsicGuess_const(const cv::aruco::EstimateParameters* instance) {
			bool ret = instance->useExtrinsicGuess;
			return ret;
	}
	
	// useExtrinsicGuess /usr/include/opencv2/aruco.hpp:286
	void cv_aruco_EstimateParameters_setPropUseExtrinsicGuess_bool(cv::aruco::EstimateParameters* instance, bool val) {
			instance->useExtrinsicGuess = val;
	}
	
	// solvePnPMethod /usr/include/opencv2/aruco.hpp:287
	void cv_aruco_EstimateParameters_getPropSolvePnPMethod_const(const cv::aruco::EstimateParameters* instance, cv::SolvePnPMethod* ocvrs_return) {
			cv::SolvePnPMethod ret = instance->solvePnPMethod;
			*ocvrs_return = ret;
	}
	
	// solvePnPMethod /usr/include/opencv2/aruco.hpp:287
	void cv_aruco_EstimateParameters_setPropSolvePnPMethod_SolvePnPMethod(cv::aruco::EstimateParameters* instance, cv::SolvePnPMethod val) {
			instance->solvePnPMethod = val;
	}
	
	void cv_EstimateParameters_delete(cv::aruco::EstimateParameters* instance) {
		delete instance;
	}
	// EstimateParameters() /usr/include/opencv2/aruco.hpp:289
	void cv_aruco_EstimateParameters_EstimateParameters(Result<cv::aruco::EstimateParameters*>* ocvrs_return) {
		try {
			cv::aruco::EstimateParameters* ret = new cv::aruco::EstimateParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::aruco::EstimateParameters*>))
	}
	
	// create() /usr/include/opencv2/aruco.hpp:292
	void cv_aruco_EstimateParameters_create(Result<cv::Ptr<cv::aruco::EstimateParameters>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::EstimateParameters> ret = cv::aruco::EstimateParameters::create();
			Ok(new cv::Ptr<cv::aruco::EstimateParameters>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::EstimateParameters>*>))
	}
	
	cv::aruco::Board* cv_GridBoard_to_Board(cv::aruco::GridBoard* instance) {
		return dynamic_cast<cv::aruco::Board*>(instance);
	}
	
	void cv_GridBoard_delete(cv::aruco::GridBoard* instance) {
		delete instance;
	}
	// draw(cv::Size, cv::OutputArray, int, int) /usr/include/opencv2/aruco.hpp:418
	void cv_aruco_GridBoard_draw_Size_const__OutputArrayR_int_int(cv::aruco::GridBoard* instance, cv::Size* outSize, const cv::_OutputArray* img, int marginSize, int borderBits, Result_void* ocvrs_return) {
		try {
			instance->draw(*outSize, *img, marginSize, borderBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(int, int, float, float, const Ptr<cv::aruco::Dictionary> &, int) /usr/include/opencv2/aruco.hpp:435
	void cv_aruco_GridBoard_create_int_int_float_float_const_Ptr_Dictionary_R_int(int markersX, int markersY, float markerLength, float markerSeparation, const cv::Ptr<cv::aruco::Dictionary>* dictionary, int firstMarker, Result<cv::Ptr<cv::aruco::GridBoard>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::aruco::GridBoard> ret = cv::aruco::GridBoard::create(markersX, markersY, markerLength, markerSeparation, *dictionary, firstMarker);
			Ok(new cv::Ptr<cv::aruco::GridBoard>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::aruco::GridBoard>*>))
	}
	
	// getGridSize() /usr/include/opencv2/aruco.hpp:441
	void cv_aruco_GridBoard_getGridSize_const(const cv::aruco::GridBoard* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getGridSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// getMarkerLength() /usr/include/opencv2/aruco.hpp:446
	void cv_aruco_GridBoard_getMarkerLength_const(const cv::aruco::GridBoard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMarkerLength();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// getMarkerSeparation() /usr/include/opencv2/aruco.hpp:451
	void cv_aruco_GridBoard_getMarkerSeparation_const(const cv::aruco::GridBoard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMarkerSeparation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
}
