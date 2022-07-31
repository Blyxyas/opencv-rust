extern "C" {
	void cv_PtrOfDisparityWLSFilter_delete(cv::Ptr<cv::ximgproc::DisparityWLSFilter>* instance) {
		delete instance;
	}

	const cv::ximgproc::DisparityWLSFilter* cv_PtrOfDisparityWLSFilter_get_inner_ptr(const cv::Ptr<cv::ximgproc::DisparityWLSFilter>* instance) {
		return instance->get();
	}

	cv::ximgproc::DisparityWLSFilter* cv_PtrOfDisparityWLSFilter_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::DisparityWLSFilter>* instance) {
		return instance->get();
	}
}

