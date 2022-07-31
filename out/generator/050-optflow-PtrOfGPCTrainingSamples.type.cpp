extern "C" {
	cv::Ptr<cv::optflow::GPCTrainingSamples>* cv_PtrOfGPCTrainingSamples_new(cv::optflow::GPCTrainingSamples* val) {
		return new cv::Ptr<cv::optflow::GPCTrainingSamples>(val);
	}
	
	void cv_PtrOfGPCTrainingSamples_delete(cv::Ptr<cv::optflow::GPCTrainingSamples>* instance) {
		delete instance;
	}

	const cv::optflow::GPCTrainingSamples* cv_PtrOfGPCTrainingSamples_get_inner_ptr(const cv::Ptr<cv::optflow::GPCTrainingSamples>* instance) {
		return instance->get();
	}

	cv::optflow::GPCTrainingSamples* cv_PtrOfGPCTrainingSamples_get_inner_ptr_mut(cv::Ptr<cv::optflow::GPCTrainingSamples>* instance) {
		return instance->get();
	}
}

