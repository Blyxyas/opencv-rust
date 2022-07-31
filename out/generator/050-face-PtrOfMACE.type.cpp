extern "C" {
	void cv_PtrOfMACE_delete(cv::Ptr<cv::face::MACE>* instance) {
		delete instance;
	}

	const cv::face::MACE* cv_PtrOfMACE_get_inner_ptr(const cv::Ptr<cv::face::MACE>* instance) {
		return instance->get();
	}

	cv::face::MACE* cv_PtrOfMACE_get_inner_ptr_mut(cv::Ptr<cv::face::MACE>* instance) {
		return instance->get();
	}
}

