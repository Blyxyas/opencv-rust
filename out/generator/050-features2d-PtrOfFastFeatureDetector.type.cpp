extern "C" {
	void cv_PtrOfFastFeatureDetector_delete(cv::Ptr<cv::FastFeatureDetector>* instance) {
		delete instance;
	}

	const cv::FastFeatureDetector* cv_PtrOfFastFeatureDetector_get_inner_ptr(const cv::Ptr<cv::FastFeatureDetector>* instance) {
		return instance->get();
	}

	cv::FastFeatureDetector* cv_PtrOfFastFeatureDetector_get_inner_ptr_mut(cv::Ptr<cv::FastFeatureDetector>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfFastFeatureDetector_to_PtrOfFeature2D(cv::Ptr<cv::FastFeatureDetector>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

