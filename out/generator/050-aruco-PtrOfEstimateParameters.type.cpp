extern "C" {
	cv::Ptr<cv::aruco::EstimateParameters>* cv_PtrOfEstimateParameters_new(cv::aruco::EstimateParameters* val) {
		return new cv::Ptr<cv::aruco::EstimateParameters>(val);
	}
	
	void cv_PtrOfEstimateParameters_delete(cv::Ptr<cv::aruco::EstimateParameters>* instance) {
		delete instance;
	}

	const cv::aruco::EstimateParameters* cv_PtrOfEstimateParameters_get_inner_ptr(const cv::Ptr<cv::aruco::EstimateParameters>* instance) {
		return instance->get();
	}

	cv::aruco::EstimateParameters* cv_PtrOfEstimateParameters_get_inner_ptr_mut(cv::Ptr<cv::aruco::EstimateParameters>* instance) {
		return instance->get();
	}
}

