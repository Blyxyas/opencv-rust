extern "C" {
	void cv_PtrOfMergeDebevec_delete(cv::Ptr<cv::MergeDebevec>* instance) {
		delete instance;
	}

	const cv::MergeDebevec* cv_PtrOfMergeDebevec_get_inner_ptr(const cv::Ptr<cv::MergeDebevec>* instance) {
		return instance->get();
	}

	cv::MergeDebevec* cv_PtrOfMergeDebevec_get_inner_ptr_mut(cv::Ptr<cv::MergeDebevec>* instance) {
		return instance->get();
	}
}

