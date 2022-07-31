extern "C" {
	void cv_PtrOfBackgroundSubtractorMOG2_delete(cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
		delete instance;
	}

	const cv::BackgroundSubtractorMOG2* cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr(const cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
		return instance->get();
	}

	cv::BackgroundSubtractorMOG2* cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr_mut(cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
		return instance->get();
	}
}

