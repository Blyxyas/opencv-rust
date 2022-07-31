extern "C" {
	cv::Ptr<cv::xfeatures2d::LATCH>* cv_PtrOfLATCH_new(cv::xfeatures2d::LATCH* val) {
		return new cv::Ptr<cv::xfeatures2d::LATCH>(val);
	}
	
	void cv_PtrOfLATCH_delete(cv::Ptr<cv::xfeatures2d::LATCH>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::LATCH* cv_PtrOfLATCH_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::LATCH>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::LATCH* cv_PtrOfLATCH_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::LATCH>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfLATCH_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::LATCH>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

