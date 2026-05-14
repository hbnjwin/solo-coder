package com.linkyoyo.tobacco.controller;

import com.linkyoyo.tobacco.result.R;
import com.linkyoyo.tobacco.entity.StandardDocToc;
import com.linkyoyo.tobacco.service.StandardDocTocService;
import com.linkyoyo.tobacco.util.TocParserUtil;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

import javax.validation.constraints.NotNull;

/**
 * 标准文档目次控制器
 */
@RestController
@RequestMapping("/standardDocToc")
@Slf4j
@Validated
public class StandardDocTocController {

    @Autowired
    private StandardDocTocService standardDocTocService;

    @Value("${paraSet.uploadPath:uploads}")
    private String uploadPath;

    @Value("${paraSet.tempPath:temp}")
    private String tempPath;

    /**
     * 获取标准文档的目次
     * @param standardDocId 标准文档ID
     * @return 目次列表
     */
    @GetMapping("/list")
    public R getTableOfContents(@NotNull @RequestParam("standardDocId") Integer standardDocId) {
        return R.ok(standardDocTocService.getTableOfContentsByStandardDocId(standardDocId));
    }

    /**
     * 解析并保存目次信息
     * @param standardDocId 标准文档ID
     * @param tocText 目次文本
     * @return 保存的记录数
     */
    @PostMapping("/parse")
    public R parseAndSaveTableOfContents(
            @NotNull @RequestParam("standardDocId") Integer standardDocId,
            @NotNull @RequestParam("tocText") String tocText) {
        // 使用工具类解析目次文本
        List<StandardDocToc> tocEntries = TocParserUtil.parseToStandardDocToc(standardDocId, tocText);
        // 批量保存目次条目
        Integer count = standardDocTocService.batchSaveTableOfContents(standardDocId, tocEntries);
        return R.ok(count);
    }

    /**
     * 从标准文档Markdown内容中提取目次并保存
     * @param standardDocId 标准文档ID
     * @param markdownContent Markdown内容
     * @return 保存的记录数
     */
    @PostMapping("/parseFromMarkdown")
    public R parseAndSaveTableOfContentsFromMarkdown(
            @NotNull @RequestParam("standardDocId") Integer standardDocId,
            @NotNull @RequestParam("markdownContent") String markdownContent) {
        // 从 Markdown 内容中提取目次部分
        String tocText = TocParserUtil.extractTocFromMarkdown(markdownContent);

        // 如果没有找到目次，返回空结果
        if (tocText == null || tocText.isEmpty()) {
            log.info("标准文档ID: {} 中未找到目次部分", standardDocId);
            return R.ok(0);
        }

        // 解析目次文本
        List<StandardDocToc> tocEntries = TocParserUtil.parseToStandardDocToc(standardDocId, tocText);

        // 如果解析出的目次条目为空，返回空结果
        if (tocEntries.isEmpty()) {
            log.info("标准文档ID: {} 的目次解析结果为空", standardDocId);
            return R.ok(0);
        }

        // 批量保存目次条目
        Integer count = standardDocTocService.batchSaveTableOfContents(standardDocId, tocEntries);
        return R.ok(count);
    }

    /**
     * 添加目次条目
     * @param standardDocToc 目次条目
     * @return 保存的目次条目
     */
    @PostMapping("/add")
    public R addTableOfContentsEntry(@RequestBody StandardDocToc standardDocToc) {
        return R.ok(standardDocTocService.addTableOfContentsEntry(standardDocToc));
    }

    /**
     * 更新目次条目
     * @param standardDocToc 目次条目
     * @return 更新后的目次条目
     */
    @PostMapping("/update")
    public R updateTableOfContentsEntry(@RequestBody StandardDocToc standardDocToc) {
        return R.ok(standardDocTocService.updateTableOfContentsEntry(standardDocToc));
    }

    /**
     * 删除目次条目
     * @param id 目次条目ID
     * @return 操作结果
     */
    @DeleteMapping("/delete")
    public R deleteTableOfContentsEntry(@NotNull @RequestParam("id") Integer id) {
        standardDocTocService.deleteTableOfContentsEntry(id);
        return R.ok();
    }

    /**
     * 删除标准文档的所有目次
     * @param standardDocId 标准文档ID
     * @return 操作结果
     */
    @DeleteMapping("/deleteAll")
    public R deleteAllTableOfContents(@NotNull @RequestParam("standardDocId") Integer standardDocId) {
        standardDocTocService.deleteAllTableOfContentsByStandardDocId(standardDocId);
        return R.ok();
    }

    /**
     * 从文件路径中读取Markdown文件并提取目次
     * @param standardDocId 标准文档ID
     * @param filePath 文件路径，相对于uploadPath
     * @return 保存的记录数
     */
    @PostMapping("/parseFromFile")
    public R parseAndSaveTableOfContentsFromFile(
            @NotNull @RequestParam("standardDocId") Integer standardDocId,
            @NotNull @RequestParam("filePath") String filePath) {
        try {
            // 构建完整的文件路径
            String fullPath = Paths.get(uploadPath, filePath).toString();
            log.info("读取文件: {}", fullPath);

            // 读取文件内容
            String markdownContent = new String(Files.readAllBytes(Paths.get(fullPath)));

            // 从 Markdown 内容中提取目次部分
            String tocText = TocParserUtil.extractTocFromMarkdown(markdownContent);

            // 如果没有找到目次，返回空结果
            if (tocText == null || tocText.isEmpty()) {
                log.info("文件: {} 中未找到目次部分", filePath);
                return R.ok(0);
            }

            // 解析目次文本
            List<StandardDocToc> tocEntries = TocParserUtil.parseToStandardDocToc(standardDocId, tocText);

            // 如果解析出的目次条目为空，返回空结果
            if (tocEntries.isEmpty()) {
                log.info("文件: {} 的目次解析结果为空", filePath);
                return R.ok(0);
            }

            // 批量保存目次条目
            Integer count = standardDocTocService.batchSaveTableOfContents(standardDocId, tocEntries);
            return R.ok(count);
        } catch (IOException e) {
            log.error("读取文件失败: {}", filePath, e);
            return R.warning("读取文件失败: " + e.getMessage());
        }
    }

    /**
     * 上传Markdown文件并解析目次
     * @param standardDocId 标准文档ID
     * @param file 上传的Markdown文件
     * @return 保存的记录数
     */
    @PostMapping("/uploadAndParse")
    public R uploadAndParseMarkdownFile(
            @NotNull @RequestParam("standardDocId") Integer standardDocId,
            @NotNull @RequestParam("file") MultipartFile file) {
        if (file.isEmpty()) {
            return R.warning("上传的文件为空");
        }

        // 检查文件类型
        String originalFilename = file.getOriginalFilename();
        if (originalFilename == null || (!originalFilename.endsWith(".md") && !originalFilename.endsWith(".markdown"))) {
            return R.warning("只支持上传Markdown文件(.md或.markdown)");
        }

        try {
            // 直接从 MultipartFile 获取文件内容
            String markdownContent = new String(file.getBytes());
            log.info("直接读取上传文件内容: {}", originalFilename);

            // 从 Markdown 内容中提取目次部分
            String tocText = TocParserUtil.extractTocFromMarkdown(markdownContent);

            // 如果没有找到目次，返回空结果
            if (tocText == null || tocText.isEmpty()) {
                log.info("文件: {} 中未找到目次部分", originalFilename);

                return R.ok(0);
            }

            // 解析目次文本
            List<StandardDocToc> tocEntries = TocParserUtil.parseToStandardDocToc(standardDocId, tocText);

            // 如果解析出的目次条目为空，返回空结果
            if (tocEntries.isEmpty()) {
                log.info("文件: {} 的目次解析结果为空", originalFilename);

                return R.ok(0);
            }

            // 批量保存目次条目
            Integer count = standardDocTocService.batchSaveTableOfContents(standardDocId, tocEntries);
            return R.ok(count);
        } catch (IOException e) {
            log.error("处理上传文件失败: {}", originalFilename, e);
            return R.warning("处理上传文件失败: " + e.getMessage());
        }
    }
}
