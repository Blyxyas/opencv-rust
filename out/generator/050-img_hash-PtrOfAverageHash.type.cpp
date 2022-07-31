extern "C" {
	cv::Ptr<cv::img_hash::AverageHash>* cv_PtrOfAverageHash_new(cv::img_hash::AverageHash* val) {
		return new cv::Ptr<cv::img_hash::AverageHash>(val);
	}
	
	void cv_PtrOfAverageHash_delete(cv::Ptr<cv::img_hash::AverageHash>* instance) {
		delete instance;
	}

	const cv::img_hash::AverageHash* cv_PtrOfAverageHash_get_inner_ptr(const cv::Ptr<cv::img_hash::AverageHash>* instance) {
		return instance->get();
	}

	cv::img_hash::AverageHash* cv_PtrOfAverageHash_get_inner_ptr_mut(cv::Ptr<cv::img_hash::AverageHash>* instance) {
		return instance->get();
	}
}

