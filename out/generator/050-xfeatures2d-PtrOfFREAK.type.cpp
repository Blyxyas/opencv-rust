extern "C" {
	cv::Ptr<cv::xfeatures2d::FREAK>* cv_PtrOfFREAK_new(cv::xfeatures2d::FREAK* val) {
		return new cv::Ptr<cv::xfeatures2d::FREAK>(val);
	}
	
	void cv_PtrOfFREAK_delete(cv::Ptr<cv::xfeatures2d::FREAK>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::FREAK* cv_PtrOfFREAK_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::FREAK>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::FREAK* cv_PtrOfFREAK_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::FREAK>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfFREAK_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::FREAK>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

