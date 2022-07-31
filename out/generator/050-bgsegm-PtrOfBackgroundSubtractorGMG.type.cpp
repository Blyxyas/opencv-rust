extern "C" {
	void cv_PtrOfBackgroundSubtractorGMG_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
		delete instance;
	}

	const cv::bgsegm::BackgroundSubtractorGMG* cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr(const cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
		return instance->get();
	}

	cv::bgsegm::BackgroundSubtractorGMG* cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr_mut(cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
		return instance->get();
	}
}

