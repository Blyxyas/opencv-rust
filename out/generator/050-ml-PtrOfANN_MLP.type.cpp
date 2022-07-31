extern "C" {
	void cv_PtrOfANN_MLP_delete(cv::Ptr<cv::ml::ANN_MLP>* instance) {
		delete instance;
	}

	const cv::ml::ANN_MLP* cv_PtrOfANN_MLP_get_inner_ptr(const cv::Ptr<cv::ml::ANN_MLP>* instance) {
		return instance->get();
	}

	cv::ml::ANN_MLP* cv_PtrOfANN_MLP_get_inner_ptr_mut(cv::Ptr<cv::ml::ANN_MLP>* instance) {
		return instance->get();
	}
}

