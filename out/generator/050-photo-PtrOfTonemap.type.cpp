extern "C" {
	void cv_PtrOfTonemap_delete(cv::Ptr<cv::Tonemap>* instance) {
		delete instance;
	}

	const cv::Tonemap* cv_PtrOfTonemap_get_inner_ptr(const cv::Ptr<cv::Tonemap>* instance) {
		return instance->get();
	}

	cv::Tonemap* cv_PtrOfTonemap_get_inner_ptr_mut(cv::Ptr<cv::Tonemap>* instance) {
		return instance->get();
	}
}

