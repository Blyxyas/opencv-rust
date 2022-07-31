extern "C" {
	cv::Ptr<cv::dnn::BatchNormLayer>* cv_PtrOfBatchNormLayer_new(cv::dnn::BatchNormLayer* val) {
		return new cv::Ptr<cv::dnn::BatchNormLayer>(val);
	}
	
	void cv_PtrOfBatchNormLayer_delete(cv::Ptr<cv::dnn::BatchNormLayer>* instance) {
		delete instance;
	}

	const cv::dnn::BatchNormLayer* cv_PtrOfBatchNormLayer_get_inner_ptr(const cv::Ptr<cv::dnn::BatchNormLayer>* instance) {
		return instance->get();
	}

	cv::dnn::BatchNormLayer* cv_PtrOfBatchNormLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::BatchNormLayer>* instance) {
		return instance->get();
	}
}

