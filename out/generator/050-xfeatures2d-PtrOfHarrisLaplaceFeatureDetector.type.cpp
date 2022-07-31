extern "C" {
	cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>* cv_PtrOfHarrisLaplaceFeatureDetector_new(cv::xfeatures2d::HarrisLaplaceFeatureDetector* val) {
		return new cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>(val);
	}
	
	void cv_PtrOfHarrisLaplaceFeatureDetector_delete(cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::HarrisLaplaceFeatureDetector* cv_PtrOfHarrisLaplaceFeatureDetector_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::HarrisLaplaceFeatureDetector* cv_PtrOfHarrisLaplaceFeatureDetector_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfHarrisLaplaceFeatureDetector_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

