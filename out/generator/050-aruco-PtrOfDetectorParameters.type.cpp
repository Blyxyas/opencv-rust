extern "C" {
	cv::Ptr<cv::aruco::DetectorParameters>* cv_PtrOfDetectorParameters_new(cv::aruco::DetectorParameters* val) {
		return new cv::Ptr<cv::aruco::DetectorParameters>(val);
	}
	
	void cv_PtrOfDetectorParameters_delete(cv::Ptr<cv::aruco::DetectorParameters>* instance) {
		delete instance;
	}

	const cv::aruco::DetectorParameters* cv_PtrOfDetectorParameters_get_inner_ptr(const cv::Ptr<cv::aruco::DetectorParameters>* instance) {
		return instance->get();
	}

	cv::aruco::DetectorParameters* cv_PtrOfDetectorParameters_get_inner_ptr_mut(cv::Ptr<cv::aruco::DetectorParameters>* instance) {
		return instance->get();
	}
}

