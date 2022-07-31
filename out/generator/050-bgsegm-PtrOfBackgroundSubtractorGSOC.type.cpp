extern "C" {
	void cv_PtrOfBackgroundSubtractorGSOC_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>* instance) {
		delete instance;
	}

	const cv::bgsegm::BackgroundSubtractorGSOC* cv_PtrOfBackgroundSubtractorGSOC_get_inner_ptr(const cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>* instance) {
		return instance->get();
	}

	cv::bgsegm::BackgroundSubtractorGSOC* cv_PtrOfBackgroundSubtractorGSOC_get_inner_ptr_mut(cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>* instance) {
		return instance->get();
	}
}

