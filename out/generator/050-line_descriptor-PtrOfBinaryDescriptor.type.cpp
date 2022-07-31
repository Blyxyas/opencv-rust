extern "C" {
	cv::Ptr<cv::line_descriptor::BinaryDescriptor>* cv_PtrOfBinaryDescriptor_new(cv::line_descriptor::BinaryDescriptor* val) {
		return new cv::Ptr<cv::line_descriptor::BinaryDescriptor>(val);
	}
	
	void cv_PtrOfBinaryDescriptor_delete(cv::Ptr<cv::line_descriptor::BinaryDescriptor>* instance) {
		delete instance;
	}

	const cv::line_descriptor::BinaryDescriptor* cv_PtrOfBinaryDescriptor_get_inner_ptr(const cv::Ptr<cv::line_descriptor::BinaryDescriptor>* instance) {
		return instance->get();
	}

	cv::line_descriptor::BinaryDescriptor* cv_PtrOfBinaryDescriptor_get_inner_ptr_mut(cv::Ptr<cv::line_descriptor::BinaryDescriptor>* instance) {
		return instance->get();
	}
}

