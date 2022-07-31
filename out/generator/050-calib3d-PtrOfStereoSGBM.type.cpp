extern "C" {
	void cv_PtrOfStereoSGBM_delete(cv::Ptr<cv::StereoSGBM>* instance) {
		delete instance;
	}

	const cv::StereoSGBM* cv_PtrOfStereoSGBM_get_inner_ptr(const cv::Ptr<cv::StereoSGBM>* instance) {
		return instance->get();
	}

	cv::StereoSGBM* cv_PtrOfStereoSGBM_get_inner_ptr_mut(cv::Ptr<cv::StereoSGBM>* instance) {
		return instance->get();
	}
}

