extern "C" {
	void cv_PtrOfAdaptiveManifoldFilter_delete(cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>* instance) {
		delete instance;
	}

	const cv::ximgproc::AdaptiveManifoldFilter* cv_PtrOfAdaptiveManifoldFilter_get_inner_ptr(const cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>* instance) {
		return instance->get();
	}

	cv::ximgproc::AdaptiveManifoldFilter* cv_PtrOfAdaptiveManifoldFilter_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>* instance) {
		return instance->get();
	}
}

