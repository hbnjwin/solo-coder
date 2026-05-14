<template>
	<div class="base-layout">
		<el-collapse class="m-b-20" v-model="collapse1">
			<el-collapse-item title="Consistency" name="1" class="base-common">
				<template #title="{ isActive }">
					<el-icon v-if="!isActive" style="vertical-align: middle">
						<CaretRight />
					</el-icon>
					<el-icon v-if="isActive" style="vertical-align: middle"><CaretBottom /></el-icon>
					<div class="self-title m-l-5">基础信息</div>
				</template>
				<template #icon="{ isActive }"><span></span></template>
				<div class="base-content">
					<div class="base-common">
						<div class="center m-b-16">
							<p class="title">标准状态：</p>
							<div class="button">{{ info.statusLabel }}</div>
						</div>
						<div class="center">
							<p class="title">提出单位：</p>
							<p class="content">{{ info.proposeUnit }}</p>
						</div>
					</div>
					<div class="base-common">
						<div class="center m-b-16">
							<p class="title">标准类型：</p>
							<p class="content">{{ info.standardLevelName }}</p>
						</div>
						<div class="center">
							<p class="title">发布单位：</p>
							<p class="content">{{ info.publishUnit.includes(']') ? JSON.parse(info.publishUnit).join('、') : info.publishUnit }}</p>
						</div>
					</div>
					<div class="base-common">
						<div class="center m-b-16">
							<p class="title">发布日期：</p>
							<p class="content">{{ info.publishDate }}</p>
						</div>
						<div class="center">
							<p class="title">归口单位：</p>
							<p class="content">{{ info.standardDeparmentName }}</p>
						</div>
					</div>
				</div>
			</el-collapse-item>
		</el-collapse>
		<el-collapse class="m-b-20" v-model="collapse2">
			<el-collapse-item title="Consistency" name="1" class="base-common">
				<template #title="{ isActive }">
					<el-icon v-if="!isActive" style="vertical-align: middle">
						<CaretRight />
					</el-icon>
					<el-icon v-if="isActive" style="vertical-align: middle"><CaretBottom /></el-icon>
					<div class="self-title m-l-5">起草单位</div>
				</template>
				<template #icon="{ isActive }"><span></span></template>
				<div class="base-content">
					<p class="draft">{{ info.draftUnit.includes(']') ? JSON.parse(info.draftUnit).join(' | ') : info.draftUnit }}</p>
				</div>
			</el-collapse-item>
		</el-collapse>
		<el-collapse class="m-b-20" v-model="collapse3">
			<el-collapse-item title="Consistency" name="1" class="base-common">
				<template #title="{ isActive }">
					<el-icon v-if="!isActive" style="vertical-align: middle">
						<CaretRight />
					</el-icon>
					<el-icon v-if="isActive" style="vertical-align: middle"><CaretBottom /></el-icon>
					<div class="self-title m-l-5">起草人</div>
				</template>
				<template #icon="{ isActive }"><span></span></template>
				<div class="base-content">
					<p class="draft">{{ info.drafter.includes(']') ? JSON.parse(info.drafter).join(' | ') : info.drafter }}</p>
				</div>
			</el-collapse-item>
		</el-collapse>
		<el-collapse class="m-b-20" v-model="collapse4">
			<el-collapse-item title="Consistency" name="1" class="base-common">
				<template #title="{ isActive }">
					<el-icon v-if="!isActive" style="vertical-align: middle">
						<CaretRight />
					</el-icon>
					<el-icon v-if="isActive" style="vertical-align: middle"><CaretBottom /></el-icon>
					<div class="self-title m-l-5">适用范围</div>
				</template>
				<template #icon="{ isActive }"><span></span></template>
				<div class="base-content">
					<p class="draft">{{ info.scope }}</p>
				</div>
			</el-collapse-item>
		</el-collapse>
		<el-collapse class="m-b-20" v-model="collapse5">
			<el-collapse-item title="Consistency" name="1" class="base-common">
				<template #title="{ isActive }">
					<el-icon v-if="!isActive" style="vertical-align: middle">
						<CaretRight />
					</el-icon>
					<el-icon v-if="isActive" style="vertical-align: middle"><CaretBottom /></el-icon>
					<div class="self-title m-l-5">标准历程</div>
				</template>
				<template #icon="{ isActive }"><span></span></template>
				<div class="base-content base-content-timeline">
					<el-steps style="width: 100%" :active="2" align-center>
						<el-step>
							<template #title>
								<div class="base-step-title">发布于 {{ info.publishDate }}</div>
							</template>
						</el-step>
						<el-step>
							<template #title>
								<div class="base-step-title">实施于 {{ info.implementDate }}</div>
							</template>
						</el-step>
						<el-step title="废止">
							<template #title>
								<div class="base-step-title">废止</div>
							</template>
						</el-step>
					</el-steps>
				</div>
			</el-collapse-item>
		</el-collapse>
		<el-collapse class="m-b-20" v-model="collapse6">
			<el-collapse-item title="Consistency" name="1" class="base-common">
				<template #title="{ isActive }">
					<el-icon v-if="!isActive" style="vertical-align: middle">
						<CaretRight />
					</el-icon>
					<el-icon v-if="isActive" style="vertical-align: middle"><CaretBottom /></el-icon>
					<div class="self-title m-l-5">参考文献</div>
				</template>
				<template #icon="{ isActive }"><span></span></template>
				<div class="base-content base-column">
					<div v-if="info.referenceDocs">
						<div class="reference" v-for="(item, index) in JSON.parse(info.referenceDocs)" :key="index">
							<img class="file-img m-r-8" src="@/assets/image/ai/query/文件.png" alt="" />
							<p class="file-p">{{ item }}</p>
						</div>
					</div>
				</div>
			</el-collapse-item>
		</el-collapse>
	</div>
</template>

<script setup>
import { ref } from 'vue';
import { CaretRight, CaretBottom } from '@element-plus/icons-vue';

const collapse1 = ref(['1']);
const collapse2 = ref(['1']);
const collapse3 = ref(['1']);
const collapse4 = ref(['1']);
const collapse5 = ref(['1']);
const collapse6 = ref(['1']);

const statusOptions = [
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
const props = defineProps({
	info: {
		type: Object,
		required: () => {},
	},
});
const renderStatusText = (state) => {
	return statusOptions.find((item) => item.value === state)?.label || '';
};
</script>

<style lang="less" scoped>
.base-layout {
	:deep(.el-collapse-item__header) {
		background-color: #eff2f7;
		height: 28px;
		line-height: 28px;
		padding: 0 16px;
		border-radius: 10px 10px 0 0;
		border-color: #f0f3f9;
	}
	:deep(.el-collapse-item__content) {
		border: 1px solid #f0f3f9;
		border-radius: 0 0 10px 10px;
	}
	:deep(.el-collapse) {
		--el-collapse-border-color: transparent;
	}
	.base-common {
		:deep(.el-collapse-item__content) {
			padding-bottom: 0;
		}
		:deep(.el-collapse-item__title) {
			display: flex;
			align-items: center;
		}
		.self-title {
			background-color: #f0f3f9;
		}
		.base-content {
			padding: 30px;
			display: flex;
			justify-content: space-between;
			.base-common {
				display: flex;
				flex-direction: column;
				.center {
					display: flex;
					align-items: center;
					.title {
						font-size: 16px;
						color: #6b7280;
					}
					.content {
						font-size: 16px;
						color: #111827;
					}
					.button {
						padding: 3px 9px;
						border-radius: 4px;
						background: #f0f9eb;
						box-sizing: border-box;
						border: 1px solid #e1f3d8;
						font-size: 12px;
						color: #67c23a;
					}
				}
			}
			.draft {
				font-size: 14px;
				color: #606266;
			}
			.file-img {
				width: 14px;
				height: 14px;
				line-height: 14px;
			}
			.file-p {
				font-size: 14px;
				color: #0078e9;
				line-height: 14px;
			}
		}
		.reference {
			display: flex;
			align-items: center;
			margin-bottom: 8px;
		}
		.base-column {
			flex-direction: column;
			align-content: flex-start;
		}
		.base-content-timeline {
			justify-content: center;
		}
	}
}
.base-step-title {
	font-size: 14px;
}
</style>
