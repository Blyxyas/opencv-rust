extern "C" {
	void cv_PtrOfDTFilter_delete(cv::Ptr<cv::ximgproc::DTFilter>* instance) {
		delete instance;
	}

	const cv::ximgproc::DTFilter* cv_PtrOfDTFilter_get_inner_ptr(const cv::Ptr<cv::ximgproc::DTFilter>* instance) {
		return instance->get();
	}

	cv::ximgproc::DTFilter* cv_PtrOfDTFilter_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::DTFilter>* instance) {
		return instance->get();
	}
}

