extern "C" {
	cv::Ptr<cv::dnn::EltwiseLayerInt8>* cv_PtrOfEltwiseLayerInt8_new(cv::dnn::EltwiseLayerInt8* val) {
		return new cv::Ptr<cv::dnn::EltwiseLayerInt8>(val);
	}
	
	void cv_PtrOfEltwiseLayerInt8_delete(cv::Ptr<cv::dnn::EltwiseLayerInt8>* instance) {
		delete instance;
	}

	const cv::dnn::EltwiseLayerInt8* cv_PtrOfEltwiseLayerInt8_get_inner_ptr(const cv::Ptr<cv::dnn::EltwiseLayerInt8>* instance) {
		return instance->get();
	}

	cv::dnn::EltwiseLayerInt8* cv_PtrOfEltwiseLayerInt8_get_inner_ptr_mut(cv::Ptr<cv::dnn::EltwiseLayerInt8>* instance) {
		return instance->get();
	}
}

