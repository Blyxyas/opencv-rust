extern "C" {
	cv::Ptr<cv::dnn::DequantizeLayer>* cv_PtrOfDequantizeLayer_new(cv::dnn::DequantizeLayer* val) {
		return new cv::Ptr<cv::dnn::DequantizeLayer>(val);
	}
	
	void cv_PtrOfDequantizeLayer_delete(cv::Ptr<cv::dnn::DequantizeLayer>* instance) {
		delete instance;
	}

	const cv::dnn::DequantizeLayer* cv_PtrOfDequantizeLayer_get_inner_ptr(const cv::Ptr<cv::dnn::DequantizeLayer>* instance) {
		return instance->get();
	}

	cv::dnn::DequantizeLayer* cv_PtrOfDequantizeLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::DequantizeLayer>* instance) {
		return instance->get();
	}
}

