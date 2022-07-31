extern "C" {
	void cv_PtrOfSVM_Kernel_delete(cv::Ptr<cv::ml::SVM::Kernel>* instance) {
		delete instance;
	}

	const cv::ml::SVM::Kernel* cv_PtrOfSVM_Kernel_get_inner_ptr(const cv::Ptr<cv::ml::SVM::Kernel>* instance) {
		return instance->get();
	}

	cv::ml::SVM::Kernel* cv_PtrOfSVM_Kernel_get_inner_ptr_mut(cv::Ptr<cv::ml::SVM::Kernel>* instance) {
		return instance->get();
	}
}

