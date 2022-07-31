extern "C" {
	void cv_PtrOfTrainData_delete(cv::Ptr<cv::ml::TrainData>* instance) {
		delete instance;
	}

	const cv::ml::TrainData* cv_PtrOfTrainData_get_inner_ptr(const cv::Ptr<cv::ml::TrainData>* instance) {
		return instance->get();
	}

	cv::ml::TrainData* cv_PtrOfTrainData_get_inner_ptr_mut(cv::Ptr<cv::ml::TrainData>* instance) {
		return instance->get();
	}
}

