extern "C" {
	void cv_PtrOfDenseOpticalFlow_delete(cv::Ptr<cv::DenseOpticalFlow>* instance) {
		delete instance;
	}

	const cv::DenseOpticalFlow* cv_PtrOfDenseOpticalFlow_get_inner_ptr(const cv::Ptr<cv::DenseOpticalFlow>* instance) {
		return instance->get();
	}

	cv::DenseOpticalFlow* cv_PtrOfDenseOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::DenseOpticalFlow>* instance) {
		return instance->get();
	}
}

