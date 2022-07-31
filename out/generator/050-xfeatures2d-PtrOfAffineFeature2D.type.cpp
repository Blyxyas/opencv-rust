extern "C" {
	void cv_PtrOfAffineFeature2D_delete(cv::Ptr<cv::xfeatures2d::AffineFeature2D>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::AffineFeature2D* cv_PtrOfAffineFeature2D_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::AffineFeature2D>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::AffineFeature2D* cv_PtrOfAffineFeature2D_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::AffineFeature2D>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfAffineFeature2D_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::AffineFeature2D>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

