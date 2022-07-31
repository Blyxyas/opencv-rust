extern "C" {
	void cv_PtrOfOLSTracker_delete(cv::Ptr<cv::rapid::OLSTracker>* instance) {
		delete instance;
	}

	const cv::rapid::OLSTracker* cv_PtrOfOLSTracker_get_inner_ptr(const cv::Ptr<cv::rapid::OLSTracker>* instance) {
		return instance->get();
	}

	cv::rapid::OLSTracker* cv_PtrOfOLSTracker_get_inner_ptr_mut(cv::Ptr<cv::rapid::OLSTracker>* instance) {
		return instance->get();
	}
}

