import appRequest from '../app-request.js';

export default {
	getStatisticsData() {
		return appRequest.get(`/api/statistics/data`);
	},
};
