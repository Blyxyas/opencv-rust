extern "C" {
	cv::Ptr<cv::img_hash::BlockMeanHash>* cv_PtrOfBlockMeanHash_new(cv::img_hash::BlockMeanHash* val) {
		return new cv::Ptr<cv::img_hash::BlockMeanHash>(val);
	}
	
	void cv_PtrOfBlockMeanHash_delete(cv::Ptr<cv::img_hash::BlockMeanHash>* instance) {
		delete instance;
	}

	const cv::img_hash::BlockMeanHash* cv_PtrOfBlockMeanHash_get_inner_ptr(const cv::Ptr<cv::img_hash::BlockMeanHash>* instance) {
		return instance->get();
	}

	cv::img_hash::BlockMeanHash* cv_PtrOfBlockMeanHash_get_inner_ptr_mut(cv::Ptr<cv::img_hash::BlockMeanHash>* instance) {
		return instance->get();
	}
}

