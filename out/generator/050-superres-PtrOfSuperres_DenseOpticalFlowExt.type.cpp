extern "C" {
	void cv_PtrOfSuperres_DenseOpticalFlowExt_delete(cv::Ptr<cv::superres::DenseOpticalFlowExt>* instance) {
		delete instance;
	}

	const cv::superres::DenseOpticalFlowExt* cv_PtrOfSuperres_DenseOpticalFlowExt_get_inner_ptr(const cv::Ptr<cv::superres::DenseOpticalFlowExt>* instance) {
		return instance->get();
	}

	cv::superres::DenseOpticalFlowExt* cv_PtrOfSuperres_DenseOpticalFlowExt_get_inner_ptr_mut(cv::Ptr<cv::superres::DenseOpticalFlowExt>* instance) {
		return instance->get();
	}
}

