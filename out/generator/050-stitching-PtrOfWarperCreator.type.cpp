extern "C" {
	void cv_PtrOfWarperCreator_delete(cv::Ptr<cv::WarperCreator>* instance) {
		delete instance;
	}

	const cv::WarperCreator* cv_PtrOfWarperCreator_get_inner_ptr(const cv::Ptr<cv::WarperCreator>* instance) {
		return instance->get();
	}

	cv::WarperCreator* cv_PtrOfWarperCreator_get_inner_ptr_mut(cv::Ptr<cv::WarperCreator>* instance) {
		return instance->get();
	}
}

