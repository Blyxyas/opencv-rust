extern "C" {
	void cv_PtrOfBackgroundSubtractorMOG_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
		delete instance;
	}

	const cv::bgsegm::BackgroundSubtractorMOG* cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr(const cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
		return instance->get();
	}

	cv::bgsegm::BackgroundSubtractorMOG* cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr_mut(cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
		return instance->get();
	}
}

