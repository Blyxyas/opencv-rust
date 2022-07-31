extern "C" {
	void cv_PtrOfNormalBayesClassifier_delete(cv::Ptr<cv::ml::NormalBayesClassifier>* instance) {
		delete instance;
	}

	const cv::ml::NormalBayesClassifier* cv_PtrOfNormalBayesClassifier_get_inner_ptr(const cv::Ptr<cv::ml::NormalBayesClassifier>* instance) {
		return instance->get();
	}

	cv::ml::NormalBayesClassifier* cv_PtrOfNormalBayesClassifier_get_inner_ptr_mut(cv::Ptr<cv::ml::NormalBayesClassifier>* instance) {
		return instance->get();
	}
}

