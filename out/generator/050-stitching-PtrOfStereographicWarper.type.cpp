extern "C" {
	cv::Ptr<cv::StereographicWarper>* cv_PtrOfStereographicWarper_new(cv::StereographicWarper* val) {
		return new cv::Ptr<cv::StereographicWarper>(val);
	}
	
	void cv_PtrOfStereographicWarper_delete(cv::Ptr<cv::StereographicWarper>* instance) {
		delete instance;
	}

	const cv::StereographicWarper* cv_PtrOfStereographicWarper_get_inner_ptr(const cv::Ptr<cv::StereographicWarper>* instance) {
		return instance->get();
	}

	cv::StereographicWarper* cv_PtrOfStereographicWarper_get_inner_ptr_mut(cv::Ptr<cv::StereographicWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfStereographicWarper_to_PtrOfWarperCreator(cv::Ptr<cv::StereographicWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

