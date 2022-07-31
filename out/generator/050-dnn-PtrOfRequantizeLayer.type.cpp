extern "C" {
	cv::Ptr<cv::dnn::RequantizeLayer>* cv_PtrOfRequantizeLayer_new(cv::dnn::RequantizeLayer* val) {
		return new cv::Ptr<cv::dnn::RequantizeLayer>(val);
	}
	
	void cv_PtrOfRequantizeLayer_delete(cv::Ptr<cv::dnn::RequantizeLayer>* instance) {
		delete instance;
	}

	const cv::dnn::RequantizeLayer* cv_PtrOfRequantizeLayer_get_inner_ptr(const cv::Ptr<cv::dnn::RequantizeLayer>* instance) {
		return instance->get();
	}

	cv::dnn::RequantizeLayer* cv_PtrOfRequantizeLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::RequantizeLayer>* instance) {
		return instance->get();
	}
}

