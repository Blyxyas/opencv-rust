extern "C" {
	void cv_PtrOfKAZE_delete(cv::Ptr<cv::KAZE>* instance) {
		delete instance;
	}

	const cv::KAZE* cv_PtrOfKAZE_get_inner_ptr(const cv::Ptr<cv::KAZE>* instance) {
		return instance->get();
	}

	cv::KAZE* cv_PtrOfKAZE_get_inner_ptr_mut(cv::Ptr<cv::KAZE>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfKAZE_to_PtrOfFeature2D(cv::Ptr<cv::KAZE>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

