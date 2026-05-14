<template>
	<div class="common-wrapper">
		<div class="m-b-10">
			<div class="StepDiv">
				<div class="title">标准管理与查询:</div>
				<Step class="" :step-list="stepList"></Step>
			</div>
			<div class="topBox_table_box">
				<div class="flex-start m-b-0 itemBox">
					<el-button type="primary" @click="handleUpload" :icon="Upload" class="m-r-20 uploadButton">上传文件</el-button>
					<el-dropdown @command="handleDownDetail">
						<el-button type="primary" class="uploadButton"><i class="icon iconfont icon-daochu1 m-r-10"></i>导 出</el-button>
						<template #dropdown>
							<el-dropdown-menu>
								<el-dropdown-item :command="{ command: 0 }">企业标准统计表</el-dropdown-item>
								<el-dropdown-item :command="{ command: 1 }">企业标准明细表</el-dropdown-item>
							</el-dropdown-menu>
						</template>
					</el-dropdown>
					<div class="flex-divider"></div>
					<RadioGroup :radio-group-list="radioGroupList" :radio-name="'标准状态：'" :radio-state="radioState" @radio-click="handleRadioClick"></RadioGroup>
					<el-input v-model="table.search.keyWord" placeholder="请输入搜索内容" class="m-l-15 input-border-style" style="width: 260px">
						<template #prefix>
							<el-icon class="el-input__icon" @click="getTableList">
								<Search color="#fff" />
							</el-icon>
						</template>
					</el-input>
					<table-columns :table-name="table.name" :default-columns="table.columns" />
				</div>
			</div>
		</div>
		<div class="flex-container">
			<div class="fixed-width">
				<el-select v-model="company" style="width: 100%; font-weight: 600; color: #1d2129; font-size: 12px" placeholder="请选择公司 " @change="onCompanyChange">
					<el-option v-for="item in companyOptions" :key="item.id" :label="item.deptName" :value="item.id" />
				</el-select>
				<div class="queryLine"></div>
				<div class="tree">
					<div v-for="item in standardSystemTreeList" :key="item.id" class="tree-item" :class="systemId !== item.id ? '' : 'tree-itemHover'" @click="onNodeClick(item.id)">
						{{ item.systemName }}
					</div>
				</div>
			</div>
			<div class="flex-grow">
				<el-table v-if="table.visible" :data="table.bodys" v-loading="table.loading" highlight-current-row max-height="calc(100vh - 370px)" :header-cell-style="{ background: '#F2F3F5', color: '#333' }" :cell-style="{ padding: '0px' }" :row-style="{ height: '45px', fontSize: '13px' }" @sort-change="handleTableSortChange">
					<el-table-column v-for="column in table.visibleColumns" :key="column.prop" :label="column.label" :prop="column.prop" :align="column.align" :sortable="column.sortable" :min-width="column.minWidth" show-overflow-tooltip>
						<template v-if="column.prop === 'status'" #default="{ row }">
							<el-tag :style="{ border: 'none', color: row.status === 1 ? '#fff' : row.status === 0 ? '#fff' : '#000' }" :color="row.status === 1 ? '#2478E5' : row.status === 0 ? '#D54941' : '#E7E7E7'">
								{{ row.status === 1 ? '现行有效' : row.status === 0 ? '待确认' : '已废止' }}
							</el-tag>
						</template>
					</el-table-column>
					<el-table-column label="操作" align="center" fixed="right" width="206">
						<template #default="{ row, $index }">
							<div class="flex-center">
								<el-button class="notop no-margin-left" size="small" :disabled="row.status !== 0 || groupCode !== row.groupName" @click="handleEdit(row)" title="编辑">编辑</el-button>
								<el-button class="notop" size="small" style="margin: 0px 6px" :disabled="row.status !== 0 || groupCode !== row.groupName" @click="getSetStatus(row, 1, '')" title="确认">确认</el-button>
								<el-dropdown trigger="click" @command="handleCommand">
									<el-button size="small" class="notop no-margin-left"
										>更多
										<el-icon>
											<ArrowDown />
										</el-icon>
									</el-button>

									<template #dropdown>
										<el-dropdown-menu>
											<el-dropdown-item :command="{ command: 'preview', row, index: $index }">预 览</el-dropdown-item>
											<el-dropdown-item :disabled="row.status === 0" :command="{ command: 'download', row, index: $index }">下 载</el-dropdown-item>
											<el-dropdown-item :disabled="row.status !== 1 || groupCode !== row.groupName" :command="{ command: 'repeal', row, index: $index }">废 止</el-dropdown-item>
											<el-dropdown-item :disabled="row.status !== 0 || groupCode !== row.groupName" :command="{ command: 'delete', row, index: $index }">删 除</el-dropdown-item>
										</el-dropdown-menu>
									</template>
								</el-dropdown>
							</div>
						</template>
					</el-table-column>
				</el-table>
				<table-pagination :pagination="table.pagination" @success="getTableList" />
			</div>
		</div>
		<vue-pdf-embed :source="pdfUrl" :page="1" />
		<UploadDialog ref="uploadDialogRef" @success="getsSuccess" :systemNameOptions="standardSystemTreeList" :standardDeparmentNameOptions="standardDepartmentTreeList"></UploadDialog>
	</div>
</template>

<script setup>
import { saveAs } from 'file-saver';
import { STANDARD_MANAGEMENT } from '@/api';
import { Upload, EditPen, Search, DocumentChecked } from '@element-plus/icons-vue';
import Step from '@/components/step/index.vue';
import RadioGroup from '@/components/radio-group/index.vue';
import TablePagination from '@/components/table-pagination/index.vue';
import TableColumns from '@/components/table-columns/index.vue';
import UploadDialog from './components/upload-dialog.vue';
import VuePdfEmbed from 'vue-pdf-embed';

const store = useStore();
const router = useRouter();
const groupCode = computed(() => store.state.authCenter.user.groupCode);
const deptId = computed(() => store.state.authCenter.user.deptId);
const deptName = computed(() => store.state.authCenter.user.deptName);
const companyStore = computed(() => store.state.query.companyStore);
const systemIdStore = computed(() => store.state.query.systemIdStore);

const stepList = ref([
	{
		label: '上传',
		remark: '文件',
		value: 1,
		show: true,
	},
	{
		label: null,
		remark: '自动解析',
		value: 2,
		show: true,
	},
	{
		label: '确认',
		remark: '信息',
		value: 3,
		show: true,
	},
	{
		label: '查询或预览',
		remark: '标准',
		value: 4,
		show: true,
	},
]);
const radioGroupList = ref([
	{
		label: '全部',
		value: 0,
		count: 0,
	},
	{
		label: '待确认',
		value: 1,
		count: 0,
	},
	{
		label: '现行有效',
		value: 2,
		count: 0,
	},
]);
const radioState = ref(0);
const table = reactive({
	loading: false,
	bodys: [],
	name: 'queryTable',
	visible: computed(() => store.state.table.queryTable.visible),
	visibleColumns: computed(() => store.getters['table/queryTableVisibleColumns']),
	pagination: {
		page: 1,
		pageSize: 10,
		total: 0,
		sort: 'createDate',
		order: 'desc',
	},
	search: {
		where: 'standardNo,standardName,statusLabel,standardLevelName,standardDeparmentName',
		keyWord: '',
		options: [
			{
				label: '标准编号',
				value: 'standardNo',
				placeholder: '请输入标准编号',
			},
			{
				label: '标准名称',
				value: 'standardName',
				placeholder: '请输入标准名称',
			},
			{
				label: '标准状态',
				value: 'status',
				placeholder: '请输入标准状态',
			},
			{
				label: '标准级别',
				value: 'standardLevelName',
				placeholder: '请输入标准级别',
			},
			{
				label: '归口部门',
				value: 'standardDeparmentName',
				placeholder: '请输入归口部门',
			},
		],
	},
	columns: [
		{
			order: 1,
			visible: true,
			label: '上传单位',
			prop: 'deptName',
			align: 'left',
			sortable: 'custom',
			minWidth: 140,
		},
		{
			order: 2,
			visible: true,
			label: '归口部门',
			prop: 'standardDeparmentName',
			align: 'left',
			sortable: 'custom',
			minWidth: 120,
		},
		{
			order: 3,
			visible: true,
			label: '标准编号',
			prop: 'standardNo',
			align: 'left',
			sortable: 'custom',
			minWidth: 150,
		},
		{
			order: 4,
			visible: true,
			label: '标准名称',
			prop: 'standardName',
			align: 'left',
			sortable: 'custom',
			minWidth: 200,
		},
		{
			order: 5,
			visible: true,
			label: '标准状态',
			prop: 'status',
			align: 'left',
			sortable: 'custom',
			minWidth: 120,
		},
		{
			order: 6,
			visible: true,
			label: '标准级别',
			prop: 'standardLevelName',
			align: 'left',
			sortable: 'custom',
			minWidth: 120,
		},
		{
			order: 7,
			visible: true,
			label: '上传人',
			prop: 'creater',
			align: 'left',
			sortable: 'custom',
			minWidth: 120,
		},
		{
			order: 8,
			visible: true,
			label: '子体系',
			prop: 'systemName',
			align: 'left',
			sortable: 'custom',
			minWidth: 120,
		},
		{
			order: 9,
			visible: true,
			label: '上传时间',
			prop: 'createDate',
			align: 'left',
			sortable: 'custom',
			minWidth: 180,
		},
		{
			order: 10,
			visible: true,
			label: '上传状态',
			prop: 'uploadStatus',
			align: 'left',
			sortable: 'custom',
			minWidth: 120,
		},
	],
});
const companyOptions = ref([]);

const systemId = ref(null);
const company = ref(null);
const standardSystemTreeList = ref([]);
const standardDepartmentTreeList = ref([]);

onMounted(() => {
	systemId.value = systemIdStore.value;
	company.value = companyStore.value;
	getTreeByCompany();
	getTreeExclude(deptId.value);
	if (store.state.authCenter.user.deptId === 5) {
		getDepartmentDocList();
	} else {
		companyOptions.value = [
			{
				id: deptId.value,
				deptName: deptName.value,
			},
		];
		company.value = companyOptions.value[0].id;
		getTableList();
	}
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

const getDepartmentDocList = () => {
	STANDARD_MANAGEMENT.getDepartmentDocList({
		page: 1,
		pageSize: 1000,
	})
		.then(({ data }) => {
			companyOptions.value = data.data.list;
			company.value = data.data.list[0].id;
			getTableList();
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

const onCompanyChange = (value) => {
	systemId.value = null;
	getTreeExclude(value);
	getTableList();
	store.commit('query/getCompanyStore', company.value);
	store.commit('query/getsystemIdStore', systemId.value);
};

const getTableList = () => {
	table.loading = true;
	const {
		search: { where, keyWord },
		pagination,
	} = table;
	const params = {
		...pagination,
		where,
		keyWord,
		systemId: systemId.value,
		enterpriseId: company.value,
		status: radioState.value,
	};
	STANDARD_MANAGEMENT.getList(params)
		.then(({ data }) => {
			const { list, total } = data.data;
			table.loading = false;
			table.bodys = list;
			table.pagination.total = total;
			radioGroupList.value = [
				{
					label: '全部',
					value: 0,
					count: 0 || data.allCount,
				},
				{
					label: '待确认',
					value: 1,
					count: 0 || data.noPubCount,
				},
				{
					label: '现行有效',
					value: 2,
					count: 0 || data.effectCount,
				},
			];
		})
		.catch((err) => {
			console.log(err);
			table.loading = false;
		});
};

const handleRadioClick = (data) => {
	radioState.value = data.value;
	getTableList();
};

const getsSuccess = (data) => {
	router.push({
		name: 'StandardManagement.QueryEdit',
		query: {
			id: data.id,
		},
	});
};

const onNodeClick = (row) => {
	systemId.value = row;
	getTableList();
	store.commit('query/getCompanyStore', company.value);
	store.commit('query/getsystemIdStore', systemId.value);
};

const uploadDialogRef = ref(null);
const handleUpload = () => {
	uploadDialogRef.value.show(systemId.value);
};

const handleCommand = ({ command, row, index }) => {
	switch (command) {
		case 'download':
			handleDownload(row);
			break;
		case 'preview':
			handlePreview(row);
			break;
		case 'repeal':
			getSetStatus(row, -1, '废止');
			break;
		case 'delete':
			getDelete(row);
			break;
		default:
			break;
	}
};

const getDelete = ({ id }) => {
	ElMessageBox.confirm('是否确认删除？', '提 示', {
		confirmButtonText: '确定',
		cancelButtonText: '取消',
		cancelButtonClass: 'common-button-wite', //修11.11
		confirmButtonClass: 'common-button-blue', //修11.11
		type: 'warning',
		center: true,
	})
		.then(() => {
			return STANDARD_MANAGEMENT.getDelete({
				id,
			});
		})
		.then(({ data }) => {
			getTableList();
			ElMessage({
				message: '删除成功！',
				type: 'success',
			});
		})
		.catch((err) => console.log(err));
};

const getSetStatus = ({ id }, status, str) => {
	ElMessageBox.confirm(`是否确认${str}？`, '提 示', {
		confirmButtonText: '确定',
		cancelButtonText: '取消',
		cancelButtonClass: 'common-button-wite', //修11.11
		confirmButtonClass: 'common-button-blue', //修11.11
		type: 'warning',
		center: true,
	})
		.then(() => {
			return STANDARD_MANAGEMENT.getSetStatus({
				id,
				status,
			});
		})
		.then(({ data }) => {
			getTableList();
			ElMessage({
				message: '确认成功！',
				type: 'success',
			});
		})
		.catch((err) => console.log(err));
};

const handleEdit = ({ id }) => {
	router.push({
		name: 'StandardManagement.QueryEdit',
		query: {
			id,
		},
	});
};

const handleDownload = (row) => {
	STANDARD_MANAGEMENT.getFile(row.id)
		.then(({ data }) => {
			saveAs(data, row.fileUrl);
			ElMessage({
				message: '导出文件成功！',
				type: 'success',
			});
		})
		.catch((err) => console.log(err));
};

const pdfUrl = ref('');
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
			// pdfUrl.value = URL.createObjectURL(data);
		})
		.catch((err) => console.log(err));
};

const handleDownDetail = ({ command }) => {
	switch (command) {
		case 0:
			downloadFile1();
			break;
		case 1:
			downloadFile2();
			break;
		default:
			break;
	}
};

const downloadFile1 = () => {
	STANDARD_MANAGEMENT.getDownTotal(company.value)
		.then(({ data }) => {
			saveAs(data, `企业标准统计表.xlsx`);
			ElMessage({
				message: 'success',
				type: 'success',
			});
		})
		.catch((err) => console.log(err));
};

const downloadFile2 = () => {
	if (!systemId.value)
		return ElMessage({
			type: 'warning',
			message: '请选择子体系',
		});
	STANDARD_MANAGEMENT.getDownDetail(company.value, systemId.value)
		.then(({ data }) => {
			saveAs(data, `企业标准明细表.xlsx`);
			ElMessage({
				message: 'success',
				type: 'success',
			});
		})
		.catch((err) => console.log(err));
};

const handleTableSortChange = ({ column, prop, order }) => {
	if (!column) {
		table.pagination = {
			page: 1,
			pageSize: 10,
			total: 0,
			sort: '',
			order: 'desc',
		};
	} else {
		const fmt_order = {
			ascending: 'asc',
			descending: 'desc',
		};
		table.pagination.page = 1;
		table.pagination.sort = order ? prop : '';
		table.pagination.order = order ? fmt_order[order] : 'desc';
	}
	getTableList();
};

watch(
	() => table.search.keyWord,
	() => {
		getTableList();
	},
	{
		deep: true,
	},
);
</script>

<style lang="less" scoped>
.flex-container {
	position: relative;
	margin-top: 16px;
}

.fixed-width {
	position: absolute;
	left: 0;
	top: 0;
	bottom: 0;
	width: calc(200px - 16px);
	padding: 8px;
	background: linear-gradient(#f1f5f9 0%, #f2f3f5 100%);
	border-radius: 4px 4px 4px 4px;
	border: 1px solid #e5e6eb;
	min-height: 420px;
	max-height: calc(100vh - 370px);
}

.flex-grow {
	margin-left: 220px;
	/* 与固定宽度相同 */
	min-height: 300px;
}

.tree {
	white-space: nowrap;

	.tree-item {
		color: #1d2129;
		font-size: 14px;
		overflow: auto;
		-ms-overflow-style: none;
		scrollbar-width: none;
		border-radius: 2px;
		padding: 8px 20px;
		margin: 5px 0;
		background-color: transparent !important;
	}

	.tree-item:hover {
		background-color: #fff !important;
		color: #fff !important;
		border-radius: 2px;
		cursor: pointer;
		color: #0b59c3 !important;
		border-left: 3px solid #0b59c3;
	}

	.tree-itemHover {
		background-color: #fff !important;
		color: #fff !important;
		border-radius: 2px;
		cursor: pointer;
		color: #0b59c3 !important;
		border-left: 3px solid #0b59c3;
	}
}
</style>
