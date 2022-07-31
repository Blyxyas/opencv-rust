extern "C" {
	void cv_PtrOfLinemod_Modality_delete(cv::Ptr<cv::linemod::Modality>* instance) {
		delete instance;
	}

	const cv::linemod::Modality* cv_PtrOfLinemod_Modality_get_inner_ptr(const cv::Ptr<cv::linemod::Modality>* instance) {
		return instance->get();
	}

	cv::linemod::Modality* cv_PtrOfLinemod_Modality_get_inner_ptr_mut(cv::Ptr<cv::linemod::Modality>* instance) {
		return instance->get();
	}
}

