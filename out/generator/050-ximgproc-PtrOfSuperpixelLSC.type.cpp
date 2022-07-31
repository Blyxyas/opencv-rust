extern "C" {
	void cv_PtrOfSuperpixelLSC_delete(cv::Ptr<cv::ximgproc::SuperpixelLSC>* instance) {
		delete instance;
	}

	const cv::ximgproc::SuperpixelLSC* cv_PtrOfSuperpixelLSC_get_inner_ptr(const cv::Ptr<cv::ximgproc::SuperpixelLSC>* instance) {
		return instance->get();
	}

	cv::ximgproc::SuperpixelLSC* cv_PtrOfSuperpixelLSC_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::SuperpixelLSC>* instance) {
		return instance->get();
	}
}

