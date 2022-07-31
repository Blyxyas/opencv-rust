extern "C" {
	void cv_PtrOfRFFeatureGetter_delete(cv::Ptr<cv::ximgproc::RFFeatureGetter>* instance) {
		delete instance;
	}

	const cv::ximgproc::RFFeatureGetter* cv_PtrOfRFFeatureGetter_get_inner_ptr(const cv::Ptr<cv::ximgproc::RFFeatureGetter>* instance) {
		return instance->get();
	}

	cv::ximgproc::RFFeatureGetter* cv_PtrOfRFFeatureGetter_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::RFFeatureGetter>* instance) {
		return instance->get();
	}
}

