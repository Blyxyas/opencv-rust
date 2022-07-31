extern "C" {
	cv::Ptr<cv::dnn::SliceLayer>* cv_PtrOfSliceLayer_new(cv::dnn::SliceLayer* val) {
		return new cv::Ptr<cv::dnn::SliceLayer>(val);
	}
	
	void cv_PtrOfSliceLayer_delete(cv::Ptr<cv::dnn::SliceLayer>* instance) {
		delete instance;
	}

	const cv::dnn::SliceLayer* cv_PtrOfSliceLayer_get_inner_ptr(const cv::Ptr<cv::dnn::SliceLayer>* instance) {
		return instance->get();
	}

	cv::dnn::SliceLayer* cv_PtrOfSliceLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::SliceLayer>* instance) {
		return instance->get();
	}
}

