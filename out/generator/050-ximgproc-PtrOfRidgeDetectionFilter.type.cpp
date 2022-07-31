extern "C" {
	void cv_PtrOfRidgeDetectionFilter_delete(cv::Ptr<cv::ximgproc::RidgeDetectionFilter>* instance) {
		delete instance;
	}

	const cv::ximgproc::RidgeDetectionFilter* cv_PtrOfRidgeDetectionFilter_get_inner_ptr(const cv::Ptr<cv::ximgproc::RidgeDetectionFilter>* instance) {
		return instance->get();
	}

	cv::ximgproc::RidgeDetectionFilter* cv_PtrOfRidgeDetectionFilter_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::RidgeDetectionFilter>* instance) {
		return instance->get();
	}
}

