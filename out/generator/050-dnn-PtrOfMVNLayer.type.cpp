extern "C" {
	cv::Ptr<cv::dnn::MVNLayer>* cv_PtrOfMVNLayer_new(cv::dnn::MVNLayer* val) {
		return new cv::Ptr<cv::dnn::MVNLayer>(val);
	}
	
	void cv_PtrOfMVNLayer_delete(cv::Ptr<cv::dnn::MVNLayer>* instance) {
		delete instance;
	}

	const cv::dnn::MVNLayer* cv_PtrOfMVNLayer_get_inner_ptr(const cv::Ptr<cv::dnn::MVNLayer>* instance) {
		return instance->get();
	}

	cv::dnn::MVNLayer* cv_PtrOfMVNLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::MVNLayer>* instance) {
		return instance->get();
	}
}

