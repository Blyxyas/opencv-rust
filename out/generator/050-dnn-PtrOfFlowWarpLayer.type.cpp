extern "C" {
	cv::Ptr<cv::dnn::FlowWarpLayer>* cv_PtrOfFlowWarpLayer_new(cv::dnn::FlowWarpLayer* val) {
		return new cv::Ptr<cv::dnn::FlowWarpLayer>(val);
	}
	
	void cv_PtrOfFlowWarpLayer_delete(cv::Ptr<cv::dnn::FlowWarpLayer>* instance) {
		delete instance;
	}

	const cv::dnn::FlowWarpLayer* cv_PtrOfFlowWarpLayer_get_inner_ptr(const cv::Ptr<cv::dnn::FlowWarpLayer>* instance) {
		return instance->get();
	}

	cv::dnn::FlowWarpLayer* cv_PtrOfFlowWarpLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::FlowWarpLayer>* instance) {
		return instance->get();
	}
}

