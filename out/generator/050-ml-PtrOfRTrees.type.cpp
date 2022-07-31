extern "C" {
	void cv_PtrOfRTrees_delete(cv::Ptr<cv::ml::RTrees>* instance) {
		delete instance;
	}

	const cv::ml::RTrees* cv_PtrOfRTrees_get_inner_ptr(const cv::Ptr<cv::ml::RTrees>* instance) {
		return instance->get();
	}

	cv::ml::RTrees* cv_PtrOfRTrees_get_inner_ptr_mut(cv::Ptr<cv::ml::RTrees>* instance) {
		return instance->get();
	}
}

