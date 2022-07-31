extern "C" {
	void cv_PtrOfAgastFeatureDetector_delete(cv::Ptr<cv::AgastFeatureDetector>* instance) {
		delete instance;
	}

	const cv::AgastFeatureDetector* cv_PtrOfAgastFeatureDetector_get_inner_ptr(const cv::Ptr<cv::AgastFeatureDetector>* instance) {
		return instance->get();
	}

	cv::AgastFeatureDetector* cv_PtrOfAgastFeatureDetector_get_inner_ptr_mut(cv::Ptr<cv::AgastFeatureDetector>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfAgastFeatureDetector_to_PtrOfFeature2D(cv::Ptr<cv::AgastFeatureDetector>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

