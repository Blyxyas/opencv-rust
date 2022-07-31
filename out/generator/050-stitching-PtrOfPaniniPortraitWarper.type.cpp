extern "C" {
	cv::Ptr<cv::PaniniPortraitWarper>* cv_PtrOfPaniniPortraitWarper_new(cv::PaniniPortraitWarper* val) {
		return new cv::Ptr<cv::PaniniPortraitWarper>(val);
	}
	
	void cv_PtrOfPaniniPortraitWarper_delete(cv::Ptr<cv::PaniniPortraitWarper>* instance) {
		delete instance;
	}

	const cv::PaniniPortraitWarper* cv_PtrOfPaniniPortraitWarper_get_inner_ptr(const cv::Ptr<cv::PaniniPortraitWarper>* instance) {
		return instance->get();
	}

	cv::PaniniPortraitWarper* cv_PtrOfPaniniPortraitWarper_get_inner_ptr_mut(cv::Ptr<cv::PaniniPortraitWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfPaniniPortraitWarper_to_PtrOfWarperCreator(cv::Ptr<cv::PaniniPortraitWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

