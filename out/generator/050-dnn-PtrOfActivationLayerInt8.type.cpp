extern "C" {
	cv::Ptr<cv::dnn::ActivationLayerInt8>* cv_PtrOfActivationLayerInt8_new(cv::dnn::ActivationLayerInt8* val) {
		return new cv::Ptr<cv::dnn::ActivationLayerInt8>(val);
	}
	
	void cv_PtrOfActivationLayerInt8_delete(cv::Ptr<cv::dnn::ActivationLayerInt8>* instance) {
		delete instance;
	}

	const cv::dnn::ActivationLayerInt8* cv_PtrOfActivationLayerInt8_get_inner_ptr(const cv::Ptr<cv::dnn::ActivationLayerInt8>* instance) {
		return instance->get();
	}

	cv::dnn::ActivationLayerInt8* cv_PtrOfActivationLayerInt8_get_inner_ptr_mut(cv::Ptr<cv::dnn::ActivationLayerInt8>* instance) {
		return instance->get();
	}
}

