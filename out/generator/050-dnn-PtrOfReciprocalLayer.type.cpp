extern "C" {
	cv::Ptr<cv::dnn::ReciprocalLayer>* cv_PtrOfReciprocalLayer_new(cv::dnn::ReciprocalLayer* val) {
		return new cv::Ptr<cv::dnn::ReciprocalLayer>(val);
	}
	
	void cv_PtrOfReciprocalLayer_delete(cv::Ptr<cv::dnn::ReciprocalLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ReciprocalLayer* cv_PtrOfReciprocalLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ReciprocalLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ReciprocalLayer* cv_PtrOfReciprocalLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ReciprocalLayer>* instance) {
		return instance->get();
	}
}

