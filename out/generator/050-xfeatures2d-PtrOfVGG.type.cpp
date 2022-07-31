extern "C" {
	void cv_PtrOfVGG_delete(cv::Ptr<cv::xfeatures2d::VGG>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::VGG* cv_PtrOfVGG_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::VGG>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::VGG* cv_PtrOfVGG_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::VGG>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfVGG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::VGG>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

