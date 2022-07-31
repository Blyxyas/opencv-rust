extern "C" {
	void cv_PtrOfStereoBM_delete(cv::Ptr<cv::StereoBM>* instance) {
		delete instance;
	}

	const cv::StereoBM* cv_PtrOfStereoBM_get_inner_ptr(const cv::Ptr<cv::StereoBM>* instance) {
		return instance->get();
	}

	cv::StereoBM* cv_PtrOfStereoBM_get_inner_ptr_mut(cv::Ptr<cv::StereoBM>* instance) {
		return instance->get();
	}
}

