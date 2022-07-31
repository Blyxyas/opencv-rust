extern "C" {
	cv::Ptr<const cv::optflow::PCAPrior>* cv_PtrOfPCAPrior_new(const cv::optflow::PCAPrior* val) {
		return new cv::Ptr<const cv::optflow::PCAPrior>(val);
	}
	
	void cv_PtrOfPCAPrior_delete(cv::Ptr<const cv::optflow::PCAPrior>* instance) {
		delete instance;
	}

	const cv::optflow::PCAPrior* cv_PtrOfPCAPrior_get_inner_ptr(const cv::Ptr<const cv::optflow::PCAPrior>* instance) {
		return instance->get();
	}

	const cv::optflow::PCAPrior* cv_PtrOfPCAPrior_get_inner_ptr_mut(cv::Ptr<const cv::optflow::PCAPrior>* instance) {
		return instance->get();
	}
}

