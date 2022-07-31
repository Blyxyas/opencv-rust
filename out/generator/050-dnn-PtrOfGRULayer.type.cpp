extern "C" {
	cv::Ptr<cv::dnn::GRULayer>* cv_PtrOfGRULayer_new(cv::dnn::GRULayer* val) {
		return new cv::Ptr<cv::dnn::GRULayer>(val);
	}
	
	void cv_PtrOfGRULayer_delete(cv::Ptr<cv::dnn::GRULayer>* instance) {
		delete instance;
	}

	const cv::dnn::GRULayer* cv_PtrOfGRULayer_get_inner_ptr(const cv::Ptr<cv::dnn::GRULayer>* instance) {
		return instance->get();
	}

	cv::dnn::GRULayer* cv_PtrOfGRULayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::GRULayer>* instance) {
		return instance->get();
	}
}

