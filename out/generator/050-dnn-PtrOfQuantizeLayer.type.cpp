extern "C" {
	cv::Ptr<cv::dnn::QuantizeLayer>* cv_PtrOfQuantizeLayer_new(cv::dnn::QuantizeLayer* val) {
		return new cv::Ptr<cv::dnn::QuantizeLayer>(val);
	}
	
	void cv_PtrOfQuantizeLayer_delete(cv::Ptr<cv::dnn::QuantizeLayer>* instance) {
		delete instance;
	}

	const cv::dnn::QuantizeLayer* cv_PtrOfQuantizeLayer_get_inner_ptr(const cv::Ptr<cv::dnn::QuantizeLayer>* instance) {
		return instance->get();
	}

	cv::dnn::QuantizeLayer* cv_PtrOfQuantizeLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::QuantizeLayer>* instance) {
		return instance->get();
	}
}

