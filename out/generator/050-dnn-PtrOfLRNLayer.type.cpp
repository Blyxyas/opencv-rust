extern "C" {
	cv::Ptr<cv::dnn::LRNLayer>* cv_PtrOfLRNLayer_new(cv::dnn::LRNLayer* val) {
		return new cv::Ptr<cv::dnn::LRNLayer>(val);
	}
	
	void cv_PtrOfLRNLayer_delete(cv::Ptr<cv::dnn::LRNLayer>* instance) {
		delete instance;
	}

	const cv::dnn::LRNLayer* cv_PtrOfLRNLayer_get_inner_ptr(const cv::Ptr<cv::dnn::LRNLayer>* instance) {
		return instance->get();
	}

	cv::dnn::LRNLayer* cv_PtrOfLRNLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::LRNLayer>* instance) {
		return instance->get();
	}
}

