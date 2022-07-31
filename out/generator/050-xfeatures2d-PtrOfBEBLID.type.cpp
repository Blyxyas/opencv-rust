extern "C" {
	cv::Ptr<cv::xfeatures2d::BEBLID>* cv_PtrOfBEBLID_new(cv::xfeatures2d::BEBLID* val) {
		return new cv::Ptr<cv::xfeatures2d::BEBLID>(val);
	}
	
	void cv_PtrOfBEBLID_delete(cv::Ptr<cv::xfeatures2d::BEBLID>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::BEBLID* cv_PtrOfBEBLID_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::BEBLID>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::BEBLID* cv_PtrOfBEBLID_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::BEBLID>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfBEBLID_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::BEBLID>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

