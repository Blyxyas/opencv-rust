extern "C" {
	void cv_PtrOfLogisticRegression_delete(cv::Ptr<cv::ml::LogisticRegression>* instance) {
		delete instance;
	}

	const cv::ml::LogisticRegression* cv_PtrOfLogisticRegression_get_inner_ptr(const cv::Ptr<cv::ml::LogisticRegression>* instance) {
		return instance->get();
	}

	cv::ml::LogisticRegression* cv_PtrOfLogisticRegression_get_inner_ptr_mut(cv::Ptr<cv::ml::LogisticRegression>* instance) {
		return instance->get();
	}
}

