extern "C" {
	cv::Ptr<cv::BFMatcher>* cv_PtrOfBFMatcher_new(cv::BFMatcher* val) {
		return new cv::Ptr<cv::BFMatcher>(val);
	}
	
	void cv_PtrOfBFMatcher_delete(cv::Ptr<cv::BFMatcher>* instance) {
		delete instance;
	}

	const cv::BFMatcher* cv_PtrOfBFMatcher_get_inner_ptr(const cv::Ptr<cv::BFMatcher>* instance) {
		return instance->get();
	}

	cv::BFMatcher* cv_PtrOfBFMatcher_get_inner_ptr_mut(cv::Ptr<cv::BFMatcher>* instance) {
		return instance->get();
	}
}

