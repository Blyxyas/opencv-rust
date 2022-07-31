extern "C" {
	void cv_PtrOfStereoMatcher_delete(cv::Ptr<cv::StereoMatcher>* instance) {
		delete instance;
	}

	const cv::StereoMatcher* cv_PtrOfStereoMatcher_get_inner_ptr(const cv::Ptr<cv::StereoMatcher>* instance) {
		return instance->get();
	}

	cv::StereoMatcher* cv_PtrOfStereoMatcher_get_inner_ptr_mut(cv::Ptr<cv::StereoMatcher>* instance) {
		return instance->get();
	}
}

