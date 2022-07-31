extern "C" {
	cv::Ptr<cv::xfeatures2d::MSDDetector>* cv_PtrOfMSDDetector_new(cv::xfeatures2d::MSDDetector* val) {
		return new cv::Ptr<cv::xfeatures2d::MSDDetector>(val);
	}
	
	void cv_PtrOfMSDDetector_delete(cv::Ptr<cv::xfeatures2d::MSDDetector>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::MSDDetector* cv_PtrOfMSDDetector_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::MSDDetector>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::MSDDetector* cv_PtrOfMSDDetector_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::MSDDetector>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfMSDDetector_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::MSDDetector>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

