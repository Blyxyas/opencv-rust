extern "C" {
	cv::Ptr<cv::dnn::AccumLayer>* cv_PtrOfAccumLayer_new(cv::dnn::AccumLayer* val) {
		return new cv::Ptr<cv::dnn::AccumLayer>(val);
	}
	
	void cv_PtrOfAccumLayer_delete(cv::Ptr<cv::dnn::AccumLayer>* instance) {
		delete instance;
	}

	const cv::dnn::AccumLayer* cv_PtrOfAccumLayer_get_inner_ptr(const cv::Ptr<cv::dnn::AccumLayer>* instance) {
		return instance->get();
	}

	cv::dnn::AccumLayer* cv_PtrOfAccumLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::AccumLayer>* instance) {
		return instance->get();
	}
}

