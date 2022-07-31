extern "C" {
	void cv_PtrOfBackgroundSubtractorCNT_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>* instance) {
		delete instance;
	}

	const cv::bgsegm::BackgroundSubtractorCNT* cv_PtrOfBackgroundSubtractorCNT_get_inner_ptr(const cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>* instance) {
		return instance->get();
	}

	cv::bgsegm::BackgroundSubtractorCNT* cv_PtrOfBackgroundSubtractorCNT_get_inner_ptr_mut(cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>* instance) {
		return instance->get();
	}
}

