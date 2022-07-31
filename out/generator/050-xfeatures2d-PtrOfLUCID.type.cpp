extern "C" {
	cv::Ptr<cv::xfeatures2d::LUCID>* cv_PtrOfLUCID_new(cv::xfeatures2d::LUCID* val) {
		return new cv::Ptr<cv::xfeatures2d::LUCID>(val);
	}
	
	void cv_PtrOfLUCID_delete(cv::Ptr<cv::xfeatures2d::LUCID>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::LUCID* cv_PtrOfLUCID_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::LUCID>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::LUCID* cv_PtrOfLUCID_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::LUCID>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfLUCID_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::LUCID>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

