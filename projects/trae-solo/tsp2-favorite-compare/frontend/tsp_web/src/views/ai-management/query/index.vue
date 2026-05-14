<template>
	<div class="content">
		<div class="top common" :style="{ height: isActive ? 'auto' : '125px' }">
			<div class="search-title">
				<div>
					<img src="@/assets/image/ai/query/竖杠.png" alt="" />
					检索条件
				</div>
				<div class="weather-icon" @click="handleCollapseChange">
					<img v-if="isActive" src="@/assets/image/ai/query/收起.png" alt="" />
					<img v-if="!isActive" src="@/assets/image/ai/query/展开.png" alt="" />
					{{ isActive ? '收起' : '展开' }}
				</div>
			</div>
			<el-form :inline="true" :model="formSearch" label-width="120px" class="p-t-20">
				<el-row :gutter="35">
					<el-col :span="8"
						><el-form-item label="检索范围：" class="buttonp">
							<el-radio-group v-model="formSearch.searchType" fill="#33B7B3">
								<el-radio-button v-for="item in searchOptions" :key="item.value" :value="item.value" :label="item.label" />
							</el-radio-group> </el-form-item
					></el-col>
					<el-col :span="16">
						<el-form-item label="检索关键字：" class="buttonp">
							<el-input v-model="searchInput" style="width: 100%" placeholder="请输入关键字进行精确检索" :prefix-icon="Search" />
						</el-form-item>
					</el-col>
				</el-row>
				<el-row :gutter="35" v-show="isActive">
					<el-col :span="8"
						><el-form-item label="标准级别：">
							<el-select placeholder="请选择标准级别" clearable v-model="formSearch.standardLevelName">
								<el-option v-for="item in standardLevelOptions" :label="item" :value="item" :key="item" />
							</el-select> </el-form-item
					></el-col>
					<el-col :span="8"
						><el-form-item label="标准子体系：">
							<el-select placeholder="请选择" clearable v-model="formSearch.systemId">
								<el-option v-for="item in standardSystemTreeList" :label="item.systemName" :value="item.id" :key="item.id" />
							</el-select> </el-form-item
					></el-col>
					<el-col :span="8"
						><el-form-item label="归口部门：">
							<el-select placeholder="请选择" clearable v-model="formSearch.standardDeparmentId">
								<el-option v-for="item in standardDepartmentTreeList" :label="item.standardDepartmentName" :value="item.id" :key="item.id" />
							</el-select> </el-form-item
					></el-col>
				</el-row>
				<el-row :gutter="35" v-show="isActive">
					<el-col :span="8"
						><el-form-item label="起草人："> <el-input placeholder="请输入" v-model="formSearch.drafter"></el-input> </el-form-item
					></el-col>
					<el-col :span="8"
						><el-form-item label="起草单位："> <el-input placeholder="请输入" v-model="formSearch.draftUnit"></el-input> </el-form-item
					></el-col>
					<el-col :span="8"
						><el-form-item label="发布日期："> <el-date-picker v-model="formSearch.publishDate" type="date" value-format="YYYY-MM-DD" format="YYYY-MM-DD" placeholder="请选择发布日期" style="width: 100%" /> </el-form-item
					></el-col>
				</el-row>
				<el-row :gutter="35" v-show="isActive">
					<el-col :span="8"
						><el-form-item label="实施日期"> <el-input v-model="formSearch.implementDate" placeholder="YYYY-MM-DD/YYYY年XX月XX日"></el-input> </el-form-item
					></el-col>
				</el-row>
				<el-row :gutter="35" v-show="isActive">
					<el-col :span="8"
						><el-form-item label="标准状态">
							<el-radio-group v-model="formSearch.status" fill="#33B7B3">
								<el-radio-button v-for="item in statusOptions" :label="item.label" :value="item.value" :key="item.value" />
							</el-radio-group> </el-form-item
					></el-col>
				</el-row>
			</el-form>

			<div class="button-warp">
				<el-button type="primary" class="button m-r-20" @click="handleSearch">
					<img src="@/assets/image/ai/query/搜索.png" alt="" />
					搜索
				</el-button>
				<div class="reset">
					<el-button type="primary" class="button" @click="handleReset">
						<img src="@/assets/image/ai/query/重置.png" alt="" />
						重置
					</el-button>
				</div>
			</div>
		</div>
		<div class="bottom common">
			<div class="top">
				<div class="search-title">
					<img src="@/assets/image/ai/query/竖杠.png" alt="" />
					查询结果
				</div>
				<div class="total">
					共<span>{{ table.pagination.total }}</span
					>条记录
				</div>
			</div>
			<div class="bottom">
				<el-table :data="table.bodys" style="width: 100%">
					<el-table-column prop="standardNo" label="标准编号" width="180" show-overflow-tooltip />
					<el-table-column prop="standardName" label="标准名称" width="260" show-overflow-tooltip>
						<template #default="{ row }">
							<span style="color: #0078e9; cursor: pointer" @click="handleCommand({ command: 'base', row })">{{ row.standardName }}</span>
						</template>
					</el-table-column>
					<el-table-column prop="standardDeparmentName" label="归口单位" width="260" show-overflow-tooltip />
					<el-table-column prop="publishDate" label="发布日期" width="120" show-overflow-tooltip />
					<el-table-column prop="status" label="状态">
						<template #default="{ row }">
							<span class="common-state" :style="{ 'background-color': backgroundState(row.status), color: colorState(row.status) }">{{ renderStatusText(row.status) }}</span>
						</template>
					</el-table-column>
					<el-table-column prop="operation" label="操作" fixed="right" width="350">
						<template #default="{ row }">
							<div class="operation">
								<span @click="handleCommand({ command: 'pdf', row })">PDF预览</span>
								<span @click="handleCommand({ command: 'base', row })">结构化预览</span>
								<span @click="handleCommand({ command: 'graph', row })">知识图谱</span>
								<span>收藏</span>
							</div>
						</template>
					</el-table-column>
				</el-table>
				<table-pagination :pagination="table.pagination" @success="handleSearch" />
			</div>
		</div>
	</div>
</template>

<script setup>
import { ref } from 'vue';
import { Search } from '@element-plus/icons-vue';
import { useRoute, useRouter } from 'vue-router';
import { STANDARD_MANAGEMENT, AI_MANAGEMENT } from '@/api';
import TablePagination from '@/components/table-pagination/index.vue';
const store = useStore();
const router = useRouter();
const deptId = computed(() => store.state.authCenter.user.deptId);
const searchCondition = computed(() => store.state.query.searchCondition);

const searchOptions = [
	{
		label: '标题',
		value: 1,
	},
	{
		label: '适用范围',
		value: 2,
	},
];
const statusOptions = [
	{
		label: '全部',
		value: 2,
	},
	{
		label: '现行',
		value: 1,
	},
	{
		label: '未生效',
		value: 0,
	},
	{
		label: '废止',
		value: -1,
	},
];
const standardLevelOptions = ['国家标准', '行业标准', '企业标准', '地方标准'];
const isActive = ref(true);
const searchInput = ref('');

const initialState = {
	standardLevelName: null, //标准级别
	systemId: null, //体系
	standardDeparmentId: null, //归口部门
	drafter: null, //起草人
	draftUnit: null, //起草单位
	publishDate: null, //发布日期
	implementDate: null, //实施日期
	status: 2, //状态 0:未发布 1:现行有效 -1:废止 2:全部
	searchType: 1,
};
const formSearch = reactive({ ...initialState });
const initialTableState = {
	loading: false,
	bodys: [],
	name: 'aiManagementTable',
	pagination: {
		page: 1,
		pageSize: 10,
		total: 0,
		sort: '',
		order: 'desc',
	},
};
const table = reactive({ ...initialTableState });
const standardDepartmentTreeList = ref([]);
const standardSystemTreeList = ref([]);
const handleCommand = ({ command, row }) => {
	switch (command) {
		case 'pdf':
			handlePreview(row);
			break;
		case 'base':
			router.push({
				name: 'AiManagement.Query.BaseMessage',
				query: {
					id: row.id,
					type: 'base',
				},
			});
			break;
		case 'graph':
			router.push({
				name: 'AiManagement.Query.BaseMessage',
				query: {
					id: row.id,
					type: 'graph',
				},
			});
			break;
		default:
			break;
	}
};
const backgroundState = (state) => {
	switch (state) {
		case 1:
			return '#F0F9EB';
		case 0:
			return '#FDF6EC';
		case -1:
			return '#FEF0F0';
		default:
			return '#F0F9EB';
	}
};
const colorState = (state) => {
	switch (state) {
		case 1:
			return '#67C23A';
		case 0:
			return '#E6A23C';
		case -1:
			return '#F56C6C';
		default:
			return '#67C23A';
	}
};

const renderStatusText = (state) => {
	return statusOptions.find((item) => item.value === state)?.label || '';
};

const handleCollapseChange = () => {
	isActive.value = !isActive.value;
};
onMounted(() => {
	if (Object.keys(searchCondition.value).length > 0) {
		const { searchform, tableData } = searchCondition.value;
		const { input, form } = searchform;
		searchInput.value = input;
		// 逐个属性重置（保持引用不变）
		Object.keys(tableData).forEach((key) => {
			table[key] = tableData[key];
		});
		Object.keys(form).forEach((key) => {
			formSearch[key] = form[key];
		});
		store.commit('query/getSearchCondition', {});
	}
	getTreeByCompany();
	getTreeExclude(deptId.value);
});

onBeforeUnmount(() => {
	const searchCondition = {
		searchform: { input: searchInput.value, form: { ...formSearch } },
		tableData: { ...table },
	};
	store.commit('query/getSearchCondition', searchCondition);
});

const getTreeByCompany = () => {
	STANDARD_MANAGEMENT.getTreeByCompany({
		deptId: deptId.value,
	})
		.then(({ data }) => {
			standardDepartmentTreeList.value = data.data;
		})
		.catch((err) => {
			console.log(err);
		});
};

const getTreeExclude = (id) => {
	STANDARD_MANAGEMENT.getTreeExclude({
		deptId: id ? id : deptId.value,
		maxLevel: 1,
	})
		.then(({ data }) => {
			standardSystemTreeList.value = data.data;
		})
		.catch((err) => {
			console.log(err);
		});
};

const handleSearch = () => {
	let params = {};
	const { page, pageSize } = table.pagination;
	if (formSearch.searchType == 1) {
		formSearch['standardName'] = searchInput.value;
		if (Object.keys(formSearch).includes('scope')) {
			delete formSearch.scope;
		}
	} else {
		formSearch['scope'] = searchInput.value;
		if (Object.keys(formSearch).includes('standardName')) {
			delete formSearch.standardName;
		}
	}
	const formParams = { ...formSearch };
	if (formParams.status === 2) {
		delete formParams.status;
	}
	params['condition'] = JSON.stringify({ ...formParams });
	AI_MANAGEMENT.getSearchList({
		...params,
		page,
		pageSize,
	})
		.then(({ data }) => {
			const { total, list } = data.data;
			table.bodys = list;
			table.pagination.total = total;
		})
		.catch((err) => {
			console.log(err);
		});
};

const handlePreview = (row) => {
	STANDARD_MANAGEMENT.getFile(row.id)
		.then(({ data }) => {
			const binaryData = [];
			binaryData.push(data);
			const pdfUrl = window.URL.createObjectURL(
				new Blob(binaryData, {
					type: 'application/pdf',
				}),
			);
			window.open(pdfUrl);
		})
		.catch((err) => console.log(err));
};

const handleReset = () => {
	store.commit('query/getSearchCondition', {});
	searchInput.value = '';
	// 逐个属性重置（保持引用不变）
	Object.keys(initialState).forEach((key) => {
		formSearch[key] = initialState[key];
	});
};
</script>

<style lang="less" scoped>
.content {
	height: calc(100vh - 190px);
	display: flex;
	justify-content: flex-start;
	flex-direction: column;
	align-items: center;
	padding: 10px 0;
	.common {
		padding: 20px;
		width: 68%;
		border-radius: 8px;
		background: #ffffff;
		box-shadow: 0px 3px 8px 0px rgba(0, 0, 0, 0.16);
	}

	.search-title {
		img {
			width: 4px;
			margin-right: 6px;
		}

		font-size: 18px;
		font-weight: bold;
		color: #1a1a1a;
		display: flex;
		align-items: center;
	}

	.top {
		margin-bottom: 30px;
		position: relative;

		:deep(.el-radio-button__inner) {
			border: none;
		}

		.weather-icon {
			display: flex;
			align-items: center;
			color: #409eff;
			font-size: 14px;
			margin-left: 20px;
			cursor: pointer;

			img {
				width: 6px;
				margin-right: 6px;
			}
		}

		:deep(.el-form-item) {
			width: 100%;
		}

		:deep(.el-collapse-item__content) {
			padding-bottom: 0;
		}

		:deep(.el-collapse-item__header) {
			height: auto;
			display: inline-block;
		}

		:deep(.el-collapse) {
			--el-collapse-border-color: transparent;
		}

		:deep(.el-collapse-icon-position-right .el-collapse-item__header) {
			padding-right: 0;
		}

		.reset {
			display: inline;

			:deep(.el-button) {
				color: black;
				background-color: white;
			}
		}

		.button-warp {
			display: flex;
			justify-content: center;
		}

		.button {
			img {
				width: 20px;
				margin-right: 10px;
			}
		}
	}

	.bottom {
		.bottom {
			:deep(.el-scrollbar__wrap) {
				padding: 1px 0;
			}
			:deep(.el-table) {
				--el-table-header-text-color: none;
			}

			:deep(.el-table th.el-table__cell) {
				background-color: #ebeef5;
			}

			.common-state {
				font-size: 12px;
				padding: 6px 13px;
				border-radius: 9999px;
			}

			.operation {
				span {
					color: #409eff;
					font-weight: 500;
					font-size: 14px;
					margin-right: 10px;
					cursor: pointer;
				}
			}
		}

		.top {
			display: flex;
			justify-content: space-between;
			align-items: center;

			.total {
				font-size: 14px;
				color: #767676;

				span {
					color: #409eff;
					margin: 0 5px;
				}
			}
		}
	}
}
</style>
