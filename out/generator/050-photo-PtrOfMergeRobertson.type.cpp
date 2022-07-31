extern "C" {
	void cv_PtrOfMergeRobertson_delete(cv::Ptr<cv::MergeRobertson>* instance) {
		delete instance;
	}

	const cv::MergeRobertson* cv_PtrOfMergeRobertson_get_inner_ptr(const cv::Ptr<cv::MergeRobertson>* instance) {
		return instance->get();
	}

	cv::MergeRobertson* cv_PtrOfMergeRobertson_get_inner_ptr_mut(cv::Ptr<cv::MergeRobertson>* instance) {
		return instance->get();
	}
}

