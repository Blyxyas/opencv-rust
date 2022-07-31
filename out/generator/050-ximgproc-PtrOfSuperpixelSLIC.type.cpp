extern "C" {
	void cv_PtrOfSuperpixelSLIC_delete(cv::Ptr<cv::ximgproc::SuperpixelSLIC>* instance) {
		delete instance;
	}

	const cv::ximgproc::SuperpixelSLIC* cv_PtrOfSuperpixelSLIC_get_inner_ptr(const cv::Ptr<cv::ximgproc::SuperpixelSLIC>* instance) {
		return instance->get();
	}

	cv::ximgproc::SuperpixelSLIC* cv_PtrOfSuperpixelSLIC_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::SuperpixelSLIC>* instance) {
		return instance->get();
	}
}

