extern "C" {
	cv::Ptr<cv::dnn::ResizeLayer>* cv_PtrOfResizeLayer_new(cv::dnn::ResizeLayer* val) {
		return new cv::Ptr<cv::dnn::ResizeLayer>(val);
	}
	
	void cv_PtrOfResizeLayer_delete(cv::Ptr<cv::dnn::ResizeLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ResizeLayer* cv_PtrOfResizeLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ResizeLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ResizeLayer* cv_PtrOfResizeLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ResizeLayer>* instance) {
		return instance->get();
	}
}

