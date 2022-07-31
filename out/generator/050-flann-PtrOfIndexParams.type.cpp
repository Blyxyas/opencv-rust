extern "C" {
	cv::Ptr<cv::flann::IndexParams>* cv_PtrOfIndexParams_new(cv::flann::IndexParams* val) {
		return new cv::Ptr<cv::flann::IndexParams>(val);
	}
	
	void cv_PtrOfIndexParams_delete(cv::Ptr<cv::flann::IndexParams>* instance) {
		delete instance;
	}

	const cv::flann::IndexParams* cv_PtrOfIndexParams_get_inner_ptr(const cv::Ptr<cv::flann::IndexParams>* instance) {
		return instance->get();
	}

	cv::flann::IndexParams* cv_PtrOfIndexParams_get_inner_ptr_mut(cv::Ptr<cv::flann::IndexParams>* instance) {
		return instance->get();
	}
}

