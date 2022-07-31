extern "C" {
	cv::Ptr<cv::dnn::InnerProductLayerInt8>* cv_PtrOfInnerProductLayerInt8_new(cv::dnn::InnerProductLayerInt8* val) {
		return new cv::Ptr<cv::dnn::InnerProductLayerInt8>(val);
	}
	
	void cv_PtrOfInnerProductLayerInt8_delete(cv::Ptr<cv::dnn::InnerProductLayerInt8>* instance) {
		delete instance;
	}

	const cv::dnn::InnerProductLayerInt8* cv_PtrOfInnerProductLayerInt8_get_inner_ptr(const cv::Ptr<cv::dnn::InnerProductLayerInt8>* instance) {
		return instance->get();
	}

	cv::dnn::InnerProductLayerInt8* cv_PtrOfInnerProductLayerInt8_get_inner_ptr_mut(cv::Ptr<cv::dnn::InnerProductLayerInt8>* instance) {
		return instance->get();
	}
}

