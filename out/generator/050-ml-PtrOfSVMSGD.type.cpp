extern "C" {
	void cv_PtrOfSVMSGD_delete(cv::Ptr<cv::ml::SVMSGD>* instance) {
		delete instance;
	}

	const cv::ml::SVMSGD* cv_PtrOfSVMSGD_get_inner_ptr(const cv::Ptr<cv::ml::SVMSGD>* instance) {
		return instance->get();
	}

	cv::ml::SVMSGD* cv_PtrOfSVMSGD_get_inner_ptr_mut(cv::Ptr<cv::ml::SVMSGD>* instance) {
		return instance->get();
	}
}

