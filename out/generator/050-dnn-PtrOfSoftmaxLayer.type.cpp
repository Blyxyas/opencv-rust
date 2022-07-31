extern "C" {
	cv::Ptr<cv::dnn::SoftmaxLayer>* cv_PtrOfSoftmaxLayer_new(cv::dnn::SoftmaxLayer* val) {
		return new cv::Ptr<cv::dnn::SoftmaxLayer>(val);
	}
	
	void cv_PtrOfSoftmaxLayer_delete(cv::Ptr<cv::dnn::SoftmaxLayer>* instance) {
		delete instance;
	}

	const cv::dnn::SoftmaxLayer* cv_PtrOfSoftmaxLayer_get_inner_ptr(const cv::Ptr<cv::dnn::SoftmaxLayer>* instance) {
		return instance->get();
	}

	cv::dnn::SoftmaxLayer* cv_PtrOfSoftmaxLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::SoftmaxLayer>* instance) {
		return instance->get();
	}
}

