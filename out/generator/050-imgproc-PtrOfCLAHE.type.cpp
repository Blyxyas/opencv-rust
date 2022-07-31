extern "C" {
	void cv_PtrOfCLAHE_delete(cv::Ptr<cv::CLAHE>* instance) {
		delete instance;
	}

	const cv::CLAHE* cv_PtrOfCLAHE_get_inner_ptr(const cv::Ptr<cv::CLAHE>* instance) {
		return instance->get();
	}

	cv::CLAHE* cv_PtrOfCLAHE_get_inner_ptr_mut(cv::Ptr<cv::CLAHE>* instance) {
		return instance->get();
	}
}

