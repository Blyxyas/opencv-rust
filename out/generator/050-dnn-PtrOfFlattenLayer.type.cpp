extern "C" {
	cv::Ptr<cv::dnn::FlattenLayer>* cv_PtrOfFlattenLayer_new(cv::dnn::FlattenLayer* val) {
		return new cv::Ptr<cv::dnn::FlattenLayer>(val);
	}
	
	void cv_PtrOfFlattenLayer_delete(cv::Ptr<cv::dnn::FlattenLayer>* instance) {
		delete instance;
	}

	const cv::dnn::FlattenLayer* cv_PtrOfFlattenLayer_get_inner_ptr(const cv::Ptr<cv::dnn::FlattenLayer>* instance) {
		return instance->get();
	}

	cv::dnn::FlattenLayer* cv_PtrOfFlattenLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::FlattenLayer>* instance) {
		return instance->get();
	}
}

