extern "C" {
	cv::Ptr<cv::dnn::ScaleLayerInt8>* cv_PtrOfScaleLayerInt8_new(cv::dnn::ScaleLayerInt8* val) {
		return new cv::Ptr<cv::dnn::ScaleLayerInt8>(val);
	}
	
	void cv_PtrOfScaleLayerInt8_delete(cv::Ptr<cv::dnn::ScaleLayerInt8>* instance) {
		delete instance;
	}

	const cv::dnn::ScaleLayerInt8* cv_PtrOfScaleLayerInt8_get_inner_ptr(const cv::Ptr<cv::dnn::ScaleLayerInt8>* instance) {
		return instance->get();
	}

	cv::dnn::ScaleLayerInt8* cv_PtrOfScaleLayerInt8_get_inner_ptr_mut(cv::Ptr<cv::dnn::ScaleLayerInt8>* instance) {
		return instance->get();
	}
}

