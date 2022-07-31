extern "C" {
	void cv_PtrOfFormatter_delete(cv::Ptr<cv::Formatter>* instance) {
		delete instance;
	}

	const cv::Formatter* cv_PtrOfFormatter_get_inner_ptr(const cv::Ptr<cv::Formatter>* instance) {
		return instance->get();
	}

	cv::Formatter* cv_PtrOfFormatter_get_inner_ptr_mut(cv::Ptr<cv::Formatter>* instance) {
		return instance->get();
	}
}

