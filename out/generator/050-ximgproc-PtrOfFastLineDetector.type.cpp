extern "C" {
	void cv_PtrOfFastLineDetector_delete(cv::Ptr<cv::ximgproc::FastLineDetector>* instance) {
		delete instance;
	}

	const cv::ximgproc::FastLineDetector* cv_PtrOfFastLineDetector_get_inner_ptr(const cv::Ptr<cv::ximgproc::FastLineDetector>* instance) {
		return instance->get();
	}

	cv::ximgproc::FastLineDetector* cv_PtrOfFastLineDetector_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::FastLineDetector>* instance) {
		return instance->get();
	}
}

