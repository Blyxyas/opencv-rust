extern "C" {
	cv::Ptr<cv::dnn::BaseConvolutionLayer>* cv_PtrOfBaseConvolutionLayer_new(cv::dnn::BaseConvolutionLayer* val) {
		return new cv::Ptr<cv::dnn::BaseConvolutionLayer>(val);
	}
	
	void cv_PtrOfBaseConvolutionLayer_delete(cv::Ptr<cv::dnn::BaseConvolutionLayer>* instance) {
		delete instance;
	}

	const cv::dnn::BaseConvolutionLayer* cv_PtrOfBaseConvolutionLayer_get_inner_ptr(const cv::Ptr<cv::dnn::BaseConvolutionLayer>* instance) {
		return instance->get();
	}

	cv::dnn::BaseConvolutionLayer* cv_PtrOfBaseConvolutionLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::BaseConvolutionLayer>* instance) {
		return instance->get();
	}
}

