extern "C" {
	cv::Ptr<cv::dnn::NormalizeBBoxLayer>* cv_PtrOfNormalizeBBoxLayer_new(cv::dnn::NormalizeBBoxLayer* val) {
		return new cv::Ptr<cv::dnn::NormalizeBBoxLayer>(val);
	}
	
	void cv_PtrOfNormalizeBBoxLayer_delete(cv::Ptr<cv::dnn::NormalizeBBoxLayer>* instance) {
		delete instance;
	}

	const cv::dnn::NormalizeBBoxLayer* cv_PtrOfNormalizeBBoxLayer_get_inner_ptr(const cv::Ptr<cv::dnn::NormalizeBBoxLayer>* instance) {
		return instance->get();
	}

	cv::dnn::NormalizeBBoxLayer* cv_PtrOfNormalizeBBoxLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::NormalizeBBoxLayer>* instance) {
		return instance->get();
	}
}

