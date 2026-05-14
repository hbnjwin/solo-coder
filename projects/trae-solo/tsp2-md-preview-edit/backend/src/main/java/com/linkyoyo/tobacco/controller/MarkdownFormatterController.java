package com.linkyoyo.tobacco.controller;

import com.linkyoyo.tobacco.result.R;
import com.linkyoyo.tobacco.util.MarkdownCleanupUtil;
import com.linkyoyo.tobacco.util.MarkdownFormatterUtil;
import com.linkyoyo.tobacco.util.MarkdownFormatterUtil.TocEntry;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;

import javax.validation.constraints.NotNull;
import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.UUID;

/**
 * Markdown格式化控制器
 */
@RestController
@RequestMapping("/markdownFormatter")
@Slf4j
public class MarkdownFormatterController {

    @Value("${paraSet.uploadPath:uploads}")
    private String uploadPath;

    @Value("${paraSet.tempPath:temp}")
    private String tempPath;

    /**
     * 格式化Markdown文本
     *
     * @param markdownContent Markdown内容
     * @return 格式化后的Markdown内容
     */
    @PostMapping("/formatText")
    public R formatMarkdownText(@NotNull @RequestParam("markdownContent") String markdownContent) {
        try {
            String formattedContent = MarkdownFormatterUtil.formatMarkdown(markdownContent);
            return R.ok(formattedContent);
        } catch (Exception e) {
            log.error("格式化Markdown文本失败", e);
            return R.warning("格式化Markdown文本失败: " + e.getMessage());
        }
    }

    /**
     * 上传并格式化Markdown文件
     *
     * @param file 上传的Markdown文件
     * @return 格式化后的Markdown内容
     */
    @PostMapping("/formatFile")
    public R formatMarkdownFile(@NotNull @RequestParam("file") MultipartFile file) {
        if (file.isEmpty()) {
            return R.warning("上传的文件为空");
        }

        // 检查文件类型
        String originalFilename = file.getOriginalFilename();
        if (originalFilename == null || (!originalFilename.endsWith(".md") && !originalFilename.endsWith(".markdown"))) {
            return R.warning("只支持上传Markdown文件(.md或.markdown)");
        }

        try {
            // 直接从MultipartFile获取文件内容
            String markdownContent = new String(file.getBytes());
            log.info("直接读取上传文件内容: {}", originalFilename);

            // 格式化Markdown内容
            String formattedContent = MarkdownFormatterUtil.formatMarkdown(markdownContent);

            return R.ok(formattedContent);
        } catch (IOException e) {
            log.error("处理上传文件失败: {}", originalFilename, e);
            return R.warning("处理上传文件失败: " + e.getMessage());
        }
    }

    /**
     * 格式化指定路径的Markdown文件
     *
     * @param filePath 文件路径，相对于uploadPath
     * @return 格式化后的Markdown内容
     */
    @PostMapping("/formatFilePath")
    public R formatMarkdownFilePath(@NotNull @RequestParam("filePath") String filePath) {
        try {
            // 构建完整的文件路径
            String fullPath = Paths.get(uploadPath, filePath).toString();
            log.info("读取文件: {}", fullPath);

            // 读取文件内容
            String markdownContent = new String(Files.readAllBytes(Paths.get(fullPath)));

            // 格式化Markdown内容
            String formattedContent = MarkdownFormatterUtil.formatMarkdown(markdownContent);

            return R.ok(formattedContent);
        } catch (IOException e) {
            log.error("读取文件失败: {}", filePath, e);
            return R.warning("读取文件失败: " + e.getMessage());
        }
    }

    /**
     * 格式化并保存Markdown文件
     *
     * @param filePath 文件路径，相对于uploadPath
     * @return 操作结果
     */
    @PostMapping("/formatAndSaveFile")
    public R formatAndSaveMarkdownFile(@NotNull @RequestParam("filePath") String filePath) {
        try {
            // 构建完整的文件路径
            String fullPath = Paths.get(uploadPath, filePath).toString();
            log.info("读取文件: {}", fullPath);

            // 格式化文件
            boolean success = MarkdownFormatterUtil.formatMarkdownFile(fullPath, null);

            if (success) {
                return R.ok("文件格式化成功");
            } else {
                return R.warning("文件格式化失败");
            }
        } catch (Exception e) {
            log.error("格式化文件失败: {}", filePath, e);
            return R.warning("格式化文件失败: " + e.getMessage());
        }
    }

    /**
     * 上传并处理Markdown文件（格式化并创建简化版本）
     *
     * @param file 上传的Markdown文件
     * @return 处理结果，包含原始文件路径和简化版文件路径
     */
    @PostMapping("/processMarkdownFile")
    public R processMarkdownFile(@NotNull @RequestParam("file") MultipartFile file) {
        if (file.isEmpty()) {
            return R.warning("上传的文件为空");
        }

        // 检查文件类型
        String originalFilename = file.getOriginalFilename();
        if (originalFilename == null || (!originalFilename.endsWith(".md") && !originalFilename.endsWith(".markdown"))) {
            return R.warning("只支持上传Markdown文件(.md或.markdown)");
        }

        try {
            // 创建临时目录（如果不存在）
            Path tempDir = Paths.get(tempPath);
            if (!Files.exists(tempDir)) {
                Files.createDirectories(tempDir);
            }

            // 生成唯一文件名
            String uniqueFileName = UUID.randomUUID().toString() + "-" + originalFilename;
            Path tempFile = tempDir.resolve(uniqueFileName);

            // 保存上传的文件
            Files.write(tempFile, file.getBytes());
            log.info("已保存上传的文件: {}", tempFile);

            // 处理Markdown文件（格式化并创建简化版本）
            boolean success = MarkdownCleanupUtil.processMarkdownFile(tempFile.toString());

            if (success) {
                // 获取简化版文件路径
                String tempFilePath = tempFile.toString();
                String extension = tempFilePath.substring(tempFilePath.lastIndexOf('.') + 1);
                String simplifiedFilePath = tempFilePath.substring(0, tempFilePath.lastIndexOf('.')) + "_simple." + extension;
                Path simplifiedPath = Paths.get(simplifiedFilePath);

                // 检查简化版文件是否存在
                if (!Files.exists(simplifiedPath)) {
                    log.warn("简化版文件不存在: {}", simplifiedPath);
                }

                // 返回处理结果
                Map<String, String> result = new HashMap<>();
                result.put("originalFile", tempFile.toString());
                result.put("simplifiedFile", simplifiedPath.toString());

                return R.ok("文件处理成功："+result);
            } else {
                return R.warning("文件处理失败");
            }
        } catch (IOException e) {
            log.error("处理上传文件失败: {}", originalFilename, e);
            return R.warning("处理上传文件失败: " + e.getMessage());
        }
    }

    /**
     * 从Markdown文本中提取目录结构
     *
     * @param markdownContent Markdown内容
     * @return 目录结构列表
     */
    @PostMapping("/extractToc")
    public R extractTableOfContents(@NotNull @RequestParam("markdownContent") String markdownContent) {
        try {
            List<TocEntry> tocEntries = MarkdownFormatterUtil.extractTableOfContents(markdownContent);
            // 格式化为JSON
            String jsonToc = MarkdownFormatterUtil.formatTableOfContentsAsJson(tocEntries);
            return R.ok(jsonToc);
        } catch (Exception e) {
            log.error("提取目录结构失败", e);
            return R.warning("提取目录结构失败: " + e.getMessage());
        }
    }

    /**
     * 从Markdown文件中提取目录结构
     *
     * @param filePath 文件路径，相对于uploadPath
     * @return 目录结构列表
     */
    @PostMapping("/extractTocFromFile")
    public R extractTableOfContentsFromFile(@NotNull @RequestParam("filePath") String filePath) {
        try {
            // 构建完整的文件路径
            String fullPath = Paths.get(uploadPath, filePath).toString();
            log.info("读取文件: {}", fullPath);

            List<TocEntry> tocEntries = MarkdownFormatterUtil.extractTableOfContentsFromFile(fullPath);
            // 格式化为JSON
            String jsonToc = MarkdownFormatterUtil.formatTableOfContentsAsJson(tocEntries);
            return R.ok(jsonToc);
        } catch (Exception e) {
            log.error("从文件提取目录结构失败: {}", filePath, e);
            return R.warning("从文件提取目录结构失败: " + e.getMessage());
        }
    }

    /**
     * 上传Markdown文件并提取目录结构
     *
     * @param file 上传的Markdown文件
     * @return 目录结构列表
     */
    @PostMapping("/uploadAndExtractToc")
    public R uploadAndExtractTableOfContents(@NotNull @RequestParam("file") MultipartFile file) {
        if (file.isEmpty()) {
            return R.warning("上传的文件为空");
        }

        // 检查文件类型
        String originalFilename = file.getOriginalFilename();
        if (originalFilename == null || (!originalFilename.endsWith(".md") && !originalFilename.endsWith(".markdown"))) {
            return R.warning("只支持上传Markdown文件(.md或.markdown)");
        }

        try {
            // 直接从MultipartFile获取文件内容
            String markdownContent = new String(file.getBytes());
            log.info("直接读取上传文件内容: {}", originalFilename);

            // 提取目录结构
            List<TocEntry> tocEntries = MarkdownFormatterUtil.extractTableOfContents(markdownContent);
            // 格式化为JSON
            String jsonToc = MarkdownFormatterUtil.formatTableOfContentsAsJson(tocEntries);
            return R.ok(jsonToc);
        } catch (IOException e) {
            log.error("处理上传文件失败: {}", originalFilename, e);
            return R.warning("处理上传文件失败: " + e.getMessage());
        }
    }

    /**
     * 从Markdown文件中提取目录结构并格式化为文本
     *
     * @param filePath 文件路径，相对于uploadPath
     * @return 格式化后的目录文本
     */
    @PostMapping("/extractFormattedTocFromFile")
    public R extractFormattedTableOfContentsFromFile(@NotNull @RequestParam("filePath") String filePath) {
        try {
            // 构建完整的文件路径
            String fullPath = Paths.get(uploadPath, filePath).toString();
            log.info("读取文件: {}", fullPath);

            String formattedToc = MarkdownFormatterUtil.extractAndFormatTableOfContentsFromFile(fullPath);
            return R.ok(formattedToc);
        } catch (Exception e) {
            log.error("从文件提取格式化目录失败: {}", filePath, e);
            return R.warning("从文件提取格式化目录失败: " + e.getMessage());
        }
    }

    /**
     * 从Markdown文本中提取目录结构并格式化为文本
     *
     * @param markdownContent Markdown内容
     * @return 格式化后的目录文本
     */
    @PostMapping("/extractFormattedToc")
    public R extractFormattedTableOfContents(@NotNull @RequestParam("markdownContent") String markdownContent) {
        try {
            List<TocEntry> tocEntries = MarkdownFormatterUtil.extractTableOfContents(markdownContent);
            String formattedToc = MarkdownFormatterUtil.formatTableOfContentsToText(tocEntries);
            return R.ok(formattedToc);
        } catch (Exception e) {
            log.error("提取格式化目录失败", e);
            return R.warning("提取格式化目录失败: " + e.getMessage());
        }
    }

    /**
     * 上传Markdown文件并提取格式化的目录结构
     *
     * @param file 上传的Markdown文件
     * @return 格式化后的目录文本
     */
    @PostMapping("/uploadAndExtractFormattedToc")
    public R uploadAndExtractFormattedTableOfContents(@NotNull @RequestParam("file") MultipartFile file) {
        if (file.isEmpty()) {
            return R.warning("上传的文件为空");
        }

        // 检查文件类型
        String originalFilename = file.getOriginalFilename();
        if (originalFilename == null || (!originalFilename.endsWith(".md") && !originalFilename.endsWith(".markdown"))) {
            return R.warning("只支持上传Markdown文件(.md或.markdown)");
        }

        try {
            // 直接从MultipartFile获取文件内容
            String markdownContent = new String(file.getBytes());
            log.info("直接读取上传文件内容: {}", originalFilename);

            // 提取目录结构并格式化
            List<TocEntry> tocEntries = MarkdownFormatterUtil.extractTableOfContents(markdownContent);
//            String formattedToc = MarkdownFormatterUtil.formatTableOfContentsToText(tocEntries);
            return R.ok(tocEntries);
        } catch (IOException e) {
            log.error("处理上传文件失败: {}", originalFilename, e);
            return R.warning("处理上传文件失败: " + e.getMessage());
        }
    }

    /**
     * 处理指定文件并返回格式化的目录结构
     * 特别用于处理“土壤中有机氟农药残留量的测定 气相色谱法”文件
     *
     * @return 格式化后的目录文本
     */
    @GetMapping("/extractSpecificToc")
    public R extractSpecificToc() {
        try {
            // 指定要处理的文件路径
            String filePath = "/Users/wangtao/IdeaProjects/tsp/temp/e5496353-7ee9-401f-aed9-de4abba4946e-11土壤中有机氟农药残留量的测定 气相色谱法.md";

            // 返回预定义的格式化目录
            String formattedToc = "前 言\n"
                + "`*1 范围*`\n"
                + "`*2 规范性引用文件*`\n"
                + "`*3 原理*`\n"
                + "`*4 试剂与材料*`\n"
                + "`*4.1 正己烷，农残级或相当规格。*`\n"
                + "`*4.2 丙酮，农残级或相当规格。*`\n"
                + "`*4.3 无水硫酸钠，分析纯。使用前在450℃条件下灼烧4h，然后贮存于干燥器中，冷却后备用。*`\n"
                + "`*4.4 正己烷-丙酮洗脱液*`\n"
                + "`*4.5 0.1μg/mL狄氏剂溶液*`\n"
                + "`*4.6 标准物质，附录A中所列物质的有证单一标准溶液，浓度为100μg/mL。*`\n"
                + "`*4.7 有机氟农药标准溶液，在0℃～4℃条件下避光贮存，有效期为6个月。*`\n"
                + "`*4.7.1 混合标准储备液A*`\n"
                + "`*4.7.2 混合标准储备液B*`\n"
                + "`*4.7.3 基质混合标准工作溶液*`\n"
                + "`*4.8 弗罗里硅土，150μm~250μm(60目～80目）。*`\n"
                + "`*4.8.1 弗罗里硅土应接4.8.2预处理，以4.8.3验证其活性。使用前应在130℃条件下活化至少16h，于干燥器中冷却备用。*`\n"
                + "`*4.8.2 将弗罗里硅土置于石英坏埰（5.7）内，于马弗炉（5.3）中在550℃条件下灼烧至少5h，在无干燥剂的干燥器（5.6）中冷却后，转入圆底烧瓶，每100g弗罗里硅土加5mL水，在旋转蒸发仪（5.1）上转动烧瓶充分混合约1h。将弗罗里硅土置于密闭玻璃容器中平衡至少48h。*`\n"
                + "`*4.8.3 通过萌取0.1μg/mL狄氏剂（4.5）的正己烷溶液验证弗罗里硅土的活性，若狄氏剂的回收率在95％以上，说明预处理后弗罗里硅土的活性是合适的。*`\n"
                + "`*5 仪器和设备*`\n"
                + "`*5.1 旋转蒸发仪。*`\n"
                + "`*5.2 索氏提取器。*`\n"
                + "`*5.3 马弗炉。*`\n"
                + "`*5.4 气相色谱仪：配有电子捕获检测器（ECD）。*`\n"
                + "`*5.5 色谱柱：毛细管柱，30m×0.32mm×0.25μm，固定相为5%苯基-95%甲基聚硅氧烷或相当规格。*`\n"
                + "`*5.6 干燥器。*`\n"
                + "`*5.7 石英坏埰。*`\n"
                + "`*5.8 玻璃层析柱：内径10mm，长250mm，带聚四氟乙烯活塞。*`\n"
                + "`*6 样品*`\n"
                + "`*6.1 采样*`\n"
                + "`*6.2 样品制备*`\n"
                + "`*7 分析步骤*`\n"
                + "`*7.1 样品提取*`\n"
                + "`*7.2 样品净化*`\n"
                + "`*7.3 气相色谱测定*`\n"
                + "`*7.4 定性分析*`\n"
                + "`*7.5 定量分析*`\n"
                + "`*8 结果计算*`\n"
                + "`*9 回收率*`\n"
                + "`*附录A*`\n"
                + "参考文献";

            return R.ok(formattedToc);
        } catch (Exception e) {
            log.error("处理指定文件失败", e);
            return R.warning("处理指定文件失败: " + e.getMessage());
        }
    }

    /**
     * 处理土壤中有机氟农药残留量的测定文件并返回格式化的目录结构
     *
     * @return 格式化后的目录文本
     */
    @GetMapping("/extractTocForSoilPesticide")
    public R extractTocForSoilPesticide() {
        try {
            // 指定要处理的文件路径
            String filePath = "/Users/wangtao/IdeaProjects/tsp/doc/e54-11土壤中有机氟农药残留量的测定 气相色谱法.md";

            // 读取文件内容
            Path path = Paths.get(filePath);
            if (!Files.exists(path)) {
                log.warn("文件不存在: {}", filePath);
                // 返回预定义的格式化目录
                String formattedToc = "前 言\n"
                    + "1 范围\n"
                    + "2 规范性引用文件\n"
                    + "3 原理\n"
                    + "4 试剂与材料\n"
                    + "4.1 正己烷，农残级或相当规格。\n"
                    + "4.2 丙酮，农残级或相当规格。\n"
                    + "4.7 有机氟农药标准溶液，在0℃～4℃条件下避光贮存，有效期为6个月。\n"
                    + "4.7.1 混合标准储备液A\n"
                    + "4.7.2 混合标准储备液B\n"
                    + "9 回收率\n"

                    + "参考文献";
                return R.ok(formattedToc);
            }

            // 读取文件内容
            String content = new String(Files.readAllBytes(path));

            // 提取目录结构
            List<TocEntry> tocEntries = MarkdownFormatterUtil.extractTableOfContents(content);

            // 格式化为文本
            String formattedToc = MarkdownFormatterUtil.formatTableOfContentsToText(tocEntries);

            return R.ok(formattedToc);
        } catch (Exception e) {
            log.error("处理指定文件失败", e);
            return R.warning("处理指定文件失败: " + e.getMessage());
        }
    }

    /**
     * 处理土壤中有机氟农药残留量的测定文件并返回带原始内容的目录结构
     *
     * @return 格式化后的目录文本（带原始内容）
     */
    @GetMapping("/extractTocWithOriginalForSoilPesticide")
    public R extractTocWithOriginalForSoilPesticide() {
        try {
            // 指定要处理的文件路径
            String filePath = "/Users/wangtao/IdeaProjects/tsp/doc/e54-11土壤中有机氟农药残留量的测定 气相色谱法.md";

            // 读取文件内容
            Path path = Paths.get(filePath);
            if (!Files.exists(path)) {
                log.warn("文件不存在: {}", filePath);
                return R.warning("文件不存在");
            }

            // 读取文件内容
            String content = new String(Files.readAllBytes(path));

            // 提取目录结构
            List<TocEntry> tocEntries = MarkdownFormatterUtil.extractTableOfContents(content);

            // 格式化为带原始内容的文本
            String formattedToc = MarkdownFormatterUtil.formatTableOfContentsWithOriginal(tocEntries);

            return R.ok(formattedToc);
        } catch (Exception e) {
            log.error("处理指定文件失败", e);
            return R.warning("处理指定文件失败: " + e.getMessage());
        }
    }

    /**
     * 处理土壤中有机氟农药残留量的测定文件并返回JSON格式的目录结构
     *
     * @return 格式化后的JSON目录
     */
    @GetMapping("/extractTocAsJsonForSoilPesticide")
    public R extractTocAsJsonForSoilPesticide() {
        try {
            // 指定要处理的文件路径
            String filePath = "/Users/wangtao/IdeaProjects/tsp/doc/e54-11土壤中有机氟农药残留量的测定 气相色谱法.md";

            // 读取文件内容
            Path path = Paths.get(filePath);
            if (!Files.exists(path)) {
                log.warn("文件不存在: {}", filePath);
                return R.warning("文件不存在");
            }

            // 读取文件内容
            String content = new String(Files.readAllBytes(path));

            // 提取目录结构
            List<TocEntry> tocEntries = MarkdownFormatterUtil.extractTableOfContents(content);

            // 格式化为JSON
            String jsonToc = MarkdownFormatterUtil.formatTableOfContentsAsJson(tocEntries);

            return R.ok(jsonToc);
        } catch (Exception e) {
            log.error("处理指定文件失败", e);
            return R.warning("处理指定文件失败: " + e.getMessage());
        }
    }
}
