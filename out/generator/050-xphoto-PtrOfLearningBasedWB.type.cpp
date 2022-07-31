extern "C" {
	void cv_PtrOfLearningBasedWB_delete(cv::Ptr<cv::xphoto::LearningBasedWB>* instance) {
		delete instance;
	}

	const cv::xphoto::LearningBasedWB* cv_PtrOfLearningBasedWB_get_inner_ptr(const cv::Ptr<cv::xphoto::LearningBasedWB>* instance) {
		return instance->get();
	}

	cv::xphoto::LearningBasedWB* cv_PtrOfLearningBasedWB_get_inner_ptr_mut(cv::Ptr<cv::xphoto::LearningBasedWB>* instance) {
		return instance->get();
	}
}

