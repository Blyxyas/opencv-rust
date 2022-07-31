extern "C" {
	void cv_PtrOfBaseCascadeClassifier_delete(cv::Ptr<cv::BaseCascadeClassifier>* instance) {
		delete instance;
	}

	const cv::BaseCascadeClassifier* cv_PtrOfBaseCascadeClassifier_get_inner_ptr(const cv::Ptr<cv::BaseCascadeClassifier>* instance) {
		return instance->get();
	}

	cv::BaseCascadeClassifier* cv_PtrOfBaseCascadeClassifier_get_inner_ptr_mut(cv::Ptr<cv::BaseCascadeClassifier>* instance) {
		return instance->get();
	}
}

