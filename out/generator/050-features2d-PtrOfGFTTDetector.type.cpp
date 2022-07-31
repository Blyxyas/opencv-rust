extern "C" {
	void cv_PtrOfGFTTDetector_delete(cv::Ptr<cv::GFTTDetector>* instance) {
		delete instance;
	}

	const cv::GFTTDetector* cv_PtrOfGFTTDetector_get_inner_ptr(const cv::Ptr<cv::GFTTDetector>* instance) {
		return instance->get();
	}

	cv::GFTTDetector* cv_PtrOfGFTTDetector_get_inner_ptr_mut(cv::Ptr<cv::GFTTDetector>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfGFTTDetector_to_PtrOfFeature2D(cv::Ptr<cv::GFTTDetector>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

