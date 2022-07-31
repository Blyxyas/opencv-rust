extern "C" {
	cv::Ptr<cv::dnn::ScaleLayer>* cv_PtrOfScaleLayer_new(cv::dnn::ScaleLayer* val) {
		return new cv::Ptr<cv::dnn::ScaleLayer>(val);
	}
	
	void cv_PtrOfScaleLayer_delete(cv::Ptr<cv::dnn::ScaleLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ScaleLayer* cv_PtrOfScaleLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ScaleLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ScaleLayer* cv_PtrOfScaleLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ScaleLayer>* instance) {
		return instance->get();
	}
}

