extern "C" {
	void cv_PtrOfFormatted_delete(cv::Ptr<cv::Formatted>* instance) {
		delete instance;
	}

	const cv::Formatted* cv_PtrOfFormatted_get_inner_ptr(const cv::Ptr<cv::Formatted>* instance) {
		return instance->get();
	}

	cv::Formatted* cv_PtrOfFormatted_get_inner_ptr_mut(cv::Ptr<cv::Formatted>* instance) {
		return instance->get();
	}
}

