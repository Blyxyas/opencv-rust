extern "C" {
	void cv_PtrOfMergeMertens_delete(cv::Ptr<cv::MergeMertens>* instance) {
		delete instance;
	}

	const cv::MergeMertens* cv_PtrOfMergeMertens_get_inner_ptr(const cv::Ptr<cv::MergeMertens>* instance) {
		return instance->get();
	}

	cv::MergeMertens* cv_PtrOfMergeMertens_get_inner_ptr_mut(cv::Ptr<cv::MergeMertens>* instance) {
		return instance->get();
	}
}

