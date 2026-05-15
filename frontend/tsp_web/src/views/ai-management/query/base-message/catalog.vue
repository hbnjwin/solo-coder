<template>
	<div class="base-layout">
		<div class="log-left">
			<el-tree style="width: 100%" :data="treeData" @node-click="handleClickNode" />
		</div>
		<div class="log-right" ref="markdownContainer" v-html="md.render(content)"></div>
	</div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import MarkdownIt from 'markdown-it';
import mdKatex from '@traptitech/markdown-it-katex';

const props = defineProps({
	info: {
		type: Object,
		required: () => {},
	},
	content: {
		type: String,
		required: '',
	},
});

const markdownContainer = ref(null);

// 初始化 MarkdownIt 实例
const md = new MarkdownIt({
	html: true, // 允许 HTML 标签
	breaks: true, // 转换换行符为 <br>
	xhtmlOut: true, // 使用 XHTML 语法
}).use(mdKatex); // 使用 @traptitech/markdown-it-katex 插件

// 自定义标题的 ID
md.renderer.rules.heading_open = (tokens, idx) => {
	const title = tokens[idx + 1].content;
	const id = title.split(' ').join('-');
	return `<${tokens[idx].tag} id="section-${id}">`;
};

const treeData = ref([]);

onMounted(() => {
	// 使用示例
	const tree = convertToTree(props.info);
	treeData.value = tree;
});

const convertToTree = (tocList) => {
	if (!Array.isArray(tocList) || tocList.length === 0) {
		return [];
	}

	const tree = [];
	// 存储各层级路径对应的最后一个节点（用于快速查找父节点）
	const pathMap = new Map();
	// 存储无sectionNumber的特殊节点（如前言、引言）
	const specialNodes = [];

	tocList.forEach((item) => {
		const { sectionNumber, ...node } = item;
		const newNode = { ...node, label: node.sectionTitle, children: [] };

		// 处理无sectionNumber的特殊节点（前言、引言等）
		if (!sectionNumber || sectionNumber.trim() === '') {
			specialNodes.push(newNode);
			return;
		}

		// 解析sectionNumber的层级路径
		const path = sectionNumber.split('.');
		const level = path.length;
		const pathKey = path.join('.');

		// 根节点（如"1", "2"等一级标题）
		if (level === 1) {
			tree.push(newNode);
			pathMap.set(pathKey, newNode);
			return;
		}

		// 构建父级路径（如"5.3.1"的父级路径为"5.3"）
		const parentPath = path.slice(0, level - 1).join('.');
		const parentNode = pathMap.get(parentPath);

		if (parentNode) {
			parentNode.children.push(newNode);
			pathMap.set(pathKey, newNode);
		} else {
			console.warn(`未找到父节点: ${sectionNumber}, 父路径: ${parentPath}`);
			// 若父节点不存在，作为根节点处理（可选）
			tree.push(newNode);
			pathMap.set(pathKey, newNode);
		}
	});

	// 将特殊节点添加到树的最前方
	return [...specialNodes, ...tree];
};

const handleClickNode = (node) => {
	const id = `section-${node.label.split(' ').join('-')}`;
	const escapedId = CSS.escape(id);
	const element = markdownContainer.value.querySelector(`#${escapedId}`);
	if (element) {
		element.scrollIntoView({ behavior: 'smooth' });
	}
};
</script>

<style lang="less" scoped>
.base-layout {
	display: flex;
	.log-left {
		height: 100%;
		overflow-y: auto;
		width: 30%;
	}
	.log-right {
		overflow-y: auto;
		width: 100%;
		padding: 24px;
		border: 1px solid #f3f4f6;
		:deep(h1) {
			font-size: 24px;
			color: #333;
			line-height: 40px;
		}
		:deep(h2) {
			font-size: 20px;
			line-height: 34px;
		}
		:deep(h3) {
			font-size: 16px;
			line-height: 30px;
		}
		:deep(h4) {
			font-size: 14px;
			line-height: 30px;
		}
		:deep(p){
			font-size: 14px;
			line-height: 30px;
		}
		:deep(img){
			width: 340px;
		}
		:deep(table) {
			display: block;
			margin: 1rem 0;
			overflow-x: auto;
			border-collapse: collapse;
		}
		:deep(thead){
			background-color: #f8f8f8;
			border-right: 1px solid #dfe2e5;
			th{
				border-right: 1px solid #dfe2e5;
				line-height: 26px;
				padding: 0.6em 1em;
				font-weight: 600;
			}
			th:first-child{
				border-left: 1px solid #dfe2e5;
			}
		}
		:deep(tr) {
			font-size: 14px;
			border-top: 1px solid #dfe2e5;
		}
		:deep(td, th) {
			padding: 0.6em 1em;
			border: 1px solid #dfe2e5;
		}
		:deep(tr:nth-child(2n)) {
			background-color: #f6f8fa;
		}
		
		:deep(tr:nth-child(even)) {
			background-color: #f9f9f9;
		}
		
		:deep(tr:hover) {
			background-color: #f1f1f1;
		}
		:deep(pre){
			position: relative;
			margin: 0.85rem 0;
			padding: 1.25rem 1.5rem;
			overflow: auto;
			line-height: 1.4;
			background-color: #1b1f230d;
			border-radius: 6px;
			&:hover {
				.copy-button {
					visibility: visible;
				}
			}
			code {
				background-color: transparent;
			}
		}
	}
}
</style>