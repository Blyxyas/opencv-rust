extern "C" {
	void cv_PtrOfSURF_delete(cv::Ptr<cv::xfeatures2d::SURF>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::SURF* cv_PtrOfSURF_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::SURF>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::SURF* cv_PtrOfSURF_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::SURF>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfSURF_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::SURF>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

