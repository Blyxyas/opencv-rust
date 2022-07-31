extern "C" {
	cv::Ptr<cv::utils::nested::OriginalClassName>* cv_PtrOfOriginalClassName_new(cv::utils::nested::OriginalClassName* val) {
		return new cv::Ptr<cv::utils::nested::OriginalClassName>(val);
	}
	
	void cv_PtrOfOriginalClassName_delete(cv::Ptr<cv::utils::nested::OriginalClassName>* instance) {
		delete instance;
	}

	const cv::utils::nested::OriginalClassName* cv_PtrOfOriginalClassName_get_inner_ptr(const cv::Ptr<cv::utils::nested::OriginalClassName>* instance) {
		return instance->get();
	}

	cv::utils::nested::OriginalClassName* cv_PtrOfOriginalClassName_get_inner_ptr_mut(cv::Ptr<cv::utils::nested::OriginalClassName>* instance) {
		return instance->get();
	}
}

