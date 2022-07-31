extern "C" {
	void cv_PtrOfAKAZE_delete(cv::Ptr<cv::AKAZE>* instance) {
		delete instance;
	}

	const cv::AKAZE* cv_PtrOfAKAZE_get_inner_ptr(const cv::Ptr<cv::AKAZE>* instance) {
		return instance->get();
	}

	cv::AKAZE* cv_PtrOfAKAZE_get_inner_ptr_mut(cv::Ptr<cv::AKAZE>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfAKAZE_to_PtrOfFeature2D(cv::Ptr<cv::AKAZE>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

