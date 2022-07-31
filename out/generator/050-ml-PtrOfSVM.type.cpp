extern "C" {
	void cv_PtrOfSVM_delete(cv::Ptr<cv::ml::SVM>* instance) {
		delete instance;
	}

	const cv::ml::SVM* cv_PtrOfSVM_get_inner_ptr(const cv::Ptr<cv::ml::SVM>* instance) {
		return instance->get();
	}

	cv::ml::SVM* cv_PtrOfSVM_get_inner_ptr_mut(cv::Ptr<cv::ml::SVM>* instance) {
		return instance->get();
	}
}

