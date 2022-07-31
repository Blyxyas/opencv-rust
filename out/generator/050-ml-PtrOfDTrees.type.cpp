extern "C" {
	void cv_PtrOfDTrees_delete(cv::Ptr<cv::ml::DTrees>* instance) {
		delete instance;
	}

	const cv::ml::DTrees* cv_PtrOfDTrees_get_inner_ptr(const cv::Ptr<cv::ml::DTrees>* instance) {
		return instance->get();
	}

	cv::ml::DTrees* cv_PtrOfDTrees_get_inner_ptr_mut(cv::Ptr<cv::ml::DTrees>* instance) {
		return instance->get();
	}
}

