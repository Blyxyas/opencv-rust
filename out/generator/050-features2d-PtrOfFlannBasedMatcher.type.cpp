extern "C" {
	cv::Ptr<cv::FlannBasedMatcher>* cv_PtrOfFlannBasedMatcher_new(cv::FlannBasedMatcher* val) {
		return new cv::Ptr<cv::FlannBasedMatcher>(val);
	}
	
	void cv_PtrOfFlannBasedMatcher_delete(cv::Ptr<cv::FlannBasedMatcher>* instance) {
		delete instance;
	}

	const cv::FlannBasedMatcher* cv_PtrOfFlannBasedMatcher_get_inner_ptr(const cv::Ptr<cv::FlannBasedMatcher>* instance) {
		return instance->get();
	}

	cv::FlannBasedMatcher* cv_PtrOfFlannBasedMatcher_get_inner_ptr_mut(cv::Ptr<cv::FlannBasedMatcher>* instance) {
		return instance->get();
	}
}

