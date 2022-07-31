extern "C" {
	void cv_PtrOfDescriptorMatcher_delete(cv::Ptr<cv::DescriptorMatcher>* instance) {
		delete instance;
	}

	const cv::DescriptorMatcher* cv_PtrOfDescriptorMatcher_get_inner_ptr(const cv::Ptr<cv::DescriptorMatcher>* instance) {
		return instance->get();
	}

	cv::DescriptorMatcher* cv_PtrOfDescriptorMatcher_get_inner_ptr_mut(cv::Ptr<cv::DescriptorMatcher>* instance) {
		return instance->get();
	}
}

