extern "C" {
	cv::Ptr<cv::PaniniWarper>* cv_PtrOfPaniniWarper_new(cv::PaniniWarper* val) {
		return new cv::Ptr<cv::PaniniWarper>(val);
	}
	
	void cv_PtrOfPaniniWarper_delete(cv::Ptr<cv::PaniniWarper>* instance) {
		delete instance;
	}

	const cv::PaniniWarper* cv_PtrOfPaniniWarper_get_inner_ptr(const cv::Ptr<cv::PaniniWarper>* instance) {
		return instance->get();
	}

	cv::PaniniWarper* cv_PtrOfPaniniWarper_get_inner_ptr_mut(cv::Ptr<cv::PaniniWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfPaniniWarper_to_PtrOfWarperCreator(cv::Ptr<cv::PaniniWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

