extern "C" {
	cv::Ptr<cv::xfeatures2d::StarDetector>* cv_PtrOfStarDetector_new(cv::xfeatures2d::StarDetector* val) {
		return new cv::Ptr<cv::xfeatures2d::StarDetector>(val);
	}
	
	void cv_PtrOfStarDetector_delete(cv::Ptr<cv::xfeatures2d::StarDetector>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::StarDetector* cv_PtrOfStarDetector_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::StarDetector>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::StarDetector* cv_PtrOfStarDetector_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::StarDetector>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfStarDetector_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::StarDetector>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

