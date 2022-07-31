extern "C" {
	cv::Ptr<cv::dnn::ReorgLayer>* cv_PtrOfReorgLayer_new(cv::dnn::ReorgLayer* val) {
		return new cv::Ptr<cv::dnn::ReorgLayer>(val);
	}
	
	void cv_PtrOfReorgLayer_delete(cv::Ptr<cv::dnn::ReorgLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ReorgLayer* cv_PtrOfReorgLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ReorgLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ReorgLayer* cv_PtrOfReorgLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ReorgLayer>* instance) {
		return instance->get();
	}
}

