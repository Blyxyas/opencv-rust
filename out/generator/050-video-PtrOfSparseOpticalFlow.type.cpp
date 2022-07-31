extern "C" {
	void cv_PtrOfSparseOpticalFlow_delete(cv::Ptr<cv::SparseOpticalFlow>* instance) {
		delete instance;
	}

	const cv::SparseOpticalFlow* cv_PtrOfSparseOpticalFlow_get_inner_ptr(const cv::Ptr<cv::SparseOpticalFlow>* instance) {
		return instance->get();
	}

	cv::SparseOpticalFlow* cv_PtrOfSparseOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::SparseOpticalFlow>* instance) {
		return instance->get();
	}
}

