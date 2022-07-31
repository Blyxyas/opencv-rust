extern "C" {
	void cv_PtrOfDAISY_delete(cv::Ptr<cv::xfeatures2d::DAISY>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::DAISY* cv_PtrOfDAISY_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::DAISY>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::DAISY* cv_PtrOfDAISY_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::DAISY>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfDAISY_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::DAISY>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

