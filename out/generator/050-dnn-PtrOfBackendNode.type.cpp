extern "C" {
	cv::Ptr<cv::dnn::BackendNode>* cv_PtrOfBackendNode_new(cv::dnn::BackendNode* val) {
		return new cv::Ptr<cv::dnn::BackendNode>(val);
	}
	
	void cv_PtrOfBackendNode_delete(cv::Ptr<cv::dnn::BackendNode>* instance) {
		delete instance;
	}

	const cv::dnn::BackendNode* cv_PtrOfBackendNode_get_inner_ptr(const cv::Ptr<cv::dnn::BackendNode>* instance) {
		return instance->get();
	}

	cv::dnn::BackendNode* cv_PtrOfBackendNode_get_inner_ptr_mut(cv::Ptr<cv::dnn::BackendNode>* instance) {
		return instance->get();
	}
}

