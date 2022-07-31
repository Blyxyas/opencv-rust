extern "C" {
	cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* cv_PtrOfRLOFOpticalFlowParameter_new(cv::optflow::RLOFOpticalFlowParameter* val) {
		return new cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>(val);
	}
	
	void cv_PtrOfRLOFOpticalFlowParameter_delete(cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* instance) {
		delete instance;
	}

	const cv::optflow::RLOFOpticalFlowParameter* cv_PtrOfRLOFOpticalFlowParameter_get_inner_ptr(const cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* instance) {
		return instance->get();
	}

	cv::optflow::RLOFOpticalFlowParameter* cv_PtrOfRLOFOpticalFlowParameter_get_inner_ptr_mut(cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* instance) {
		return instance->get();
	}
}

