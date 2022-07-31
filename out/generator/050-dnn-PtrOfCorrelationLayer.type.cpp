extern "C" {
	cv::Ptr<cv::dnn::CorrelationLayer>* cv_PtrOfCorrelationLayer_new(cv::dnn::CorrelationLayer* val) {
		return new cv::Ptr<cv::dnn::CorrelationLayer>(val);
	}
	
	void cv_PtrOfCorrelationLayer_delete(cv::Ptr<cv::dnn::CorrelationLayer>* instance) {
		delete instance;
	}

	const cv::dnn::CorrelationLayer* cv_PtrOfCorrelationLayer_get_inner_ptr(const cv::Ptr<cv::dnn::CorrelationLayer>* instance) {
		return instance->get();
	}

	cv::dnn::CorrelationLayer* cv_PtrOfCorrelationLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::CorrelationLayer>* instance) {
		return instance->get();
	}
}

