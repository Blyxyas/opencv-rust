extern "C" {
	cv::Ptr<cv::ximgproc::ContourFitting>* cv_PtrOfContourFitting_new(cv::ximgproc::ContourFitting* val) {
		return new cv::Ptr<cv::ximgproc::ContourFitting>(val);
	}
	
	void cv_PtrOfContourFitting_delete(cv::Ptr<cv::ximgproc::ContourFitting>* instance) {
		delete instance;
	}

	const cv::ximgproc::ContourFitting* cv_PtrOfContourFitting_get_inner_ptr(const cv::Ptr<cv::ximgproc::ContourFitting>* instance) {
		return instance->get();
	}

	cv::ximgproc::ContourFitting* cv_PtrOfContourFitting_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::ContourFitting>* instance) {
		return instance->get();
	}
}

