extern "C" {
	void cv_PtrOfAffineFeature_delete(cv::Ptr<cv::AffineFeature>* instance) {
		delete instance;
	}

	const cv::AffineFeature* cv_PtrOfAffineFeature_get_inner_ptr(const cv::Ptr<cv::AffineFeature>* instance) {
		return instance->get();
	}

	cv::AffineFeature* cv_PtrOfAffineFeature_get_inner_ptr_mut(cv::Ptr<cv::AffineFeature>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfAffineFeature_to_PtrOfFeature2D(cv::Ptr<cv::AffineFeature>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

