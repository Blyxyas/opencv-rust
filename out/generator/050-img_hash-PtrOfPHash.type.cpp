extern "C" {
	cv::Ptr<cv::img_hash::PHash>* cv_PtrOfPHash_new(cv::img_hash::PHash* val) {
		return new cv::Ptr<cv::img_hash::PHash>(val);
	}
	
	void cv_PtrOfPHash_delete(cv::Ptr<cv::img_hash::PHash>* instance) {
		delete instance;
	}

	const cv::img_hash::PHash* cv_PtrOfPHash_get_inner_ptr(const cv::Ptr<cv::img_hash::PHash>* instance) {
		return instance->get();
	}

	cv::img_hash::PHash* cv_PtrOfPHash_get_inner_ptr_mut(cv::Ptr<cv::img_hash::PHash>* instance) {
		return instance->get();
	}
}

