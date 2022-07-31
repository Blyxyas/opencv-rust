extern "C" {
	void cv_PtrOfBIF_delete(cv::Ptr<cv::face::BIF>* instance) {
		delete instance;
	}

	const cv::face::BIF* cv_PtrOfBIF_get_inner_ptr(const cv::Ptr<cv::face::BIF>* instance) {
		return instance->get();
	}

	cv::face::BIF* cv_PtrOfBIF_get_inner_ptr_mut(cv::Ptr<cv::face::BIF>* instance) {
		return instance->get();
	}
}

