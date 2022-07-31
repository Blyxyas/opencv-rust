extern "C" {
	void cv_PtrOfRICInterpolator_delete(cv::Ptr<cv::ximgproc::RICInterpolator>* instance) {
		delete instance;
	}

	const cv::ximgproc::RICInterpolator* cv_PtrOfRICInterpolator_get_inner_ptr(const cv::Ptr<cv::ximgproc::RICInterpolator>* instance) {
		return instance->get();
	}

	cv::ximgproc::RICInterpolator* cv_PtrOfRICInterpolator_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::RICInterpolator>* instance) {
		return instance->get();
	}
}

