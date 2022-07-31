extern "C" {
	void cv_PtrOfSuperpixelSEEDS_delete(cv::Ptr<cv::ximgproc::SuperpixelSEEDS>* instance) {
		delete instance;
	}

	const cv::ximgproc::SuperpixelSEEDS* cv_PtrOfSuperpixelSEEDS_get_inner_ptr(const cv::Ptr<cv::ximgproc::SuperpixelSEEDS>* instance) {
		return instance->get();
	}

	cv::ximgproc::SuperpixelSEEDS* cv_PtrOfSuperpixelSEEDS_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::SuperpixelSEEDS>* instance) {
		return instance->get();
	}
}

