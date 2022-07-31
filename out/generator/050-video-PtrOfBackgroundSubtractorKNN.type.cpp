extern "C" {
	void cv_PtrOfBackgroundSubtractorKNN_delete(cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
		delete instance;
	}

	const cv::BackgroundSubtractorKNN* cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr(const cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
		return instance->get();
	}

	cv::BackgroundSubtractorKNN* cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr_mut(cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
		return instance->get();
	}
}

