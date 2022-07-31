extern "C" {
	void cv_PtrOfBackgroundSubtractorLSBP_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>* instance) {
		delete instance;
	}

	const cv::bgsegm::BackgroundSubtractorLSBP* cv_PtrOfBackgroundSubtractorLSBP_get_inner_ptr(const cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>* instance) {
		return instance->get();
	}

	cv::bgsegm::BackgroundSubtractorLSBP* cv_PtrOfBackgroundSubtractorLSBP_get_inner_ptr_mut(cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>* instance) {
		return instance->get();
	}
}

