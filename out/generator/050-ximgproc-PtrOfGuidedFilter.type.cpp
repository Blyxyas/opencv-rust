extern "C" {
	void cv_PtrOfGuidedFilter_delete(cv::Ptr<cv::ximgproc::GuidedFilter>* instance) {
		delete instance;
	}

	const cv::ximgproc::GuidedFilter* cv_PtrOfGuidedFilter_get_inner_ptr(const cv::Ptr<cv::ximgproc::GuidedFilter>* instance) {
		return instance->get();
	}

	cv::ximgproc::GuidedFilter* cv_PtrOfGuidedFilter_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::GuidedFilter>* instance) {
		return instance->get();
	}
}

