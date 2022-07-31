extern "C" {
	void cv_PtrOfDualTVL1OpticalFlow_delete(cv::Ptr<cv::optflow::DualTVL1OpticalFlow>* instance) {
		delete instance;
	}

	const cv::optflow::DualTVL1OpticalFlow* cv_PtrOfDualTVL1OpticalFlow_get_inner_ptr(const cv::Ptr<cv::optflow::DualTVL1OpticalFlow>* instance) {
		return instance->get();
	}

	cv::optflow::DualTVL1OpticalFlow* cv_PtrOfDualTVL1OpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::optflow::DualTVL1OpticalFlow>* instance) {
		return instance->get();
	}
}

