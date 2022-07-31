extern "C" {
	void cv_PtrOfFastGlobalSmootherFilter_delete(cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>* instance) {
		delete instance;
	}

	const cv::ximgproc::FastGlobalSmootherFilter* cv_PtrOfFastGlobalSmootherFilter_get_inner_ptr(const cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>* instance) {
		return instance->get();
	}

	cv::ximgproc::FastGlobalSmootherFilter* cv_PtrOfFastGlobalSmootherFilter_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>* instance) {
		return instance->get();
	}
}

