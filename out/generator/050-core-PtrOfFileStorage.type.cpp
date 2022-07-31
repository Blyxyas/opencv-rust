extern "C" {
	cv::Ptr<cv::FileStorage>* cv_PtrOfFileStorage_new(cv::FileStorage* val) {
		return new cv::Ptr<cv::FileStorage>(val);
	}
	
	void cv_PtrOfFileStorage_delete(cv::Ptr<cv::FileStorage>* instance) {
		delete instance;
	}

	const cv::FileStorage* cv_PtrOfFileStorage_get_inner_ptr(const cv::Ptr<cv::FileStorage>* instance) {
		return instance->get();
	}

	cv::FileStorage* cv_PtrOfFileStorage_get_inner_ptr_mut(cv::Ptr<cv::FileStorage>* instance) {
		return instance->get();
	}
}

