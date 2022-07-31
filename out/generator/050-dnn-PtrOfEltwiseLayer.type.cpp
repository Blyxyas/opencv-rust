extern "C" {
	cv::Ptr<cv::dnn::EltwiseLayer>* cv_PtrOfEltwiseLayer_new(cv::dnn::EltwiseLayer* val) {
		return new cv::Ptr<cv::dnn::EltwiseLayer>(val);
	}
	
	void cv_PtrOfEltwiseLayer_delete(cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
		delete instance;
	}

	const cv::dnn::EltwiseLayer* cv_PtrOfEltwiseLayer_get_inner_ptr(const cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
		return instance->get();
	}

	cv::dnn::EltwiseLayer* cv_PtrOfEltwiseLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
		return instance->get();
	}
}

