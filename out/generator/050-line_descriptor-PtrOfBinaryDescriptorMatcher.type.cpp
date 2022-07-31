extern "C" {
	cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>* cv_PtrOfBinaryDescriptorMatcher_new(cv::line_descriptor::BinaryDescriptorMatcher* val) {
		return new cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>(val);
	}
	
	void cv_PtrOfBinaryDescriptorMatcher_delete(cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>* instance) {
		delete instance;
	}

	const cv::line_descriptor::BinaryDescriptorMatcher* cv_PtrOfBinaryDescriptorMatcher_get_inner_ptr(const cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>* instance) {
		return instance->get();
	}

	cv::line_descriptor::BinaryDescriptorMatcher* cv_PtrOfBinaryDescriptorMatcher_get_inner_ptr_mut(cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>* instance) {
		return instance->get();
	}
}

