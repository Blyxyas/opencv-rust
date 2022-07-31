extern "C" {
	void cv_PtrOfSparseRLOFOpticalFlow_delete(cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>* instance) {
		delete instance;
	}

	const cv::optflow::SparseRLOFOpticalFlow* cv_PtrOfSparseRLOFOpticalFlow_get_inner_ptr(const cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>* instance) {
		return instance->get();
	}

	cv::optflow::SparseRLOFOpticalFlow* cv_PtrOfSparseRLOFOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>* instance) {
		return instance->get();
	}
}

