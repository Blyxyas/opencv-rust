extern "C" {
	cv::Ptr<cv::dnn::BatchNormLayerInt8>* cv_PtrOfBatchNormLayerInt8_new(cv::dnn::BatchNormLayerInt8* val) {
		return new cv::Ptr<cv::dnn::BatchNormLayerInt8>(val);
	}
	
	void cv_PtrOfBatchNormLayerInt8_delete(cv::Ptr<cv::dnn::BatchNormLayerInt8>* instance) {
		delete instance;
	}

	const cv::dnn::BatchNormLayerInt8* cv_PtrOfBatchNormLayerInt8_get_inner_ptr(const cv::Ptr<cv::dnn::BatchNormLayerInt8>* instance) {
		return instance->get();
	}

	cv::dnn::BatchNormLayerInt8* cv_PtrOfBatchNormLayerInt8_get_inner_ptr_mut(cv::Ptr<cv::dnn::BatchNormLayerInt8>* instance) {
		return instance->get();
	}
}

