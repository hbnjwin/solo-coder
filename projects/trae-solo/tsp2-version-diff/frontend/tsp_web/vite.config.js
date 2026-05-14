import { defineConfig, loadEnv } from 'vite';
import vue from '@vitejs/plugin-vue';
import { resolve } from 'path';
import AutoImport from 'unplugin-auto-import/vite';
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers';

export default defineConfig({
	resolve: {
		alias: {
			'@': resolve(__dirname, './src'),
			'vue-i18n': 'vue-i18n/dist/vue-i18n.cjs.js',
		},
	},

	server: {
		host: 'localhost',
		port: 3000,
		open: true,
		proxy: {
			'/api': {
				changeOrigin: true,
				target: 'http://127.0.0.1:8020',
				rewrite: path => path.replace(/^\/api/, ''),
			},
		},
	},
	base: './',
	build: {
		outDir: 'dist',
		// assetsDir: 'assets',
		sourcemap: false,
		minify: 'terser',
		cssCodeSplit: true,
		chunkSizeWarningLimit: 1500,
		//去除 console debugger
		// terserOptions: {
		// 	compress: {
		// 		// warnings: false,
		// 		drop_console: true,
		// 		drop_debugger: true,
		// 		pure_funcs: ['console.log'],
		// 	}
		// },
		// rollupOptions: {
		// 	output: {
		// 		// 最小化拆分包
		// 		// manualChunks: (id) => {
		// 		// 	if (id.includes('node_modules')) {
		// 		// 		return id.toString().split('node_modules/')[1].split('/')[0].toString();
		// 		// 	}
		// 		// },
		// 		// manualChunks: {
		// 		// 	// 拆分代码，这个就是分包，配置完后自动按需加载，现在还比不上webpack的splitchunk，不过也能用了。
		// 		// 	vue: ['vue', 'vue-router', 'vuex'],
		// 		// 	echarts: ['echarts'],
		// 		// },
		// 		// 用于从入口点创建的块的打包输出格式[name]表示文件名,[hash]表示该文件内容hash值
		// 		entryFileNames: 'js/[name].[hash].js',
		// 		// 用于命名代码拆分时创建的共享块的输出命名
		// 		chunkFileNames: 'js/[name].[hash].js',
		// 		// 用于输出静态资源的命名，[ext]表示文件扩展名
		// 		assetFileNames: 'assets/[name].[hash].[ext]',
		// 		// 拆分js到模块文件夹
		// 		// chunkFileNames: (chunkInfo) => {
		// 		// 	const facadeModuleId = chunkInfo.facadeModuleId ? chunkInfo.facadeModuleId.split('/') : [];
		// 		// 	const fileName = facadeModuleId[facadeModuleId.length - 2] || '[name]';
		// 		// 	return `js/${fileName}/[name].[hash].js`;
		// 		// },
		// 	},
		// },
	},

	plugins: [
		vue(),
		AutoImport({
			imports: ['vue', 'vuex', 'vue-router'],
			resolvers: [ElementPlusResolver()],
			dts: 'auto-import.d.ts',
			include: [
				/\.[tj]sx?$/,
				/\.vue$/,
				/\.vue\?vue/,
				/\.md$/
			],
			eslintrc: {
				enabled: 'false',
				filepath: './.eslintrc-auto-import.json',
				globalsPropValue: true
			}
		})
	],
});
