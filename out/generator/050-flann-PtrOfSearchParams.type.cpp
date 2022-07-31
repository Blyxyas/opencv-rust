extern "C" {
	cv::Ptr<cv::flann::SearchParams>* cv_PtrOfSearchParams_new(cv::flann::SearchParams* val) {
		return new cv::Ptr<cv::flann::SearchParams>(val);
	}
	
	void cv_PtrOfSearchParams_delete(cv::Ptr<cv::flann::SearchParams>* instance) {
		delete instance;
	}

	const cv::flann::SearchParams* cv_PtrOfSearchParams_get_inner_ptr(const cv::Ptr<cv::flann::SearchParams>* instance) {
		return instance->get();
	}

	cv::flann::SearchParams* cv_PtrOfSearchParams_get_inner_ptr_mut(cv::Ptr<cv::flann::SearchParams>* instance) {
		return instance->get();
	}
}

