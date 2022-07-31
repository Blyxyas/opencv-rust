extern "C" {
	void cv_PtrOfTBMR_delete(cv::Ptr<cv::xfeatures2d::TBMR>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::TBMR* cv_PtrOfTBMR_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::TBMR>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::TBMR* cv_PtrOfTBMR_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::TBMR>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfTBMR_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::TBMR>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

