extern "C" {
	cv::Ptr<cv::dnn::ReduceLayerInt8>* cv_PtrOfReduceLayerInt8_new(cv::dnn::ReduceLayerInt8* val) {
		return new cv::Ptr<cv::dnn::ReduceLayerInt8>(val);
	}
	
	void cv_PtrOfReduceLayerInt8_delete(cv::Ptr<cv::dnn::ReduceLayerInt8>* instance) {
		delete instance;
	}

	const cv::dnn::ReduceLayerInt8* cv_PtrOfReduceLayerInt8_get_inner_ptr(const cv::Ptr<cv::dnn::ReduceLayerInt8>* instance) {
		return instance->get();
	}

	cv::dnn::ReduceLayerInt8* cv_PtrOfReduceLayerInt8_get_inner_ptr_mut(cv::Ptr<cv::dnn::ReduceLayerInt8>* instance) {
		return instance->get();
	}
}

