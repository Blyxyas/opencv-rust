extern "C" {
	void cv_PtrOfSuperres_DualTVL1OpticalFlow_delete(cv::Ptr<cv::superres::DualTVL1OpticalFlow>* instance) {
		delete instance;
	}

	const cv::superres::DualTVL1OpticalFlow* cv_PtrOfSuperres_DualTVL1OpticalFlow_get_inner_ptr(const cv::Ptr<cv::superres::DualTVL1OpticalFlow>* instance) {
		return instance->get();
	}

	cv::superres::DualTVL1OpticalFlow* cv_PtrOfSuperres_DualTVL1OpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::superres::DualTVL1OpticalFlow>* instance) {
		return instance->get();
	}
}

