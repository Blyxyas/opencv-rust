extern "C" {
	cv::Ptr<cv::dnn::ReLULayer>* cv_PtrOfReLULayer_new(cv::dnn::ReLULayer* val) {
		return new cv::Ptr<cv::dnn::ReLULayer>(val);
	}
	
	void cv_PtrOfReLULayer_delete(cv::Ptr<cv::dnn::ReLULayer>* instance) {
		delete instance;
	}

	const cv::dnn::ReLULayer* cv_PtrOfReLULayer_get_inner_ptr(const cv::Ptr<cv::dnn::ReLULayer>* instance) {
		return instance->get();
	}

	cv::dnn::ReLULayer* cv_PtrOfReLULayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ReLULayer>* instance) {
		return instance->get();
	}
}

