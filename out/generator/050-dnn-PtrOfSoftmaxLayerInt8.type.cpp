extern "C" {
	cv::Ptr<cv::dnn::SoftmaxLayerInt8>* cv_PtrOfSoftmaxLayerInt8_new(cv::dnn::SoftmaxLayerInt8* val) {
		return new cv::Ptr<cv::dnn::SoftmaxLayerInt8>(val);
	}
	
	void cv_PtrOfSoftmaxLayerInt8_delete(cv::Ptr<cv::dnn::SoftmaxLayerInt8>* instance) {
		delete instance;
	}

	const cv::dnn::SoftmaxLayerInt8* cv_PtrOfSoftmaxLayerInt8_get_inner_ptr(const cv::Ptr<cv::dnn::SoftmaxLayerInt8>* instance) {
		return instance->get();
	}

	cv::dnn::SoftmaxLayerInt8* cv_PtrOfSoftmaxLayerInt8_get_inner_ptr_mut(cv::Ptr<cv::dnn::SoftmaxLayerInt8>* instance) {
		return instance->get();
	}
}

