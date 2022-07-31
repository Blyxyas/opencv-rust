extern "C" {
	void cv_PtrOfDenseRLOFOpticalFlow_delete(cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>* instance) {
		delete instance;
	}

	const cv::optflow::DenseRLOFOpticalFlow* cv_PtrOfDenseRLOFOpticalFlow_get_inner_ptr(const cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>* instance) {
		return instance->get();
	}

	cv::optflow::DenseRLOFOpticalFlow* cv_PtrOfDenseRLOFOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>* instance) {
		return instance->get();
	}
}

