extern "C" {
	cv::Ptr<cv::dnn::PoolingLayerInt8>* cv_PtrOfPoolingLayerInt8_new(cv::dnn::PoolingLayerInt8* val) {
		return new cv::Ptr<cv::dnn::PoolingLayerInt8>(val);
	}
	
	void cv_PtrOfPoolingLayerInt8_delete(cv::Ptr<cv::dnn::PoolingLayerInt8>* instance) {
		delete instance;
	}

	const cv::dnn::PoolingLayerInt8* cv_PtrOfPoolingLayerInt8_get_inner_ptr(const cv::Ptr<cv::dnn::PoolingLayerInt8>* instance) {
		return instance->get();
	}

	cv::dnn::PoolingLayerInt8* cv_PtrOfPoolingLayerInt8_get_inner_ptr_mut(cv::Ptr<cv::dnn::PoolingLayerInt8>* instance) {
		return instance->get();
	}
}

