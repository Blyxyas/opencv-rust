extern "C" {
	cv::Ptr<cv::videostab::LogToStdout>* cv_PtrOfLogToStdout_new(cv::videostab::LogToStdout* val) {
		return new cv::Ptr<cv::videostab::LogToStdout>(val);
	}
	
	void cv_PtrOfLogToStdout_delete(cv::Ptr<cv::videostab::LogToStdout>* instance) {
		delete instance;
	}

	const cv::videostab::LogToStdout* cv_PtrOfLogToStdout_get_inner_ptr(const cv::Ptr<cv::videostab::LogToStdout>* instance) {
		return instance->get();
	}

	cv::videostab::LogToStdout* cv_PtrOfLogToStdout_get_inner_ptr_mut(cv::Ptr<cv::videostab::LogToStdout>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::ILog>* cv_PtrOfLogToStdout_to_PtrOfILog(cv::Ptr<cv::videostab::LogToStdout>* instance) {
		return new cv::Ptr<cv::videostab::ILog>(instance->dynamicCast<cv::videostab::ILog>());
	}
}

