extern "C" {
	void cv_PtrOfBoostDesc_delete(cv::Ptr<cv::xfeatures2d::BoostDesc>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::BoostDesc* cv_PtrOfBoostDesc_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::BoostDesc>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::BoostDesc* cv_PtrOfBoostDesc_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::BoostDesc>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfBoostDesc_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::BoostDesc>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

