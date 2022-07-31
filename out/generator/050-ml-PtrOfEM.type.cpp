extern "C" {
	void cv_PtrOfEM_delete(cv::Ptr<cv::ml::EM>* instance) {
		delete instance;
	}

	const cv::ml::EM* cv_PtrOfEM_get_inner_ptr(const cv::Ptr<cv::ml::EM>* instance) {
		return instance->get();
	}

	cv::ml::EM* cv_PtrOfEM_get_inner_ptr_mut(cv::Ptr<cv::ml::EM>* instance) {
		return instance->get();
	}
}

