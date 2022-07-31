extern "C" {
	void cv_PtrOfBoost_delete(cv::Ptr<cv::ml::Boost>* instance) {
		delete instance;
	}

	const cv::ml::Boost* cv_PtrOfBoost_get_inner_ptr(const cv::Ptr<cv::ml::Boost>* instance) {
		return instance->get();
	}

	cv::ml::Boost* cv_PtrOfBoost_get_inner_ptr_mut(cv::Ptr<cv::ml::Boost>* instance) {
		return instance->get();
	}
}

