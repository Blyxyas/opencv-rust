extern "C" {
	cv::Ptr<cv::optflow::GPCTree>* cv_PtrOfGPCTree_new(cv::optflow::GPCTree* val) {
		return new cv::Ptr<cv::optflow::GPCTree>(val);
	}
	
	void cv_PtrOfGPCTree_delete(cv::Ptr<cv::optflow::GPCTree>* instance) {
		delete instance;
	}

	const cv::optflow::GPCTree* cv_PtrOfGPCTree_get_inner_ptr(const cv::Ptr<cv::optflow::GPCTree>* instance) {
		return instance->get();
	}

	cv::optflow::GPCTree* cv_PtrOfGPCTree_get_inner_ptr_mut(cv::Ptr<cv::optflow::GPCTree>* instance) {
		return instance->get();
	}
}

