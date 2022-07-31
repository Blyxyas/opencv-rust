extern "C" {
	cv::Ptr<cv::aruco::Dictionary>* cv_PtrOfDictionary_new(cv::aruco::Dictionary* val) {
		return new cv::Ptr<cv::aruco::Dictionary>(val);
	}
	
	void cv_PtrOfDictionary_delete(cv::Ptr<cv::aruco::Dictionary>* instance) {
		delete instance;
	}

	const cv::aruco::Dictionary* cv_PtrOfDictionary_get_inner_ptr(const cv::Ptr<cv::aruco::Dictionary>* instance) {
		return instance->get();
	}

	cv::aruco::Dictionary* cv_PtrOfDictionary_get_inner_ptr_mut(cv::Ptr<cv::aruco::Dictionary>* instance) {
		return instance->get();
	}
}

