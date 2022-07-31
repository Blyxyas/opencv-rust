extern "C" {
	void cv_PtrOfKNearest_delete(cv::Ptr<cv::ml::KNearest>* instance) {
		delete instance;
	}

	const cv::ml::KNearest* cv_PtrOfKNearest_get_inner_ptr(const cv::Ptr<cv::ml::KNearest>* instance) {
		return instance->get();
	}

	cv::ml::KNearest* cv_PtrOfKNearest_get_inner_ptr_mut(cv::Ptr<cv::ml::KNearest>* instance) {
		return instance->get();
	}
}

